use crate::error::{GpibError, IbError};
use crate::lowlevel::multidevice;
use crate::lowlevel::traditional::{ibdev, ibonl, ibrd, ibwait, ibwrt};
use crate::lowlevel::utility::{Addr4882, AsyncIbcntl};
use crate::status::IbStatus;
use crate::types::{IbEosMode, IbOnline, IbSendEOI, IbTimeout, PrimaryAddress, SecondaryAddress};
use std::default::Default;
use std::fmt;
use std::os::raw::{c_int, c_void};

pub struct Parameters {
    pub timeout: IbTimeout,
    pub send_eoi: IbSendEOI,
    pub eos_mode: IbEosMode,
}

impl Default for Parameters {
    fn default() -> Self {
        Self {
            timeout: IbTimeout::T1s,
            send_eoi: IbSendEOI::default(),
            eos_mode: IbEosMode::default(),
        }
    }
}

#[derive(Clone)]
pub struct Board {
    board_number: c_int,
}

#[derive(Clone)]
pub struct Instrument {
    board: Board,
    addr: Addr4882,
}

pub struct InstrumentHandle {
    ud: c_int,
}

impl Board {
    pub fn with_board_number(board_number: c_int) -> Self {
        Board { board_number }
    }

    /// clear devices
    pub fn clear_devices(&self, address_list: &Vec<Addr4882>) -> Result<(), GpibError> {
        multidevice::DevClearList(self.board_number, address_list)
    }

    /// perform interface clear.
    /// The interface clear causes all devices to untalk and unlisten, puts them into serial poll disabled state
    /// (don't worry, you will still be able to conduct serial polls), and the board becomes controller-in-charge.
    pub fn interface_clear(&self) -> Result<(), GpibError> {
        multidevice::SendIFC(self.board_number)
    }

    /// find listeners on the board
    pub fn find_listeners(&self) -> Result<Vec<Instrument>, GpibError> {
        Ok(multidevice::FindAllLstn(self.board_number)?
            .into_iter()
            .map(|addr| Instrument {
                board: self.clone(),
                addr: addr,
            })
            .collect())
    }

    /// write data to multiple devices
    pub fn send_list(
        &self,
        address_list: &Vec<Addr4882>,
        data: &[u8],
        mode: IbSendEOI,
    ) -> Result<(), GpibError> {
        multidevice::SendList(self.board_number, address_list, data, mode)
    }
}

impl Default for Board {
    fn default() -> Self {
        Board { board_number: 0 }
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Board({})", self.board_number)
    }
}

impl fmt::Debug for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Board({})", self.board_number)
    }
}

impl Instrument {
    pub fn from_visa_string(address: &str) -> Result<Self, GpibError> {
        let v: Vec<&str> = address.split("::").collect();
        if v.len() < 2 {
            return Err(GpibError::ValueError(format!(
                "Invalid address '{}'.",
                address
            )));
        }
        if v[0].starts_with("GPIB") {
            let (_, board_number) = v[0].split_at(4);
            let board_number = i32::from_str_radix(board_number, 10).map_err(|e| {
                GpibError::ValueError(format!(
                    "Unable to parse GPIB Board index from string '{}' ({:?})",
                    board_number, e,
                ))
            })?;
            let primary_address = i32::from_str_radix(v[1], 10).map_err(|e| {
                GpibError::ValueError(format!(
                    "Unable to parse GPIB primary address from string '{}' ({:?})",
                    v[1], e,
                ))
            })?;
            Ok(Self {
                board: Board { board_number },
                addr: Addr4882::new(
                    PrimaryAddress::new(primary_address)?,
                    SecondaryAddress::default(),
                )?,
            })
        } else {
            Err(GpibError::ValueError(
                "Address is expected as GPIBN::primary_address::INSTR".to_owned(),
            ))
        }
    }

    pub fn visa_string(&self) -> String {
        format!(
            "GPIB{}::{}::INSTR",
            self.board.board_number,
            self.addr.pad(),
        )
    }

    pub fn open(&self, params: Parameters) -> Result<InstrumentHandle, GpibError> {
        let ud = ibdev(
            self.board.board_number,
            self.addr.primary_address()?,
            self.addr.secondary_address()?,
            params.timeout,
            params.send_eoi,
            params.eos_mode,
        )?;
        Ok(InstrumentHandle { ud })
    }
}

impl fmt::Display for Instrument {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.visa_string())
    }
}

impl fmt::Debug for Instrument {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Instrument({:?}, {:?})", self.board, self.addr)
    }
}

impl InstrumentHandle {
    pub fn blocking_read(&self) -> Result<String, GpibError> {
        const BUFFER_SIZE: usize = 1024;
        let mut result: Vec<u8> = Vec::new();
        loop {
            let mut buffer: [u8; BUFFER_SIZE] = [0; BUFFER_SIZE];
            let (status, n_read) = ibrd(self.ud, &mut buffer)?;
            if n_read > 0 {
                result.extend(buffer[0..n_read].to_vec());
            }
            if status.end || n_read < BUFFER_SIZE || n_read == 0 {
                break;
            }
        }
        let answer = String::from_utf8(result)?;
        Ok(answer)
    }

    #[cfg(feature = "async-tokio")]
    pub async fn read(&self) -> Result<String, GpibError> {
        const BUFFER_SIZE: usize = 1024;
        let mut result: Vec<u8> = Vec::new();
        loop {
            let mut buffer: [u8; BUFFER_SIZE] = [0; BUFFER_SIZE];
            let status = IbStatus::from_ibsta(unsafe {
                linux_gpib_sys::ibrda(
                    self.ud,
                    buffer.as_mut_ptr() as *mut c_void,
                    buffer.len().try_into()?,
                )
            });
            if status.err {
                return Err(GpibError::DriverError(
                    status,
                    IbError::current_thread_local_error()?,
                ));
            }
            let status = ibwait(
                self.ud,
                IbStatus::default()
                    .with_timo(true)
                    .with_cmpl(true)
                    .with_end(true),
            )
            .await?;
            if status.timo {
                return Err(GpibError::Timeout);
            }
            let n_read: usize = AsyncIbcntl().try_into()?;
            if n_read > 0 {
                result.extend(buffer[0..n_read].to_vec());
            }
            if status.end || n_read < BUFFER_SIZE || n_read == 0 {
                break;
            }
        }
        let answer = String::from_utf8(result)?;
        Ok(answer)
    }

    pub fn blocking_write(&self, data: &str) -> Result<(), GpibError> {
        let _n_written = ibwrt(self.ud, data.as_bytes())?;
        Ok(())
    }

    #[cfg(feature = "async-tokio")]
    pub async fn write(&self, data: &str) -> Result<(), GpibError> {
        let data = data.as_bytes();
        let status = IbStatus::from_ibsta(unsafe {
            linux_gpib_sys::ibwrta(
                self.ud,
                data.as_ptr() as *const c_void,
                data.len().try_into()?,
            )
        });
        if status.err {
            return Err(GpibError::DriverError(
                status,
                IbError::current_thread_local_error()?,
            ));
        }
        let status = ibwait(
            self.ud,
            IbStatus::default()
                .with_timo(true)
                .with_cmpl(true)
                .with_end(true)
                .with_rqs(true),
        )
        .await?;
        if status.timo {
            Err(GpibError::Timeout)
        } else if status.cmpl || status.end {
            Ok(())
        } else {
            Err(GpibError::ValueError(format!(
                "Unexpected status after waiting: {:?}",
                status
            )))
        }
    }

    pub fn blocking_query(&self, data: &str) -> Result<String, GpibError> {
        self.blocking_write(data)?;
        self.blocking_read()
    }

    #[cfg(feature = "async-tokio")]
    pub async fn query(&self, data: &str) -> Result<String, GpibError> {
        self.write(data).await?;
        self.read().await
    }
}

impl Drop for InstrumentHandle {
    fn drop(&mut self) {
        match ibonl(self.ud, IbOnline::Close) {
            Ok(()) => {}
            Err(e) => {
                println!("Error while closing (ud = {}): {:?}", self.ud, e);
            }
        }
    }
}

impl fmt::Display for InstrumentHandle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.ud)
    }
}

impl fmt::Debug for InstrumentHandle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "InstrumentHandle({})", self.ud)
    }
}
