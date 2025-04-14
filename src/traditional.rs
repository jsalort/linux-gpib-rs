use crate::error::{GpibError, IbError};
use crate::status::IbStatus;
use std::ffi::{CStr, CString};
use std::fmt;
use std::os::raw::{c_char, c_int, c_short, c_void};
use std::path::Path;
use std::pin::Pin;

pub enum IbOption {
    PAD,
    SAD,
    TMO,
    EOT,
    PPC,
    READDR,
    AUTOPOLL,
    CICPROT,
    SC,
    SRE,
    EOSrd,
    EOSwrt,
    EOScmp,
    EOSchar,
    PP2,
    TIMING,
    ReadAdjust,
    WriteAdjust,
    EventQueue,
    SPollBit,
    SendLLO,
    SPollTime,
    PPollTime,
    EndBitIsNormal,
    UnAddr,
    HSCableLength,
    Ist,
    Rsv,
    BNA,
    SevenBitEOS,
}

impl fmt::Display for IbOption {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            IbOption::PAD => {
                write!(f, "IbOption::PAD")
            }
            IbOption::SAD => {
                write!(f, "IbOption::SAD")
            }
            IbOption::TMO => {
                write!(f, "IbOption::TMO")
            }
            IbOption::EOT => {
                write!(f, "IbOption::EOT")
            }
            IbOption::PPC => {
                write!(f, "IbOption::PPC")
            }
            IbOption::READDR => {
                write!(f, "IbOption::READDR")
            }
            IbOption::AUTOPOLL => {
                write!(f, "IbOption::AUTOPOLL")
            }
            IbOption::CICPROT => {
                write!(f, "IbOption::CICPROT")
            }
            IbOption::SC => {
                write!(f, "IbOption::SC")
            }
            IbOption::SRE => {
                write!(f, "IbOption::SRE")
            }
            IbOption::EOSrd => {
                write!(f, "IbOption::EOSrd")
            }
            IbOption::EOSwrt => {
                write!(f, "IbOption::EOSwrt")
            }
            IbOption::EOScmp => {
                write!(f, "IbOption::EOScmp")
            }
            IbOption::EOSchar => {
                write!(f, "IbOption::EOSchar")
            }
            IbOption::PP2 => {
                write!(f, "IbOption::PP2")
            }
            IbOption::TIMING => {
                write!(f, "IbOption::TIMING")
            }
            IbOption::ReadAdjust => {
                write!(f, "IbOption::ReadAdjust")
            }
            IbOption::WriteAdjust => {
                write!(f, "IbOption::WriteAdjust")
            }
            IbOption::EventQueue => {
                write!(f, "IbOption::EventQueue")
            }
            IbOption::SPollBit => {
                write!(f, "IbOption::SPollBit")
            }
            IbOption::SendLLO => {
                write!(f, "IbOption::SendLLO")
            }
            IbOption::SPollTime => {
                write!(f, "IbOption::SPollTime")
            }
            IbOption::PPollTime => {
                write!(f, "IbOption::PPollTime")
            }
            IbOption::EndBitIsNormal => {
                write!(f, "IbOption::EndBitIsNormal")
            }
            IbOption::UnAddr => {
                write!(f, "IbOption::UnAddr")
            }
            IbOption::HSCableLength => {
                write!(f, "IbOption::HSCableLength")
            }
            IbOption::Ist => {
                write!(f, "IbOption::Ist")
            }
            IbOption::Rsv => {
                write!(f, "IbOption::Rsv")
            }
            IbOption::BNA => {
                write!(f, "IbOption::BNA")
            }
            IbOption::SevenBitEOS => {
                write!(f, "IbOption::SevenBitEOS")
            }
        }
    }
}

impl fmt::Debug for IbOption {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            IbOption::PAD => {
                write!(f, "IbOption::PAD (0x1): GPIB primary address")
            }
            IbOption::SAD => {
                write!(f, "IbOption::SAD (0x2): GPIB secondary address (0 for none, 0x60 to 0x7e for secondary addresses 0 to 30)")
            }
            IbOption::TMO => {
                write!(f, "IbOption::TMO (0x3): Timeout setting for io operations (a number from 0 to 17). See ibmto().")
            }
            IbOption::EOT => {
                write!(f, "IbOption::EOT (0x4): Nonzero if EOI is asserted with last byte on writes. See ibeot().")
            }
            IbOption::PPC => {
                write!(
                    f,
                    "IbOption::PPC (0x5): Parallel poll configuration. See ibppc()."
                )
            }
            IbOption::READDR => {
                write!(
                    f,
                    "IbOption::READDR (0x6): Useless, included for compatibility only."
                )
            }
            IbOption::AUTOPOLL => {
                write!(
                    f,
                    "IbOption::AUTOPOLL (0x7): Nonzero if automatic serial polling is enabled."
                )
            }
            IbOption::CICPROT => {
                write!(
                    f,
                    "IbOption::CICPROT (0x8): Useless, included for compatibility only."
                )
            }
            IbOption::SC => {
                write!(
                    f,
                    "IbOption::SC (0xa): Nonzero if board is system controller. See ibrsc(). "
                )
            }
            IbOption::SRE => {
                write!(f, "IbOption::SRE (0xb): Nonzero if board autmatically asserts REN line when it becomes the system controller. See ibsre().")
            }
            IbOption::EOSrd => {
                write!(f, "IbOption::EOSrd (0xc): Nonzero if termination of reads on reception of the end-of-string character is enabled. See ibeos(), in particular the REOS bit.")
            }
            IbOption::EOSwrt => {
                write!(f, "IbOption::EOSwrt (0xd): Nonzero if EOI is asserted whenever end-of-string character is sent. See ibeos(), in particular the XEOS bit.")
            }
            IbOption::EOScmp => {
                write!(f, "IbOption::EOScmp (0xe): Nonzero if all 8 bits are used to match end-of-string character. Zero if only least significant 7 bits are used. See ibeos(), in particular the BIN bit.")
            }
            IbOption::EOSchar => {
                write!(f, "IbOption::EOSchar (0xf): The end-of-string byte.")
            }
            IbOption::PP2 => {
                write!(f, "IbOption::PP2 (0x10): Nonzero if in local parallel poll configure mode. Zero if in remote parallel poll configure mode.")
            }
            IbOption::TIMING => {
                write!(f, "IbOption::TIMING (0x11): Number indicating T1 delay. 1 for 2 microseconds, 2 for 500 nanoseconds, 3 for 350 nanoseconds. The values are declared in the header files as the constants T1_DELAY_2000ns, T1_DELAY_500ns, and T1_DELAY_350ns.")
            }
            IbOption::ReadAdjust => {
                write!(f, "IbOption::ReadAdjust (0x13): Nonzero if byte pairs are automatically swapped during reads.")
            }
            IbOption::WriteAdjust => {
                write!(f, "IbOption::WriteAdjust (0x14): Nonzero if byte pairs are automatically swapped during writes.")
            }
            IbOption::EventQueue => {
                write!(
                    f,
                    "IbOption::EventQueue (0x15): Nonzero if event queue is enabled."
                )
            }
            IbOption::SPollBit => {
                write!(f, "IbOption::SPollBit (0x16): Nonzero if the use of the SPOLL bit in ibsta is enabled.")
            }
            IbOption::SendLLO => {
                write!(f, "IbOption::SendLLO (0x17): Nonzero if devices connected to this board are automatically put into local lockout mode when brought online with ibfind() or ibdev().")
            }
            IbOption::SPollTime => {
                write!(f, "IbOption::SPollTime (0x18): Timeout for serial polls. The value of the result is between 0 and 17, and has the same meaning as in ibtmo().")
            }
            IbOption::PPollTime => {
                write!(f, "IbOption::PPollTime (0x19): Timeout for parallel polls. The value of the result is between 0 and 17, and has the same meaning as in ibtmo().")
            }
            IbOption::EndBitIsNormal => {
                write!(f, "IbOption::EndBitIsNormal (0x1a): Nonzero if END bit of ibsta is set on reception of end-of-string character or EOI. Zero if END bit is only set on EOI.")
            }
            IbOption::UnAddr => {
                write!(f, "IbOption::UnAddr (0x1b): Nonzero if UNT (untalk) and UNL (unlisten) commands are automatically sent after a completed io operation using this descriptor.")
            }
            IbOption::HSCableLength => {
                write!(
                    f,
                    "IbOption::HSCableLength (0x1f): Useless, included only for compatibility."
                )
            }
            IbOption::Ist => {
                write!(
                    f,
                    "IbOption::Ist (0x20): Individual status bit, a.k.a. 'ist'."
                )
            }
            IbOption::Rsv => {
                write!(f, "IbOption::Rsv (0x21): The current status byte this board will use to respond to serial polls.")
            }
            IbOption::BNA => {
                write!(f, "IbOption::BNA (0x200): For a device: the board index (minor number) of interface board through which the device is being accessed. For a board: the board index of the board itself.")
            }
            IbOption::SevenBitEOS => {
                write!(f, "IbOption::SevenBitEOS (0x1000): Nonzero if board supports 7 bit EOS comparisons. See ibeos(), in particular the BIN bit. This is a Linux-GPIB extension.")
            }
        }
    }
}

impl IbOption {
    pub fn as_option(&self) -> c_int {
        match self {
            IbOption::PAD => 0x1,
            IbOption::SAD => 0x2,
            IbOption::TMO => 0x3,
            IbOption::EOT => 0x4,
            IbOption::PPC => 0x5,
            IbOption::READDR => 0x6,
            IbOption::AUTOPOLL => 0x7,
            IbOption::CICPROT => 0x8,
            IbOption::SC => 0xa,
            IbOption::SRE => 0xb,
            IbOption::EOSrd => 0xc,
            IbOption::EOSwrt => 0xd,
            IbOption::EOScmp => 0xe,
            IbOption::EOSchar => 0xf,
            IbOption::PP2 => 0x10,
            IbOption::TIMING => 0x11,
            IbOption::ReadAdjust => 0x13,
            IbOption::WriteAdjust => 0x14,
            IbOption::EventQueue => 0x15,
            IbOption::SPollBit => 0x16,
            IbOption::SendLLO => 0x17,
            IbOption::SPollTime => 0x18,
            IbOption::PPollTime => 0x19,
            IbOption::EndBitIsNormal => 0x1a,
            IbOption::UnAddr => 0x1b,
            IbOption::HSCableLength => 0x1f,
            IbOption::Ist => 0x20,
            IbOption::Rsv => 0x21,
            IbOption::BNA => 0x200,
            IbOption::SevenBitEOS => 0x1000,
        }
    }
}

/// ibask -- query configuration (board or device)
/// See: https://linux-gpib.sourceforge.io/doc_html/reference-function-ibask.html
pub fn ibask(ud: c_int, option: IbOption) -> Result<c_int, GpibError> {
    let option = option.as_option();
    let mut result: c_int = 0;
    let status = IbStatus::from_ibsta(unsafe {
        linux_gpib_sys::ibask(ud, option, &mut result as *mut c_int)
    });
    if status.err {
        Err(GpibError::DriverError(status, IbError::current_error()?))
    } else {
        Ok(result)
    }
}

/// ibbna -- change access board (device)
/// See: https://linux-gpib.sourceforge.io/doc_html/reference-function-ibbna.html
pub fn ibbna(ud: c_int, name: &str) -> Result<(), GpibError> {
    let name = CString::new(name)?;
    let status =
        IbStatus::from_ibsta(unsafe { linux_gpib_sys::ibbna(ud, name.as_ptr() as *mut c_char) });
    if status.err {
        Err(GpibError::DriverError(status, IbError::current_error()?))
    } else {
        Ok(())
    }
}

/// ibcac -- assert ATN (board)
/// See: https://linux-gpib.sourceforge.io/doc_html/reference-function-ibcac.html
pub fn ibcac(ud: c_int, synchronous: c_int) -> Result<(), GpibError> {
    let status = IbStatus::from_ibsta(unsafe { linux_gpib_sys::ibcac(ud, synchronous) });
    if status.err {
        Err(GpibError::DriverError(status, IbError::current_error()?))
    } else {
        Ok(())
    }
}

/// ibclr -- clear device (device)
/// See: https://linux-gpib.sourceforge.io/doc_html/reference-function-ibclr.html
pub fn ibclr(ud: c_int) -> Result<(), GpibError> {
    let status = IbStatus::from_ibsta(unsafe { linux_gpib_sys::ibclr(ud) });
    if status.err {
        Err(GpibError::DriverError(status, IbError::current_error()?))
    } else {
        Ok(())
    }
}

/// ibcmd -- write command bytes (board)
/// See: https://linux-gpib.sourceforge.io/doc_html/reference-function-ibcmd.html
pub fn ibcmd(ud: c_int, commands: &[u8]) -> Result<(), GpibError> {
    let status = IbStatus::from_ibsta(unsafe {
        linux_gpib_sys::ibcmd(
            ud,
            commands.as_ptr() as *const c_void,
            commands.len().try_into()?,
        )
    });
    if status.err {
        Err(GpibError::DriverError(status, IbError::current_error()?))
    } else {
        Ok(())
    }
}

/// ibcmda -- write command bytes asynchronously (board)
/// See: https://linux-gpib.sourceforge.io/doc_html/reference-function-ibcmda.html
pub fn ibcmda(ud: c_int, commands: Pin<&mut [u8]>) -> Result<(), GpibError> {
    let status = IbStatus::from_ibsta(unsafe {
        linux_gpib_sys::ibcmda(
            ud,
            commands.as_ptr() as *const c_void,
            commands.len().try_into()?,
        )
    });
    if status.err {
        Err(GpibError::DriverError(status, IbError::current_error()?))
    } else {
        Ok(())
    }
}

/// ibconfig -- change configuration (board or device)
/// See: https://linux-gpib.sourceforge.io/doc_html/reference-function-ibconfig.html
pub fn ibconfig(ud: c_int, option: IbOption, setting: c_int) -> Result<(), GpibError> {
    let option = option.as_option();
    let status = IbStatus::from_ibsta(unsafe { linux_gpib_sys::ibconfig(ud, option, setting) });
    if status.err {
        Err(GpibError::DriverError(status, IbError::current_error()?))
    } else {
        Ok(())
    }
}

pub enum IbTimeout {
    TNone,
    T10us,
    T30us,
    T100us,
    T300us,
    T1ms,
    T3ms,
    T10ms,
    T30ms,
    T100ms,
    T300ms,
    T1s,
    T3s,
    T10s,
    T30s,
    T100s,
    T300s,
    T1000s,
}

impl fmt::Display for IbTimeout {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            IbTimeout::TNone => {
                write!(f, "Never timeout")
            }
            IbTimeout::T10us => {
                write!(f, "10 microseconds")
            }
            IbTimeout::T30us => {
                write!(f, "30 microseconds")
            }
            IbTimeout::T100us => {
                write!(f, "100 microseconds")
            }
            IbTimeout::T300us => {
                write!(f, "300 microseconds")
            }
            IbTimeout::T1ms => {
                write!(f, "1 millisecond")
            }
            IbTimeout::T3ms => {
                write!(f, "3 milliseconds")
            }
            IbTimeout::T10ms => {
                write!(f, "10 milliseconds")
            }
            IbTimeout::T30ms => {
                write!(f, "30 milliseconds")
            }
            IbTimeout::T100ms => {
                write!(f, "100 milliseconds")
            }
            IbTimeout::T300ms => {
                write!(f, "300 milliseconds")
            }
            IbTimeout::T1s => {
                write!(f, "1 second")
            }
            IbTimeout::T3s => {
                write!(f, "3 seconds")
            }
            IbTimeout::T10s => {
                write!(f, "10 seconds")
            }
            IbTimeout::T30s => {
                write!(f, "30 seconds")
            }
            IbTimeout::T100s => {
                write!(f, "100 seconds")
            }
            IbTimeout::T300s => {
                write!(f, "300 seconds")
            }
            IbTimeout::T1000s => {
                write!(f, "1000 seconds")
            }
        }
    }
}

impl IbTimeout {
    fn as_timeout(&self) -> c_int {
        match self {
            IbTimeout::TNone => 0,
            IbTimeout::T10us => 1,
            IbTimeout::T30us => 2,
            IbTimeout::T100us => 3,
            IbTimeout::T300us => 4,
            IbTimeout::T1ms => 5,
            IbTimeout::T3ms => 6,
            IbTimeout::T10ms => 7,
            IbTimeout::T30ms => 8,
            IbTimeout::T100ms => 9,
            IbTimeout::T300ms => 10,
            IbTimeout::T1s => 11,
            IbTimeout::T3s => 12,
            IbTimeout::T10s => 13,
            IbTimeout::T30s => 14,
            IbTimeout::T100s => 15,
            IbTimeout::T300s => 16,
            IbTimeout::T1000s => 17,
        }
    }
}

#[derive(Copy, Clone)]
pub struct PrimaryAddress {
    pad: c_int,
}

impl PrimaryAddress {
    pub fn new(pad: c_int) -> Result<PrimaryAddress, GpibError> {
        if pad >= 0 && pad <= 30 {
            Ok(PrimaryAddress { pad })
        } else {
            Err(GpibError::ValueError(format!(
                "Primary address must be between 0 and 30. Got: {}.",
                pad
            )))
        }
    }

    fn as_pad(&self) -> c_int {
        self.pad
    }
}

#[derive(Copy, Clone)]
pub struct SecondaryAddress {
    sad: c_int,
}

impl SecondaryAddress {
    pub fn new(sad: c_int) -> Result<SecondaryAddress, GpibError> {
        let desc = "Secondary address must be between 0 and 30 (without the 0x60 prefix), or equivalently between 0x60 and 0x7e (with the 0x60 addition). sad = 0 disables secondary address.";
        let sad = if sad < 0 {
            return Err(GpibError::ValueError(desc.to_owned()));
        } else if sad == 0 {
            // disable secondary address
            sad
        } else if sad <= 30 {
            // sad are between 0 and 30 but
            // NI convention adds 0x60 to the secondary address
            sad + 0x60
        } else if sad >= 0x60 && sad <= 0x7e {
            sad
        } else {
            return Err(GpibError::ValueError(desc.to_owned()));
        };
        Ok(SecondaryAddress { sad })
    }

    fn as_sad(&self) -> c_int {
        self.sad
    }
}

impl Default for SecondaryAddress {
    fn default() -> SecondaryAddress {
        SecondaryAddress { sad: 0 }
    }
}

#[derive(Copy, Clone)]
pub enum IbSendEOI {
    Disabled,
    Enabled(c_int),
}

impl IbSendEOI {
    fn as_eot(&self) -> c_int {
        match self {
            IbSendEOI::Disabled => 0,
            IbSendEOI::Enabled(val) => *val,
        }
    }
}

impl Default for IbSendEOI {
    fn default() -> IbSendEOI {
        IbSendEOI::Disabled
    }
}

#[derive(Copy, Clone)]
pub struct IbEosMode {
    pub reos: bool,
    pub xeos: bool,
    pub bin: bool,
}

impl fmt::Display for IbEosMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut description = String::new();
        if self.reos {
            description.push_str("REOS ");
        }
        if self.xeos {
            description.push_str("XEOS ");
        }
        if self.bin {
            description.push_str("BIN");
        }
        if description.len() > 0 {
            write!(f, "EosMod({description})")
        } else {
            write!(f, "EosMod(No flag set)")
        }
    }
}

impl fmt::Debug for IbEosMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut description = String::new();
        if self.reos {
            description.push_str(
                "REOS (0x400) Enable termination of reads when eos character is received.",
            );
        }
        if self.xeos {
            description.push_str("XEOS (0x800) Assert the EOI line whenever the eos character is sent during writes.");
        }
        if self.bin {
            description.push_str("BIN (0x1000) Match eos character using all 8 bits (instead of only looking at the 7 least significant bits).");
        }
        if description.len() > 0 {
            write!(f, "EosMod({description})")
        } else {
            write!(f, "EosMod(No flag set)")
        }
    }
}

impl IbEosMode {
    pub fn as_mode(&self) -> c_int {
        let mut mode = 0;
        if self.reos {
            mode = mode | 0x400;
        }
        if self.xeos {
            mode = mode | 0x800;
        }
        if self.bin {
            mode = mode | 0x1000;
        }
        mode
    }
}

impl Default for IbEosMode {
    fn default() -> IbEosMode {
        IbEosMode {
            reos: true,
            xeos: false,
            bin: false,
        }
    }
}

/// ibdev -- open a device (device)
/// See: https://linux-gpib.sourceforge.io/doc_html/reference-function-ibdev.html
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
    if ud >= 0 {
        Ok(ud)
    } else {
        Err(GpibError::DriverError(
            IbStatus::current_status(),
            IbError::current_error()?,
        ))
    }
}

/// ibeos -- set end-of-string mode (board or device)
/// See: https://linux-gpib.sourceforge.io/doc_html/reference-function-ibeos.html
pub fn ibeos(ud: c_int, eosmod: IbEosMode) -> Result<(), GpibError> {
    let eosmod = eosmod.as_mode();
    let status = IbStatus::from_ibsta(unsafe { linux_gpib_sys::ibeos(ud, eosmod) });
    if status.err {
        Err(GpibError::DriverError(status, IbError::current_error()?))
    } else {
        Ok(())
    }
}

/// ibeot -- assert EOI with last data byte (board or device)
/// See: https://linux-gpib.sourceforge.io/doc_html/reference-function-ibeot.html
pub fn ibeot(ud: c_int, send_eoi: IbSendEOI) -> Result<(), GpibError> {
    let status = IbStatus::from_ibsta(unsafe { linux_gpib_sys::ibeot(ud, send_eoi.as_eot()) });
    if status.err {
        Err(GpibError::DriverError(status, IbError::current_error()?))
    } else {
        Ok(())
    }
}

pub enum IbEvent {
    None,
    DevTrg,
    DevClr,
    IFC,
}

impl fmt::Display for IbEvent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            IbEvent::None => {
                write!(f, "None")
            }
            IbEvent::DevTrg => {
                write!(f, "DevTrg")
            }
            IbEvent::DevClr => {
                write!(f, "DevClr")
            }
            IbEvent::IFC => {
                write!(f, "IFC")
            }
        }
    }
}

impl fmt::Debug for IbEvent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            IbEvent::None => {
                write!(f, "None (The board's event queue is empty)")
            }
            IbEvent::DevTrg => {
                write!(f, "DevTrg (The board has received a trigger command from the controller-in-charge)")
            }
            IbEvent::DevClr => {
                write!(
                    f,
                    "DevClr (The board has received a clear command from the controller-in-charge)"
                )
            }
            IbEvent::IFC => {
                write!(f, "IFC (The board has received an interface clear from the system controller. Note, some models of GPIB interface board lack the ability to report interface clear events)")
            }
        }
    }
}

impl IbEvent {
    fn from_value(value: c_short) -> Result<IbEvent, GpibError> {
        match value {
            0 => Ok(IbEvent::None),
            1 => Ok(IbEvent::DevTrg),
            2 => Ok(IbEvent::DevClr),
            3 => Ok(IbEvent::IFC),
            other => Err(GpibError::ValueError(format!(
                "Unexpected value ({}) for event.",
                other,
            ))),
        }
    }
}

/// ibevent -- get events from event queue (board)
/// See: https://linux-gpib.sourceforge.io/doc_html/reference-function-ibevent.html
pub fn ibevent(ud: c_int) -> Result<IbEvent, GpibError> {
    let mut event_value: c_short = 0;
    let status = IbStatus::from_ibsta(unsafe {
        linux_gpib_sys::ibevent(ud, &mut event_value as *mut c_short)
    });
    if status.err {
        Err(GpibError::DriverError(status, IbError::current_error()?))
    } else {
        Ok(IbEvent::from_value(event_value)?)
    }
}

/// ibfind -- open a board or device (board or device)
/// See: https://linux-gpib.sourceforge.io/doc_html/reference-function-ibfind.html
pub fn ibfind(name: &str) -> Result<c_int, GpibError> {
    let name = CString::new(name)?;
    let ud = unsafe { linux_gpib_sys::ibfind(name.as_ptr()) };
    if ud >= 0 {
        Ok(ud)
    } else {
        Err(GpibError::DriverError(
            IbStatus::current_status(),
            IbError::current_error()?,
        ))
    }
}

/// ibgts -- release ATN (board)
/// See: https://linux-gpib.sourceforge.io/doc_html/reference-function-ibgts.html
pub fn ibgts(ud: c_int, shadow_handshake: c_int) -> Result<(), GpibError> {
    let status = IbStatus::from_ibsta(unsafe { linux_gpib_sys::ibgts(ud, shadow_handshake) });
    if status.err {
        Err(GpibError::DriverError(status, IbError::current_error()?))
    } else {
        Ok(())
    }
}

/// ibist -- set individual status bit (board)
/// See: https://linux-gpib.sourceforge.io/doc_html/reference-function-ibist.html
pub fn ibist(ud: c_int, ist: c_int) -> Result<(), GpibError> {
    let status = IbStatus::from_ibsta(unsafe { linux_gpib_sys::ibist(ud, ist) });
    if status.err {
        Err(GpibError::DriverError(status, IbError::current_error()?))
    } else {
        Ok(())
    }
}

pub struct IbLineStatus {
    pub valid_dav: bool,
    pub valid_ndac: bool,
    pub valid_nrfd: bool,
    pub valid_ifc: bool,
    pub valid_ren: bool,
    pub valid_srq: bool,
    pub valid_atn: bool,
    pub valid_eoi: bool,
    pub bus_dav: bool,
    pub bus_ndac: bool,
    pub bus_nrfd: bool,
    pub bus_ifc: bool,
    pub bus_ren: bool,
    pub bus_srq: bool,
    pub bus_atn: bool,
    pub bus_eoi: bool,
}

impl IbLineStatus {
    fn from_line_status(line_status: c_short) -> IbLineStatus {
        let valid_dav = (line_status & 0x1) != 0;
        let valid_ndac = (line_status & 0x2) != 0;
        let valid_nrfd = (line_status & 0x4) != 0;
        let valid_ifc = (line_status & 0x8) != 0;
        let valid_ren = (line_status & 0x10) != 0;
        let valid_srq = (line_status & 0x20) != 0;
        let valid_atn = (line_status & 0x40) != 0;
        let valid_eoi = (line_status & 0x80) != 0;
        let bus_dav = (line_status & 0x100) != 0;
        let bus_ndac = (line_status & 0x200) != 0;
        let bus_nrfd = (line_status & 0x400) != 0;
        let bus_ifc = (line_status & 0x800) != 0;
        let bus_ren = (line_status & 0x1000) != 0;
        let bus_srq = (line_status & 0x2000) != 0;
        let bus_atn = (line_status & 0x4000) != 0;
        let bus_eoi = (line_status & 0x8000u16 as i16) != 0;
        Self {
            valid_dav,
            valid_ndac,
            valid_nrfd,
            valid_ifc,
            valid_ren,
            valid_srq,
            valid_atn,
            valid_eoi,
            bus_dav,
            bus_ndac,
            bus_nrfd,
            bus_ifc,
            bus_ren,
            bus_srq,
            bus_atn,
            bus_eoi,
        }
    }
}

/// iblines -- monitor bus lines (board)
/// See: https://linux-gpib.sourceforge.io/doc_html/reference-function-iblines.html
pub fn iblines(ud: c_int) -> Result<IbLineStatus, GpibError> {
    let mut line_status: c_short = 0;
    let status = IbStatus::from_ibsta(unsafe {
        linux_gpib_sys::iblines(ud, &mut line_status as *mut c_short)
    });
    if status.err {
        Err(GpibError::DriverError(status, IbError::current_error()?))
    } else {
        Ok(IbLineStatus::from_line_status(line_status))
    }
}

/// ibln -- check if listener is present (board or device)
/// See: https://linux-gpib.sourceforge.io/doc_html/reference-function-ibln.html
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
        Err(GpibError::DriverError(status, IbError::current_error()?))
    } else {
        Ok(found_listener != 0)
    }
}

/// ibloc -- go to local mode (board or device)
/// See: https://linux-gpib.sourceforge.io/doc_html/reference-function-ibloc.html
pub fn ibloc(ud: c_int) -> Result<(), GpibError> {
    let status = IbStatus::from_ibsta(unsafe { linux_gpib_sys::ibloc(ud) });
    if status.err {
        Err(GpibError::DriverError(status, IbError::current_error()?))
    } else {
        Ok(())
    }
}

pub enum IbOnline {
    Close,
    Reset(c_int),
}

impl IbOnline {
    fn as_online(&self) -> c_int {
        match self {
            IbOnline::Close => 0,
            IbOnline::Reset(val) => *val,
        }
    }
}

/// ibonl -- close or reinitialize descriptor (board or device)
/// See: https://linux-gpib.sourceforge.io/doc_html/reference-function-ibonl.html
pub fn ibonl(ud: c_int, online: IbOnline) -> Result<(), GpibError> {
    let online = online.as_online();
    let status = IbStatus::from_ibsta(unsafe { linux_gpib_sys::ibonl(ud, online) });
    if status.err {
        Err(GpibError::DriverError(status, IbError::current_error()?))
    } else {
        Ok(())
    }
}

/// ibpad -- set primary GPIB address (board or device)
/// See: https://linux-gpib.sourceforge.io/doc_html/reference-function-ibpad.html
pub fn ibpad(ud: c_int, primary_address: PrimaryAddress) -> Result<(), GpibError> {
    let status =
        IbStatus::from_ibsta(unsafe { linux_gpib_sys::ibpad(ud, primary_address.as_pad()) });
    if status.err {
        Err(GpibError::DriverError(status, IbError::current_error()?))
    } else {
        Ok(())
    }
}

/// ibpct -- pass control (board)
/// See: https://linux-gpib.sourceforge.io/doc_html/reference-function-ibpct.html
pub fn ibpct(ud: c_int) -> Result<(), GpibError> {
    let status = IbStatus::from_ibsta(unsafe { linux_gpib_sys::ibpct(ud) });
    if status.err {
        Err(GpibError::DriverError(status, IbError::current_error()?))
    } else {
        Ok(())
    }
}

/// ibppc -- parallel poll configure (board or device)
/// See: https://linux-gpib.sourceforge.io/doc_html/reference-function-ibppc.html
pub fn ibppc(ud: c_int, configuration: c_int) -> Result<(), GpibError> {
    let status = IbStatus::from_ibsta(unsafe { linux_gpib_sys::ibppc(ud, configuration) });
    if status.err {
        Err(GpibError::DriverError(status, IbError::current_error()?))
    } else {
        Ok(())
    }
}

/// ibrd -- read data bytes (board or device)
/// See: https://linux-gpib.sourceforge.io/doc_html/reference-function-ibrd.html
pub fn ibrd(ud: c_int, buffer: &mut [u8]) -> Result<usize, GpibError> {
    let status = IbStatus::from_ibsta(unsafe {
        linux_gpib_sys::ibrd(
            ud,
            buffer.as_mut_ptr() as *mut c_void,
            buffer.len().try_into()?,
        )
    });
    if status.err {
        Err(GpibError::DriverError(status, IbError::current_error()?))
    } else {
        let bytes_read = unsafe { linux_gpib_sys::ibcntl };
        if bytes_read > buffer.len().try_into()? {
            Err(GpibError::ValueError(format!(
                "bytes_read ({}) > buffer.len() ({})",
                bytes_read,
                buffer.len(),
            )))
        } else {
            Ok(bytes_read.try_into()?)
        }
    }
}

/// ibrda -- read data bytes asynchronously (board or device)
/// See: https://linux-gpib.sourceforge.io/doc_html/reference-function-ibrda.html
pub fn ibrda(ud: c_int, mut buffer: Pin<&mut [u8]>) -> Result<(), GpibError> {
    let status = IbStatus::from_ibsta(unsafe {
        linux_gpib_sys::ibrda(
            ud,
            buffer.as_mut_ptr() as *mut c_void,
            buffer.len().try_into()?,
        )
    });
    if status.err {
        Err(GpibError::DriverError(status, IbError::current_error()?))
    } else {
        Ok(())
    }
}

/// ibrdf -- read data bytes to file (board or device)
/// See: https://linux-gpib.sourceforge.io/doc_html/reference-function-ibrdf.html
pub fn ibrdf(ud: c_int, file_path: &Path) -> Result<(), GpibError> {
    let file_path = CString::new(file_path.to_str().ok_or(GpibError::ValueError(format!(
        "Unable to convert path '{:?}' to string",
        file_path
    )))?)?;
    let status = IbStatus::from_ibsta(unsafe { linux_gpib_sys::ibrdf(ud, file_path.as_ptr()) });
    if status.err {
        Err(GpibError::DriverError(status, IbError::current_error()?))
    } else {
        Ok(())
    }
}

/// ibrpp -- perform a parallel poll (board or device)
/// See: https://linux-gpib.sourceforge.io/doc_html/reference-function-ibrpp.html
pub fn ibrpp(ud: c_int) -> Result<c_char, GpibError> {
    let mut ppoll_result: c_char = 0;
    let status = IbStatus::from_ibsta(unsafe {
        linux_gpib_sys::ibrpp(ud, &mut ppoll_result as *mut c_char)
    });
    if status.err {
        Err(GpibError::DriverError(status, IbError::current_error()?))
    } else {
        Ok(ppoll_result)
    }
}

/// ibrsc -- request system control (board)
/// See: https://linux-gpib.sourceforge.io/doc_html/reference-function-ibrsc.html
pub fn ibrsc(ud: c_int, request_control: c_int) -> Result<(), GpibError> {
    let status = IbStatus::from_ibsta(unsafe { linux_gpib_sys::ibrsc(ud, request_control) });
    if status.err {
        Err(GpibError::DriverError(status, IbError::current_error()?))
    } else {
        Ok(())
    }
}

/// ibrsp --  read status byte / serial poll (device)
/// See: https://linux-gpib.sourceforge.io/doc_html/reference-function-ibrsp.html
pub fn ibrsp(ud: c_int) -> Result<c_char, GpibError> {
    let mut result: c_char = 0;
    let status =
        IbStatus::from_ibsta(unsafe { linux_gpib_sys::ibrsp(ud, &mut result as *mut c_char) });
    if status.err {
        Err(GpibError::DriverError(status, IbError::current_error()?))
    } else {
        Ok(result)
    }
}

/// ibrsv -- request service (board)
/// See: https://linux-gpib.sourceforge.io/doc_html/reference-function-ibrsv.html
pub fn ibrsv(ud: c_int, status_byte: c_int) -> Result<(), GpibError> {
    let status = IbStatus::from_ibsta(unsafe { linux_gpib_sys::ibrsv(ud, status_byte) });
    if status.err {
        Err(GpibError::DriverError(status, IbError::current_error()?))
    } else {
        Ok(())
    }
}

/// ibrsv2 -- request service (board)
/// See: https://linux-gpib.sourceforge.io/doc_html/reference-function-ibrsv2.html
pub fn ibrsv2(
    ud: c_int,
    status_byte: c_int,
    new_reason_for_request: c_int,
) -> Result<(), GpibError> {
    let status = IbStatus::from_ibsta(unsafe {
        linux_gpib_sys::ibrsv2(ud, status_byte, new_reason_for_request)
    });
    if status.err {
        Err(GpibError::DriverError(status, IbError::current_error()?))
    } else {
        Ok(())
    }
}

/// ibsad -- set secondary GPIB address (board or device)
/// See: https://linux-gpib.sourceforge.io/doc_html/reference-function-ibsad.html
pub fn ibsad(ud: c_int, secondary_address: SecondaryAddress) -> Result<(), GpibError> {
    let status =
        IbStatus::from_ibsta(unsafe { linux_gpib_sys::ibsad(ud, secondary_address.as_sad()) });
    if status.err {
        Err(GpibError::DriverError(status, IbError::current_error()?))
    } else {
        Ok(())
    }
}

/// ibsic -- perform interface clear (board)
/// See: https://linux-gpib.sourceforge.io/doc_html/reference-function-ibsic.html
pub fn ibsic(ud: c_int) -> Result<(), GpibError> {
    let status = IbStatus::from_ibsta(unsafe { linux_gpib_sys::ibsic(ud) });
    if status.err {
        Err(GpibError::DriverError(status, IbError::current_error()?))
    } else {
        Ok(())
    }
}

/// ibspb --  obtain length of serial poll bytes queue (device)
/// See: https://linux-gpib.sourceforge.io/doc_html/reference-function-ibspb.html
pub fn ibspb(ud: c_int) -> Result<c_short, GpibError> {
    let mut result: c_short = 0;
    let status =
        IbStatus::from_ibsta(unsafe { linux_gpib_sys::ibspb(ud, &mut result as *mut c_short) });
    if status.err {
        Err(GpibError::DriverError(status, IbError::current_error()?))
    } else {
        Ok(result)
    }
}

/// ibsre -- set remote enable (board)
/// See: https://linux-gpib.sourceforge.io/doc_html/reference-function-ibsre.html
pub fn ibsre(ud: c_int, enable: c_int) -> Result<(), GpibError> {
    let status = IbStatus::from_ibsta(unsafe { linux_gpib_sys::ibsre(ud, enable) });
    if status.err {
        Err(GpibError::DriverError(status, IbError::current_error()?))
    } else {
        Ok(())
    }
}

/// ibstop -- abort asynchronous i/o operation (board or device)
/// See: https://linux-gpib.sourceforge.io/doc_html/reference-function-ibstop.html
pub fn ibstop(ud: c_int) -> Result<(), GpibError> {
    let status = IbStatus::from_ibsta(unsafe { linux_gpib_sys::ibstop(ud) });
    if status.err {
        Err(GpibError::DriverError(status, IbError::current_error()?))
    } else {
        Ok(())
    }
}

/// ibtmo -- adjust io timeout (board or device)
/// See: https://linux-gpib.sourceforge.io/doc_html/reference-function-ibtmo.html
pub fn ibtmo(ud: c_int, timeout: IbTimeout) -> Result<(), GpibError> {
    let timeout = timeout.as_timeout();
    let status = IbStatus::from_ibsta(unsafe { linux_gpib_sys::ibtmo(ud, timeout) });
    if status.err {
        Err(GpibError::DriverError(status, IbError::current_error()?))
    } else {
        Ok(())
    }
}

/// ibtrg -- trigger device (device)
/// See: https://linux-gpib.sourceforge.io/doc_html/reference-function-ibtrg.html
pub fn ibtrg(ud: c_int) -> Result<(), GpibError> {
    let status = IbStatus::from_ibsta(unsafe { linux_gpib_sys::ibtrg(ud) });
    if status.err {
        Err(GpibError::DriverError(status, IbError::current_error()?))
    } else {
        Ok(())
    }
}

/// ibvers -- Obtain the current linux gpib version
/// See: https://linux-gpib.sourceforge.io/doc_html/reference-function-ibvers.html
pub fn ibvers() -> Result<String, GpibError> {
    let mut buffer_ptr: *mut c_char = std::ptr::null_mut();
    unsafe { linux_gpib_sys::ibvers(&mut buffer_ptr as *mut *mut c_char) }
    Ok(unsafe { CStr::from_ptr(buffer_ptr) }.to_str()?.to_owned())
}

/// ibwait -- wait for event (board or device)
/// See: https://linux-gpib.sourceforge.io/doc_html/reference-function-ibwait.html
pub fn ibwait(ud: c_int, status_mask: IbStatus) -> Result<(), GpibError> {
    let status_mask = status_mask.as_ibsta();
    let status = IbStatus::from_ibsta(unsafe { linux_gpib_sys::ibwait(ud, status_mask) });
    if status.err {
        Err(GpibError::DriverError(status, IbError::current_error()?))
    } else {
        Ok(())
    }
}

/// ibwrt -- write data bytes (board or device)
/// See: https://linux-gpib.sourceforge.io/doc_html/reference-function-ibwrt.html
pub fn ibwrt(ud: c_int, data: &[u8]) -> Result<usize, GpibError> {
    let status = IbStatus::from_ibsta(unsafe {
        linux_gpib_sys::ibwrt(ud, data.as_ptr() as *const c_void, data.len().try_into()?)
    });
    if status.err {
        Err(GpibError::DriverError(status, IbError::current_error()?))
    } else {
        Ok(unsafe { linux_gpib_sys::ibcntl }.try_into()?)
    }
}

/// ibwrta -- write data bytes asynchronously (board or device)
/// See: https://linux-gpib.sourceforge.io/doc_html/reference-function-ibwrta.html
pub fn ibwrta(ud: c_int, data: Pin<&[u8]>) -> Result<(), GpibError> {
    let status = IbStatus::from_ibsta(unsafe {
        linux_gpib_sys::ibwrta(ud, data.as_ptr() as *const c_void, data.len().try_into()?)
    });
    if status.err {
        Err(GpibError::DriverError(status, IbError::current_error()?))
    } else {
        Ok(())
    }
}

/// ibwrtf -- write data bytes from file (board or device)
/// See: https://linux-gpib.sourceforge.io/doc_html/reference-function-ibwrtf.html
pub fn ibwrtf(ud: c_int, file_path: &Path) -> Result<usize, GpibError> {
    let file_path = CString::new(file_path.to_str().ok_or(GpibError::ValueError(format!(
        "Unable to convert path '{:?}' to string",
        file_path
    )))?)?;
    let status = IbStatus::from_ibsta(unsafe { linux_gpib_sys::ibwrtf(ud, file_path.as_ptr()) });
    if status.err {
        Err(GpibError::DriverError(status, IbError::current_error()?))
    } else {
        Ok(unsafe { linux_gpib_sys::ibcntl }.try_into()?)
    }
}
