#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x04],
    #[doc = "0x04 - Power Management Controller FSM (Finite State Machines) status"]
    pub status: STATUS,
    #[doc = "0x08 - Reset Control \\[Reset by: PoR, Pin Reset, Brown Out Detectors Reset, Deep Power Down Reset, Software Reset\\]"]
    pub resetctrl: RESETCTRL,
    _reserved2: [u8; 0x04],
    #[doc = "0x10 - DCDC (first) control register \\[Reset by: PoR, Pin Reset, Brown Out Detectors Reset, Deep Power Down Reset, Software Reset\\]"]
    pub dcdc0: DCDC0,
    #[doc = "0x14 - DCDC (second) control register \\[Reset by: PoR, Pin Reset, Brown Out Detectors Reset, Deep Power Down Reset, Software Reset\\]"]
    pub dcdc1: DCDC1,
    _reserved4: [u8; 0x04],
    #[doc = "0x1c - Power Management Unit (PMU) and Always-On domains LDO control \\[Reset by: PoR, Pin Reset, Brown Out Detectors Reset, Deep Power Down Reset, Software Reset\\]"]
    pub ldopmu: LDOPMU,
    _reserved5: [u8; 0x10],
    #[doc = "0x30 - VBAT Brown Out Dectector (BoD) control register \\[Reset by: PoR, Pin Reset, Software Reset\\]"]
    pub bodvbat: BODVBAT,
    _reserved6: [u8; 0x0c],
    #[doc = "0x40 - Analog References fast wake-up Control register \\[Reset by: PoR\\]"]
    pub reffastwkup: REFFASTWKUP,
    _reserved7: [u8; 0x08],
    #[doc = "0x4c - 32 KHz Crystal oscillator (XTAL) control register \\[Reset by: PoR, Brown Out Detectors Reset\\]"]
    pub xtal32k: XTAL32K,
    #[doc = "0x50 - Analog Comparator control register \\[Reset by: PoR, Pin Reset, Brown Out Detectors Reset, Deep Power Down Reset, Software Reset\\]"]
    pub comp: COMP,
    _reserved9: [u8; 0x10],
    #[doc = "0x64 - Deep Power Down wake-up source \\[Reset by: PoR, Pin Reset, Software Reset\\]"]
    pub wakeupioctrl: WAKEUPIOCTRL,
    #[doc = "0x68 - Allows to identify the Wake-up I/O source from Deep Power Down mode"]
    pub wakeiocause: WAKEIOCAUSE,
    _reserved11: [u8; 0x08],
    #[doc = "0x74 - FRO and XTAL status register \\[Reset by: PoR, Brown Out Detectors Reset\\]"]
    pub statusclk: STATUSCLK,
    _reserved12: [u8; 0x0c],
    #[doc = "0x84 - General purpose always on domain data storage \\[Reset by: PoR, Brown Out Detectors Reset\\]"]
    pub aoreg1: AOREG1,
    _reserved13: [u8; 0x08],
    #[doc = "0x90 - Dummy Control bus to PMU \\[Reset by: PoR, Pin Reset, Brown Out Detectors Reset, Deep Power Down Reset, Software Reset\\]"]
    pub miscctrl: MISCCTRL,
    _reserved14: [u8; 0x04],
    #[doc = "0x98 - RTC 1 KHZ and 1 Hz clocks source control register \\[Reset by: PoR, Brown Out Detectors Reset\\]"]
    pub rtcosc32k: RTCOSC32K,
    #[doc = "0x9c - OS Timer control register \\[Reset by: PoR, Brown Out Detectors Reset\\]"]
    pub ostimer: OSTIMER,
    _reserved16: [u8; 0x18],
    #[doc = "0xb8 - Controls the power to various analog blocks \\[Reset by: PoR, Pin Reset, Brown Out Detectors Reset, Deep Power Down Reset, Software Reset\\]"]
    pub pdruncfg0: PDRUNCFG0,
    _reserved17: [u8; 0x04],
    #[doc = "0xc0 - Controls the power to various analog blocks \\[Reset by: PoR, Pin Reset, Brown Out Detectors Reset, Deep Power Down Reset, Software Reset\\]"]
    pub pdruncfgset0: PDRUNCFGSET0,
    _reserved18: [u8; 0x04],
    #[doc = "0xc8 - Controls the power to various analog blocks \\[Reset by: PoR, Pin Reset, Brown Out Detectors Reset, Deep Power Down Reset, Software Reset\\]"]
    pub pdruncfgclr0: PDRUNCFGCLR0,
    _reserved19: [u8; 0x08],
    #[doc = "0xd4 - All SRAMs common control signals \\[Reset by: PoR, Pin Reset, Brown Out Detectors Reset, Software Reset\\]"]
    pub sramctrl: SRAMCTRL,
}
#[doc = "STATUS (r) register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Power Management Controller FSM (Finite State Machines) status"]
pub mod status;
#[doc = "RESETCTRL (rw) register accessor: an alias for `Reg<RESETCTRL_SPEC>`"]
pub type RESETCTRL = crate::Reg<resetctrl::RESETCTRL_SPEC>;
#[doc = "Reset Control \\[Reset by: PoR, Pin Reset, Brown Out Detectors Reset, Deep Power Down Reset, Software Reset\\]"]
pub mod resetctrl;
#[doc = "DCDC0 (rw) register accessor: an alias for `Reg<DCDC0_SPEC>`"]
pub type DCDC0 = crate::Reg<dcdc0::DCDC0_SPEC>;
#[doc = "DCDC (first) control register \\[Reset by: PoR, Pin Reset, Brown Out Detectors Reset, Deep Power Down Reset, Software Reset\\]"]
pub mod dcdc0;
#[doc = "DCDC1 (rw) register accessor: an alias for `Reg<DCDC1_SPEC>`"]
pub type DCDC1 = crate::Reg<dcdc1::DCDC1_SPEC>;
#[doc = "DCDC (second) control register \\[Reset by: PoR, Pin Reset, Brown Out Detectors Reset, Deep Power Down Reset, Software Reset\\]"]
pub mod dcdc1;
#[doc = "LDOPMU (rw) register accessor: an alias for `Reg<LDOPMU_SPEC>`"]
pub type LDOPMU = crate::Reg<ldopmu::LDOPMU_SPEC>;
#[doc = "Power Management Unit (PMU) and Always-On domains LDO control \\[Reset by: PoR, Pin Reset, Brown Out Detectors Reset, Deep Power Down Reset, Software Reset\\]"]
pub mod ldopmu;
#[doc = "BODVBAT (rw) register accessor: an alias for `Reg<BODVBAT_SPEC>`"]
pub type BODVBAT = crate::Reg<bodvbat::BODVBAT_SPEC>;
#[doc = "VBAT Brown Out Dectector (BoD) control register \\[Reset by: PoR, Pin Reset, Software Reset\\]"]
pub mod bodvbat;
#[doc = "REFFASTWKUP (rw) register accessor: an alias for `Reg<REFFASTWKUP_SPEC>`"]
pub type REFFASTWKUP = crate::Reg<reffastwkup::REFFASTWKUP_SPEC>;
#[doc = "Analog References fast wake-up Control register \\[Reset by: PoR\\]"]
pub mod reffastwkup;
#[doc = "XTAL32K (rw) register accessor: an alias for `Reg<XTAL32K_SPEC>`"]
pub type XTAL32K = crate::Reg<xtal32k::XTAL32K_SPEC>;
#[doc = "32 KHz Crystal oscillator (XTAL) control register \\[Reset by: PoR, Brown Out Detectors Reset\\]"]
pub mod xtal32k;
#[doc = "COMP (rw) register accessor: an alias for `Reg<COMP_SPEC>`"]
pub type COMP = crate::Reg<comp::COMP_SPEC>;
#[doc = "Analog Comparator control register \\[Reset by: PoR, Pin Reset, Brown Out Detectors Reset, Deep Power Down Reset, Software Reset\\]"]
pub mod comp;
#[doc = "WAKEUPIOCTRL (rw) register accessor: an alias for `Reg<WAKEUPIOCTRL_SPEC>`"]
pub type WAKEUPIOCTRL = crate::Reg<wakeupioctrl::WAKEUPIOCTRL_SPEC>;
#[doc = "Deep Power Down wake-up source \\[Reset by: PoR, Pin Reset, Software Reset\\]"]
pub mod wakeupioctrl;
#[doc = "WAKEIOCAUSE (rw) register accessor: an alias for `Reg<WAKEIOCAUSE_SPEC>`"]
pub type WAKEIOCAUSE = crate::Reg<wakeiocause::WAKEIOCAUSE_SPEC>;
#[doc = "Allows to identify the Wake-up I/O source from Deep Power Down mode"]
pub mod wakeiocause;
#[doc = "STATUSCLK (rw) register accessor: an alias for `Reg<STATUSCLK_SPEC>`"]
pub type STATUSCLK = crate::Reg<statusclk::STATUSCLK_SPEC>;
#[doc = "FRO and XTAL status register \\[Reset by: PoR, Brown Out Detectors Reset\\]"]
pub mod statusclk;
#[doc = "AOREG1 (rw) register accessor: an alias for `Reg<AOREG1_SPEC>`"]
pub type AOREG1 = crate::Reg<aoreg1::AOREG1_SPEC>;
#[doc = "General purpose always on domain data storage \\[Reset by: PoR, Brown Out Detectors Reset\\]"]
pub mod aoreg1;
#[doc = "MISCCTRL (rw) register accessor: an alias for `Reg<MISCCTRL_SPEC>`"]
pub type MISCCTRL = crate::Reg<miscctrl::MISCCTRL_SPEC>;
#[doc = "Dummy Control bus to PMU \\[Reset by: PoR, Pin Reset, Brown Out Detectors Reset, Deep Power Down Reset, Software Reset\\]"]
pub mod miscctrl;
#[doc = "RTCOSC32K (rw) register accessor: an alias for `Reg<RTCOSC32K_SPEC>`"]
pub type RTCOSC32K = crate::Reg<rtcosc32k::RTCOSC32K_SPEC>;
#[doc = "RTC 1 KHZ and 1 Hz clocks source control register \\[Reset by: PoR, Brown Out Detectors Reset\\]"]
pub mod rtcosc32k;
#[doc = "OSTIMER (rw) register accessor: an alias for `Reg<OSTIMER_SPEC>`"]
pub type OSTIMER = crate::Reg<ostimer::OSTIMER_SPEC>;
#[doc = "OS Timer control register \\[Reset by: PoR, Brown Out Detectors Reset\\]"]
pub mod ostimer;
#[doc = "PDRUNCFG0 (rw) register accessor: an alias for `Reg<PDRUNCFG0_SPEC>`"]
pub type PDRUNCFG0 = crate::Reg<pdruncfg0::PDRUNCFG0_SPEC>;
#[doc = "Controls the power to various analog blocks \\[Reset by: PoR, Pin Reset, Brown Out Detectors Reset, Deep Power Down Reset, Software Reset\\]"]
pub mod pdruncfg0;
#[doc = "PDRUNCFGSET0 (w) register accessor: an alias for `Reg<PDRUNCFGSET0_SPEC>`"]
pub type PDRUNCFGSET0 = crate::Reg<pdruncfgset0::PDRUNCFGSET0_SPEC>;
#[doc = "Controls the power to various analog blocks \\[Reset by: PoR, Pin Reset, Brown Out Detectors Reset, Deep Power Down Reset, Software Reset\\]"]
pub mod pdruncfgset0;
#[doc = "PDRUNCFGCLR0 (w) register accessor: an alias for `Reg<PDRUNCFGCLR0_SPEC>`"]
pub type PDRUNCFGCLR0 = crate::Reg<pdruncfgclr0::PDRUNCFGCLR0_SPEC>;
#[doc = "Controls the power to various analog blocks \\[Reset by: PoR, Pin Reset, Brown Out Detectors Reset, Deep Power Down Reset, Software Reset\\]"]
pub mod pdruncfgclr0;
#[doc = "SRAMCTRL (rw) register accessor: an alias for `Reg<SRAMCTRL_SPEC>`"]
pub type SRAMCTRL = crate::Reg<sramctrl::SRAMCTRL_SPEC>;
#[doc = "All SRAMs common control signals \\[Reset by: PoR, Pin Reset, Brown Out Detectors Reset, Software Reset\\]"]
pub mod sramctrl;