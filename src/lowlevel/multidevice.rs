#![allow(non_snake_case)]

use crate::error::{GpibError, IbError};
use crate::lowlevel::utility::{Addr4882, ThreadIbcnt, ThreadIbcntl};
use crate::status::IbStatus;
use crate::types::{IbSendEOI, PrimaryAddress, SecondaryAddress};
use linux_gpib_sys::Addr4882_t;
use std::default::Default;
use std::os::raw::{c_int, c_short, c_void};

/// find devices
///
/// FindLstn() will check the primary addresses in the padList array for devices. The GPIB addresses of all devices found will be stored in the resultList array, and ibcnt will be set to the number of devices found. The maxNumResults parameter limits the maximum number of results that will be returned, and is usually set to the number of elements in the resultList array. If more than maxNumResults devices are found, an ETAB error is returned in iberr. The padList should consist of primary addresses only, with no secondary addresses (all possible secondary addresses will be checked as necessary).
///
/// Your GPIB board must have the capability to monitor the NDAC bus line in order to use this function (see iblines).
///
/// This function has the additional effect of addressing the board as talker for the duration of the Find Listeners protocol, which is beyond what IEEE 488.2 specifies. This is done because some boards cannot reliably read the state of the NDAC bus line unless they are the talker. Being the talker causes the board's gpib transceiver to configure NDAC as an input, so its state can be reliably read from the bus through the transceiver.
///
/// See: [Linux GPIB Reference](https://linux-gpib.sourceforge.io/doc_html/reference-function-findlstn.html)
///
pub fn FindLstn(board_desc: c_int, padList: Vec<Addr4882>) -> Result<Vec<Addr4882>, GpibError> {
    let mut result: Vec<Addr4882_t> = Vec::new();
    result.resize_with(31, || linux_gpib_sys::NOADDR);
    let mut padList = padList
        .into_iter()
        .map(|a| a.addr)
        .collect::<Vec<Addr4882_t>>();
    padList.push(linux_gpib_sys::NOADDR);
    unsafe {
        linux_gpib_sys::FindLstn(
            board_desc,
            padList.as_ptr(),
            result.as_mut_ptr(),
            padList.len().try_into()?,
        )
    };
    let status = IbStatus::current_thread_local_status();
    if status.err {
        let error = IbError::current_thread_local_error()?;
        match error {
            IbError::EARG => {
                eprintln!(
                    "Invalid primary address at index {} in padlist",
                    ThreadIbcnt()
                );
            }
            IbError::EBUS => {
                eprintln!("No devices are connected to the GPIB.");
            }
            IbError::ETAB => {
                eprintln!("The number of devices found on the GPIB exceed limit.");
            }
            _ => {}
        }
        Err(GpibError::DriverError(status, error))
    } else {
        let n_values: usize = ThreadIbcntl().try_into()?;
        result.truncate(n_values);
        Ok(result.into_iter().map(|a| Addr4882 { addr: a }).collect())
    }
}

/// Find all listeners on board.
pub fn FindAllLstn(board_desc: c_int) -> Result<Vec<Addr4882>, GpibError> {
    let padList = (1..31)
        .into_iter()
        .map(|pad| Addr4882::new(PrimaryAddress::new(pad)?, SecondaryAddress::default()))
        .collect::<Result<Vec<Addr4882>, GpibError>>()?;
    FindLstn(board_desc, padList)
}

/// clear a device
///
/// DevClear() causes the interface board specified by board_desc to send the clear command to the GPIB addresses specified by address. The results of the serial polls are stored into resultList. If you wish to clear multiple devices simultaneously, use DevClearList()
pub fn DevClear(board: c_int, address: Addr4882) -> Result<(), GpibError> {
    unsafe {
        linux_gpib_sys::DevClear(board, address.addr);
    }
    let status = IbStatus::current_thread_local_status();
    if status.err {
        Err(GpibError::DriverError(
            status,
            IbError::current_thread_local_error()?,
        ))
    } else {
        Ok(())
    }
}

/// clear multiple devices
///
/// DevClear() causes the interface board specified by board_desc to send the clear command simultaneously to all the GPIB addresses specified by the addressList array. If addressList is empty or NULL, then the clear command is sent to all devices on the bus. If you only wish to clear a single device, DevClear() or ibclr() may be slightly more convenient.
pub fn DevClearList(board: c_int, addresses: &Vec<Addr4882>) -> Result<(), GpibError> {
    let mut instruments = addresses
        .iter()
        .map(|a| a.addr)
        .collect::<Vec<Addr4882_t>>();
    instruments.push(linux_gpib_sys::NOADDR);
    unsafe {
        linux_gpib_sys::DevClearList(board, instruments.as_ptr());
    }
    let status = IbStatus::current_thread_local_status();
    if status.err {
        Err(GpibError::DriverError(
            status,
            IbError::current_thread_local_error()?,
        ))
    } else {
        Ok(())
    }
}

/// put devices into local mode.
///
/// 	EnableLocal() addresses all of the devices in the addressList array as listeners then sends the GTL (go to local) command byte, causing them to enter local mode. This requires that the board is the controller-in-charge. Note that while the REN (remote enable) bus line is asserted, the devices will return to remote mode the next time they are addressed.
///
///If addressList is empty or NULL, then the REN line is unasserted and all devices enter local mode. The board must be system controller to change the state of the REN line.
///
/// See: [Linux GPIB Reference](https://linux-gpib.sourceforge.io/doc_html/reference-function-enablelocal.html)
pub fn EnableLocal(board: c_int, addresses: &Vec<Addr4882>) -> Result<(), GpibError> {
    let mut instruments = addresses
        .iter()
        .map(|a| a.addr)
        .collect::<Vec<Addr4882_t>>();
    instruments.push(linux_gpib_sys::NOADDR);
    unsafe {
        linux_gpib_sys::EnableLocal(board, instruments.as_ptr());
    }
    let status = IbStatus::current_thread_local_status();
    if status.err {
        Err(GpibError::DriverError(
            status,
            IbError::current_thread_local_error()?,
        ))
    } else {
        Ok(())
    }
}

/// put devices into remote mode.
///
/// EnableRemote() asserts the REN (remote enable) line, and addresses all of the devices in the addressList array as listeners (causing them to enter remote mode). The board must be system controller.
///
/// See: [Linux GPIB Reference](https://linux-gpib.sourceforge.io/doc_html/reference-function-enableremote.html)
pub fn EnableRemote(board: c_int, addresses: &Vec<Addr4882>) -> Result<(), GpibError> {
    let mut instruments = addresses
        .iter()
        .map(|a| a.addr)
        .collect::<Vec<Addr4882_t>>();
    instruments.push(linux_gpib_sys::NOADDR);
    unsafe {
        linux_gpib_sys::EnableRemote(board, instruments.as_ptr());
    }
    let status = IbStatus::current_thread_local_status();
    if status.err {
        Err(GpibError::DriverError(
            status,
            IbError::current_thread_local_error()?,
        ))
    } else {
        Ok(())
    }
}

/// find device requesting service and read its status byte
///
/// FindRQS will serial poll the GPIB addresses specified in the addressList array until it finds a device requesting service. The status byte of the device requesting service and its address are returned. If no device requesting service is found, an ETAB error is returned.
///
/// See: [Linux GPIB Reference](https://linux-gpib.sourceforge.io/doc_html/reference-function-findrqs.html)
pub fn FindRQS(board: c_int, addresses: &Vec<Addr4882>) -> Result<(Addr4882, c_short), GpibError> {
    let mut instruments = addresses
        .iter()
        .map(|a| a.addr)
        .collect::<Vec<Addr4882_t>>();
    instruments.push(linux_gpib_sys::NOADDR);
    let mut status_byte: c_short = 0;
    unsafe {
        linux_gpib_sys::FindRQS(board, instruments.as_ptr(), &mut status_byte);
    }
    let status = IbStatus::current_thread_local_status();
    if status.err {
        Err(GpibError::DriverError(
            status,
            IbError::current_thread_local_error()?,
        ))
    } else {
        let index: usize = ThreadIbcnt().try_into()?;
        if index >= addresses.len() {
            Err(GpibError::ValueError(
                "index stored in Ibcnt is larger than addresses array length".to_owned(),
            ))
        } else {
            Ok((addresses[index], status_byte))
        }
    }
}

/// make device controller-in-charge
///
/// PassControl() causes the board specified by board_desc to pass control to the device specified by address. On success, the device becomes the new controller-in-charge.
///
/// See: [Linux GPIB Reference](https://linux-gpib.sourceforge.io/doc_html/reference-function-passcontrol.html)
pub fn PassControl(board: c_int, address: Addr4882) -> Result<(), GpibError> {
    unsafe {
        linux_gpib_sys::PassControl(board, address.addr);
    }
    let status = IbStatus::current_thread_local_status();
    if status.err {
        Err(GpibError::DriverError(
            status,
            IbError::current_thread_local_error()?,
        ))
    } else {
        Ok(())
    }
}

/// parallel poll devices
///
/// PPoll() is similar to the 'traditional' API function ibrpp(). It causes the interface board to perform a parallel poll, and stores the parallel poll byte in the location specified by result. Bits 0 to 7 of the parallel poll byte correspond to the dio lines 1 to 8, with a 1 indicating the corresponding dio line is asserted. The devices on the bus you wish to poll should be configured beforehand with PPollConfig(). The board must be controller-in-charge to perform a parallel poll.
///
/// See: [Linux GPIB Reference](https://linux-gpib.sourceforge.io/doc_html/reference-function-ppoll.html)
pub fn PPoll(board: c_int) -> Result<c_short, GpibError> {
    let mut result: c_short = 0;
    unsafe {
        linux_gpib_sys::PPoll(board, &mut result);
    }
    let status = IbStatus::current_thread_local_status();
    if status.err {
        Err(GpibError::DriverError(
            status,
            IbError::current_thread_local_error()?,
        ))
    } else {
        Ok(result)
    }
}

/// configure a device's parallel poll response
///
/// PPollConfig() configures the device specified by address to respond to parallel polls. The dio_line (valid values are 1 through 8) specifies which dio line the device being configured should use to send back its parallel poll response. The line_sense argument specifies the polarity of the response. If line_sense is nonzero, then the specified dio line will be asserted to indicate that the 'individual status bit' (or 'ist') is 1. If sense is zero, then the specified dio line will be asserted when ist is zero.
///
/// See: [Linux GPIB Reference](https://linux-gpib.sourceforge.io/doc_html/reference-function-ppollconfig.html)
pub fn PPollConfig(
    board: c_int,
    address: Addr4882,
    dio_line: c_int,
    line_sense: c_int,
) -> Result<(), GpibError> {
    unsafe {
        linux_gpib_sys::PPollConfig(board, address.addr, dio_line, line_sense);
    }
    let status = IbStatus::current_thread_local_status();
    if status.err {
        Err(GpibError::DriverError(
            status,
            IbError::current_thread_local_error()?,
        ))
    } else {
        Ok(())
    }
}

/// disable devices' parallel poll response
///
/// See: [Linux GPIB Reference](https://linux-gpib.sourceforge.io/doc_html/reference-function-ppollunconfig.html)
pub fn PPollUnconfig(board: c_int, addresses: &Vec<Addr4882>) -> Result<(), GpibError> {
    let mut instruments = addresses
        .iter()
        .map(|a| a.addr)
        .collect::<Vec<Addr4882_t>>();
    instruments.push(linux_gpib_sys::NOADDR);
    unsafe {
        linux_gpib_sys::PPollUnconfig(board, instruments.as_ptr());
    }
    let status = IbStatus::current_thread_local_status();
    if status.err {
        Err(GpibError::DriverError(
            status,
            IbError::current_thread_local_error()?,
        ))
    } else {
        Ok(())
    }
}

/// read data
///
/// 	RcvRespMsg() reads data from the bus. A device must have already been addressed as talker (and the board as listener) before calling this function. Addressing may be accomplished with the ReceiveSetup() function.
///
/// Up to count bytes are read into the array specified by buffer. The termination argument specifies the 8-bit end-of-string character (which must be a value from 0 to 255) whose reception will terminate a read. termination can also be set to the 'STOPend' constant, in which case no end-of-string character will be used. Assertion of the EOI line will always end a read.
///
/// You may find it simpler to use the slightly higher level function Receive(), since it does not require addressing and reading of data to be performed separately.
///
/// See: [Linux GPIB Reference](https://linux-gpib.sourceforge.io/doc_html/reference-function-rcvrespmsg.html)
pub fn RcvRespMsg(board: c_int, buffer: &mut [u8], termination: c_int) -> Result<(), GpibError> {
    unsafe {
        linux_gpib_sys::RcvRespMsg(
            board,
            buffer.as_mut_ptr() as *mut c_void,
            buffer.len().try_into()?,
            termination,
        );
    }
    let status = IbStatus::current_thread_local_status();
    if status.err {
        Err(GpibError::DriverError(
            status,
            IbError::current_thread_local_error()?,
        ))
    } else {
        Ok(())
    }
}

/// serial poll a device
///
/// ReadStatusByte() causes the board specified by the board descriptor board_desc to serial poll the GPIB address specified by address. The status byte is stored at the location specified by the result pointer. If you wish to serial poll multiple devices, it may be slightly more efficient to use AllSPoll(). Serial polls may also be conducted with the 'traditional API' function ibrsp().
///
/// See: [Linux GPIB Reference](https://linux-gpib.sourceforge.io/doc_html/reference-function-readstatusbyte.html)
pub fn ReadStatusByte(board: c_int, address: Addr4882) -> Result<c_short, GpibError> {
    let mut result: c_short = 0;
    unsafe {
        linux_gpib_sys::ReadStatusByte(board, address.addr, &mut result);
    }
    let status = IbStatus::current_thread_local_status();
    if status.err {
        Err(GpibError::DriverError(
            status,
            IbError::current_thread_local_error()?,
        ))
    } else {
        Ok(result)
    }
}

/// perform receive addressing and read data
///
/// Receive() performs the necessary addressing, then reads data from the device specified by address. It is equivalent to a ReceiveSetup() call followed by a RcvRespMsg() call.
///
/// See: [Linux GPIB Reference](https://linux-gpib.sourceforge.io/doc_html/reference-function-receive.html)
pub fn Receive(
    board: c_int,
    address: Addr4882,
    buffer: &mut [u8],
    termination: c_int,
) -> Result<(), GpibError> {
    unsafe {
        linux_gpib_sys::Receive(
            board,
            address.addr,
            buffer.as_mut_ptr() as *mut c_void,
            buffer.len().try_into()?,
            termination,
        );
    }
    let status = IbStatus::current_thread_local_status();
    if status.err {
        Err(GpibError::DriverError(
            status,
            IbError::current_thread_local_error()?,
        ))
    } else {
        Ok(())
    }
}

/// perform receive addressing
///
/// 	ReceiveSetup() addresses the device specified by address as talker, and addresses the interface board as listener. A subsequent RcvRespMsg() call will read data from the device.
///
///You may find it simpler to use the slightly higher level function Receive(), since it does not require addressing and reading of data to be performed separately.
///
/// See: [Linux GPIB Reference](https://linux-gpib.sourceforge.io/doc_html/reference-function-receivesetup.html)
pub fn ReceiveSetup(board: c_int, address: Addr4882) -> Result<(), GpibError> {
    unsafe {
        linux_gpib_sys::ReceiveSetup(board, address.addr);
    }
    let status = IbStatus::current_thread_local_status();
    if status.err {
        Err(GpibError::DriverError(
            status,
            IbError::current_thread_local_error()?,
        ))
    } else {
        Ok(())
    }
}

/// reset system
///
/// ResetSys() has the following effects:
///
/// - The remote enable bus line is asserted.
/// - An interface clear is performed (the interface clear bus line is asserted for at least 100 microseconds).
/// - The device clear command is sent to all the devices on the bus.
/// - The *RST message is sent to every device specified in the addressList.
///
/// See: [Linux GPIB Reference](https://linux-gpib.sourceforge.io/doc_html/reference-function-resetsys.html)
pub fn ResetSys(board: c_int, addresses: &Vec<Addr4882>) -> Result<(), GpibError> {
    let mut instruments = addresses
        .iter()
        .map(|a| a.addr)
        .collect::<Vec<Addr4882_t>>();
    instruments.push(linux_gpib_sys::NOADDR);
    unsafe {
        linux_gpib_sys::ResetSys(board, instruments.as_ptr());
    }
    let status = IbStatus::current_thread_local_status();
    if status.err {
        Err(GpibError::DriverError(
            status,
            IbError::current_thread_local_error()?,
        ))
    } else {
        Ok(())
    }
}

/// perform send addressing and write data
///
/// Send() addresses the device specified by address as listener, then writes data onto the bus. It is equivalent to a SendList() except it only uses a single GPIB address to specify the listener instead of allowing an array of listeners.
///
/// See: [Linux GPIB Reference](https://linux-gpib.sourceforge.io/doc_html/reference-function-send.html)
pub fn Send(
    board: c_int,
    address: Addr4882,
    buffer: &[u8],
    eot_mode: IbSendEOI,
) -> Result<(), GpibError> {
    unsafe {
        linux_gpib_sys::Send(
            board,
            address.addr,
            buffer.as_ptr() as *const c_void,
            buffer.len().try_into()?,
            eot_mode.as_eot(),
        );
    }
    let status = IbStatus::current_thread_local_status();
    if status.err {
        Err(GpibError::DriverError(
            status,
            IbError::current_thread_local_error()?,
        ))
    } else {
        Ok(())
    }
}

/// perform interface clear
///
/// SendIFC() resets the GPIB bus by asserting the 'interface clear' (IFC) bus line for a duration of at least 100 microseconds. The board specified by board_desc must be the system controller in order to assert IFC. The interface clear causes all devices to untalk and unlisten, puts them into serial poll disabled state (don't worry, you will still be able to conduct serial polls), and the board becomes controller-in-charge.
pub fn SendIFC(board: c_int) -> Result<(), GpibError> {
    unsafe {
        linux_gpib_sys::SendIFC(board);
    }
    let status = IbStatus::current_thread_local_status();
    if status.err {
        Err(GpibError::DriverError(
            status,
            IbError::current_thread_local_error()?,
        ))
    } else {
        Ok(())
    }
}

/// write data to multiple devices
///
/// See: [Linux GPIB Reference](https://linux-gpib.sourceforge.io/doc_html/reference-function-sendlist.html)
pub fn SendList(
    board: c_int,
    addresses: &Vec<Addr4882>,
    buffer: &[u8],
    eot_mode: IbSendEOI,
) -> Result<(), GpibError> {
    let mut instruments = addresses
        .iter()
        .map(|a| a.addr)
        .collect::<Vec<Addr4882_t>>();
    instruments.push(linux_gpib_sys::NOADDR);
    unsafe {
        linux_gpib_sys::SendList(
            board,
            instruments.as_ptr(),
            buffer.as_ptr() as *const c_void,
            buffer.len().try_into()?,
            eot_mode.as_eot(),
        );
    }
    let status = IbStatus::current_thread_local_status();
    if status.err {
        Err(GpibError::DriverError(
            status,
            IbError::current_thread_local_error()?,
        ))
    } else {
        Ok(())
    }
}

/// put devices into local lockout mode
///
/// See: [Linux GPIB Reference](https://linux-gpib.sourceforge.io/doc_html/reference-function-sendllo.html)
pub fn SendLLO(board: c_int) -> Result<(), GpibError> {
    unsafe {
        linux_gpib_sys::SendLLO(board);
    }
    let status = IbStatus::current_thread_local_status();
    if status.err {
        Err(GpibError::DriverError(
            status,
            IbError::current_thread_local_error()?,
        ))
    } else {
        Ok(())
    }
}

/// put devices into remote with lockout state
///
/// See: [Linux GPIB Reference](https://linux-gpib.sourceforge.io/doc_html/reference-function-setrwls.html)
pub fn SetRWLS(board: c_int, addresses: &Vec<Addr4882>) -> Result<(), GpibError> {
    let mut instruments = addresses
        .iter()
        .map(|a| a.addr)
        .collect::<Vec<Addr4882_t>>();
    instruments.push(linux_gpib_sys::NOADDR);
    unsafe {
        linux_gpib_sys::SetRWLS(board, instruments.as_ptr());
    }
    let status = IbStatus::current_thread_local_status();
    if status.err {
        Err(GpibError::DriverError(
            status,
            IbError::current_thread_local_error()?,
        ))
    } else {
        Ok(())
    }
}

///  query state of SRQ bus line
///
/// Returns true if the SRQ line is asserted, false if it is not asserted.
///
/// See: [Linux GPIB Reference](https://linux-gpib.sourceforge.io/doc_html/reference-function-testsrq.html)
pub fn TestSRQ(board: c_int) -> Result<bool, GpibError> {
    let mut result: c_short = 10;
    unsafe {
        linux_gpib_sys::TestSRQ(board, &mut result);
    }
    let status = IbStatus::current_thread_local_status();
    if status.err {
        Err(GpibError::DriverError(
            status,
            IbError::current_thread_local_error()?,
        ))
    } else {
        match result {
            0 => Ok(false),
            1 => Ok(true),
            other => Err(GpibError::ValueError(format!("Unexpected value {}", other))),
        }
    }
}

/// perform self-test queries on devices
///
/// See: [Linux GPIB Reference](https://linux-gpib.sourceforge.io/doc_html/reference-function-testsys.html)
pub fn TestSys(board: c_int, addresses: &Vec<Addr4882>) -> Result<Vec<c_short>, GpibError> {
    let mut instruments = addresses
        .iter()
        .map(|a| a.addr)
        .collect::<Vec<Addr4882_t>>();
    instruments.push(linux_gpib_sys::NOADDR);
    let mut results: Vec<c_short> = Vec::with_capacity(addresses.len());
    results.resize(addresses.len(), 0);
    unsafe {
        linux_gpib_sys::TestSys(board, instruments.as_ptr(), results.as_mut_ptr());
    }
    let status = IbStatus::current_thread_local_status();
    if status.err {
        Err(GpibError::DriverError(
            status,
            IbError::current_thread_local_error()?,
        ))
    } else {
        Ok(results)
    }
}

/// trigger a device
///
/// See: [Linux GPIB Reference](https://linux-gpib.sourceforge.io/doc_html/reference-function-trigger.html)
pub fn Trigger(board: c_int, address: Addr4882) -> Result<(), GpibError> {
    unsafe {
        linux_gpib_sys::Trigger(board, address.addr);
    }
    let status = IbStatus::current_thread_local_status();
    if status.err {
        Err(GpibError::DriverError(
            status,
            IbError::current_thread_local_error()?,
        ))
    } else {
        Ok(())
    }
}

/// trigger multiple devices
///
/// See: [Linux GPIB Reference](https://linux-gpib.sourceforge.io/doc_html/reference-function-triggerlist.html)
pub fn TriggerList(board: c_int, addresses: &Vec<Addr4882>) -> Result<(), GpibError> {
    let mut instruments = addresses
        .iter()
        .map(|a| a.addr)
        .collect::<Vec<Addr4882_t>>();
    instruments.push(linux_gpib_sys::NOADDR);
    unsafe {
        linux_gpib_sys::TriggerList(board, instruments.as_ptr());
    }
    let status = IbStatus::current_thread_local_status();
    if status.err {
        Err(GpibError::DriverError(
            status,
            IbError::current_thread_local_error()?,
        ))
    } else {
        Ok(())
    }
}

#[cfg(feature = "async-tokio")]
/// sleep until the SRQ bus line is asserted
///
/// See: [Linux GPIB Reference](https://linux-gpib.sourceforge.io/doc_html/reference-function-waitsrq.html)
pub async fn WaitSRQ(board: c_int) -> Result<c_short, GpibError> {
    tokio::task::spawn_blocking(move || {
        let mut result: c_short = 0;
        unsafe {
            linux_gpib_sys::WaitSRQ(board, &mut result);
        }
        let status = IbStatus::current_thread_local_status();
        if status.err {
            Err(GpibError::DriverError(
                status,
                IbError::current_thread_local_error()?,
            ))
        } else {
            Ok(result)
        }
    })
    .await?
}
