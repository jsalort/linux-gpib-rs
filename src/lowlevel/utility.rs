#![allow(non_snake_case)]

use crate::error::GpibError;
use crate::types::{PrimaryAddress, SecondaryAddress};
use linux_gpib_sys::{Addr4882_t, NOADDR};
use std::default::Default;
use std::fmt;
use std::os::raw::{c_int, c_long};

/// pack primary and secondary address into an Addr4882_t value
pub fn MakeAddr(pad: u16, sad: u16) -> Addr4882_t {
    let first_part: Addr4882_t = pad & 0xff;
    let second_part: u16 = (sad << 8) & 0xff00;
    first_part | second_part
}

/// extract primary address from an Addr4882_t value
pub fn GetPAD(address: Addr4882_t) -> u16 {
    address & 0xff
}

/// extract secondary address from an Addr4882_t value
pub fn GetSAD(address: Addr4882_t) -> u16 {
    (address >> 8) & 0xff
}

/// ibcnt value for the last asynchronous I/O operation resynchronized to the current thread is returned.
pub fn AsyncIbcnt() -> c_int {
    unsafe { linux_gpib_sys::AsyncIbcnt() }
}

/// ibcntl value for the last asynchronous I/O operation resynchronized to the current thread is returned.
pub fn AsyncIbcntl() -> c_long {
    unsafe { linux_gpib_sys::AsyncIbcntl() }
}

/// thread-specific ibcnt value
pub fn ThreadIbcnt() -> c_int {
    unsafe { linux_gpib_sys::ThreadIbcnt() }
}

/// thread-specific ibcntl value
pub fn ThreadIbcntl() -> c_long {
    unsafe { linux_gpib_sys::ThreadIbcntl() }
}

/// thread-specific iberr value
pub fn ThreadIberr() -> c_int {
    unsafe { linux_gpib_sys::ThreadIberr() }
}

/// thread-specific ibsta value
pub fn ThreadIbsta() -> c_int {
    unsafe { linux_gpib_sys::ThreadIbsta() }
}

/// iberr value for last asynchronous I/O operation
///
/// AsyncIberr() returns a thread-local error number related to the global variable iberr. Its value corresponds to the result of the last asynchronous I/O operation resynchronized to the current thread by an ibwait or ibstop call. This function only reflects the result of the asynchronous I/O operation itself and not, for example, the ibwait which resynchronized the asynchronous result to the current thread. Thus the result from AsyncIberr() is easier to interpret than ThreadIberr(), since it is unambiguous whether the value is associated with the asynchronous I/O result, or with the function call used to resynchronize (ibwait or ibstop).
///
/// See: [Linux GPIB Reference](https://linux-gpib.sourceforge.io/doc_html/reference-function-async-iberr.html)
pub fn AsyncIberr() -> c_int {
    unsafe { linux_gpib_sys::AsyncIberr() }
}

/// ibsta value for last asynchronous I/O operation
///
/// AsyncIbsta() returns a thread-local status value related to the global variable ibsta. Its value corresponds to the result of the last asynchronous I/O operation resynchronized to the current thread by an ibwait or ibstop call. This function only reflects the result of the asynchronous I/O operation itself and not, for example, the ibwait which resynchronized the asynchronous result to the current thread. Thus the result from AsyncIbsta() is easier to interpret than ThreadIbsta(), since it is unambiguous whether the value is associated with the asynchronous I/O result, or with the function call used to resynchronize (ibwait or ibstop).
///
/// Only the status bits END | ERR | TIMO | CMPL are valid in the returned status byte. The rest of the bits should be ignored and will be set to zero.
///
/// See: [Linux GPIB Reference](https://linux-gpib.sourceforge.io/doc_html/reference-function-async-ibsta.html)
pub fn AsyncIbsta() -> c_int {
    unsafe { linux_gpib_sys::AsyncIbsta() }
}

#[derive(Clone, Copy)]
pub struct Addr4882 {
    pub addr: linux_gpib_sys::Addr4882_t,
}

impl Addr4882 {
    pub fn new(
        primary_address: PrimaryAddress,
        secondary_address: SecondaryAddress,
    ) -> Result<Self, GpibError> {
        let addr = MakeAddr(
            primary_address.as_pad().try_into()?,
            secondary_address.as_sad().try_into()?,
        );
        Ok(Self { addr })
    }

    pub fn no_addr() -> Self {
        Self { addr: NOADDR }
    }

    pub fn pad(&self) -> u16 {
        GetPAD(self.addr)
    }

    pub fn sad(&self) -> u16 {
        GetSAD(self.addr)
    }

    pub fn primary_address(&self) -> Result<PrimaryAddress, GpibError> {
        PrimaryAddress::new(self.pad().into())
    }

    pub fn secondary_address(&self) -> Result<SecondaryAddress, GpibError> {
        SecondaryAddress::new(self.sad().into())
    }
}

impl Default for Addr4882 {
    fn default() -> Self {
        Addr4882::no_addr()
    }
}

impl fmt::Debug for Addr4882 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Addr4882({},{})", self.pad(), self.sad())
    }
}

impl fmt::Display for Addr4882 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}", self.pad(), self.sad())
    }
}
