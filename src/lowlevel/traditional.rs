use crate::error::{GpibError, IbError};
#[cfg(feature = "nigpib")]
use crate::lowlevel::utility::Ibcnt;
#[cfg(feature = "linuxgpib")]
use crate::lowlevel::utility::{AsyncIbcntl, ThreadIbcnt, ThreadIbcntl};
use crate::status::IbStatus;
use crate::types::{
    IbEosMode, IbEvent, IbLineStatus, IbOnline, IbOption, IbSendEOI, IbTimeout, PrimaryAddress,
    SecondaryAddress,
};
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int, c_short, c_void};
use std::path::Path;

/// ibask -- query configuration (board or device)
/// See: [Linux GPIB Reference](https://linux-gpib.sourceforge.io/doc_html/reference-function-ibask.html)
pub fn ibask(ud: c_int, option: IbOption) -> Result<c_int, GpibError> {
    let option = option.as_option();
    let mut result: c_int = 0;
    let status = IbStatus::from_ibsta(unsafe {
        linux_gpib_sys::ibask(ud, option, &mut result as *mut c_int)
    });
    if status.err {
        Err(GpibError::DriverError(
            status,
            #[cfg(feature = "linuxgpib")]
            IbError::current_thread_local_error()?,
            #[cfg(feature = "nigpib")]
            unsafe { IbError::current_global_error() }?,
        ))
    } else {
        Ok(result)
    }
}

#[cfg(feature = "linuxgpib")]
/// ibbna -- change access board (device)
/// See: [Linux GPIB Reference](https://linux-gpib.sourceforge.io/doc_html/reference-function-ibbna.html)
pub fn ibbna(ud: c_int, name: &str) -> Result<(), GpibError> {
    let name = CString::new(name)?;
    let status =
        IbStatus::from_ibsta(unsafe { linux_gpib_sys::ibbna(ud, name.as_ptr() as *mut c_char) });
    if status.err {
        Err(GpibError::DriverError(
            status,
            #[cfg(feature = "linuxgpib")]
            IbError::current_thread_local_error()?,
            #[cfg(feature = "nigpib")]
            unsafe { IbError::current_global_error() }?,
        ))
    } else {
        Ok(())
    }
}

/// ibcac -- assert ATN (board)
///
/// ibcac() causes the board specified by the board descriptor ud to become active controller by asserting the ATN line. The board must be controller-in-change in order to assert ATN. If synchronous is nonzero, then the board will wait for a data byte on the bus to complete its transfer before asserting ATN. If the synchronous attempt times out, or synchronous is zero, then ATN will be asserted immediately.
///
/// It is generally not necessary to call ibcac(). It is provided for advanced users who want direct, low-level access to the GPIB bus.
///
/// See: [Linux GPIB Reference](https://linux-gpib.sourceforge.io/doc_html/reference-function-ibcac.html)
pub fn ibcac(ud: c_int, synchronous: c_int) -> Result<(), GpibError> {
    let status = IbStatus::from_ibsta(unsafe { linux_gpib_sys::ibcac(ud, synchronous) });
    if status.err {
        Err(GpibError::DriverError(
            status,
            #[cfg(feature = "linuxgpib")]
            IbError::current_thread_local_error()?,
            #[cfg(feature = "nigpib")]
            unsafe { IbError::current_global_error() }?,
        ))
    } else {
        Ok(())
    }
}

/// ibclr -- clear device (device)
/// See: [Linux GPIB Reference](https://linux-gpib.sourceforge.io/doc_html/reference-function-ibclr.html)
pub fn ibclr(ud: c_int) -> Result<(), GpibError> {
    log::debug!("ibclr({})", ud);
    let status = IbStatus::from_ibsta(unsafe { linux_gpib_sys::ibclr(ud) });
    log::debug!("ibclr({}) -> {:?}", ud, status);
    if status.err {
        Err(GpibError::DriverError(
            status,
            #[cfg(feature = "linuxgpib")]
            IbError::current_thread_local_error()?,
            #[cfg(feature = "nigpib")]
            unsafe { IbError::current_global_error() }?,
        ))
    } else {
        Ok(())
    }
}

/// ibcmd -- write command bytes (board)
/// See: [Linux GPIB Reference](https://linux-gpib.sourceforge.io/doc_html/reference-function-ibcmd.html)
pub fn ibcmd(ud: c_int, commands: &[u8]) -> Result<(), GpibError> {
    let status = IbStatus::from_ibsta(unsafe {
        linux_gpib_sys::ibcmd(
            ud,
            commands.as_ptr() as *const c_void,
            commands.len().try_into()?,
        )
    });
    if status.err {
        Err(GpibError::DriverError(
            status,
            #[cfg(feature = "linuxgpib")]
            IbError::current_thread_local_error()?,
            #[cfg(feature = "nigpib")]
            unsafe { IbError::current_global_error() }?,
        ))
    } else {
        Ok(())
    }
}

/// ibconfig -- change configuration (board or device)
/// See: [Linux GPIB Reference](https://linux-gpib.sourceforge.io/doc_html/reference-function-ibconfig.html)
pub fn ibconfig(ud: c_int, option: IbOption, setting: c_int) -> Result<(), GpibError> {
    let option = option.as_option();
    let status = IbStatus::from_ibsta(unsafe { linux_gpib_sys::ibconfig(ud, option, setting) });
    if status.err {
        Err(GpibError::DriverError(
            status,
            #[cfg(feature = "linuxgpib")]
            IbError::current_thread_local_error()?,
            #[cfg(feature = "nigpib")]
            unsafe { IbError::current_global_error() }?,
        ))
    } else {
        Ok(())
    }
}

/// open a device (device)
/// See: [Linux GPIB Reference](https://linux-gpib.sourceforge.io/doc_html/reference-function-ibdev.html)
pub fn ibdev(
    board_index: c_int,
    primary_address: PrimaryAddress,
    secondary_address: SecondaryAddress,
    timeout: IbTimeout,
    send_eoi: IbSendEOI,
    eos: IbEosMode,
) -> Result<c_int, GpibError> {
    let ud = unsafe {
        linux_gpib_sys::ibdev(
            board_index,
            primary_address.as_pad(),
            secondary_address.as_sad(),
            timeout.as_timeout(),
            send_eoi.as_eot(),
            eos.as_mode(),
        )
    };
    log::debug!(
        "ibdev({}, {}, {}, {}, {}, {}) -> {}",
        board_index,
        primary_address,
        secondary_address,
        timeout,
        send_eoi,
        eos,
        ud
    );
    if ud >= 0 {
        Ok(ud)
    } else {
        #[cfg(feature = "linuxgpib")]
        return Err(GpibError::DriverError(
            IbStatus::current_thread_local_status(),
            IbError::current_thread_local_error()?,
        ));
        #[cfg(feature = "nigpib")]
        return Err(GpibError::DriverError(
            unsafe { IbStatus::current_global_status() },
            unsafe { IbError::current_global_error() }?,
        ));
    }
}

/// ibeos -- set end-of-string mode (board or device)
/// See: [Linux GPIB Reference](https://linux-gpib.sourceforge.io/doc_html/reference-function-ibeos.html)
pub fn ibeos(ud: c_int, eosmod: IbEosMode) -> Result<(), GpibError> {
    let eosmod = eosmod.as_mode();
    let status = IbStatus::from_ibsta(unsafe { linux_gpib_sys::ibeos(ud, eosmod) });
    if status.err {
        Err(GpibError::DriverError(
            status,
            #[cfg(feature = "linuxgpib")]
            IbError::current_thread_local_error()?,
            #[cfg(feature = "nigpib")]
            unsafe { IbError::current_global_error() }?,
        ))
    } else {
        Ok(())
    }
}

/// ibeot -- assert EOI with last data byte (board or device)
/// See: [Linux GPIB Reference](https://linux-gpib.sourceforge.io/doc_html/reference-function-ibeot.html)
pub fn ibeot(ud: c_int, send_eoi: IbSendEOI) -> Result<(), GpibError> {
    let status = IbStatus::from_ibsta(unsafe { linux_gpib_sys::ibeot(ud, send_eoi.as_eot()) });
    if status.err {
        Err(GpibError::DriverError(
            status,
            #[cfg(feature = "linuxgpib")]
            IbError::current_thread_local_error()?,
            #[cfg(feature = "nigpib")]
            unsafe { IbError::current_global_error() }?,
        ))
    } else {
        Ok(())
    }
}

#[cfg(feature = "linuxgpib")]
/// ibevent -- get events from event queue (board)
/// See: [Linux GPIB Reference](https://linux-gpib.sourceforge.io/doc_html/reference-function-ibevent.html)
pub fn ibevent(ud: c_int) -> Result<IbEvent, GpibError> {
    let mut event_value: c_short = 0;
    let status = IbStatus::from_ibsta(unsafe {
        linux_gpib_sys::ibevent(ud, &mut event_value as *mut c_short)
    });
    if status.err {
        Err(GpibError::DriverError(
            status,
            #[cfg(feature = "linuxgpib")]
            IbError::current_thread_local_error()?,
            #[cfg(feature = "nigpib")]
            unsafe { IbError::current_global_error() }?,
        ))
    } else {
        Ok(IbEvent::from_value(event_value)?)
    }
}

/// ibfind -- open a board or device (board or device)
/// See: [Linux GPIB Reference](https://linux-gpib.sourceforge.io/doc_html/reference-function-ibfind.html)
pub fn ibfind(name: &str) -> Result<c_int, GpibError> {
    let name = CString::new(name)?;
    let ud = unsafe { linux_gpib_sys::ibfind(name.as_ptr()) };
    if ud >= 0 {
        Ok(ud)
    } else {
        Err(GpibError::DriverError(
            #[cfg(feature = "linuxgpib")]
            IbStatus::current_thread_local_status(),
            #[cfg(feature = "nigpib")]
            unsafe {
                IbStatus::current_global_status()
            },
            #[cfg(feature = "linuxgpib")]
            IbError::current_thread_local_error()?,
            #[cfg(feature = "nigpib")]
            unsafe { IbError::current_global_error() }?,
        ))
    }
}

/// ibgts -- release ATN (board)
/// See: [Linux GPIB Reference](https://linux-gpib.sourceforge.io/doc_html/reference-function-ibgts.html)
pub fn ibgts(ud: c_int, shadow_handshake: c_int) -> Result<(), GpibError> {
    let status = IbStatus::from_ibsta(unsafe { linux_gpib_sys::ibgts(ud, shadow_handshake) });
    if status.err {
        Err(GpibError::DriverError(
            status,
            #[cfg(feature = "linuxgpib")]
            IbError::current_thread_local_error()?,
            #[cfg(feature = "nigpib")]
            unsafe { IbError::current_global_error() }?,
        ))
    } else {
        Ok(())
    }
}

/// ibist -- set individual status bit (board)
/// See: [Linux GPIB Reference](https://linux-gpib.sourceforge.io/doc_html/reference-function-ibist.html)
pub fn ibist(ud: c_int, ist: c_int) -> Result<(), GpibError> {
    let status = IbStatus::from_ibsta(unsafe { linux_gpib_sys::ibist(ud, ist) });
    if status.err {
        Err(GpibError::DriverError(
            status,
            #[cfg(feature = "linuxgpib")]
            IbError::current_thread_local_error()?,
            #[cfg(feature = "nigpib")]
            unsafe { IbError::current_global_error() }?,
        ))
    } else {
        Ok(())
    }
}

/// iblines -- monitor bus lines (board)
/// See: [Linux GPIB Reference](https://linux-gpib.sourceforge.io/doc_html/reference-function-iblines.html)
pub fn iblines(ud: c_int) -> Result<IbLineStatus, GpibError> {
    let mut line_status: c_short = 0;
    let status = IbStatus::from_ibsta(unsafe {
        linux_gpib_sys::iblines(ud, &mut line_status as *mut c_short)
    });
    if status.err {
        Err(GpibError::DriverError(
            status,
            #[cfg(feature = "linuxgpib")]
            IbError::current_thread_local_error()?,
            #[cfg(feature = "nigpib")]
            unsafe { IbError::current_global_error() }?,
        ))
    } else {
        Ok(IbLineStatus::from_line_status(line_status))
    }
}

/// ibln -- check if listener is present (board or device)
/// See: [Linux GPIB Reference](https://linux-gpib.sourceforge.io/doc_html/reference-function-ibln.html)
pub fn ibln(
    ud: c_int,
    primary_address: PrimaryAddress,
    secondary_address: SecondaryAddress,
) -> Result<bool, GpibError> {
    let mut found_listener: c_short = 0;
    let status = IbStatus::from_ibsta(unsafe {
        linux_gpib_sys::ibln(
            ud,
            primary_address.as_pad(),
            secondary_address.as_sad(),
            &mut found_listener as *mut c_short,
        )
    });
    if status.err {
        Err(GpibError::DriverError(
            status,
            #[cfg(feature = "linuxgpib")]
            IbError::current_thread_local_error()?,
            #[cfg(feature = "nigpib")]
            unsafe { IbError::current_global_error() }?,
        ))
    } else {
        Ok(found_listener != 0)
    }
}

/// ibloc -- go to local mode (board or device)
/// See: [Linux GPIB Reference](https://linux-gpib.sourceforge.io/doc_html/reference-function-ibloc.html)
pub fn ibloc(ud: c_int) -> Result<(), GpibError> {
    let status = IbStatus::from_ibsta(unsafe { linux_gpib_sys::ibloc(ud) });
    if status.err {
        Err(GpibError::DriverError(
            status,
            #[cfg(feature = "linuxgpib")]
            IbError::current_thread_local_error()?,
            #[cfg(feature = "nigpib")]
            unsafe { IbError::current_global_error() }?,
        ))
    } else {
        Ok(())
    }
}

/// ibonl -- close or reinitialize descriptor (board or device)
/// See: [Linux GPIB Reference](https://linux-gpib.sourceforge.io/doc_html/reference-function-ibonl.html)
pub fn ibonl(ud: c_int, online: IbOnline) -> Result<(), GpibError> {
    log::debug!("ibonl({}, {})", ud, online);
    let online = online.as_online();
    let status = IbStatus::from_ibsta(unsafe { linux_gpib_sys::ibonl(ud, online) });
    log::debug!("ibonl({}, {}) -> {:?}", ud, online, status);
    if status.err {
        Err(GpibError::DriverError(
            status,
            #[cfg(feature = "linuxgpib")]
            IbError::current_thread_local_error()?,
            #[cfg(feature = "nigpib")]
            unsafe { IbError::current_global_error() }?,
        ))
    } else {
        Ok(())
    }
}

/// ibpad -- set primary GPIB address (board or device)
/// See: [Linux GPIB Reference](https://linux-gpib.sourceforge.io/doc_html/reference-function-ibpad.html)
pub fn ibpad(ud: c_int, primary_address: PrimaryAddress) -> Result<(), GpibError> {
    let status =
        IbStatus::from_ibsta(unsafe { linux_gpib_sys::ibpad(ud, primary_address.as_pad()) });
    if status.err {
        Err(GpibError::DriverError(
            status,
            #[cfg(feature = "linuxgpib")]
            IbError::current_thread_local_error()?,
            #[cfg(feature = "nigpib")]
            unsafe { IbError::current_global_error() }?,
        ))
    } else {
        Ok(())
    }
}

/// ibpct -- pass control (board)
///
/// ibpct() passes control to the device specified by the device descriptor ud. The device becomes the new controller-in-charge.
///
/// See: [Linux GPIB Reference](https://linux-gpib.sourceforge.io/doc_html/reference-function-ibpct.html)
pub fn ibpct(ud: c_int) -> Result<(), GpibError> {
    let status = IbStatus::from_ibsta(unsafe { linux_gpib_sys::ibpct(ud) });
    if status.err {
        Err(GpibError::DriverError(
            status,
            #[cfg(feature = "linuxgpib")]
            IbError::current_thread_local_error()?,
            #[cfg(feature = "nigpib")]
            unsafe { IbError::current_global_error() }?,
        ))
    } else {
        Ok(())
    }
}

/// ibppc -- parallel poll configure (board or device)
/// See: [Linux GPIB Reference](https://linux-gpib.sourceforge.io/doc_html/reference-function-ibppc.html)
pub fn ibppc(ud: c_int, configuration: c_int) -> Result<(), GpibError> {
    let status = IbStatus::from_ibsta(unsafe { linux_gpib_sys::ibppc(ud, configuration) });
    if status.err {
        Err(GpibError::DriverError(
            status,
            #[cfg(feature = "linuxgpib")]
            IbError::current_thread_local_error()?,
            #[cfg(feature = "nigpib")]
            unsafe { IbError::current_global_error() }?,
        ))
    } else {
        Ok(())
    }
}

/// read data bytes (board or device)
/// See: [Linux GPIB Reference](https://linux-gpib.sourceforge.io/doc_html/reference-function-ibrd.html)
pub fn ibrd(ud: c_int, buffer: &mut [u8]) -> Result<(IbStatus, usize), GpibError> {
    let status = IbStatus::from_ibsta(unsafe {
        linux_gpib_sys::ibrd(
            ud,
            buffer.as_mut_ptr() as *mut c_void,
            buffer.len().try_into()?,
        )
    });
    log::debug!("ibrd({}, count = {}) -> {:?}", ud, buffer.len(), status);
    if status.err {
        Err(GpibError::DriverError(
            status,
            #[cfg(feature = "linuxgpib")]
            IbError::current_thread_local_error()?,
            #[cfg(feature = "nigpib")]
            unsafe { IbError::current_global_error() }?,
        ))
    } else {
        #[cfg(feature = "linuxgpib")]
        let bytes_read = ThreadIbcntl();
        #[cfg(feature = "nigpib")]
        let bytes_read = Ibcnt();
        if bytes_read > buffer.len().try_into()? {
            Err(GpibError::ValueError(format!(
                "bytes_read ({}) > buffer.len() ({})",
                bytes_read,
                buffer.len(),
            )))
        } else {
            log::debug!("-> {} bytes read", bytes_read);
            Ok((status, bytes_read.try_into()?))
        }
    }
}

/// read data bytes asynchronously (board or device)
///
/// This function is unsafe because Rust will not be able to check the lifetime
/// of buffer. It needs to remain available until the asynchronous read completes.
pub unsafe fn ibrda(ud: c_int, buffer: &mut [u8]) -> Result<(), GpibError> {
    let status = IbStatus::from_ibsta(unsafe {
        linux_gpib_sys::ibrda(
            ud,
            buffer.as_mut_ptr() as *mut c_void,
            buffer.len().try_into()?,
        )
    });
    log::debug!("ibrda({}) -> {:?}", ud, status);
    if status.err {
        return Err(GpibError::DriverError(
            status,
            #[cfg(feature = "linuxgpib")]
            IbError::current_thread_local_error()?,
            #[cfg(feature = "nigpib")]
            unsafe { IbError::current_global_error() }?,
        ));
    } else {
        Ok(())
    }
}

/// read data bytes to file (board or device)
/// See: [Linux GPIB Reference](https://linux-gpib.sourceforge.io/doc_html/reference-function-ibrdf.html)
pub fn ibrdf(ud: c_int, file_path: &Path) -> Result<(), GpibError> {
    let file_path = CString::new(file_path.to_str().ok_or(GpibError::ValueError(format!(
        "Unable to convert path '{:?}' to string",
        file_path
    )))?)?;
    let status = IbStatus::from_ibsta(unsafe { linux_gpib_sys::ibrdf(ud, file_path.as_ptr()) });
    if status.err {
        Err(GpibError::DriverError(
            status,
            #[cfg(feature = "linuxgpib")]
            IbError::current_thread_local_error()?,
            #[cfg(feature = "nigpib")]
            unsafe { IbError::current_global_error() }?,
        ))
    } else {
        Ok(())
    }
}

/// perform a parallel poll (board or device)
/// See: [Linux GPIB Reference](https://linux-gpib.sourceforge.io/doc_html/reference-function-ibrpp.html)
pub fn ibrpp(ud: c_int) -> Result<c_char, GpibError> {
    let mut ppoll_result: c_char = 0;
    let status = IbStatus::from_ibsta(unsafe {
        linux_gpib_sys::ibrpp(ud, &mut ppoll_result as *mut c_char)
    });
    if status.err {
        Err(GpibError::DriverError(
            status,
            #[cfg(feature = "linuxgpib")]
            IbError::current_thread_local_error()?,
            #[cfg(feature = "nigpib")]
            unsafe { IbError::current_global_error() }?,
        ))
    } else {
        Ok(ppoll_result)
    }
}

/// ibrsc -- request system control (board)
/// See: [Linux GPIB Reference](https://linux-gpib.sourceforge.io/doc_html/reference-function-ibrsc.html)
pub fn ibrsc(ud: c_int, request_control: c_int) -> Result<(), GpibError> {
    let status = IbStatus::from_ibsta(unsafe { linux_gpib_sys::ibrsc(ud, request_control) });
    if status.err {
        Err(GpibError::DriverError(
            status,
            #[cfg(feature = "linuxgpib")]
            IbError::current_thread_local_error()?,
            #[cfg(feature = "nigpib")]
            unsafe { IbError::current_global_error() }?,
        ))
    } else {
        Ok(())
    }
}

/// ibrsp --  read status byte / serial poll (device)
/// See: [Linux GPIB Reference](https://linux-gpib.sourceforge.io/doc_html/reference-function-ibrsp.html)
pub fn ibrsp(ud: c_int) -> Result<c_char, GpibError> {
    let mut result: c_char = 0;
    let status =
        IbStatus::from_ibsta(unsafe { linux_gpib_sys::ibrsp(ud, &mut result as *mut c_char) });
    if status.err {
        Err(GpibError::DriverError(
            status,
            #[cfg(feature = "linuxgpib")]
            IbError::current_thread_local_error()?,
            #[cfg(feature = "nigpib")]
            unsafe { IbError::current_global_error() }?,
        ))
    } else {
        Ok(result)
    }
}

/// ibrsv -- request service (board)
/// See: [Linux GPIB Reference](https://linux-gpib.sourceforge.io/doc_html/reference-function-ibrsv.html)
pub fn ibrsv(ud: c_int, status_byte: c_int) -> Result<(), GpibError> {
    let status = IbStatus::from_ibsta(unsafe { linux_gpib_sys::ibrsv(ud, status_byte) });
    if status.err {
        Err(GpibError::DriverError(
            status,
            #[cfg(feature = "linuxgpib")]
            IbError::current_thread_local_error()?,
            #[cfg(feature = "nigpib")]
            unsafe { IbError::current_global_error() }?,
        ))
    } else {
        Ok(())
    }
}

#[cfg(feature = "linuxgpib")]
/// ibrsv2 -- request service (board)
/// See: [Linux GPIB Reference](https://linux-gpib.sourceforge.io/doc_html/reference-function-ibrsv2.html)
pub fn ibrsv2(
    ud: c_int,
    status_byte: c_int,
    new_reason_for_request: c_int,
) -> Result<(), GpibError> {
    let status = IbStatus::from_ibsta(unsafe {
        linux_gpib_sys::ibrsv2(ud, status_byte, new_reason_for_request)
    });
    if status.err {
        Err(GpibError::DriverError(
            status,
            IbError::current_thread_local_error()?,
        ))
    } else {
        Ok(())
    }
}

/// ibsad -- set secondary GPIB address (board or device)
/// See: [Linux GPIB Reference](https://linux-gpib.sourceforge.io/doc_html/reference-function-ibsad.html)
pub fn ibsad(ud: c_int, secondary_address: SecondaryAddress) -> Result<(), GpibError> {
    let status =
        IbStatus::from_ibsta(unsafe { linux_gpib_sys::ibsad(ud, secondary_address.as_sad()) });
    if status.err {
        Err(GpibError::DriverError(
            status,
            #[cfg(feature = "linuxgpib")]
            IbError::current_thread_local_error()?,
            #[cfg(feature = "nigpib")]
            unsafe { IbError::current_global_error() }?,
        ))
    } else {
        Ok(())
    }
}

/// ibsic -- perform interface clear (board)
/// See: [Linux GPIB Reference](https://linux-gpib.sourceforge.io/doc_html/reference-function-ibsic.html)
pub fn ibsic(ud: c_int) -> Result<(), GpibError> {
    let status = IbStatus::from_ibsta(unsafe { linux_gpib_sys::ibsic(ud) });
    if status.err {
        Err(GpibError::DriverError(
            status,
            #[cfg(feature = "linuxgpib")]
            IbError::current_thread_local_error()?,
            #[cfg(feature = "nigpib")]
            unsafe { IbError::current_global_error() }?,
        ))
    } else {
        Ok(())
    }
}

#[cfg(feature = "linuxgpib")]
/// ibspb --  obtain length of serial poll bytes queue (device)
/// See: [Linux GPIB Reference](https://linux-gpib.sourceforge.io/doc_html/reference-function-ibspb.html)
pub fn ibspb(ud: c_int) -> Result<c_short, GpibError> {
    let mut result: c_short = 0;
    let status =
        IbStatus::from_ibsta(unsafe { linux_gpib_sys::ibspb(ud, &mut result as *mut c_short) });
    if status.err {
        Err(GpibError::DriverError(
            status,
            IbError::current_thread_local_error()?,
        ))
    } else {
        Ok(result)
    }
}

/// ibsre -- set remote enable (board)
/// See: [Linux GPIB Reference](https://linux-gpib.sourceforge.io/doc_html/reference-function-ibsre.html)
pub fn ibsre(ud: c_int, enable: c_int) -> Result<(), GpibError> {
    let status = IbStatus::from_ibsta(unsafe { linux_gpib_sys::ibsre(ud, enable) });
    if status.err {
        Err(GpibError::DriverError(
            status,
            #[cfg(feature = "linuxgpib")]
            IbError::current_thread_local_error()?,
            #[cfg(feature = "nigpib")]
            unsafe { IbError::current_global_error() }?,
        ))
    } else {
        Ok(())
    }
}

/// ibstop -- abort asynchronous i/o operation (board or device)
/// See: [Linux GPIB Reference](https://linux-gpib.sourceforge.io/doc_html/reference-function-ibstop.html)
pub fn ibstop(ud: c_int) -> Result<(), GpibError> {
    let status = IbStatus::from_ibsta(unsafe { linux_gpib_sys::ibstop(ud) });
    if status.err {
        Err(GpibError::DriverError(
            status,
            #[cfg(feature = "linuxgpib")]
            IbError::current_thread_local_error()?,
            #[cfg(feature = "nigpib")]
            unsafe { IbError::current_global_error() }?,
        ))
    } else {
        Ok(())
    }
}

/// ibtmo -- adjust io timeout (board or device)
/// See: [Linux GPIB Reference](https://linux-gpib.sourceforge.io/doc_html/reference-function-ibtmo.html)
pub fn ibtmo(ud: c_int, timeout: IbTimeout) -> Result<(), GpibError> {
    let timeout = timeout.as_timeout();
    let status = IbStatus::from_ibsta(unsafe { linux_gpib_sys::ibtmo(ud, timeout) });
    if status.err {
        Err(GpibError::DriverError(
            status,
            #[cfg(feature = "linuxgpib")]
            IbError::current_thread_local_error()?,
            #[cfg(feature = "nigpib")]
            unsafe { IbError::current_global_error() }?,
        ))
    } else {
        Ok(())
    }
}

/// ibtrg -- trigger device (device)
/// See: [Linux GPIB Reference](https://linux-gpib.sourceforge.io/doc_html/reference-function-ibtrg.html)
pub fn ibtrg(ud: c_int) -> Result<(), GpibError> {
    let status = IbStatus::from_ibsta(unsafe { linux_gpib_sys::ibtrg(ud) });
    if status.err {
        Err(GpibError::DriverError(
            status,
            #[cfg(feature = "linuxgpib")]
            IbError::current_thread_local_error()?,
            #[cfg(feature = "nigpib")]
            unsafe { IbError::current_global_error() }?,
        ))
    } else {
        Ok(())
    }
}

#[cfg(feature = "linuxgpib")]
/// ibvers -- Obtain the current linux gpib version
/// See: [Linux GPIB Reference](https://linux-gpib.sourceforge.io/doc_html/reference-function-ibvers.html)
pub fn ibvers() -> Result<String, GpibError> {
    let mut buffer_ptr: *mut c_char = std::ptr::null_mut();
    unsafe { linux_gpib_sys::ibvers(&mut buffer_ptr as *mut *mut c_char) }
    Ok(unsafe { CStr::from_ptr(buffer_ptr) }.to_str()?.to_owned())
}

#[cfg(feature = "async-tokio")]
/// wait for event (board or device)
/// See: [Linux GPIB Reference](https://linux-gpib.sourceforge.io/doc_html/reference-function-ibwait.html)
pub async fn ibwait(ud: c_int, status_mask: IbStatus) -> Result<(IbStatus, usize), GpibError> {
    let status_mask = status_mask.as_status_mask();
    let res = tokio::task::spawn_blocking(move || {
        let status = IbStatus::from_ibsta(unsafe { linux_gpib_sys::ibwait(ud, status_mask) });
        if status.err {
            Err(GpibError::DriverError(
                status,
                #[cfg(feature = "linuxgpib")]
                IbError::current_async_local_error()?,
                #[cfg(feature = "nigpib")]
                unsafe { IbError::current_global_error() }?,
            ))
        } else {
            Ok((
                status,
                #[cfg(feature = "linuxgpib")]
                AsyncIbcntl().try_into()?,
                #[cfg(feature = "nigpib")]
                Ibcnt().try_into()?,
            ))
        }
    })
    .await?;
    log::debug!("ibwait({}, {}) -> {:?}", ud, status_mask, res);
    res
}

/// ibwrt -- write data bytes (board or device)
/// See: [Linux GPIB Reference](https://linux-gpib.sourceforge.io/doc_html/reference-function-ibwrt.html)
pub fn ibwrt(ud: c_int, data: &[u8]) -> Result<usize, GpibError> {
    let status = IbStatus::from_ibsta(unsafe {
        linux_gpib_sys::ibwrt(ud, data.as_ptr() as *const c_void, data.len().try_into()?)
    });
    log::debug!(
        "ibwrt({}, {:?}) -> {:?}",
        ud,
        String::from_utf8(data.to_vec())?,
        status
    );
    if status.err {
        Err(GpibError::DriverError(
            status,
            #[cfg(feature = "linuxgpib")]
            IbError::current_thread_local_error()?,
            #[cfg(feature = "nigpib")]
            unsafe { IbError::current_global_error() }?,
        ))
    } else {
        Ok(
            #[cfg(feature = "linuxgpib")]
            ThreadIbcntl().try_into()?,
            #[cfg(feature = "nigpib")]
            Ibcnt().try_into()?,
        )
    }
}

/// write data bytes asynchronously (board or device)
///
/// Unsafe because the lifetime of buffer is not checked.
pub unsafe fn ibwrta(ud: c_int, data: &[u8]) -> Result<(), GpibError> {
    let status = IbStatus::from_ibsta(unsafe {
        linux_gpib_sys::ibwrta(ud, data.as_ptr() as *const c_void, data.len().try_into()?)
    });
    log::debug!("ibwrta({}, {:?}) -> {:?}", ud, data, status);
    if status.err {
        return Err(GpibError::DriverError(
            status,
            #[cfg(feature = "linuxgpib")]
            IbError::current_thread_local_error()?,
            #[cfg(feature = "nigpib")]
            unsafe { IbError::current_global_error() }?,
        ));
    } else {
        Ok(())
    }
}

/// ibwrtf -- write data bytes from file (board or device)
/// See: [Linux GPIB Reference](https://linux-gpib.sourceforge.io/doc_html/reference-function-ibwrtf.html)
pub fn ibwrtf(ud: c_int, file_path: &Path) -> Result<usize, GpibError> {
    let file_path = CString::new(file_path.to_str().ok_or(GpibError::ValueError(format!(
        "Unable to convert path '{:?}' to string",
        file_path
    )))?)?;
    let status = IbStatus::from_ibsta(unsafe { linux_gpib_sys::ibwrtf(ud, file_path.as_ptr()) });
    if status.err {
        Err(GpibError::DriverError(
            status,
            #[cfg(feature = "linuxgpib")]
            IbError::current_thread_local_error()?,
            #[cfg(feature = "nigpib")]
            unsafe { IbError::current_global_error() }?,
        ))
    } else {
        Ok(
            #[cfg(feature = "linuxgpib")]
            ThreadIbcntl().try_into()?,
            #[cfg(feature = "nigpib")]
            Ibcnt().try_into()?,
        )
    }
}
