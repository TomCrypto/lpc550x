#[doc = r"Register block"]
#[repr(C)]
pub struct SECCHANNEL {
    #[doc = "0x00 - Configuration register 1 for channel pair."]
    pub pcfg1: PCFG1,
    #[doc = "0x04 - Configuration register 2 for channel pair."]
    pub pcfg2: PCFG2,
    #[doc = "0x08 - Status register for channel pair."]
    pub pstat: PSTAT,
}
#[doc = "PCFG1 (rw) register accessor: an alias for `Reg<PCFG1_SPEC>`"]
pub type PCFG1 = crate::Reg<pcfg1::PCFG1_SPEC>;
#[doc = "Configuration register 1 for channel pair."]
pub mod pcfg1;
#[doc = "PCFG2 (rw) register accessor: an alias for `Reg<PCFG2_SPEC>`"]
pub type PCFG2 = crate::Reg<pcfg2::PCFG2_SPEC>;
#[doc = "Configuration register 2 for channel pair."]
pub mod pcfg2;
#[doc = "PSTAT (rw) register accessor: an alias for `Reg<PSTAT_SPEC>`"]
pub type PSTAT = crate::Reg<pstat::PSTAT_SPEC>;
#[doc = "Status register for channel pair."]
pub mod pstat;
