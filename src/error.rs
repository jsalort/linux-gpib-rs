#[cfg(feature = "linuxgpib")]
use crate::lowlevel::utility::{AsyncIberr, ThreadIberr};

use crate::status::IbStatus;
use std::convert::Infallible;
use std::error::Error;
use std::ffi::NulError;
use std::fmt;
use std::num::TryFromIntError;
use std::str::Utf8Error;
use std::string::FromUtf8Error;
#[cfg(feature = "async-tokio")]
use tokio::task::JoinError;

pub enum IbError {
    EDVR(i64), // In this case, we hold also ibcntl value
    ECIC,
    ENOL,
    EADR,
    EARG,
    ESAC,
    EABO,
    ENEB,
    EDMA,
    EOIP,
    ECAP,
    EFSO(i64), // In this case, we hold also ibcntl value
    EBUS,
    ESTB,
    ESRQ,
    ETAB,
}

pub enum GpibError {
    DriverError(IbStatus, IbError),
    Timeout,
    ValueError(String),
    #[cfg(feature = "async-tokio")]
    TokioError(JoinError),
}

impl Error for GpibError {}

impl fmt::Display for GpibError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            GpibError::DriverError(status, error) => {
                write!(f, "GpibError({}, {})", status, error)
            }
            GpibError::Timeout => {
                write!(f, "Timeout")
            }
            GpibError::ValueError(desc) => {
                write!(f, "ValueError({})", desc)
            }
            #[cfg(feature = "async-tokio")]
            GpibError::TokioError(e) => {
                write!(f, "Tokio Error ({})", e)
            }
        }
    }
}

impl fmt::Debug for GpibError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            GpibError::DriverError(status, error) => {
                write!(f, "GpibError({:?}, {:?})", status, error)
            }
            GpibError::Timeout => {
                write!(f, "Timeout")
            }
            GpibError::ValueError(desc) => {
                write!(f, "ValueError({})", desc)
            }
            #[cfg(feature = "async-tokio")]
            GpibError::TokioError(e) => {
                write!(f, "Tokio Error ({:?})", e)
            }
        }
    }
}

impl fmt::Display for IbError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            IbError::EDVR(ibcntl) => {
                write!(f, "EDVR  (ibcntl = {ibcntl})")
            }
            IbError::ECIC => {
                write!(f, "ECIC")
            }
            IbError::ENOL => {
                write!(f, "ENOL")
            }
            IbError::EADR => {
                write!(f, "EADR")
            }
            IbError::EARG => {
                write!(f, "EARG")
            }
            IbError::ESAC => {
                write!(f, "ESAC")
            }
            IbError::EABO => {
                write!(f, "EABO")
            }
            IbError::ENEB => {
                write!(f, "ENEB")
            }
            IbError::EDMA => {
                write!(f, "EDMA")
            }
            IbError::EOIP => {
                write!(f, "EOIP")
            }
            IbError::ECAP => {
                write!(f, "ECAP")
            }
            IbError::EFSO(ibcntl) => {
                write!(f, "EFSO (ibcntl = {ibcntl})")
            }
            IbError::EBUS => {
                write!(f, "EBUS")
            }
            IbError::ESTB => {
                write!(f, "ESTB")
            }
            IbError::ESRQ => {
                write!(f, "ESRQ")
            }
            IbError::ETAB => {
                write!(f, "ETAB")
            }
        }
    }
}

impl fmt::Debug for IbError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            IbError::EDVR(ibcntl) => {
                write!(f, "EDVR  (A system call has failed. ibcntl = {ibcntl})")
            }
            IbError::ECIC => {
                write!(
                    f,
                    "ECIC (Your interface board needs to be controller-in-charge, but is not)"
                )
            }
            IbError::ENOL => {
                write!(
                    f,
                    "ENOL (You have attempted to write data or command bytes, but there are no listeners currently addressed)"
                )
            }
            IbError::EADR => {
                write!(
                    f,
                    "EADR (The interface board has failed to address itself properly before starting an io operation)"
                )
            }
            IbError::EARG => {
                write!(
                    f,
                    "EARG (One or more arguments to the function call were invalid)"
                )
            }
            IbError::ESAC => {
                write!(
                    f,
                    "ESAC (The interface board needs to be system controller, but is not)"
                )
            }
            IbError::EABO => {
                write!(
                    f,
                    "EABO (A read or write of data bytes has been aborted, possibly due to a timeout or reception of a device clear command)"
                )
            }
            IbError::ENEB => {
                write!(
                    f,
                    "ENEB (The GPIB interface board does not exist, its driver is not loaded, or it is not configured properly)"
                )
            }
            IbError::EDMA => {
                write!(
                    f,
                    "EDMA (Not used DMA error, included for compatibility purposes)"
                )
            }
            IbError::EOIP => {
                write!(
                    f,
                    "EOIP (Function call can not proceed due to an asynchronous IO operation in progress)"
                )
            }
            IbError::ECAP => {
                write!(
                    f,
                    "ECAP (incapable of executing function call, due the GPIB board lacking the capability, or the capability being disabled in software)"
                )
            }
            IbError::EFSO(ibcntl) => {
                write!(f, "EFSO (file system error, ibcntl = {ibcntl})")
            }
            IbError::EBUS => {
                write!(
                    f,
                    "EBUS (an attempt to write command bytes to the bus has timed out)"
                )
            }
            IbError::ESTB => {
                write!(
                    f,
                    "ESTB (one or more serial poll status bytes have been lost. This can occur due to too many status bytes accumulating, through automatic serial polling, without being read)"
                )
            }
            IbError::ESRQ => {
                write!(
                    f,
                    "ESRQ (the serial poll request service line is stuck on. This can occur if a physical device on the bus requests service, but its GPIB address has not been opened by any process. Thus the automatic serial polling routines are unaware of the device's existence and will never serial poll it)"
                )
            }
            IbError::ETAB => {
                write!(
                    f,
                    "ETAB (this error can be returned by ibevent(), FindLstn(), or FindRQS(). See their descriptions for more information)"
                )
            }
        }
    }
}

impl IbError {
    /// Create IbError from iberr value
    pub fn from_iberr(iberr: linux_gpib_sys::iberr_type) -> Result<IbError, GpibError> {
        match iberr {
            #[cfg(feature = "linuxgpib")]
            0 => Ok(IbError::EDVR(unsafe { linux_gpib_sys::ibcntl })),
            #[cfg(feature = "nigpib")]
            0 => Ok(IbError::EDVR(unsafe { linux_gpib_sys::Ibcnt().into() })),
            1 => Ok(IbError::ECIC),
            2 => Ok(IbError::ENOL),
            3 => Ok(IbError::EADR),
            4 => Ok(IbError::EARG),
            5 => Ok(IbError::ESAC),
            6 => Ok(IbError::EABO),
            7 => Ok(IbError::ENEB),
            8 => Ok(IbError::EDMA),
            10 => Ok(IbError::EOIP),
            11 => Ok(IbError::ECAP),
            #[cfg(feature = "linuxgpib")]
            12 => Ok(IbError::EFSO(unsafe { linux_gpib_sys::ibcntl })),
            #[cfg(feature = "nigpib")]
            12 => Ok(IbError::EFSO(unsafe { linux_gpib_sys::Ibcnt().into() })),
            14 => Ok(IbError::EBUS),
            15 => Ok(IbError::ESTB),
            16 => Ok(IbError::ESRQ),
            20 => Ok(IbError::ETAB),
            other => Err(GpibError::ValueError(format!(
                "Unexpected iberr value = {}.",
                other
            ))),
        }
    }

    /// Create IbError from current Linux-GPIB global iberr variable
    pub unsafe fn current_global_error() -> Result<IbError, GpibError> {
        let status = unsafe { IbStatus::current_global_status() };
        if status.err {
            #[cfg(feature = "linuxgpib")]
            return IbError::from_iberr(unsafe { linux_gpib_sys::iberr });
            #[cfg(feature = "nigpib")]
            return IbError::from_iberr(unsafe { linux_gpib_sys::Iberr() });
        } else {
            Err(GpibError::ValueError(format!(
                "Unable to get error because is not ERR (status = {:?})",
                status
            )))
        }
    }

    #[cfg(feature = "linuxgpib")]
    /// Create IbError from current thread-local iberr value
    pub fn current_thread_local_error() -> Result<IbError, GpibError> {
        let status = IbStatus::current_thread_local_status();
        if status.err {
            IbError::from_iberr(ThreadIberr())
        } else {
            Err(GpibError::ValueError(format!(
                "Unable to get error because is not ERR (status = {:?})",
                status
            )))
        }
    }

    #[cfg(feature = "linuxgpib")]
    /// Create IbError for last asynchronous I/O operation
    pub fn current_async_local_error() -> Result<IbError, GpibError> {
        let status = IbStatus::current_async_local_status();
        if status.err {
            IbError::from_iberr(AsyncIberr())
        } else {
            Err(GpibError::ValueError(format!(
                "Unable to get error because is not ERR (status = {:?})",
                status
            )))
        }
    }
}

impl From<NulError> for GpibError {
    fn from(e: NulError) -> GpibError {
        GpibError::ValueError(format!("{:?}", e))
    }
}

impl From<TryFromIntError> for GpibError {
    fn from(e: TryFromIntError) -> GpibError {
        GpibError::ValueError(format!("{:?}", e,))
    }
}

impl From<FromUtf8Error> for GpibError {
    fn from(e: FromUtf8Error) -> GpibError {
        GpibError::ValueError(format!("{:?}", e,))
    }
}

impl From<Utf8Error> for GpibError {
    fn from(e: Utf8Error) -> GpibError {
        GpibError::ValueError(format!("{:?}", e,))
    }
}

impl From<Infallible> for GpibError {
    fn from(e: Infallible) -> GpibError {
        GpibError::ValueError(e.to_string())
    }
}

#[cfg(feature = "async-tokio")]
impl From<JoinError> for GpibError {
    fn from(e: JoinError) -> GpibError {
        GpibError::TokioError(e)
    }
}
