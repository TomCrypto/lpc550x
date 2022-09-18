#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SCT configuration register."]
    pub config: CONFIG,
    #[doc = "0x04 - SCT control register."]
    pub ctrl: CTRL,
    #[doc = "0x08 - SCT limit event select register."]
    pub limit: LIMIT,
    #[doc = "0x0c - SCT halt event select register."]
    pub halt: HALT,
    #[doc = "0x10 - SCT stop event select register."]
    pub stop: STOP,
    #[doc = "0x14 - SCT start event select register."]
    pub start: START,
    _reserved6: [u8; 0x28],
    #[doc = "0x40 - SCT counter register."]
    pub count: COUNT,
    #[doc = "0x44 - SCT state register."]
    pub state: STATE,
    #[doc = "0x48 - SCT input register."]
    pub input: INPUT,
    #[doc = "0x4c - SCT match/capture mode register."]
    pub regmode: REGMODE,
    #[doc = "0x50 - SCT output register."]
    pub output: OUTPUT,
    #[doc = "0x54 - SCT output counter direction control register."]
    pub outputdirctrl: OUTPUTDIRCTRL,
    #[doc = "0x58 - SCT conflict resolution register."]
    pub res: RES,
    #[doc = "0x5c - SCT DMA request 0 register."]
    pub dmareq0: DMAREQ0,
    #[doc = "0x60 - SCT DMA request 1 register."]
    pub dmareq1: DMAREQ1,
    _reserved15: [u8; 0x8c],
    #[doc = "0xf0 - SCT event interrupt enable register."]
    pub even: EVEN,
    #[doc = "0xf4 - SCT event flag register."]
    pub evflag: EVFLAG,
    #[doc = "0xf8 - SCT conflict interrupt enable register."]
    pub conen: CONEN,
    #[doc = "0xfc - SCT conflict flag register."]
    pub conflag: CONFLAG,
    _reserved_19_cap0: [u8; 0x04],
    _reserved_20_cap1: [u8; 0x04],
    _reserved_21_cap2: [u8; 0x04],
    _reserved_22_cap3: [u8; 0x04],
    _reserved_23_cap4: [u8; 0x04],
    _reserved_24_cap5: [u8; 0x04],
    _reserved_25_cap6: [u8; 0x04],
    _reserved_26_cap7: [u8; 0x04],
    _reserved_27_cap8: [u8; 0x04],
    _reserved_28_cap9: [u8; 0x04],
    _reserved_29_cap10: [u8; 0x04],
    _reserved_30_cap11: [u8; 0x04],
    _reserved_31_cap12: [u8; 0x04],
    _reserved_32_cap13: [u8; 0x04],
    _reserved_33_cap14: [u8; 0x04],
    _reserved_34_cap15: [u8; 0x04],
    _reserved35: [u8; 0xc0],
    _reserved_35_capctrl0: [u8; 0x04],
    _reserved_36_capctrl1: [u8; 0x04],
    _reserved_37_capctrl2: [u8; 0x04],
    _reserved_38_capctrl3: [u8; 0x04],
    _reserved_39_capctrl4: [u8; 0x04],
    _reserved_40_capctrl5: [u8; 0x04],
    _reserved_41_capctrl6: [u8; 0x04],
    _reserved_42_capctrl7: [u8; 0x04],
    _reserved_43_capctrl8: [u8; 0x04],
    _reserved_44_capctrl9: [u8; 0x04],
    _reserved_45_capctrl10: [u8; 0x04],
    _reserved_46_capctrl11: [u8; 0x04],
    _reserved_47_capctrl12: [u8; 0x04],
    _reserved_48_capctrl13: [u8; 0x04],
    _reserved_49_capctrl14: [u8; 0x04],
    _reserved_50_capctrl15: [u8; 0x04],
    _reserved51: [u8; 0xc0],
    #[doc = "0x300..0x380 - no description available."]
    pub ev: [EV; 16],
    _reserved52: [u8; 0x0180],
    #[doc = "0x500..0x550 - no description available."]
    pub out: [OUT; 10],
}
impl RegisterBlock {
    #[doc = "0x100 - SCT match value register of match channels."]
    #[inline(always)]
    pub fn match0(&self) -> &MATCH0 {
        unsafe { &*(((self as *const Self) as *const u8).add(256usize) as *const MATCH0) }
    }
    #[doc = "0x100 - SCT capture register of capture channel."]
    #[inline(always)]
    pub fn cap0(&self) -> &CAP0 {
        unsafe { &*(((self as *const Self) as *const u8).add(256usize) as *const CAP0) }
    }
    #[doc = "0x104 - SCT match value register of match channels."]
    #[inline(always)]
    pub fn match1(&self) -> &MATCH1 {
        unsafe { &*(((self as *const Self) as *const u8).add(260usize) as *const MATCH1) }
    }
    #[doc = "0x104 - SCT capture register of capture channel."]
    #[inline(always)]
    pub fn cap1(&self) -> &CAP1 {
        unsafe { &*(((self as *const Self) as *const u8).add(260usize) as *const CAP1) }
    }
    #[doc = "0x108 - SCT match value register of match channels."]
    #[inline(always)]
    pub fn match2(&self) -> &MATCH2 {
        unsafe { &*(((self as *const Self) as *const u8).add(264usize) as *const MATCH2) }
    }
    #[doc = "0x108 - SCT capture register of capture channel."]
    #[inline(always)]
    pub fn cap2(&self) -> &CAP2 {
        unsafe { &*(((self as *const Self) as *const u8).add(264usize) as *const CAP2) }
    }
    #[doc = "0x10c - SCT match value register of match channels."]
    #[inline(always)]
    pub fn match3(&self) -> &MATCH3 {
        unsafe { &*(((self as *const Self) as *const u8).add(268usize) as *const MATCH3) }
    }
    #[doc = "0x10c - SCT capture register of capture channel."]
    #[inline(always)]
    pub fn cap3(&self) -> &CAP3 {
        unsafe { &*(((self as *const Self) as *const u8).add(268usize) as *const CAP3) }
    }
    #[doc = "0x110 - SCT match value register of match channels."]
    #[inline(always)]
    pub fn match4(&self) -> &MATCH4 {
        unsafe { &*(((self as *const Self) as *const u8).add(272usize) as *const MATCH4) }
    }
    #[doc = "0x110 - SCT capture register of capture channel."]
    #[inline(always)]
    pub fn cap4(&self) -> &CAP4 {
        unsafe { &*(((self as *const Self) as *const u8).add(272usize) as *const CAP4) }
    }
    #[doc = "0x114 - SCT match value register of match channels."]
    #[inline(always)]
    pub fn match5(&self) -> &MATCH5 {
        unsafe { &*(((self as *const Self) as *const u8).add(276usize) as *const MATCH5) }
    }
    #[doc = "0x114 - SCT capture register of capture channel."]
    #[inline(always)]
    pub fn cap5(&self) -> &CAP5 {
        unsafe { &*(((self as *const Self) as *const u8).add(276usize) as *const CAP5) }
    }
    #[doc = "0x118 - SCT match value register of match channels."]
    #[inline(always)]
    pub fn match6(&self) -> &MATCH6 {
        unsafe { &*(((self as *const Self) as *const u8).add(280usize) as *const MATCH6) }
    }
    #[doc = "0x118 - SCT capture register of capture channel."]
    #[inline(always)]
    pub fn cap6(&self) -> &CAP6 {
        unsafe { &*(((self as *const Self) as *const u8).add(280usize) as *const CAP6) }
    }
    #[doc = "0x11c - SCT match value register of match channels."]
    #[inline(always)]
    pub fn match7(&self) -> &MATCH7 {
        unsafe { &*(((self as *const Self) as *const u8).add(284usize) as *const MATCH7) }
    }
    #[doc = "0x11c - SCT capture register of capture channel."]
    #[inline(always)]
    pub fn cap7(&self) -> &CAP7 {
        unsafe { &*(((self as *const Self) as *const u8).add(284usize) as *const CAP7) }
    }
    #[doc = "0x120 - SCT match value register of match channels."]
    #[inline(always)]
    pub fn match8(&self) -> &MATCH8 {
        unsafe { &*(((self as *const Self) as *const u8).add(288usize) as *const MATCH8) }
    }
    #[doc = "0x120 - SCT capture register of capture channel."]
    #[inline(always)]
    pub fn cap8(&self) -> &CAP8 {
        unsafe { &*(((self as *const Self) as *const u8).add(288usize) as *const CAP8) }
    }
    #[doc = "0x124 - SCT match value register of match channels."]
    #[inline(always)]
    pub fn match9(&self) -> &MATCH9 {
        unsafe { &*(((self as *const Self) as *const u8).add(292usize) as *const MATCH9) }
    }
    #[doc = "0x124 - SCT capture register of capture channel."]
    #[inline(always)]
    pub fn cap9(&self) -> &CAP9 {
        unsafe { &*(((self as *const Self) as *const u8).add(292usize) as *const CAP9) }
    }
    #[doc = "0x128 - SCT match value register of match channels."]
    #[inline(always)]
    pub fn match10(&self) -> &MATCH10 {
        unsafe { &*(((self as *const Self) as *const u8).add(296usize) as *const MATCH10) }
    }
    #[doc = "0x128 - SCT capture register of capture channel."]
    #[inline(always)]
    pub fn cap10(&self) -> &CAP10 {
        unsafe { &*(((self as *const Self) as *const u8).add(296usize) as *const CAP10) }
    }
    #[doc = "0x12c - SCT match value register of match channels."]
    #[inline(always)]
    pub fn match11(&self) -> &MATCH11 {
        unsafe { &*(((self as *const Self) as *const u8).add(300usize) as *const MATCH11) }
    }
    #[doc = "0x12c - SCT capture register of capture channel."]
    #[inline(always)]
    pub fn cap11(&self) -> &CAP11 {
        unsafe { &*(((self as *const Self) as *const u8).add(300usize) as *const CAP11) }
    }
    #[doc = "0x130 - SCT match value register of match channels."]
    #[inline(always)]
    pub fn match12(&self) -> &MATCH12 {
        unsafe { &*(((self as *const Self) as *const u8).add(304usize) as *const MATCH12) }
    }
    #[doc = "0x130 - SCT capture register of capture channel."]
    #[inline(always)]
    pub fn cap12(&self) -> &CAP12 {
        unsafe { &*(((self as *const Self) as *const u8).add(304usize) as *const CAP12) }
    }
    #[doc = "0x134 - SCT match value register of match channels."]
    #[inline(always)]
    pub fn match13(&self) -> &MATCH13 {
        unsafe { &*(((self as *const Self) as *const u8).add(308usize) as *const MATCH13) }
    }
    #[doc = "0x134 - SCT capture register of capture channel."]
    #[inline(always)]
    pub fn cap13(&self) -> &CAP13 {
        unsafe { &*(((self as *const Self) as *const u8).add(308usize) as *const CAP13) }
    }
    #[doc = "0x138 - SCT match value register of match channels."]
    #[inline(always)]
    pub fn match14(&self) -> &MATCH14 {
        unsafe { &*(((self as *const Self) as *const u8).add(312usize) as *const MATCH14) }
    }
    #[doc = "0x138 - SCT capture register of capture channel."]
    #[inline(always)]
    pub fn cap14(&self) -> &CAP14 {
        unsafe { &*(((self as *const Self) as *const u8).add(312usize) as *const CAP14) }
    }
    #[doc = "0x13c - SCT match value register of match channels."]
    #[inline(always)]
    pub fn match15(&self) -> &MATCH15 {
        unsafe { &*(((self as *const Self) as *const u8).add(316usize) as *const MATCH15) }
    }
    #[doc = "0x13c - SCT capture register of capture channel."]
    #[inline(always)]
    pub fn cap15(&self) -> &CAP15 {
        unsafe { &*(((self as *const Self) as *const u8).add(316usize) as *const CAP15) }
    }
    #[doc = "0x200 - SCT match reload value register."]
    #[inline(always)]
    pub fn matchrel0(&self) -> &MATCHREL0 {
        unsafe { &*(((self as *const Self) as *const u8).add(512usize) as *const MATCHREL0) }
    }
    #[doc = "0x200 - SCT capture control register."]
    #[inline(always)]
    pub fn capctrl0(&self) -> &CAPCTRL0 {
        unsafe { &*(((self as *const Self) as *const u8).add(512usize) as *const CAPCTRL0) }
    }
    #[doc = "0x204 - SCT match reload value register."]
    #[inline(always)]
    pub fn matchrel1(&self) -> &MATCHREL1 {
        unsafe { &*(((self as *const Self) as *const u8).add(516usize) as *const MATCHREL1) }
    }
    #[doc = "0x204 - SCT capture control register."]
    #[inline(always)]
    pub fn capctrl1(&self) -> &CAPCTRL1 {
        unsafe { &*(((self as *const Self) as *const u8).add(516usize) as *const CAPCTRL1) }
    }
    #[doc = "0x208 - SCT match reload value register."]
    #[inline(always)]
    pub fn matchrel2(&self) -> &MATCHREL2 {
        unsafe { &*(((self as *const Self) as *const u8).add(520usize) as *const MATCHREL2) }
    }
    #[doc = "0x208 - SCT capture control register."]
    #[inline(always)]
    pub fn capctrl2(&self) -> &CAPCTRL2 {
        unsafe { &*(((self as *const Self) as *const u8).add(520usize) as *const CAPCTRL2) }
    }
    #[doc = "0x20c - SCT match reload value register."]
    #[inline(always)]
    pub fn matchrel3(&self) -> &MATCHREL3 {
        unsafe { &*(((self as *const Self) as *const u8).add(524usize) as *const MATCHREL3) }
    }
    #[doc = "0x20c - SCT capture control register."]
    #[inline(always)]
    pub fn capctrl3(&self) -> &CAPCTRL3 {
        unsafe { &*(((self as *const Self) as *const u8).add(524usize) as *const CAPCTRL3) }
    }
    #[doc = "0x210 - SCT match reload value register."]
    #[inline(always)]
    pub fn matchrel4(&self) -> &MATCHREL4 {
        unsafe { &*(((self as *const Self) as *const u8).add(528usize) as *const MATCHREL4) }
    }
    #[doc = "0x210 - SCT capture control register."]
    #[inline(always)]
    pub fn capctrl4(&self) -> &CAPCTRL4 {
        unsafe { &*(((self as *const Self) as *const u8).add(528usize) as *const CAPCTRL4) }
    }
    #[doc = "0x214 - SCT match reload value register."]
    #[inline(always)]
    pub fn matchrel5(&self) -> &MATCHREL5 {
        unsafe { &*(((self as *const Self) as *const u8).add(532usize) as *const MATCHREL5) }
    }
    #[doc = "0x214 - SCT capture control register."]
    #[inline(always)]
    pub fn capctrl5(&self) -> &CAPCTRL5 {
        unsafe { &*(((self as *const Self) as *const u8).add(532usize) as *const CAPCTRL5) }
    }
    #[doc = "0x218 - SCT match reload value register."]
    #[inline(always)]
    pub fn matchrel6(&self) -> &MATCHREL6 {
        unsafe { &*(((self as *const Self) as *const u8).add(536usize) as *const MATCHREL6) }
    }
    #[doc = "0x218 - SCT capture control register."]
    #[inline(always)]
    pub fn capctrl6(&self) -> &CAPCTRL6 {
        unsafe { &*(((self as *const Self) as *const u8).add(536usize) as *const CAPCTRL6) }
    }
    #[doc = "0x21c - SCT match reload value register."]
    #[inline(always)]
    pub fn matchrel7(&self) -> &MATCHREL7 {
        unsafe { &*(((self as *const Self) as *const u8).add(540usize) as *const MATCHREL7) }
    }
    #[doc = "0x21c - SCT capture control register."]
    #[inline(always)]
    pub fn capctrl7(&self) -> &CAPCTRL7 {
        unsafe { &*(((self as *const Self) as *const u8).add(540usize) as *const CAPCTRL7) }
    }
    #[doc = "0x220 - SCT match reload value register."]
    #[inline(always)]
    pub fn matchrel8(&self) -> &MATCHREL8 {
        unsafe { &*(((self as *const Self) as *const u8).add(544usize) as *const MATCHREL8) }
    }
    #[doc = "0x220 - SCT capture control register."]
    #[inline(always)]
    pub fn capctrl8(&self) -> &CAPCTRL8 {
        unsafe { &*(((self as *const Self) as *const u8).add(544usize) as *const CAPCTRL8) }
    }
    #[doc = "0x224 - SCT match reload value register."]
    #[inline(always)]
    pub fn matchrel9(&self) -> &MATCHREL9 {
        unsafe { &*(((self as *const Self) as *const u8).add(548usize) as *const MATCHREL9) }
    }
    #[doc = "0x224 - SCT capture control register."]
    #[inline(always)]
    pub fn capctrl9(&self) -> &CAPCTRL9 {
        unsafe { &*(((self as *const Self) as *const u8).add(548usize) as *const CAPCTRL9) }
    }
    #[doc = "0x228 - SCT match reload value register."]
    #[inline(always)]
    pub fn matchrel10(&self) -> &MATCHREL10 {
        unsafe { &*(((self as *const Self) as *const u8).add(552usize) as *const MATCHREL10) }
    }
    #[doc = "0x228 - SCT capture control register."]
    #[inline(always)]
    pub fn capctrl10(&self) -> &CAPCTRL10 {
        unsafe { &*(((self as *const Self) as *const u8).add(552usize) as *const CAPCTRL10) }
    }
    #[doc = "0x22c - SCT match reload value register."]
    #[inline(always)]
    pub fn matchrel11(&self) -> &MATCHREL11 {
        unsafe { &*(((self as *const Self) as *const u8).add(556usize) as *const MATCHREL11) }
    }
    #[doc = "0x22c - SCT capture control register."]
    #[inline(always)]
    pub fn capctrl11(&self) -> &CAPCTRL11 {
        unsafe { &*(((self as *const Self) as *const u8).add(556usize) as *const CAPCTRL11) }
    }
    #[doc = "0x230 - SCT match reload value register."]
    #[inline(always)]
    pub fn matchrel12(&self) -> &MATCHREL12 {
        unsafe { &*(((self as *const Self) as *const u8).add(560usize) as *const MATCHREL12) }
    }
    #[doc = "0x230 - SCT capture control register."]
    #[inline(always)]
    pub fn capctrl12(&self) -> &CAPCTRL12 {
        unsafe { &*(((self as *const Self) as *const u8).add(560usize) as *const CAPCTRL12) }
    }
    #[doc = "0x234 - SCT match reload value register."]
    #[inline(always)]
    pub fn matchrel13(&self) -> &MATCHREL13 {
        unsafe { &*(((self as *const Self) as *const u8).add(564usize) as *const MATCHREL13) }
    }
    #[doc = "0x234 - SCT capture control register."]
    #[inline(always)]
    pub fn capctrl13(&self) -> &CAPCTRL13 {
        unsafe { &*(((self as *const Self) as *const u8).add(564usize) as *const CAPCTRL13) }
    }
    #[doc = "0x238 - SCT match reload value register."]
    #[inline(always)]
    pub fn matchrel14(&self) -> &MATCHREL14 {
        unsafe { &*(((self as *const Self) as *const u8).add(568usize) as *const MATCHREL14) }
    }
    #[doc = "0x238 - SCT capture control register."]
    #[inline(always)]
    pub fn capctrl14(&self) -> &CAPCTRL14 {
        unsafe { &*(((self as *const Self) as *const u8).add(568usize) as *const CAPCTRL14) }
    }
    #[doc = "0x23c - SCT match reload value register."]
    #[inline(always)]
    pub fn matchrel15(&self) -> &MATCHREL15 {
        unsafe { &*(((self as *const Self) as *const u8).add(572usize) as *const MATCHREL15) }
    }
    #[doc = "0x23c - SCT capture control register."]
    #[inline(always)]
    pub fn capctrl15(&self) -> &CAPCTRL15 {
        unsafe { &*(((self as *const Self) as *const u8).add(572usize) as *const CAPCTRL15) }
    }
}
#[doc = "CONFIG (rw) register accessor: an alias for `Reg<CONFIG_SPEC>`"]
pub type CONFIG = crate::Reg<config::CONFIG_SPEC>;
#[doc = "SCT configuration register."]
pub mod config;
#[doc = "CTRL (rw) register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "SCT control register."]
pub mod ctrl;
#[doc = "LIMIT (rw) register accessor: an alias for `Reg<LIMIT_SPEC>`"]
pub type LIMIT = crate::Reg<limit::LIMIT_SPEC>;
#[doc = "SCT limit event select register."]
pub mod limit;
#[doc = "HALT (rw) register accessor: an alias for `Reg<HALT_SPEC>`"]
pub type HALT = crate::Reg<halt::HALT_SPEC>;
#[doc = "SCT halt event select register."]
pub mod halt;
#[doc = "STOP (rw) register accessor: an alias for `Reg<STOP_SPEC>`"]
pub type STOP = crate::Reg<stop::STOP_SPEC>;
#[doc = "SCT stop event select register."]
pub mod stop;
#[doc = "START (rw) register accessor: an alias for `Reg<START_SPEC>`"]
pub type START = crate::Reg<start::START_SPEC>;
#[doc = "SCT start event select register."]
pub mod start;
#[doc = "COUNT (rw) register accessor: an alias for `Reg<COUNT_SPEC>`"]
pub type COUNT = crate::Reg<count::COUNT_SPEC>;
#[doc = "SCT counter register."]
pub mod count;
#[doc = "STATE (rw) register accessor: an alias for `Reg<STATE_SPEC>`"]
pub type STATE = crate::Reg<state::STATE_SPEC>;
#[doc = "SCT state register."]
pub mod state;
#[doc = "INPUT (r) register accessor: an alias for `Reg<INPUT_SPEC>`"]
pub type INPUT = crate::Reg<input::INPUT_SPEC>;
#[doc = "SCT input register."]
pub mod input;
#[doc = "REGMODE (rw) register accessor: an alias for `Reg<REGMODE_SPEC>`"]
pub type REGMODE = crate::Reg<regmode::REGMODE_SPEC>;
#[doc = "SCT match/capture mode register."]
pub mod regmode;
#[doc = "OUTPUT (rw) register accessor: an alias for `Reg<OUTPUT_SPEC>`"]
pub type OUTPUT = crate::Reg<output::OUTPUT_SPEC>;
#[doc = "SCT output register."]
pub mod output;
#[doc = "OUTPUTDIRCTRL (rw) register accessor: an alias for `Reg<OUTPUTDIRCTRL_SPEC>`"]
pub type OUTPUTDIRCTRL = crate::Reg<outputdirctrl::OUTPUTDIRCTRL_SPEC>;
#[doc = "SCT output counter direction control register."]
pub mod outputdirctrl;
#[doc = "RES (rw) register accessor: an alias for `Reg<RES_SPEC>`"]
pub type RES = crate::Reg<res::RES_SPEC>;
#[doc = "SCT conflict resolution register."]
pub mod res;
#[doc = "DMAREQ0 (rw) register accessor: an alias for `Reg<DMAREQ0_SPEC>`"]
pub type DMAREQ0 = crate::Reg<dmareq0::DMAREQ0_SPEC>;
#[doc = "SCT DMA request 0 register."]
pub mod dmareq0;
#[doc = "DMAREQ1 (rw) register accessor: an alias for `Reg<DMAREQ1_SPEC>`"]
pub type DMAREQ1 = crate::Reg<dmareq1::DMAREQ1_SPEC>;
#[doc = "SCT DMA request 1 register."]
pub mod dmareq1;
#[doc = "EVEN (rw) register accessor: an alias for `Reg<EVEN_SPEC>`"]
pub type EVEN = crate::Reg<even::EVEN_SPEC>;
#[doc = "SCT event interrupt enable register."]
pub mod even;
#[doc = "EVFLAG (rw) register accessor: an alias for `Reg<EVFLAG_SPEC>`"]
pub type EVFLAG = crate::Reg<evflag::EVFLAG_SPEC>;
#[doc = "SCT event flag register."]
pub mod evflag;
#[doc = "CONEN (rw) register accessor: an alias for `Reg<CONEN_SPEC>`"]
pub type CONEN = crate::Reg<conen::CONEN_SPEC>;
#[doc = "SCT conflict interrupt enable register."]
pub mod conen;
#[doc = "CONFLAG (rw) register accessor: an alias for `Reg<CONFLAG_SPEC>`"]
pub type CONFLAG = crate::Reg<conflag::CONFLAG_SPEC>;
#[doc = "SCT conflict flag register."]
pub mod conflag;
#[doc = "CAP0 (rw) register accessor: an alias for `Reg<CAP0_SPEC>`"]
pub type CAP0 = crate::Reg<cap0::CAP0_SPEC>;
#[doc = "SCT capture register of capture channel."]
pub mod cap0;
#[doc = "MATCH0 (rw) register accessor: an alias for `Reg<MATCH0_SPEC>`"]
pub type MATCH0 = crate::Reg<match0::MATCH0_SPEC>;
#[doc = "SCT match value register of match channels."]
pub mod match0;
#[doc = "CAP1 (rw) register accessor: an alias for `Reg<CAP1_SPEC>`"]
pub type CAP1 = crate::Reg<cap1::CAP1_SPEC>;
#[doc = "SCT capture register of capture channel."]
pub mod cap1;
#[doc = "MATCH1 (rw) register accessor: an alias for `Reg<MATCH1_SPEC>`"]
pub type MATCH1 = crate::Reg<match1::MATCH1_SPEC>;
#[doc = "SCT match value register of match channels."]
pub mod match1;
#[doc = "CAP2 (rw) register accessor: an alias for `Reg<CAP2_SPEC>`"]
pub type CAP2 = crate::Reg<cap2::CAP2_SPEC>;
#[doc = "SCT capture register of capture channel."]
pub mod cap2;
#[doc = "MATCH2 (rw) register accessor: an alias for `Reg<MATCH2_SPEC>`"]
pub type MATCH2 = crate::Reg<match2::MATCH2_SPEC>;
#[doc = "SCT match value register of match channels."]
pub mod match2;
#[doc = "CAP3 (rw) register accessor: an alias for `Reg<CAP3_SPEC>`"]
pub type CAP3 = crate::Reg<cap3::CAP3_SPEC>;
#[doc = "SCT capture register of capture channel."]
pub mod cap3;
#[doc = "MATCH3 (rw) register accessor: an alias for `Reg<MATCH3_SPEC>`"]
pub type MATCH3 = crate::Reg<match3::MATCH3_SPEC>;
#[doc = "SCT match value register of match channels."]
pub mod match3;
#[doc = "CAP4 (rw) register accessor: an alias for `Reg<CAP4_SPEC>`"]
pub type CAP4 = crate::Reg<cap4::CAP4_SPEC>;
#[doc = "SCT capture register of capture channel."]
pub mod cap4;
#[doc = "MATCH4 (rw) register accessor: an alias for `Reg<MATCH4_SPEC>`"]
pub type MATCH4 = crate::Reg<match4::MATCH4_SPEC>;
#[doc = "SCT match value register of match channels."]
pub mod match4;
#[doc = "CAP5 (rw) register accessor: an alias for `Reg<CAP5_SPEC>`"]
pub type CAP5 = crate::Reg<cap5::CAP5_SPEC>;
#[doc = "SCT capture register of capture channel."]
pub mod cap5;
#[doc = "MATCH5 (rw) register accessor: an alias for `Reg<MATCH5_SPEC>`"]
pub type MATCH5 = crate::Reg<match5::MATCH5_SPEC>;
#[doc = "SCT match value register of match channels."]
pub mod match5;
#[doc = "CAP6 (rw) register accessor: an alias for `Reg<CAP6_SPEC>`"]
pub type CAP6 = crate::Reg<cap6::CAP6_SPEC>;
#[doc = "SCT capture register of capture channel."]
pub mod cap6;
#[doc = "MATCH6 (rw) register accessor: an alias for `Reg<MATCH6_SPEC>`"]
pub type MATCH6 = crate::Reg<match6::MATCH6_SPEC>;
#[doc = "SCT match value register of match channels."]
pub mod match6;
#[doc = "CAP7 (rw) register accessor: an alias for `Reg<CAP7_SPEC>`"]
pub type CAP7 = crate::Reg<cap7::CAP7_SPEC>;
#[doc = "SCT capture register of capture channel."]
pub mod cap7;
#[doc = "MATCH7 (rw) register accessor: an alias for `Reg<MATCH7_SPEC>`"]
pub type MATCH7 = crate::Reg<match7::MATCH7_SPEC>;
#[doc = "SCT match value register of match channels."]
pub mod match7;
#[doc = "CAP8 (rw) register accessor: an alias for `Reg<CAP8_SPEC>`"]
pub type CAP8 = crate::Reg<cap8::CAP8_SPEC>;
#[doc = "SCT capture register of capture channel."]
pub mod cap8;
#[doc = "MATCH8 (rw) register accessor: an alias for `Reg<MATCH8_SPEC>`"]
pub type MATCH8 = crate::Reg<match8::MATCH8_SPEC>;
#[doc = "SCT match value register of match channels."]
pub mod match8;
#[doc = "CAP9 (rw) register accessor: an alias for `Reg<CAP9_SPEC>`"]
pub type CAP9 = crate::Reg<cap9::CAP9_SPEC>;
#[doc = "SCT capture register of capture channel."]
pub mod cap9;
#[doc = "MATCH9 (rw) register accessor: an alias for `Reg<MATCH9_SPEC>`"]
pub type MATCH9 = crate::Reg<match9::MATCH9_SPEC>;
#[doc = "SCT match value register of match channels."]
pub mod match9;
#[doc = "CAP10 (rw) register accessor: an alias for `Reg<CAP10_SPEC>`"]
pub type CAP10 = crate::Reg<cap10::CAP10_SPEC>;
#[doc = "SCT capture register of capture channel."]
pub mod cap10;
#[doc = "MATCH10 (rw) register accessor: an alias for `Reg<MATCH10_SPEC>`"]
pub type MATCH10 = crate::Reg<match10::MATCH10_SPEC>;
#[doc = "SCT match value register of match channels."]
pub mod match10;
#[doc = "CAP11 (rw) register accessor: an alias for `Reg<CAP11_SPEC>`"]
pub type CAP11 = crate::Reg<cap11::CAP11_SPEC>;
#[doc = "SCT capture register of capture channel."]
pub mod cap11;
#[doc = "MATCH11 (rw) register accessor: an alias for `Reg<MATCH11_SPEC>`"]
pub type MATCH11 = crate::Reg<match11::MATCH11_SPEC>;
#[doc = "SCT match value register of match channels."]
pub mod match11;
#[doc = "CAP12 (rw) register accessor: an alias for `Reg<CAP12_SPEC>`"]
pub type CAP12 = crate::Reg<cap12::CAP12_SPEC>;
#[doc = "SCT capture register of capture channel."]
pub mod cap12;
#[doc = "MATCH12 (rw) register accessor: an alias for `Reg<MATCH12_SPEC>`"]
pub type MATCH12 = crate::Reg<match12::MATCH12_SPEC>;
#[doc = "SCT match value register of match channels."]
pub mod match12;
#[doc = "CAP13 (rw) register accessor: an alias for `Reg<CAP13_SPEC>`"]
pub type CAP13 = crate::Reg<cap13::CAP13_SPEC>;
#[doc = "SCT capture register of capture channel."]
pub mod cap13;
#[doc = "MATCH13 (rw) register accessor: an alias for `Reg<MATCH13_SPEC>`"]
pub type MATCH13 = crate::Reg<match13::MATCH13_SPEC>;
#[doc = "SCT match value register of match channels."]
pub mod match13;
#[doc = "CAP14 (rw) register accessor: an alias for `Reg<CAP14_SPEC>`"]
pub type CAP14 = crate::Reg<cap14::CAP14_SPEC>;
#[doc = "SCT capture register of capture channel."]
pub mod cap14;
#[doc = "MATCH14 (rw) register accessor: an alias for `Reg<MATCH14_SPEC>`"]
pub type MATCH14 = crate::Reg<match14::MATCH14_SPEC>;
#[doc = "SCT match value register of match channels."]
pub mod match14;
#[doc = "CAP15 (rw) register accessor: an alias for `Reg<CAP15_SPEC>`"]
pub type CAP15 = crate::Reg<cap15::CAP15_SPEC>;
#[doc = "SCT capture register of capture channel."]
pub mod cap15;
#[doc = "MATCH15 (rw) register accessor: an alias for `Reg<MATCH15_SPEC>`"]
pub type MATCH15 = crate::Reg<match15::MATCH15_SPEC>;
#[doc = "SCT match value register of match channels."]
pub mod match15;
#[doc = "CAPCTRL0 (rw) register accessor: an alias for `Reg<CAPCTRL0_SPEC>`"]
pub type CAPCTRL0 = crate::Reg<capctrl0::CAPCTRL0_SPEC>;
#[doc = "SCT capture control register."]
pub mod capctrl0;
#[doc = "MATCHREL0 (rw) register accessor: an alias for `Reg<MATCHREL0_SPEC>`"]
pub type MATCHREL0 = crate::Reg<matchrel0::MATCHREL0_SPEC>;
#[doc = "SCT match reload value register."]
pub mod matchrel0;
#[doc = "CAPCTRL1 (rw) register accessor: an alias for `Reg<CAPCTRL1_SPEC>`"]
pub type CAPCTRL1 = crate::Reg<capctrl1::CAPCTRL1_SPEC>;
#[doc = "SCT capture control register."]
pub mod capctrl1;
#[doc = "MATCHREL1 (rw) register accessor: an alias for `Reg<MATCHREL1_SPEC>`"]
pub type MATCHREL1 = crate::Reg<matchrel1::MATCHREL1_SPEC>;
#[doc = "SCT match reload value register."]
pub mod matchrel1;
#[doc = "CAPCTRL2 (rw) register accessor: an alias for `Reg<CAPCTRL2_SPEC>`"]
pub type CAPCTRL2 = crate::Reg<capctrl2::CAPCTRL2_SPEC>;
#[doc = "SCT capture control register."]
pub mod capctrl2;
#[doc = "MATCHREL2 (rw) register accessor: an alias for `Reg<MATCHREL2_SPEC>`"]
pub type MATCHREL2 = crate::Reg<matchrel2::MATCHREL2_SPEC>;
#[doc = "SCT match reload value register."]
pub mod matchrel2;
#[doc = "CAPCTRL3 (rw) register accessor: an alias for `Reg<CAPCTRL3_SPEC>`"]
pub type CAPCTRL3 = crate::Reg<capctrl3::CAPCTRL3_SPEC>;
#[doc = "SCT capture control register."]
pub mod capctrl3;
#[doc = "MATCHREL3 (rw) register accessor: an alias for `Reg<MATCHREL3_SPEC>`"]
pub type MATCHREL3 = crate::Reg<matchrel3::MATCHREL3_SPEC>;
#[doc = "SCT match reload value register."]
pub mod matchrel3;
#[doc = "CAPCTRL4 (rw) register accessor: an alias for `Reg<CAPCTRL4_SPEC>`"]
pub type CAPCTRL4 = crate::Reg<capctrl4::CAPCTRL4_SPEC>;
#[doc = "SCT capture control register."]
pub mod capctrl4;
#[doc = "MATCHREL4 (rw) register accessor: an alias for `Reg<MATCHREL4_SPEC>`"]
pub type MATCHREL4 = crate::Reg<matchrel4::MATCHREL4_SPEC>;
#[doc = "SCT match reload value register."]
pub mod matchrel4;
#[doc = "CAPCTRL5 (rw) register accessor: an alias for `Reg<CAPCTRL5_SPEC>`"]
pub type CAPCTRL5 = crate::Reg<capctrl5::CAPCTRL5_SPEC>;
#[doc = "SCT capture control register."]
pub mod capctrl5;
#[doc = "MATCHREL5 (rw) register accessor: an alias for `Reg<MATCHREL5_SPEC>`"]
pub type MATCHREL5 = crate::Reg<matchrel5::MATCHREL5_SPEC>;
#[doc = "SCT match reload value register."]
pub mod matchrel5;
#[doc = "CAPCTRL6 (rw) register accessor: an alias for `Reg<CAPCTRL6_SPEC>`"]
pub type CAPCTRL6 = crate::Reg<capctrl6::CAPCTRL6_SPEC>;
#[doc = "SCT capture control register."]
pub mod capctrl6;
#[doc = "MATCHREL6 (rw) register accessor: an alias for `Reg<MATCHREL6_SPEC>`"]
pub type MATCHREL6 = crate::Reg<matchrel6::MATCHREL6_SPEC>;
#[doc = "SCT match reload value register."]
pub mod matchrel6;
#[doc = "CAPCTRL7 (rw) register accessor: an alias for `Reg<CAPCTRL7_SPEC>`"]
pub type CAPCTRL7 = crate::Reg<capctrl7::CAPCTRL7_SPEC>;
#[doc = "SCT capture control register."]
pub mod capctrl7;
#[doc = "MATCHREL7 (rw) register accessor: an alias for `Reg<MATCHREL7_SPEC>`"]
pub type MATCHREL7 = crate::Reg<matchrel7::MATCHREL7_SPEC>;
#[doc = "SCT match reload value register."]
pub mod matchrel7;
#[doc = "CAPCTRL8 (rw) register accessor: an alias for `Reg<CAPCTRL8_SPEC>`"]
pub type CAPCTRL8 = crate::Reg<capctrl8::CAPCTRL8_SPEC>;
#[doc = "SCT capture control register."]
pub mod capctrl8;
#[doc = "MATCHREL8 (rw) register accessor: an alias for `Reg<MATCHREL8_SPEC>`"]
pub type MATCHREL8 = crate::Reg<matchrel8::MATCHREL8_SPEC>;
#[doc = "SCT match reload value register."]
pub mod matchrel8;
#[doc = "CAPCTRL9 (rw) register accessor: an alias for `Reg<CAPCTRL9_SPEC>`"]
pub type CAPCTRL9 = crate::Reg<capctrl9::CAPCTRL9_SPEC>;
#[doc = "SCT capture control register."]
pub mod capctrl9;
#[doc = "MATCHREL9 (rw) register accessor: an alias for `Reg<MATCHREL9_SPEC>`"]
pub type MATCHREL9 = crate::Reg<matchrel9::MATCHREL9_SPEC>;
#[doc = "SCT match reload value register."]
pub mod matchrel9;
#[doc = "CAPCTRL10 (rw) register accessor: an alias for `Reg<CAPCTRL10_SPEC>`"]
pub type CAPCTRL10 = crate::Reg<capctrl10::CAPCTRL10_SPEC>;
#[doc = "SCT capture control register."]
pub mod capctrl10;
#[doc = "MATCHREL10 (rw) register accessor: an alias for `Reg<MATCHREL10_SPEC>`"]
pub type MATCHREL10 = crate::Reg<matchrel10::MATCHREL10_SPEC>;
#[doc = "SCT match reload value register."]
pub mod matchrel10;
#[doc = "CAPCTRL11 (rw) register accessor: an alias for `Reg<CAPCTRL11_SPEC>`"]
pub type CAPCTRL11 = crate::Reg<capctrl11::CAPCTRL11_SPEC>;
#[doc = "SCT capture control register."]
pub mod capctrl11;
#[doc = "MATCHREL11 (rw) register accessor: an alias for `Reg<MATCHREL11_SPEC>`"]
pub type MATCHREL11 = crate::Reg<matchrel11::MATCHREL11_SPEC>;
#[doc = "SCT match reload value register."]
pub mod matchrel11;
#[doc = "CAPCTRL12 (rw) register accessor: an alias for `Reg<CAPCTRL12_SPEC>`"]
pub type CAPCTRL12 = crate::Reg<capctrl12::CAPCTRL12_SPEC>;
#[doc = "SCT capture control register."]
pub mod capctrl12;
#[doc = "MATCHREL12 (rw) register accessor: an alias for `Reg<MATCHREL12_SPEC>`"]
pub type MATCHREL12 = crate::Reg<matchrel12::MATCHREL12_SPEC>;
#[doc = "SCT match reload value register."]
pub mod matchrel12;
#[doc = "CAPCTRL13 (rw) register accessor: an alias for `Reg<CAPCTRL13_SPEC>`"]
pub type CAPCTRL13 = crate::Reg<capctrl13::CAPCTRL13_SPEC>;
#[doc = "SCT capture control register."]
pub mod capctrl13;
#[doc = "MATCHREL13 (rw) register accessor: an alias for `Reg<MATCHREL13_SPEC>`"]
pub type MATCHREL13 = crate::Reg<matchrel13::MATCHREL13_SPEC>;
#[doc = "SCT match reload value register."]
pub mod matchrel13;
#[doc = "CAPCTRL14 (rw) register accessor: an alias for `Reg<CAPCTRL14_SPEC>`"]
pub type CAPCTRL14 = crate::Reg<capctrl14::CAPCTRL14_SPEC>;
#[doc = "SCT capture control register."]
pub mod capctrl14;
#[doc = "MATCHREL14 (rw) register accessor: an alias for `Reg<MATCHREL14_SPEC>`"]
pub type MATCHREL14 = crate::Reg<matchrel14::MATCHREL14_SPEC>;
#[doc = "SCT match reload value register."]
pub mod matchrel14;
#[doc = "CAPCTRL15 (rw) register accessor: an alias for `Reg<CAPCTRL15_SPEC>`"]
pub type CAPCTRL15 = crate::Reg<capctrl15::CAPCTRL15_SPEC>;
#[doc = "SCT capture control register."]
pub mod capctrl15;
#[doc = "MATCHREL15 (rw) register accessor: an alias for `Reg<MATCHREL15_SPEC>`"]
pub type MATCHREL15 = crate::Reg<matchrel15::MATCHREL15_SPEC>;
#[doc = "SCT match reload value register."]
pub mod matchrel15;
#[doc = "no description available."]
pub use self::ev::EV;
#[doc = r"Cluster"]
#[doc = "no description available."]
pub mod ev;
#[doc = "no description available."]
pub use self::out::OUT;
#[doc = r"Cluster"]
#[doc = "no description available."]
pub mod out;
