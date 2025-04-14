mod error;
mod status;
mod traditional;

pub use error::{GpibError, IbError};
pub use status::IbStatus;
pub use traditional::{
    ibask, ibbna, ibcac, ibclr, ibcmd, ibcmda, ibconfig, ibdev, ibeos, ibeot, ibevent, ibfind,
    ibgts, ibist, iblines, ibln, ibloc, ibonl, ibpad, ibpct, ibppc, ibrd, ibrda, ibrdf, ibrpp, ibrsc,
    ibrsp, ibrsv, ibrsv2, ibsad, ibsic, ibspb, ibsre, ibstop, ibtmo, ibtrg, ibvers, ibwait, ibwrt,
    ibwrta, ibwrtf, IbEosMode, IbEvent, IbLineStatus, IbOption, IbTimeout, PrimaryAddress,
    SecondaryAddress, IbSendEOI
};

#[cfg(feature = "async-tokio")]
mod asynchronous;

#[cfg(feature = "async-tokio")]
pub use asynchronous::{read, wait, write};
