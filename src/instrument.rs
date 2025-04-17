use crate::error::{GpibError, IbError};
use crate::lowlevel::multidevice;
use crate::lowlevel::traditional::{ibclr, ibdev, ibonl, ibrd, ibrda, ibwait, ibwrt, ibwrta};
use crate::lowlevel::utility::Addr4882;
use crate::status::IbStatus;
use crate::types::{IbEosMode, IbOnline, IbSendEOI, IbTimeout, PrimaryAddress, SecondaryAddress};
use crate::DEBUG;
use std::default::Default;
use std::fmt;
use std::os::raw::c_int;

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

#[derive(Clone, PartialEq)]
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
        Board {
            board_number: board_number,
        }
    }

    /// clear devices
    pub fn clear_devices(&self, instruments: &Vec<Instrument>) -> Result<(), GpibError> {
        if instruments
            .iter()
            .any(|instr| instr.board.board_number != self.board_number)
        {
            return Err(GpibError::ValueError(
                "clear_devices can only send to devices belonging to this board.".to_owned(),
            ));
        }
        let address_list = instruments.iter().map(|instr| instr.addr).collect();
        multidevice::DevClearList(self.board_number, &address_list)
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
        instruments: &Vec<Instrument>,
        data: &[u8],
        mode: IbSendEOI,
    ) -> Result<(), GpibError> {
        if instruments
            .iter()
            .any(|instr| instr.board.board_number != self.board_number)
        {
            return Err(GpibError::ValueError(
                "clear_devices can only send to devices belonging to this board.".to_owned(),
            ));
        }
        let address_list = instruments.iter().map(|instr| instr.addr).collect();
        multidevice::SendList(self.board_number, &address_list, data, mode)
    }
}

impl Default for Board {
    fn default() -> Self {
        Board::with_board_number(0)
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
    /// Send data to the instrument with the multidevice 488.2 API
    pub fn send(&self, data: &[u8], mode: IbSendEOI) -> Result<(), GpibError> {
        multidevice::Send(self.board.board_number, self.addr, data, mode)
    }

    /// Receive data from the instrument with the multidevice 488.2 API
    pub fn receive(&self) -> Result<String, GpibError> {
        const BUFFER_SIZE: usize = 1024;
        let mut result: Vec<u8> = Vec::new();
        loop {
            let mut buffer: [u8; BUFFER_SIZE] = [0; BUFFER_SIZE];
            let (status, n_read) = multidevice::Receive(
                self.board.board_number,
                self.addr,
                &mut buffer,
                linux_gpib_sys::STOPend,
            )?;
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

    /// Performs send and receive
    pub fn query(&self, data: &str) -> Result<String, GpibError> {
        self.send(data.as_bytes(), IbSendEOI::default())?;
        self.receive()
    }

    /// Create Instrument from a VISA string
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
                board: Board::with_board_number(board_number),
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

    /// Create VISA string from board and address
    pub fn visa_string(&self) -> String {
        format!(
            "GPIB{}::{}::INSTR",
            self.board.board_number,
            self.addr.pad(),
        )
    }

    /// Open with the traditional 488.1 API
    pub fn open(&self, params: Parameters) -> Result<InstrumentHandle, GpibError> {
        let ud = ibdev(
            self.board.board_number,
            self.addr.primary_address()?,
            self.addr.secondary_address()?,
            params.timeout,
            params.send_eoi,
            params.eos_mode,
        )?;
        ibclr(ud)?;
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
            unsafe { ibrda(self.ud, &mut buffer) }?;
            let (status, n_read) = ibwait(
                self.ud,
                IbStatus::default()
                    .with_timo(true)
                    .with_cmpl(true)
                    .with_end(true),
            )
            .await?;
            if status.err {
                return Err(GpibError::DriverError(
                    status,
                    IbError::current_thread_local_error()?,
                ));
            } else if status.timo {
                return Err(GpibError::Timeout);
            }
            if DEBUG {
                println!("read({}) -> {} bytes read.", self.ud, n_read);
            }
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
        unsafe { ibwrta(self.ud, data) }?;
        let (status, _count) = ibwait(
            self.ud,
            IbStatus::default()
                .with_timo(true)
                .with_cmpl(true)
                .with_end(true)
                .with_rqs(true),
        )
        .await?;
        if status.err {
            Err(GpibError::DriverError(
                status,
                IbError::current_thread_local_error()?,
            ))
        } else if status.timo {
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
