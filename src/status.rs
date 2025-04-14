use linux_gpib_sys::{
    ibsta_bit_numbers_ATN_NUM, ibsta_bit_numbers_CIC_NUM, ibsta_bit_numbers_CMPL_NUM,
    ibsta_bit_numbers_DCAS_NUM, ibsta_bit_numbers_DTAS_NUM, ibsta_bit_numbers_END_NUM,
    ibsta_bit_numbers_ERR_NUM, ibsta_bit_numbers_EVENT_NUM, ibsta_bit_numbers_LACS_NUM,
    ibsta_bit_numbers_LOK_NUM, ibsta_bit_numbers_REM_NUM, ibsta_bit_numbers_RQS_NUM,
    ibsta_bit_numbers_SPOLL_NUM, ibsta_bit_numbers_SRQI_NUM, ibsta_bit_numbers_TACS_NUM,
    ibsta_bit_numbers_TIMO_NUM,
};
use std::fmt;
use std::default::Default;

#[derive(Debug)]
pub struct IbStatus {
    pub DCAS: bool,
    pub DTAS: bool,
    pub LACS: bool,
    pub TACS: bool,
    pub ATN: bool,
    pub CIC: bool,
    pub REM: bool,
    pub LOK: bool,
    pub CMPL: bool,
    pub EVENT: bool,
    pub SPOLL: bool,
    pub RQS: bool,
    pub SRQI: bool,
    pub END: bool,
    pub TIMO: bool,
    pub ERR: bool,
}

impl IbStatus {
    /// Get current value of from Linux-GPIB ibsta global variable
    pub fn current_status() -> IbStatus {
        IbStatus::from_ibsta(unsafe { linux_gpib_sys::ibsta })
    }

    /// Convert c_int status value to IbStatus
    pub fn from_ibsta(ibsta: i32) -> IbStatus {
        let DCAS = ((1 << ibsta_bit_numbers_DCAS_NUM) & ibsta) != 0;
        let DTAS = ((1 << ibsta_bit_numbers_DTAS_NUM) & ibsta) != 0;
        let LACS = ((1 << ibsta_bit_numbers_LACS_NUM) & ibsta) != 0;
        let TACS = ((1 << ibsta_bit_numbers_TACS_NUM) & ibsta) != 0;
        let ATN = ((1 << ibsta_bit_numbers_ATN_NUM) & ibsta) != 0;
        let CIC = ((1 << ibsta_bit_numbers_CIC_NUM) & ibsta) != 0;
        let REM = ((1 << ibsta_bit_numbers_REM_NUM) & ibsta) != 0;
        let LOK = ((1 << ibsta_bit_numbers_LOK_NUM) & ibsta) != 0;
        let CMPL = ((1 << ibsta_bit_numbers_CMPL_NUM) & ibsta) != 0;
        let EVENT = ((1 << ibsta_bit_numbers_EVENT_NUM) & ibsta) != 0;
        let SPOLL = ((1 << ibsta_bit_numbers_SPOLL_NUM) & ibsta) != 0;
        let RQS = ((1 << ibsta_bit_numbers_RQS_NUM) & ibsta) != 0;
        let SRQI = ((1 << ibsta_bit_numbers_SRQI_NUM) & ibsta) != 0;
        let END = ((1 << ibsta_bit_numbers_END_NUM) & ibsta) != 0;
        let TIMO = ((1 << ibsta_bit_numbers_TIMO_NUM) & ibsta) != 0;
        let ERR = ((1 << ibsta_bit_numbers_ERR_NUM) & ibsta) != 0;
        IbStatus {
            DCAS,
            DTAS,
            LACS,
            TACS,
            ATN,
            CIC,
            REM,
            LOK,
            CMPL,
            EVENT,
            SPOLL,
            RQS,
            SRQI,
            END,
            TIMO,
            ERR,
        }
    }

    /// Convert IbStatus to Linux GPIB c_int status
    pub fn as_ibsta(&self) -> i32 {
        let mut ibsta = 0;
        if self.DCAS {
            ibsta = ibsta | (1 << ibsta_bit_numbers_DCAS_NUM);
        }
        if self.DTAS {
            ibsta = ibsta | (1 << ibsta_bit_numbers_DTAS_NUM);
        }
        if self.LACS {
            ibsta = ibsta | (1 << ibsta_bit_numbers_LACS_NUM);
        }
        if self.TACS {
            ibsta = ibsta | (1 << ibsta_bit_numbers_TACS_NUM);
        }
        if self.ATN {
            ibsta = ibsta | (1 << ibsta_bit_numbers_ATN_NUM);
        }
        if self.CIC {
            ibsta = ibsta | (1 << ibsta_bit_numbers_CIC_NUM);
        }
        if self.REM {
            ibsta = ibsta | (1 << ibsta_bit_numbers_REM_NUM);
        }
        if self.LOK {
            ibsta = ibsta | (1 << ibsta_bit_numbers_LOK_NUM);
        }
        if self.CMPL {
            ibsta = ibsta | (1 << ibsta_bit_numbers_CMPL_NUM);
        }
        if self.EVENT {
            ibsta = ibsta | (1 << ibsta_bit_numbers_EVENT_NUM);
        }
        if self.SPOLL {
            ibsta = ibsta | (1 << ibsta_bit_numbers_SPOLL_NUM);
        }
        if self.RQS {
            ibsta = ibsta | (1 << ibsta_bit_numbers_RQS_NUM);
        }
        if self.SRQI {
            ibsta = ibsta | (1 << ibsta_bit_numbers_SRQI_NUM);
        }
        if self.END {
            ibsta = ibsta | (1 << ibsta_bit_numbers_END_NUM);
        }
        if self.TIMO {
            ibsta = ibsta | (1 << ibsta_bit_numbers_TIMO_NUM);
        }
        if self.ERR {
            ibsta = ibsta | (1 << ibsta_bit_numbers_ERR_NUM);
        }
        ibsta
    }

    pub fn with_DCAS(mut self, DCAS: bool) -> Self {
        self.DCAS = DCAS;
        self
    }
    pub fn with_DTAS(mut self, DTAS: bool) -> Self {
        self.DTAS = DTAS;
        self
    }
    pub fn with_LACS(mut self, LACS: bool) -> Self {
        self.LACS = LACS;
        self
    }
    pub fn with_TACS(mut self, TACS: bool) -> Self {
        self.TACS = TACS;
        self
    }
    pub fn with_ATN(mut self, ATN: bool) -> Self {
        self.ATN = ATN;
        self
    }
    pub fn with_CIC(mut self, CIC: bool) -> Self {
        self.CIC = CIC;
        self
    }
    pub fn with_REM(mut self, REM: bool) -> Self {
        self.REM = REM;
        self
    }
    pub fn with_LOK(mut self, LOK: bool) -> Self {
        self.LOK = LOK;
        self
    }
    pub fn with_CMPL(mut self, CMPL: bool) -> Self {
        self.CMPL = CMPL;
        self
    }
    pub fn with_EVENT(mut self, EVENT: bool) -> Self {
        self.EVENT = EVENT;
        self
    }
    pub fn with_SPOLL(mut self, SPOLL: bool) -> Self {
        self.SPOLL = SPOLL;
        self
    }
    pub fn with_RQS(mut self, RQS: bool) -> Self {
        self.RQS = RQS;
        self
    }
    pub fn with_SRQI(mut self, SRQI: bool) -> Self {
        self.SRQI = SRQI;
        self
    }
    pub fn with_END(mut self, END: bool) -> Self {
        self.END = END;
        self
    }
    pub fn with_TIMO(mut self, TIMO: bool) -> Self {
        self.TIMO = TIMO;
        self
    }
    pub fn with_ERR(mut self, ERR: bool) -> Self {
        self.ERR = ERR;
        self
    }
}

impl fmt::Display for IbStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut description = String::new();
        if self.DCAS {
            description.push_str("DCAS (device clear) ");
        }
        if self.DTAS {
            description.push_str("DTAS (device trigger) ");
        }
        if self.LACS {
            description.push_str("LACS (board is currently addressed as a listener) ");
        }
        if self.TACS {
            description.push_str("TACS (board is currently addressed as a talker) ");
        }
        if self.ATN {
            description.push_str("ATN (ATN line is asserted) ");
        }
        if self.CIC {
            description.push_str("CIC (board is controller-in-charge, able to set the ATN line) ");
        }
        if self.REM {
            description.push_str("REM (board is in 'remote' state) ");
        }
        if self.LOK {
            description.push_str("LOK (board is in 'lockout' state) ");
        }
        if self.CMPL {
            description.push_str("CMPL (I/O operation complete) ");
        }
        if self.EVENT {
            description.push_str(
                "EVENT (one or more clear, trigger, or interface clear event received) ",
            );
        }
        if self.SPOLL {
            description.push_str("SPOLL (board is serial polled) ");
        }
        if self.RQS {
            description.push_str("RQS (device has requested service) ");
        }
        if self.SRQI {
            description
                .push_str("SRQI (a device connected to the board is asserting the SRQ line) ");
        }
        if self.END {
            description.push_str("END (last I/O operation ended with the EOI line asserted) ");
        }
        if self.TIMO {
            description.push_str("TIMO (last I/O operation, or ibwait, timed out) ");
        }
        if self.ERR {
            description.push_str("ERR (last function call failed)");
        }
        write!(f, "IbStatus({description})")
    }
}

impl Default for IbStatus {
    fn default() -> Self {
        Self {
            DCAS: false,
            DTAS: false,
            LACS: false,
            TACS: false,
            ATN: false,
            CIC: false,
            REM: false,
            LOK: false,
            CMPL: false,
            EVENT: false,
            SPOLL: false,
            RQS: false,
            SRQI: false,
            END: false,
            TIMO: false,
            ERR: false,
        }
    }
}
