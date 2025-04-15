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
