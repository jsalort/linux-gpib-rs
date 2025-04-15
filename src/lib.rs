//!
//! Low-level wrapper for Linux GPIB.
//! 
//! Documentation for the functions comes from [Linux GPIB Reference](https://linux-gpib.sourceforge.io/doc_html/reference.html).
//! At the moment, only the 'Traditional' API Functions are wrapped.
//! 
//! ## Requirements
//! 
//! This crate needs to link to an installed linux-gpib user library. It will look for `gpib/ib.h` in either `/usr/include` or `/usr/local/include`,
//! and for `libgpib.so` in either `/usr/lib` or `/usr/local/lib`.
//! 
//! 
//! ## Example
//! 
//! Add dependencies below to `Cargo.toml`
//! 
//! ```toml
//! linux-gpib-rs = { version = "0.1", features = ["async-tokio"] }
//! ```
//! 
//! Codes below will connect to the instrument on `GPIB0::1::INSTR` and print out its `*IDN?` response.
//! 
//! **Synchronous example**
//! 
//! We can use the low-level synchronous functions `ibrd` and `ibwrt`.
//! 
//! ```rust
//! use linux_gpib_rs::{
//!     OpenParam,
//!     open,
//!     ibwrt,
//!     ibrd,
//! };
//! use std::error::Error;
//! 
//! fn main() -> Result<(), Box<dyn Error>> {
//!     let ud = open("GPIB0::1::INSTR", OpenParam::default())?;
//!     ibwrt(ud, b"*IDN?\r\n")?;
//!     let mut buffer: [u8; 256] = [0; 256];
//!     ibrd(ud, &mut buffer)?;
//!     let iden = String::from_utf8(buffer.to_vec())?;
//!     println!("{iden}");
//!     Ok(())
//! }
//! ```
//! 
//! **Asynchronous example**
//! 
//! We can use slightly higher-level asynchronous functions `write` and `read` (based on `ibrda` and `ibwrta`).
//! This requires the `async-tokio` feature.
//! 
//! ```rust
//! use linux_gpib_rs::{open, write, read, OpenParam};
//! use std::error::Error;
//! 
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn Error>> {
//!     let ud = open("GPIB0::1::INSTR", OpenParam::default())?;
//!     write(ud, "*IDN?\r\n").await?;
//!     let iden = read(ud).await?;
//!     println!("{iden}");
//!     Ok(())
//! }
//! ```

mod error;
mod status;
mod traditional;

pub use error::{GpibError, IbError};
pub use status::IbStatus;
use std::os::raw::c_int;
pub use traditional::{
    ibask, ibbna, ibcac, ibclr, ibcmd, ibconfig, ibdev, ibeos, ibeot, ibevent, ibfind, ibgts,
    ibist, iblines, ibln, ibloc, ibonl, ibpad, ibpct, ibppc, ibrd, ibrdf, ibrpp, ibrsc, ibrsp,
    ibrsv, ibrsv2, ibsad, ibsic, ibspb, ibsre, ibstop, ibtmo, ibtrg, ibvers, ibwait, ibwrt, ibwrtf,
    IbEosMode, IbEvent, IbLineStatus, IbOnline, IbOption, IbSendEOI, IbTimeout, PrimaryAddress,
    SecondaryAddress,
};

#[cfg(feature = "async-tokio")]
mod asynchronous;

#[cfg(feature = "async-tokio")]
pub use asynchronous::{read, wait, write};

pub struct OpenParam {
    timeout: IbTimeout,
    send_eoi: IbSendEOI,
    eos_mode: IbEosMode,
}

impl Default for OpenParam {
    fn default() -> Self {
        Self {
            timeout: IbTimeout::T1s,
            send_eoi: IbSendEOI::default(),
            eos_mode: IbEosMode::default(),
        }
    }
}

/// Quickly open a device from a VISA-style address, e.g. 'GPIB0::1::INSTR'.
///
/// `timeout`, `send_eoi` and `eos_mode` are specified with an `OpenParam` structure.
/// Default parameters can be obtained with `OpenParam::default()`.
pub fn open(address: &str, params: OpenParam) -> Result<c_int, GpibError> {
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
        ibdev(
            board_number,
            PrimaryAddress::new(primary_address)?,
            SecondaryAddress::default(),
            params.timeout,
            params.send_eoi,
            params.eos_mode,
        )
    } else {
        Err(GpibError::ValueError(
            "Address is expected as GPIBN::primary_address::INSTR".to_owned(),
        ))
    }
}
