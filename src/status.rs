use linux_gpib_sys::{
    ibsta_bit_numbers_ATN_NUM, ibsta_bit_numbers_CIC_NUM, ibsta_bit_numbers_CMPL_NUM,
    ibsta_bit_numbers_DCAS_NUM, ibsta_bit_numbers_DTAS_NUM, ibsta_bit_numbers_END_NUM,
    ibsta_bit_numbers_ERR_NUM, ibsta_bit_numbers_EVENT_NUM, ibsta_bit_numbers_LACS_NUM,
    ibsta_bit_numbers_LOK_NUM, ibsta_bit_numbers_REM_NUM, ibsta_bit_numbers_RQS_NUM,
    ibsta_bit_numbers_SPOLL_NUM, ibsta_bit_numbers_SRQI_NUM, ibsta_bit_numbers_TACS_NUM,
    ibsta_bit_numbers_TIMO_NUM,
};
use std::default::Default;
use std::fmt;

pub struct IbStatus {
    pub dcas: bool,
    pub dtas: bool,
    pub lacs: bool,
    pub tacs: bool,
    pub atn: bool,
    pub cic: bool,
    pub rem: bool,
    pub lok: bool,
    pub cmpl: bool,
    pub event: bool,
    pub spoll: bool,
    pub rqs: bool,
    pub srqi: bool,
    pub end: bool,
    pub timo: bool,
    pub err: bool,
}

impl IbStatus {
    /// Get current value of from Linux-GPIB ibsta global variable
    pub fn current_status() -> IbStatus {
        IbStatus::from_ibsta(unsafe { linux_gpib_sys::ibsta })
    }

    /// Convert c_int status value to IbStatus
    pub fn from_ibsta(ibsta: i32) -> IbStatus {
        let dcas = ((1 << ibsta_bit_numbers_DCAS_NUM) & ibsta) != 0;
        let dtas = ((1 << ibsta_bit_numbers_DTAS_NUM) & ibsta) != 0;
        let lacs = ((1 << ibsta_bit_numbers_LACS_NUM) & ibsta) != 0;
        let tacs = ((1 << ibsta_bit_numbers_TACS_NUM) & ibsta) != 0;
        let atn = ((1 << ibsta_bit_numbers_ATN_NUM) & ibsta) != 0;
        let cic = ((1 << ibsta_bit_numbers_CIC_NUM) & ibsta) != 0;
        let rem = ((1 << ibsta_bit_numbers_REM_NUM) & ibsta) != 0;
        let lok = ((1 << ibsta_bit_numbers_LOK_NUM) & ibsta) != 0;
        let cmpl = ((1 << ibsta_bit_numbers_CMPL_NUM) & ibsta) != 0;
        let event = ((1 << ibsta_bit_numbers_EVENT_NUM) & ibsta) != 0;
        let spoll = ((1 << ibsta_bit_numbers_SPOLL_NUM) & ibsta) != 0;
        let rqs = ((1 << ibsta_bit_numbers_RQS_NUM) & ibsta) != 0;
        let srqi = ((1 << ibsta_bit_numbers_SRQI_NUM) & ibsta) != 0;
        let end = ((1 << ibsta_bit_numbers_END_NUM) & ibsta) != 0;
        let timo = ((1 << ibsta_bit_numbers_TIMO_NUM) & ibsta) != 0;
        let err = ((1 << ibsta_bit_numbers_ERR_NUM) & ibsta) != 0;
        IbStatus {
            dcas,
            dtas,
            lacs,
            tacs,
            atn,
            cic,
            rem,
            lok,
            cmpl,
            event,
            spoll,
            rqs,
            srqi,
            end,
            timo,
            err,
        }
    }

    /// Convert IbStatus to Linux GPIB c_int status
    pub fn as_ibsta(&self) -> i32 {
        let mut ibsta = 0;
        if self.dcas {
            ibsta = ibsta | (1 << ibsta_bit_numbers_DCAS_NUM);
        }
        if self.dtas {
            ibsta = ibsta | (1 << ibsta_bit_numbers_DTAS_NUM);
        }
        if self.lacs {
            ibsta = ibsta | (1 << ibsta_bit_numbers_LACS_NUM);
        }
        if self.tacs {
            ibsta = ibsta | (1 << ibsta_bit_numbers_TACS_NUM);
        }
        if self.atn {
            ibsta = ibsta | (1 << ibsta_bit_numbers_ATN_NUM);
        }
        if self.cic {
            ibsta = ibsta | (1 << ibsta_bit_numbers_CIC_NUM);
        }
        if self.rem {
            ibsta = ibsta | (1 << ibsta_bit_numbers_REM_NUM);
        }
        if self.lok {
            ibsta = ibsta | (1 << ibsta_bit_numbers_LOK_NUM);
        }
        if self.cmpl {
            ibsta = ibsta | (1 << ibsta_bit_numbers_CMPL_NUM);
        }
        if self.event {
            ibsta = ibsta | (1 << ibsta_bit_numbers_EVENT_NUM);
        }
        if self.spoll {
            ibsta = ibsta | (1 << ibsta_bit_numbers_SPOLL_NUM);
        }
        if self.rqs {
            ibsta = ibsta | (1 << ibsta_bit_numbers_RQS_NUM);
        }
        if self.srqi {
            ibsta = ibsta | (1 << ibsta_bit_numbers_SRQI_NUM);
        }
        if self.end {
            ibsta = ibsta | (1 << ibsta_bit_numbers_END_NUM);
        }
        if self.timo {
            ibsta = ibsta | (1 << ibsta_bit_numbers_TIMO_NUM);
        }
        if self.err {
            ibsta = ibsta | (1 << ibsta_bit_numbers_ERR_NUM);
        }
        ibsta
    }

    pub fn with_dcas(mut self, dcas: bool) -> Self {
        self.dcas = dcas;
        self
    }
    pub fn with_dtas(mut self, dtas: bool) -> Self {
        self.dtas = dtas;
        self
    }
    pub fn with_lacs(mut self, lacs: bool) -> Self {
        self.lacs = lacs;
        self
    }
    pub fn with_tacs(mut self, tacs: bool) -> Self {
        self.tacs = tacs;
        self
    }
    pub fn with_atn(mut self, atn: bool) -> Self {
        self.atn = atn;
        self
    }
    pub fn with_cic(mut self, cic: bool) -> Self {
        self.cic = cic;
        self
    }
    pub fn with_rem(mut self, rem: bool) -> Self {
        self.rem = rem;
        self
    }
    pub fn with_lok(mut self, lok: bool) -> Self {
        self.lok = lok;
        self
    }
    pub fn with_cmpl(mut self, cmpl: bool) -> Self {
        self.cmpl = cmpl;
        self
    }
    pub fn with_event(mut self, event: bool) -> Self {
        self.event = event;
        self
    }
    pub fn with_spoll(mut self, spoll: bool) -> Self {
        self.spoll = spoll;
        self
    }
    pub fn with_rqs(mut self, rqs: bool) -> Self {
        self.rqs = rqs;
        self
    }
    pub fn with_srqi(mut self, srqi: bool) -> Self {
        self.srqi = srqi;
        self
    }
    pub fn with_end(mut self, end: bool) -> Self {
        self.end = end;
        self
    }
    pub fn with_timo(mut self, timo: bool) -> Self {
        self.timo = timo;
        self
    }
    pub fn with_err(mut self, err: bool) -> Self {
        self.err = err;
        self
    }
}

impl fmt::Debug for IbStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut description = String::new();
        if self.dcas {
            description.push_str("DCAS (device clear) ");
        }
        if self.dtas {
            description.push_str("DTAS (device trigger) ");
        }
        if self.lacs {
            description.push_str("LACS (board is currently addressed as a listener) ");
        }
        if self.tacs {
            description.push_str("TACS (board is currently addressed as a talker) ");
        }
        if self.atn {
            description.push_str("ATN (ATN line is asserted) ");
        }
        if self.cic {
            description.push_str("CIC (board is controller-in-charge, able to set the ATN line) ");
        }
        if self.rem {
            description.push_str("REM (board is in 'remote' state) ");
        }
        if self.lok {
            description.push_str("LOK (board is in 'lockout' state) ");
        }
        if self.cmpl {
            description.push_str("CMPL (I/O operation complete) ");
        }
        if self.event {
            description
                .push_str("EVENT (one or more clear, trigger, or interface clear event received) ");
        }
        if self.spoll {
            description.push_str("SPOLL (board is serial polled) ");
        }
        if self.rqs {
            description.push_str("RQS (device has requested service) ");
        }
        if self.srqi {
            description
                .push_str("SRQI (a device connected to the board is asserting the SRQ line) ");
        }
        if self.end {
            description.push_str("END (last I/O operation ended with the EOI line asserted) ");
        }
        if self.timo {
            description.push_str("TIMO (last I/O operation, or ibwait, timed out) ");
        }
        if self.err {
            description.push_str("ERR (last function call failed)");
        }
        if description.len() > 0 {
            write!(f, "IbStatus({description})")
        } else {
            write!(f, "IbStatus(No flag set)")
        }
    }
}

impl fmt::Display for IbStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut description = String::new();
        if self.dcas {
            description.push_str("DCAS ");
        }
        if self.dtas {
            description.push_str("DTAS ");
        }
        if self.lacs {
            description.push_str("LACS ");
        }
        if self.tacs {
            description.push_str("TACS ");
        }
        if self.atn {
            description.push_str("ATN ");
        }
        if self.cic {
            description.push_str("CIC ");
        }
        if self.rem {
            description.push_str("REM ");
        }
        if self.lok {
            description.push_str("LOK ");
        }
        if self.cmpl {
            description.push_str("CMPL ");
        }
        if self.event {
            description.push_str("EVENT ");
        }
        if self.spoll {
            description.push_str("SPOLL ");
        }
        if self.rqs {
            description.push_str("RQS ");
        }
        if self.srqi {
            description.push_str("SRQI ");
        }
        if self.end {
            description.push_str("END ");
        }
        if self.timo {
            description.push_str("TIMO ");
        }
        if self.err {
            description.push_str("ERR");
        }
        if description.len() > 0 {
            write!(f, "IbStatus({description})")
        } else {
            write!(f, "IbStatus(No flag set)")
        }
    }
}

impl Default for IbStatus {
    fn default() -> Self {
        Self {
            dcas: false,
            dtas: false,
            lacs: false,
            tacs: false,
            atn: false,
            cic: false,
            rem: false,
            lok: false,
            cmpl: false,
            event: false,
            spoll: false,
            rqs: false,
            srqi: false,
            end: false,
            timo: false,
            err: false,
        }
    }
}
