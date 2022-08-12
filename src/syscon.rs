#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Memory Remap control register."]
    pub memoryremap: MEMORYREMAP,
    _reserved1: [u8; 0x0c],
    #[doc = "0x10 - AHB Matrix priority control register Priority values are 3 = highest, 0 = lowest."]
    pub ahbmatprio: AHBMATPRIO,
    _reserved2: [u8; 0x24],
    #[doc = "0x38 - System tick calibration for secure part of CPU0."]
    pub cpu0stckcal: CPU0STCKCAL,
    #[doc = "0x3c - System tick calibration for non-secure part of CPU0."]
    pub cpu0nstckcal: CPU0NSTCKCAL,
    _reserved4: [u8; 0x08],
    #[doc = "0x48 - NMI Source Select."]
    pub nmisrc: NMISRC,
    _reserved5: [u8; 0xb4],
    #[doc = "0x100 - Peripheral reset control 0."]
    pub presetctrl0: PRESETCTRL0,
    #[doc = "0x104 - Peripheral reset control 1."]
    pub presetctrl1: PRESETCTRL1,
    #[doc = "0x108 - Peripheral reset control 2."]
    pub presetctrl2: PRESETCTRL2,
    _reserved8: [u8; 0x14],
    #[doc = "0x120..0x12c - Peripheral reset control set register."]
    pub presetctrlset: [PRESETCTRLSET; 3],
    _reserved9: [u8; 0x14],
    #[doc = "0x140..0x14c - Peripheral reset control clear register."]
    pub presetctrlclr: [PRESETCTRLCLR; 3],
    _reserved10: [u8; 0x14],
    #[doc = "0x160 - generate a software_reset."]
    pub swr_reset: SWR_RESET,
    _reserved11: [u8; 0x9c],
    #[doc = "0x200 - AHB Clock control 0."]
    pub ahbclkctrl0: AHBCLKCTRL0,
    #[doc = "0x204 - AHB Clock control 1."]
    pub ahbclkctrl1: AHBCLKCTRL1,
    #[doc = "0x208 - AHB Clock control 2."]
    pub ahbclkctrl2: AHBCLKCTRL2,
    _reserved14: [u8; 0x14],
    #[doc = "0x220..0x22c - Peripheral reset control register."]
    pub ahbclkctrlset: [AHBCLKCTRLSET; 3],
    _reserved15: [u8; 0x14],
    #[doc = "0x240..0x24c - Peripheral reset control register."]
    pub ahbclkctrlclr: [AHBCLKCTRLCLR; 3],
    _reserved16: [u8; 0x14],
    _reserved_16_systickclksel0: [u8; 0x04],
    _reserved17: [u8; 0x04],
    #[doc = "0x268 - Trace clock source select."]
    pub traceclksel: TRACECLKSEL,
    #[doc = "0x26c - CTimer 0 clock source select."]
    pub ctimerclksel0: CTIMERCLKSEL0,
    #[doc = "0x270 - CTimer 1 clock source select."]
    pub ctimerclksel1: CTIMERCLKSEL1,
    #[doc = "0x274 - CTimer 2 clock source select."]
    pub ctimerclksel2: CTIMERCLKSEL2,
    #[doc = "0x278 - CTimer 3 clock source select."]
    pub ctimerclksel3: CTIMERCLKSEL3,
    #[doc = "0x27c - CTimer 4 clock source select."]
    pub ctimerclksel4: CTIMERCLKSEL4,
    #[doc = "0x280 - Main clock A source select."]
    pub mainclksela: MAINCLKSELA,
    #[doc = "0x284 - Main clock source select."]
    pub mainclkselb: MAINCLKSELB,
    #[doc = "0x288 - CLKOUT clock source select."]
    pub clkoutsel: CLKOUTSEL,
    _reserved26: [u8; 0x04],
    #[doc = "0x290 - PLL0 clock source select."]
    pub pll0clksel: PLL0CLKSEL,
    #[doc = "0x294 - PLL1 clock source select."]
    pub pll1clksel: PLL1CLKSEL,
    _reserved28: [u8; 0x08],
    #[doc = "0x2a0 - CAN clock source select."]
    pub canclksel: CANCLKSEL,
    #[doc = "0x2a4 - ADC clock source select."]
    pub adcclksel: ADCCLKSEL,
    _reserved30: [u8; 0x08],
    #[doc = "0x2b0 - Flexcomm Interface 0 clock source select for Fractional Rate Divider."]
    pub fcclksel0: FCCLKSEL0,
    #[doc = "0x2b4 - Flexcomm Interface 1 clock source select for Fractional Rate Divider."]
    pub fcclksel1: FCCLKSEL1,
    #[doc = "0x2b8 - Flexcomm Interface 2 clock source select for Fractional Rate Divider."]
    pub fcclksel2: FCCLKSEL2,
    #[doc = "0x2bc - Flexcomm Interface 3 clock source select for Fractional Rate Divider."]
    pub fcclksel3: FCCLKSEL3,
    #[doc = "0x2c0 - Flexcomm Interface 4 clock source select for Fractional Rate Divider."]
    pub fcclksel4: FCCLKSEL4,
    #[doc = "0x2c4 - Flexcomm Interface 5 clock source select for Fractional Rate Divider."]
    pub fcclksel5: FCCLKSEL5,
    #[doc = "0x2c8 - Flexcomm Interface 6 clock source select for Fractional Rate Divider."]
    pub fcclksel6: FCCLKSEL6,
    #[doc = "0x2cc - Flexcomm Interface 7 clock source select for Fractional Rate Divider."]
    pub fcclksel7: FCCLKSEL7,
    #[doc = "0x2d0 - HS LSPI clock source select."]
    pub hslspiclksel: HSLSPICLKSEL,
    _reserved39: [u8; 0x0c],
    #[doc = "0x2e0 - MCLK clock source select."]
    pub mclkclksel: MCLKCLKSEL,
    _reserved40: [u8; 0x0c],
    #[doc = "0x2f0 - SCTimer/PWM clock source select."]
    pub sctclksel: SCTCLKSEL,
    _reserved41: [u8; 0x0c],
    #[doc = "0x300 - System Tick Timer divider for CPU0."]
    pub systickclkdiv0: SYSTICKCLKDIV0,
    _reserved42: [u8; 0x04],
    #[doc = "0x308 - TRACE clock divider."]
    pub traceclkdiv: TRACECLKDIV,
    #[doc = "0x30c - CAN clock divider."]
    pub canclkdiv: CANCLKDIV,
    _reserved44: [u8; 0x10],
    #[doc = "0x320 - Fractional rate divider for flexcomm 0."]
    pub flexfrg0ctrl: FLEXFRG0CTRL,
    #[doc = "0x324 - Fractional rate divider for flexcomm 1."]
    pub flexfrg1ctrl: FLEXFRG1CTRL,
    #[doc = "0x328 - Fractional rate divider for flexcomm 2."]
    pub flexfrg2ctrl: FLEXFRG2CTRL,
    #[doc = "0x32c - Fractional rate divider for flexcomm 3."]
    pub flexfrg3ctrl: FLEXFRG3CTRL,
    #[doc = "0x330 - Fractional rate divider for flexcomm 4."]
    pub flexfrg4ctrl: FLEXFRG4CTRL,
    #[doc = "0x334 - Fractional rate divider for flexcomm 5."]
    pub flexfrg5ctrl: FLEXFRG5CTRL,
    #[doc = "0x338 - Fractional rate divider for flexcomm 6."]
    pub flexfrg6ctrl: FLEXFRG6CTRL,
    #[doc = "0x33c - Fractional rate divider for flexcomm 7."]
    pub flexfrg7ctrl: FLEXFRG7CTRL,
    _reserved52: [u8; 0x40],
    #[doc = "0x380 - System clock divider."]
    pub ahbclkdiv: AHBCLKDIV,
    #[doc = "0x384 - CLKOUT clock divider."]
    pub clkoutdiv: CLKOUTDIV,
    #[doc = "0x388 - FRO_HF (96MHz) clock divider."]
    pub frohfdiv: FROHFDIV,
    #[doc = "0x38c - WDT clock divider."]
    pub wdtclkdiv: WDTCLKDIV,
    _reserved56: [u8; 0x04],
    #[doc = "0x394 - ADC clock divider."]
    pub adcclkdiv: ADCCLKDIV,
    _reserved57: [u8; 0x14],
    #[doc = "0x3ac - I2S MCLK clock divider."]
    pub mclkdiv: MCLKDIV,
    _reserved58: [u8; 0x04],
    #[doc = "0x3b4 - SCT/PWM clock divider."]
    pub sctclkdiv: SCTCLKDIV,
    _reserved59: [u8; 0x0c],
    #[doc = "0x3c4 - PLL0 clock divider."]
    pub pll0clkdiv: PLL0CLKDIV,
    _reserved60: [u8; 0x34],
    #[doc = "0x3fc - Control clock configuration registers access (like xxxDIV, xxxSEL)"]
    pub clockgenupdatelockout: CLOCKGENUPDATELOCKOUT,
    #[doc = "0x400 - FMC configuration register."]
    pub fmccr: FMCCR,
    _reserved62: [u8; 0x18],
    #[doc = "0x41c - FMCflush control."]
    pub fmcflush: FMCFLUSH,
    #[doc = "0x420 - MCLK control."]
    pub mclkio: MCLKIO,
    _reserved64: [u8; 0x1c],
    #[doc = "0x440 - This 32-bit register contains the size of the image to remap, in bytes. The 12 LSBs are ignored, so the size granularity is 4KB."]
    pub flashremap_size: FLASHREMAP_SIZE,
    #[doc = "0x444 - This 32-bit register is a duplicate of FLASHREMAPSIZE for increased security."]
    pub flashremap_size_dp: FLASHREMAP_SIZE_DP,
    #[doc = "0x448 - This 32-bit register contains the offset by which the image is to be remapped. The 12 LSBs are ignored, so the remap granularity is 4KB."]
    pub flashremap_offset: FLASHREMAP_OFFSET,
    #[doc = "0x44c - This 32-bit register is a duplicate of FLASHREMAPOFFSET for increased security."]
    pub flashremap_offset_dp: FLASHREMAP_OFFSET_DP,
    _reserved68: [u8; 0x0c],
    #[doc = "0x45c - Control write access to FLASHREMAP_SIZE and FLASHREMAP_OFFSET registers."]
    pub flashremap_lock: FLASHREMAP_LOCK,
    _reserved69: [u8; 0x10],
    #[doc = "0x470 - Control CASPER integration."]
    pub casper_ctrl: CASPER_CTRL,
    _reserved70: [u8; 0xec],
    #[doc = "0x560 - PLL1 550m control."]
    pub pll1ctrl: PLL1CTRL,
    #[doc = "0x564 - PLL1 550m status."]
    pub pll1stat: PLL1STAT,
    #[doc = "0x568 - PLL1 550m N divider."]
    pub pll1ndec: PLL1NDEC,
    #[doc = "0x56c - PLL1 550m M divider."]
    pub pll1mdec: PLL1MDEC,
    #[doc = "0x570 - PLL1 550m P divider."]
    pub pll1pdec: PLL1PDEC,
    _reserved75: [u8; 0x0c],
    #[doc = "0x580 - PLL0 550m control."]
    pub pll0ctrl: PLL0CTRL,
    #[doc = "0x584 - PLL0 550m status."]
    pub pll0stat: PLL0STAT,
    #[doc = "0x588 - PLL0 550m N divider."]
    pub pll0ndec: PLL0NDEC,
    #[doc = "0x58c - PLL0 550m P divider."]
    pub pll0pdec: PLL0PDEC,
    #[doc = "0x590 - PLL0 Spread Spectrum Wrapper control register 0."]
    pub pll0sscg0: PLL0SSCG0,
    #[doc = "0x594 - PLL0 Spread Spectrum Wrapper control register 1."]
    pub pll0sscg1: PLL0SSCG1,
    _reserved81: [u8; 0x016c],
    #[doc = "0x704 - Functional retention control register."]
    pub funcretentionctrl: FUNCRETENTIONCTRL,
    _reserved82: [u8; 0x0104],
    #[doc = "0x80c - CPU Status."]
    pub cpstat: CPSTAT,
    _reserved83: [u8; 0x0110],
    #[doc = "0x920 - boot seed (256-bit random value)"]
    pub boot_seed_reg0: BOOT_SEED_REG0,
    #[doc = "0x924 - boot seed (256-bit random value)"]
    pub boot_seed_reg1: BOOT_SEED_REG1,
    #[doc = "0x928 - boot seed (256-bit random value)"]
    pub boot_seed_reg2: BOOT_SEED_REG2,
    #[doc = "0x92c - boot seed (256-bit random value)"]
    pub boot_seed_reg3: BOOT_SEED_REG3,
    #[doc = "0x930 - boot seed (256-bit random value)"]
    pub boot_seed_reg4: BOOT_SEED_REG4,
    #[doc = "0x934 - boot seed (256-bit random value)"]
    pub boot_seed_reg5: BOOT_SEED_REG5,
    #[doc = "0x938 - boot seed (256-bit random value)"]
    pub boot_seed_reg6: BOOT_SEED_REG6,
    #[doc = "0x93c - boot seed (256-bit random value)"]
    pub boot_seed_reg7: BOOT_SEED_REG7,
    #[doc = "0x940 - HMAC."]
    pub hmac_reg0: HMAC_REG0,
    #[doc = "0x944 - HMAC."]
    pub hmac_reg1: HMAC_REG1,
    #[doc = "0x948 - HMAC."]
    pub hmac_reg2: HMAC_REG2,
    #[doc = "0x94c - HMAC."]
    pub hmac_reg3: HMAC_REG3,
    #[doc = "0x950 - HMAC."]
    pub hmac_reg4: HMAC_REG4,
    #[doc = "0x954 - HMAC."]
    pub hmac_reg5: HMAC_REG5,
    #[doc = "0x958 - HMAC."]
    pub hmac_reg6: HMAC_REG6,
    #[doc = "0x95c - HMAC."]
    pub hmac_reg7: HMAC_REG7,
    #[doc = "0x960 - Control write access to boot seed security registers."]
    pub boot_lock: BOOT_LOCK,
    _reserved100: [u8; 0xb4],
    #[doc = "0xa18 - Various system clock controls : Flash clock (48 MHz) control, clocks to Frequency Measures."]
    pub clock_ctrl: CLOCK_CTRL,
    _reserved101: [u8; 0xf4],
    #[doc = "0xb10 - Comparator Interrupt control."]
    pub comp_int_ctrl: COMP_INT_CTRL,
    #[doc = "0xb14 - Comparator Interrupt status."]
    pub comp_int_status: COMP_INT_STATUS,
    _reserved103: [u8; 0x02ec],
    #[doc = "0xe04 - Control automatic clock gating."]
    pub autoclkgateoverride: AUTOCLKGATEOVERRIDE,
    #[doc = "0xe08 - Enable bypass of the first stage of synchonization inside GPIO_INT module."]
    pub gpiopsync: GPIOPSYNC,
    _reserved105: [u8; 0x017c],
    #[doc = "0xf88 - Controls whether the HASH AES hardware secret key is restricted to use by secure code."]
    pub hashresthwkey: HASHRESTHWKEY,
    _reserved106: [u8; 0x14],
    #[doc = "0xfa0 - Control write access to security registers."]
    pub debug_lock_en: DEBUG_LOCK_EN,
    #[doc = "0xfa4 - Cortex debug features control."]
    pub debug_features: DEBUG_FEATURES,
    #[doc = "0xfa8 - Cortex debug features control. (duplicate)"]
    pub debug_features_dp: DEBUG_FEATURES_DP,
    _reserved109: [u8; 0x08],
    #[doc = "0xfb4 - This register is used by ROM during DEBUG authentication mechanism to enable debug access port for CPU0."]
    pub swd_access_cpu0: SWD_ACCESS_CPU0,
    _reserved110: [u8; 0x04],
    #[doc = "0xfbc - block quiddikey/PUF all index."]
    pub key_block: KEY_BLOCK,
    #[doc = "0xfc0 - Debug authentication BEACON register."]
    pub debug_auth_beacon: DEBUG_AUTH_BEACON,
    _reserved112: [u8; 0x34],
    #[doc = "0xff8 - Device ID."]
    pub device_id0: DEVICE_ID0,
    #[doc = "0xffc - Chip revision ID and Number."]
    pub dieid: DIEID,
}
impl RegisterBlock {
    #[doc = "0x260 - Peripheral reset control register."]
    #[inline(always)]
    pub fn systickclkselx0(&self) -> &SYSTICKCLKSELX0 {
        unsafe { &*(((self as *const Self) as *const u8).add(608usize) as *const SYSTICKCLKSELX0) }
    }
    #[doc = "0x260 - System Tick Timer for CPU0 source select."]
    #[inline(always)]
    pub fn systickclksel0(&self) -> &SYSTICKCLKSEL0 {
        unsafe { &*(((self as *const Self) as *const u8).add(608usize) as *const SYSTICKCLKSEL0) }
    }
}
#[doc = "MEMORYREMAP (rw) register accessor: an alias for `Reg<MEMORYREMAP_SPEC>`"]
pub type MEMORYREMAP = crate::Reg<memoryremap::MEMORYREMAP_SPEC>;
#[doc = "Memory Remap control register."]
pub mod memoryremap;
#[doc = "AHBMATPRIO (rw) register accessor: an alias for `Reg<AHBMATPRIO_SPEC>`"]
pub type AHBMATPRIO = crate::Reg<ahbmatprio::AHBMATPRIO_SPEC>;
#[doc = "AHB Matrix priority control register Priority values are 3 = highest, 0 = lowest."]
pub mod ahbmatprio;
#[doc = "CPU0STCKCAL (rw) register accessor: an alias for `Reg<CPU0STCKCAL_SPEC>`"]
pub type CPU0STCKCAL = crate::Reg<cpu0stckcal::CPU0STCKCAL_SPEC>;
#[doc = "System tick calibration for secure part of CPU0."]
pub mod cpu0stckcal;
#[doc = "CPU0NSTCKCAL (rw) register accessor: an alias for `Reg<CPU0NSTCKCAL_SPEC>`"]
pub type CPU0NSTCKCAL = crate::Reg<cpu0nstckcal::CPU0NSTCKCAL_SPEC>;
#[doc = "System tick calibration for non-secure part of CPU0."]
pub mod cpu0nstckcal;
#[doc = "NMISRC (rw) register accessor: an alias for `Reg<NMISRC_SPEC>`"]
pub type NMISRC = crate::Reg<nmisrc::NMISRC_SPEC>;
#[doc = "NMI Source Select."]
pub mod nmisrc;
#[doc = "PRESETCTRL0 (rw) register accessor: an alias for `Reg<PRESETCTRL0_SPEC>`"]
pub type PRESETCTRL0 = crate::Reg<presetctrl0::PRESETCTRL0_SPEC>;
#[doc = "Peripheral reset control 0."]
pub mod presetctrl0;
#[doc = "PRESETCTRL1 (rw) register accessor: an alias for `Reg<PRESETCTRL1_SPEC>`"]
pub type PRESETCTRL1 = crate::Reg<presetctrl1::PRESETCTRL1_SPEC>;
#[doc = "Peripheral reset control 1."]
pub mod presetctrl1;
#[doc = "PRESETCTRL2 (rw) register accessor: an alias for `Reg<PRESETCTRL2_SPEC>`"]
pub type PRESETCTRL2 = crate::Reg<presetctrl2::PRESETCTRL2_SPEC>;
#[doc = "Peripheral reset control 2."]
pub mod presetctrl2;
#[doc = "PRESETCTRLSET (rw) register accessor: an alias for `Reg<PRESETCTRLSET_SPEC>`"]
pub type PRESETCTRLSET = crate::Reg<presetctrlset::PRESETCTRLSET_SPEC>;
#[doc = "Peripheral reset control set register."]
pub mod presetctrlset;
#[doc = "PRESETCTRLCLR (rw) register accessor: an alias for `Reg<PRESETCTRLCLR_SPEC>`"]
pub type PRESETCTRLCLR = crate::Reg<presetctrlclr::PRESETCTRLCLR_SPEC>;
#[doc = "Peripheral reset control clear register."]
pub mod presetctrlclr;
#[doc = "SWR_RESET (w) register accessor: an alias for `Reg<SWR_RESET_SPEC>`"]
pub type SWR_RESET = crate::Reg<swr_reset::SWR_RESET_SPEC>;
#[doc = "generate a software_reset."]
pub mod swr_reset;
#[doc = "AHBCLKCTRL0 (rw) register accessor: an alias for `Reg<AHBCLKCTRL0_SPEC>`"]
pub type AHBCLKCTRL0 = crate::Reg<ahbclkctrl0::AHBCLKCTRL0_SPEC>;
#[doc = "AHB Clock control 0."]
pub mod ahbclkctrl0;
#[doc = "AHBCLKCTRL1 (rw) register accessor: an alias for `Reg<AHBCLKCTRL1_SPEC>`"]
pub type AHBCLKCTRL1 = crate::Reg<ahbclkctrl1::AHBCLKCTRL1_SPEC>;
#[doc = "AHB Clock control 1."]
pub mod ahbclkctrl1;
#[doc = "AHBCLKCTRL2 (rw) register accessor: an alias for `Reg<AHBCLKCTRL2_SPEC>`"]
pub type AHBCLKCTRL2 = crate::Reg<ahbclkctrl2::AHBCLKCTRL2_SPEC>;
#[doc = "AHB Clock control 2."]
pub mod ahbclkctrl2;
#[doc = "AHBCLKCTRLSET (rw) register accessor: an alias for `Reg<AHBCLKCTRLSET_SPEC>`"]
pub type AHBCLKCTRLSET = crate::Reg<ahbclkctrlset::AHBCLKCTRLSET_SPEC>;
#[doc = "Peripheral reset control register."]
pub mod ahbclkctrlset;
#[doc = "AHBCLKCTRLCLR (rw) register accessor: an alias for `Reg<AHBCLKCTRLCLR_SPEC>`"]
pub type AHBCLKCTRLCLR = crate::Reg<ahbclkctrlclr::AHBCLKCTRLCLR_SPEC>;
#[doc = "Peripheral reset control register."]
pub mod ahbclkctrlclr;
#[doc = "SYSTICKCLKSEL0 (rw) register accessor: an alias for `Reg<SYSTICKCLKSEL0_SPEC>`"]
pub type SYSTICKCLKSEL0 = crate::Reg<systickclksel0::SYSTICKCLKSEL0_SPEC>;
#[doc = "System Tick Timer for CPU0 source select."]
pub mod systickclksel0;
#[doc = "SYSTICKCLKSELX0 (rw) register accessor: an alias for `Reg<SYSTICKCLKSELX0_SPEC>`"]
pub type SYSTICKCLKSELX0 = crate::Reg<systickclkselx0::SYSTICKCLKSELX0_SPEC>;
#[doc = "Peripheral reset control register."]
pub mod systickclkselx0;
#[doc = "TRACECLKSEL (rw) register accessor: an alias for `Reg<TRACECLKSEL_SPEC>`"]
pub type TRACECLKSEL = crate::Reg<traceclksel::TRACECLKSEL_SPEC>;
#[doc = "Trace clock source select."]
pub mod traceclksel;
#[doc = "CTIMERCLKSEL0 (rw) register accessor: an alias for `Reg<CTIMERCLKSEL0_SPEC>`"]
pub type CTIMERCLKSEL0 = crate::Reg<ctimerclksel0::CTIMERCLKSEL0_SPEC>;
#[doc = "CTimer 0 clock source select."]
pub mod ctimerclksel0;
#[doc = "CTIMERCLKSEL1 (rw) register accessor: an alias for `Reg<CTIMERCLKSEL1_SPEC>`"]
pub type CTIMERCLKSEL1 = crate::Reg<ctimerclksel1::CTIMERCLKSEL1_SPEC>;
#[doc = "CTimer 1 clock source select."]
pub mod ctimerclksel1;
#[doc = "CTIMERCLKSEL2 (rw) register accessor: an alias for `Reg<CTIMERCLKSEL2_SPEC>`"]
pub type CTIMERCLKSEL2 = crate::Reg<ctimerclksel2::CTIMERCLKSEL2_SPEC>;
#[doc = "CTimer 2 clock source select."]
pub mod ctimerclksel2;
#[doc = "CTIMERCLKSEL3 (rw) register accessor: an alias for `Reg<CTIMERCLKSEL3_SPEC>`"]
pub type CTIMERCLKSEL3 = crate::Reg<ctimerclksel3::CTIMERCLKSEL3_SPEC>;
#[doc = "CTimer 3 clock source select."]
pub mod ctimerclksel3;
#[doc = "CTIMERCLKSEL4 (rw) register accessor: an alias for `Reg<CTIMERCLKSEL4_SPEC>`"]
pub type CTIMERCLKSEL4 = crate::Reg<ctimerclksel4::CTIMERCLKSEL4_SPEC>;
#[doc = "CTimer 4 clock source select."]
pub mod ctimerclksel4;
#[doc = "MAINCLKSELA (rw) register accessor: an alias for `Reg<MAINCLKSELA_SPEC>`"]
pub type MAINCLKSELA = crate::Reg<mainclksela::MAINCLKSELA_SPEC>;
#[doc = "Main clock A source select."]
pub mod mainclksela;
#[doc = "MAINCLKSELB (rw) register accessor: an alias for `Reg<MAINCLKSELB_SPEC>`"]
pub type MAINCLKSELB = crate::Reg<mainclkselb::MAINCLKSELB_SPEC>;
#[doc = "Main clock source select."]
pub mod mainclkselb;
#[doc = "CLKOUTSEL (rw) register accessor: an alias for `Reg<CLKOUTSEL_SPEC>`"]
pub type CLKOUTSEL = crate::Reg<clkoutsel::CLKOUTSEL_SPEC>;
#[doc = "CLKOUT clock source select."]
pub mod clkoutsel;
#[doc = "PLL0CLKSEL (rw) register accessor: an alias for `Reg<PLL0CLKSEL_SPEC>`"]
pub type PLL0CLKSEL = crate::Reg<pll0clksel::PLL0CLKSEL_SPEC>;
#[doc = "PLL0 clock source select."]
pub mod pll0clksel;
#[doc = "PLL1CLKSEL (rw) register accessor: an alias for `Reg<PLL1CLKSEL_SPEC>`"]
pub type PLL1CLKSEL = crate::Reg<pll1clksel::PLL1CLKSEL_SPEC>;
#[doc = "PLL1 clock source select."]
pub mod pll1clksel;
#[doc = "CANCLKSEL (rw) register accessor: an alias for `Reg<CANCLKSEL_SPEC>`"]
pub type CANCLKSEL = crate::Reg<canclksel::CANCLKSEL_SPEC>;
#[doc = "CAN clock source select."]
pub mod canclksel;
#[doc = "ADCCLKSEL (rw) register accessor: an alias for `Reg<ADCCLKSEL_SPEC>`"]
pub type ADCCLKSEL = crate::Reg<adcclksel::ADCCLKSEL_SPEC>;
#[doc = "ADC clock source select."]
pub mod adcclksel;
#[doc = "FCCLKSEL0 (rw) register accessor: an alias for `Reg<FCCLKSEL0_SPEC>`"]
pub type FCCLKSEL0 = crate::Reg<fcclksel0::FCCLKSEL0_SPEC>;
#[doc = "Flexcomm Interface 0 clock source select for Fractional Rate Divider."]
pub mod fcclksel0;
#[doc = "FCCLKSEL1 (rw) register accessor: an alias for `Reg<FCCLKSEL1_SPEC>`"]
pub type FCCLKSEL1 = crate::Reg<fcclksel1::FCCLKSEL1_SPEC>;
#[doc = "Flexcomm Interface 1 clock source select for Fractional Rate Divider."]
pub mod fcclksel1;
#[doc = "FCCLKSEL2 (rw) register accessor: an alias for `Reg<FCCLKSEL2_SPEC>`"]
pub type FCCLKSEL2 = crate::Reg<fcclksel2::FCCLKSEL2_SPEC>;
#[doc = "Flexcomm Interface 2 clock source select for Fractional Rate Divider."]
pub mod fcclksel2;
#[doc = "FCCLKSEL3 (rw) register accessor: an alias for `Reg<FCCLKSEL3_SPEC>`"]
pub type FCCLKSEL3 = crate::Reg<fcclksel3::FCCLKSEL3_SPEC>;
#[doc = "Flexcomm Interface 3 clock source select for Fractional Rate Divider."]
pub mod fcclksel3;
#[doc = "FCCLKSEL4 (rw) register accessor: an alias for `Reg<FCCLKSEL4_SPEC>`"]
pub type FCCLKSEL4 = crate::Reg<fcclksel4::FCCLKSEL4_SPEC>;
#[doc = "Flexcomm Interface 4 clock source select for Fractional Rate Divider."]
pub mod fcclksel4;
#[doc = "FCCLKSEL5 (rw) register accessor: an alias for `Reg<FCCLKSEL5_SPEC>`"]
pub type FCCLKSEL5 = crate::Reg<fcclksel5::FCCLKSEL5_SPEC>;
#[doc = "Flexcomm Interface 5 clock source select for Fractional Rate Divider."]
pub mod fcclksel5;
#[doc = "FCCLKSEL6 (rw) register accessor: an alias for `Reg<FCCLKSEL6_SPEC>`"]
pub type FCCLKSEL6 = crate::Reg<fcclksel6::FCCLKSEL6_SPEC>;
#[doc = "Flexcomm Interface 6 clock source select for Fractional Rate Divider."]
pub mod fcclksel6;
#[doc = "FCCLKSEL7 (rw) register accessor: an alias for `Reg<FCCLKSEL7_SPEC>`"]
pub type FCCLKSEL7 = crate::Reg<fcclksel7::FCCLKSEL7_SPEC>;
#[doc = "Flexcomm Interface 7 clock source select for Fractional Rate Divider."]
pub mod fcclksel7;
#[doc = "HSLSPICLKSEL (rw) register accessor: an alias for `Reg<HSLSPICLKSEL_SPEC>`"]
pub type HSLSPICLKSEL = crate::Reg<hslspiclksel::HSLSPICLKSEL_SPEC>;
#[doc = "HS LSPI clock source select."]
pub mod hslspiclksel;
#[doc = "MCLKCLKSEL (rw) register accessor: an alias for `Reg<MCLKCLKSEL_SPEC>`"]
pub type MCLKCLKSEL = crate::Reg<mclkclksel::MCLKCLKSEL_SPEC>;
#[doc = "MCLK clock source select."]
pub mod mclkclksel;
#[doc = "SCTCLKSEL (rw) register accessor: an alias for `Reg<SCTCLKSEL_SPEC>`"]
pub type SCTCLKSEL = crate::Reg<sctclksel::SCTCLKSEL_SPEC>;
#[doc = "SCTimer/PWM clock source select."]
pub mod sctclksel;
#[doc = "SYSTICKCLKDIV0 (rw) register accessor: an alias for `Reg<SYSTICKCLKDIV0_SPEC>`"]
pub type SYSTICKCLKDIV0 = crate::Reg<systickclkdiv0::SYSTICKCLKDIV0_SPEC>;
#[doc = "System Tick Timer divider for CPU0."]
pub mod systickclkdiv0;
#[doc = "TRACECLKDIV (rw) register accessor: an alias for `Reg<TRACECLKDIV_SPEC>`"]
pub type TRACECLKDIV = crate::Reg<traceclkdiv::TRACECLKDIV_SPEC>;
#[doc = "TRACE clock divider."]
pub mod traceclkdiv;
#[doc = "CANCLKDIV (rw) register accessor: an alias for `Reg<CANCLKDIV_SPEC>`"]
pub type CANCLKDIV = crate::Reg<canclkdiv::CANCLKDIV_SPEC>;
#[doc = "CAN clock divider."]
pub mod canclkdiv;
#[doc = "FLEXFRG0CTRL (rw) register accessor: an alias for `Reg<FLEXFRG0CTRL_SPEC>`"]
pub type FLEXFRG0CTRL = crate::Reg<flexfrg0ctrl::FLEXFRG0CTRL_SPEC>;
#[doc = "Fractional rate divider for flexcomm 0."]
pub mod flexfrg0ctrl;
#[doc = "FLEXFRG1CTRL (rw) register accessor: an alias for `Reg<FLEXFRG1CTRL_SPEC>`"]
pub type FLEXFRG1CTRL = crate::Reg<flexfrg1ctrl::FLEXFRG1CTRL_SPEC>;
#[doc = "Fractional rate divider for flexcomm 1."]
pub mod flexfrg1ctrl;
#[doc = "FLEXFRG2CTRL (rw) register accessor: an alias for `Reg<FLEXFRG2CTRL_SPEC>`"]
pub type FLEXFRG2CTRL = crate::Reg<flexfrg2ctrl::FLEXFRG2CTRL_SPEC>;
#[doc = "Fractional rate divider for flexcomm 2."]
pub mod flexfrg2ctrl;
#[doc = "FLEXFRG3CTRL (rw) register accessor: an alias for `Reg<FLEXFRG3CTRL_SPEC>`"]
pub type FLEXFRG3CTRL = crate::Reg<flexfrg3ctrl::FLEXFRG3CTRL_SPEC>;
#[doc = "Fractional rate divider for flexcomm 3."]
pub mod flexfrg3ctrl;
#[doc = "FLEXFRG4CTRL (rw) register accessor: an alias for `Reg<FLEXFRG4CTRL_SPEC>`"]
pub type FLEXFRG4CTRL = crate::Reg<flexfrg4ctrl::FLEXFRG4CTRL_SPEC>;
#[doc = "Fractional rate divider for flexcomm 4."]
pub mod flexfrg4ctrl;
#[doc = "FLEXFRG5CTRL (rw) register accessor: an alias for `Reg<FLEXFRG5CTRL_SPEC>`"]
pub type FLEXFRG5CTRL = crate::Reg<flexfrg5ctrl::FLEXFRG5CTRL_SPEC>;
#[doc = "Fractional rate divider for flexcomm 5."]
pub mod flexfrg5ctrl;
#[doc = "FLEXFRG6CTRL (rw) register accessor: an alias for `Reg<FLEXFRG6CTRL_SPEC>`"]
pub type FLEXFRG6CTRL = crate::Reg<flexfrg6ctrl::FLEXFRG6CTRL_SPEC>;
#[doc = "Fractional rate divider for flexcomm 6."]
pub mod flexfrg6ctrl;
#[doc = "FLEXFRG7CTRL (rw) register accessor: an alias for `Reg<FLEXFRG7CTRL_SPEC>`"]
pub type FLEXFRG7CTRL = crate::Reg<flexfrg7ctrl::FLEXFRG7CTRL_SPEC>;
#[doc = "Fractional rate divider for flexcomm 7."]
pub mod flexfrg7ctrl;
#[doc = "AHBCLKDIV (rw) register accessor: an alias for `Reg<AHBCLKDIV_SPEC>`"]
pub type AHBCLKDIV = crate::Reg<ahbclkdiv::AHBCLKDIV_SPEC>;
#[doc = "System clock divider."]
pub mod ahbclkdiv;
#[doc = "CLKOUTDIV (rw) register accessor: an alias for `Reg<CLKOUTDIV_SPEC>`"]
pub type CLKOUTDIV = crate::Reg<clkoutdiv::CLKOUTDIV_SPEC>;
#[doc = "CLKOUT clock divider."]
pub mod clkoutdiv;
#[doc = "FROHFDIV (rw) register accessor: an alias for `Reg<FROHFDIV_SPEC>`"]
pub type FROHFDIV = crate::Reg<frohfdiv::FROHFDIV_SPEC>;
#[doc = "FRO_HF (96MHz) clock divider."]
pub mod frohfdiv;
#[doc = "WDTCLKDIV (rw) register accessor: an alias for `Reg<WDTCLKDIV_SPEC>`"]
pub type WDTCLKDIV = crate::Reg<wdtclkdiv::WDTCLKDIV_SPEC>;
#[doc = "WDT clock divider."]
pub mod wdtclkdiv;
#[doc = "ADCCLKDIV (rw) register accessor: an alias for `Reg<ADCCLKDIV_SPEC>`"]
pub type ADCCLKDIV = crate::Reg<adcclkdiv::ADCCLKDIV_SPEC>;
#[doc = "ADC clock divider."]
pub mod adcclkdiv;
#[doc = "MCLKDIV (rw) register accessor: an alias for `Reg<MCLKDIV_SPEC>`"]
pub type MCLKDIV = crate::Reg<mclkdiv::MCLKDIV_SPEC>;
#[doc = "I2S MCLK clock divider."]
pub mod mclkdiv;
#[doc = "SCTCLKDIV (rw) register accessor: an alias for `Reg<SCTCLKDIV_SPEC>`"]
pub type SCTCLKDIV = crate::Reg<sctclkdiv::SCTCLKDIV_SPEC>;
#[doc = "SCT/PWM clock divider."]
pub mod sctclkdiv;
#[doc = "PLL0CLKDIV (rw) register accessor: an alias for `Reg<PLL0CLKDIV_SPEC>`"]
pub type PLL0CLKDIV = crate::Reg<pll0clkdiv::PLL0CLKDIV_SPEC>;
#[doc = "PLL0 clock divider."]
pub mod pll0clkdiv;
#[doc = "CLOCKGENUPDATELOCKOUT (rw) register accessor: an alias for `Reg<CLOCKGENUPDATELOCKOUT_SPEC>`"]
pub type CLOCKGENUPDATELOCKOUT = crate::Reg<clockgenupdatelockout::CLOCKGENUPDATELOCKOUT_SPEC>;
#[doc = "Control clock configuration registers access (like xxxDIV, xxxSEL)"]
pub mod clockgenupdatelockout;
#[doc = "FMCCR (rw) register accessor: an alias for `Reg<FMCCR_SPEC>`"]
pub type FMCCR = crate::Reg<fmccr::FMCCR_SPEC>;
#[doc = "FMC configuration register."]
pub mod fmccr;
#[doc = "FMCFLUSH (w) register accessor: an alias for `Reg<FMCFLUSH_SPEC>`"]
pub type FMCFLUSH = crate::Reg<fmcflush::FMCFLUSH_SPEC>;
#[doc = "FMCflush control."]
pub mod fmcflush;
#[doc = "MCLKIO (rw) register accessor: an alias for `Reg<MCLKIO_SPEC>`"]
pub type MCLKIO = crate::Reg<mclkio::MCLKIO_SPEC>;
#[doc = "MCLK control."]
pub mod mclkio;
#[doc = "FLASHREMAP_SIZE (rw) register accessor: an alias for `Reg<FLASHREMAP_SIZE_SPEC>`"]
pub type FLASHREMAP_SIZE = crate::Reg<flashremap_size::FLASHREMAP_SIZE_SPEC>;
#[doc = "This 32-bit register contains the size of the image to remap, in bytes. The 12 LSBs are ignored, so the size granularity is 4KB."]
pub mod flashremap_size;
#[doc = "FLASHREMAP_SIZE_DP (rw) register accessor: an alias for `Reg<FLASHREMAP_SIZE_DP_SPEC>`"]
pub type FLASHREMAP_SIZE_DP = crate::Reg<flashremap_size_dp::FLASHREMAP_SIZE_DP_SPEC>;
#[doc = "This 32-bit register is a duplicate of FLASHREMAPSIZE for increased security."]
pub mod flashremap_size_dp;
#[doc = "FLASHREMAP_OFFSET (rw) register accessor: an alias for `Reg<FLASHREMAP_OFFSET_SPEC>`"]
pub type FLASHREMAP_OFFSET = crate::Reg<flashremap_offset::FLASHREMAP_OFFSET_SPEC>;
#[doc = "This 32-bit register contains the offset by which the image is to be remapped. The 12 LSBs are ignored, so the remap granularity is 4KB."]
pub mod flashremap_offset;
#[doc = "FLASHREMAP_OFFSET_DP (rw) register accessor: an alias for `Reg<FLASHREMAP_OFFSET_DP_SPEC>`"]
pub type FLASHREMAP_OFFSET_DP = crate::Reg<flashremap_offset_dp::FLASHREMAP_OFFSET_DP_SPEC>;
#[doc = "This 32-bit register is a duplicate of FLASHREMAPOFFSET for increased security."]
pub mod flashremap_offset_dp;
#[doc = "FLASHREMAP_LOCK (rw) register accessor: an alias for `Reg<FLASHREMAP_LOCK_SPEC>`"]
pub type FLASHREMAP_LOCK = crate::Reg<flashremap_lock::FLASHREMAP_LOCK_SPEC>;
#[doc = "Control write access to FLASHREMAP_SIZE and FLASHREMAP_OFFSET registers."]
pub mod flashremap_lock;
#[doc = "CASPER_CTRL (rw) register accessor: an alias for `Reg<CASPER_CTRL_SPEC>`"]
pub type CASPER_CTRL = crate::Reg<casper_ctrl::CASPER_CTRL_SPEC>;
#[doc = "Control CASPER integration."]
pub mod casper_ctrl;
#[doc = "PLL1CTRL (rw) register accessor: an alias for `Reg<PLL1CTRL_SPEC>`"]
pub type PLL1CTRL = crate::Reg<pll1ctrl::PLL1CTRL_SPEC>;
#[doc = "PLL1 550m control."]
pub mod pll1ctrl;
#[doc = "PLL1STAT (rw) register accessor: an alias for `Reg<PLL1STAT_SPEC>`"]
pub type PLL1STAT = crate::Reg<pll1stat::PLL1STAT_SPEC>;
#[doc = "PLL1 550m status."]
pub mod pll1stat;
#[doc = "PLL1NDEC (rw) register accessor: an alias for `Reg<PLL1NDEC_SPEC>`"]
pub type PLL1NDEC = crate::Reg<pll1ndec::PLL1NDEC_SPEC>;
#[doc = "PLL1 550m N divider."]
pub mod pll1ndec;
#[doc = "PLL1MDEC (rw) register accessor: an alias for `Reg<PLL1MDEC_SPEC>`"]
pub type PLL1MDEC = crate::Reg<pll1mdec::PLL1MDEC_SPEC>;
#[doc = "PLL1 550m M divider."]
pub mod pll1mdec;
#[doc = "PLL1PDEC (rw) register accessor: an alias for `Reg<PLL1PDEC_SPEC>`"]
pub type PLL1PDEC = crate::Reg<pll1pdec::PLL1PDEC_SPEC>;
#[doc = "PLL1 550m P divider."]
pub mod pll1pdec;
#[doc = "PLL0CTRL (rw) register accessor: an alias for `Reg<PLL0CTRL_SPEC>`"]
pub type PLL0CTRL = crate::Reg<pll0ctrl::PLL0CTRL_SPEC>;
#[doc = "PLL0 550m control."]
pub mod pll0ctrl;
#[doc = "PLL0STAT (rw) register accessor: an alias for `Reg<PLL0STAT_SPEC>`"]
pub type PLL0STAT = crate::Reg<pll0stat::PLL0STAT_SPEC>;
#[doc = "PLL0 550m status."]
pub mod pll0stat;
#[doc = "PLL0NDEC (rw) register accessor: an alias for `Reg<PLL0NDEC_SPEC>`"]
pub type PLL0NDEC = crate::Reg<pll0ndec::PLL0NDEC_SPEC>;
#[doc = "PLL0 550m N divider."]
pub mod pll0ndec;
#[doc = "PLL0PDEC (rw) register accessor: an alias for `Reg<PLL0PDEC_SPEC>`"]
pub type PLL0PDEC = crate::Reg<pll0pdec::PLL0PDEC_SPEC>;
#[doc = "PLL0 550m P divider."]
pub mod pll0pdec;
#[doc = "PLL0SSCG0 (rw) register accessor: an alias for `Reg<PLL0SSCG0_SPEC>`"]
pub type PLL0SSCG0 = crate::Reg<pll0sscg0::PLL0SSCG0_SPEC>;
#[doc = "PLL0 Spread Spectrum Wrapper control register 0."]
pub mod pll0sscg0;
#[doc = "PLL0SSCG1 (rw) register accessor: an alias for `Reg<PLL0SSCG1_SPEC>`"]
pub type PLL0SSCG1 = crate::Reg<pll0sscg1::PLL0SSCG1_SPEC>;
#[doc = "PLL0 Spread Spectrum Wrapper control register 1."]
pub mod pll0sscg1;
#[doc = "FUNCRETENTIONCTRL (rw) register accessor: an alias for `Reg<FUNCRETENTIONCTRL_SPEC>`"]
pub type FUNCRETENTIONCTRL = crate::Reg<funcretentionctrl::FUNCRETENTIONCTRL_SPEC>;
#[doc = "Functional retention control register."]
pub mod funcretentionctrl;
#[doc = "CPSTAT (rw) register accessor: an alias for `Reg<CPSTAT_SPEC>`"]
pub type CPSTAT = crate::Reg<cpstat::CPSTAT_SPEC>;
#[doc = "CPU Status."]
pub mod cpstat;
#[doc = "BOOT_SEED_REG0 (rw) register accessor: an alias for `Reg<BOOT_SEED_REG0_SPEC>`"]
pub type BOOT_SEED_REG0 = crate::Reg<boot_seed_reg0::BOOT_SEED_REG0_SPEC>;
#[doc = "boot seed (256-bit random value)"]
pub mod boot_seed_reg0;
#[doc = "BOOT_SEED_REG1 (rw) register accessor: an alias for `Reg<BOOT_SEED_REG1_SPEC>`"]
pub type BOOT_SEED_REG1 = crate::Reg<boot_seed_reg1::BOOT_SEED_REG1_SPEC>;
#[doc = "boot seed (256-bit random value)"]
pub mod boot_seed_reg1;
#[doc = "BOOT_SEED_REG2 (rw) register accessor: an alias for `Reg<BOOT_SEED_REG2_SPEC>`"]
pub type BOOT_SEED_REG2 = crate::Reg<boot_seed_reg2::BOOT_SEED_REG2_SPEC>;
#[doc = "boot seed (256-bit random value)"]
pub mod boot_seed_reg2;
#[doc = "BOOT_SEED_REG3 (rw) register accessor: an alias for `Reg<BOOT_SEED_REG3_SPEC>`"]
pub type BOOT_SEED_REG3 = crate::Reg<boot_seed_reg3::BOOT_SEED_REG3_SPEC>;
#[doc = "boot seed (256-bit random value)"]
pub mod boot_seed_reg3;
#[doc = "BOOT_SEED_REG4 (rw) register accessor: an alias for `Reg<BOOT_SEED_REG4_SPEC>`"]
pub type BOOT_SEED_REG4 = crate::Reg<boot_seed_reg4::BOOT_SEED_REG4_SPEC>;
#[doc = "boot seed (256-bit random value)"]
pub mod boot_seed_reg4;
#[doc = "BOOT_SEED_REG5 (rw) register accessor: an alias for `Reg<BOOT_SEED_REG5_SPEC>`"]
pub type BOOT_SEED_REG5 = crate::Reg<boot_seed_reg5::BOOT_SEED_REG5_SPEC>;
#[doc = "boot seed (256-bit random value)"]
pub mod boot_seed_reg5;
#[doc = "BOOT_SEED_REG6 (rw) register accessor: an alias for `Reg<BOOT_SEED_REG6_SPEC>`"]
pub type BOOT_SEED_REG6 = crate::Reg<boot_seed_reg6::BOOT_SEED_REG6_SPEC>;
#[doc = "boot seed (256-bit random value)"]
pub mod boot_seed_reg6;
#[doc = "BOOT_SEED_REG7 (rw) register accessor: an alias for `Reg<BOOT_SEED_REG7_SPEC>`"]
pub type BOOT_SEED_REG7 = crate::Reg<boot_seed_reg7::BOOT_SEED_REG7_SPEC>;
#[doc = "boot seed (256-bit random value)"]
pub mod boot_seed_reg7;
#[doc = "HMAC_REG0 (rw) register accessor: an alias for `Reg<HMAC_REG0_SPEC>`"]
pub type HMAC_REG0 = crate::Reg<hmac_reg0::HMAC_REG0_SPEC>;
#[doc = "HMAC."]
pub mod hmac_reg0;
#[doc = "HMAC_REG1 (rw) register accessor: an alias for `Reg<HMAC_REG1_SPEC>`"]
pub type HMAC_REG1 = crate::Reg<hmac_reg1::HMAC_REG1_SPEC>;
#[doc = "HMAC."]
pub mod hmac_reg1;
#[doc = "HMAC_REG2 (rw) register accessor: an alias for `Reg<HMAC_REG2_SPEC>`"]
pub type HMAC_REG2 = crate::Reg<hmac_reg2::HMAC_REG2_SPEC>;
#[doc = "HMAC."]
pub mod hmac_reg2;
#[doc = "HMAC_REG3 (rw) register accessor: an alias for `Reg<HMAC_REG3_SPEC>`"]
pub type HMAC_REG3 = crate::Reg<hmac_reg3::HMAC_REG3_SPEC>;
#[doc = "HMAC."]
pub mod hmac_reg3;
#[doc = "HMAC_REG4 (rw) register accessor: an alias for `Reg<HMAC_REG4_SPEC>`"]
pub type HMAC_REG4 = crate::Reg<hmac_reg4::HMAC_REG4_SPEC>;
#[doc = "HMAC."]
pub mod hmac_reg4;
#[doc = "HMAC_REG5 (rw) register accessor: an alias for `Reg<HMAC_REG5_SPEC>`"]
pub type HMAC_REG5 = crate::Reg<hmac_reg5::HMAC_REG5_SPEC>;
#[doc = "HMAC."]
pub mod hmac_reg5;
#[doc = "HMAC_REG6 (rw) register accessor: an alias for `Reg<HMAC_REG6_SPEC>`"]
pub type HMAC_REG6 = crate::Reg<hmac_reg6::HMAC_REG6_SPEC>;
#[doc = "HMAC."]
pub mod hmac_reg6;
#[doc = "HMAC_REG7 (rw) register accessor: an alias for `Reg<HMAC_REG7_SPEC>`"]
pub type HMAC_REG7 = crate::Reg<hmac_reg7::HMAC_REG7_SPEC>;
#[doc = "HMAC."]
pub mod hmac_reg7;
#[doc = "BOOT_LOCK (rw) register accessor: an alias for `Reg<BOOT_LOCK_SPEC>`"]
pub type BOOT_LOCK = crate::Reg<boot_lock::BOOT_LOCK_SPEC>;
#[doc = "Control write access to boot seed security registers."]
pub mod boot_lock;
#[doc = "CLOCK_CTRL (rw) register accessor: an alias for `Reg<CLOCK_CTRL_SPEC>`"]
pub type CLOCK_CTRL = crate::Reg<clock_ctrl::CLOCK_CTRL_SPEC>;
#[doc = "Various system clock controls : Flash clock (48 MHz) control, clocks to Frequency Measures."]
pub mod clock_ctrl;
#[doc = "COMP_INT_CTRL (rw) register accessor: an alias for `Reg<COMP_INT_CTRL_SPEC>`"]
pub type COMP_INT_CTRL = crate::Reg<comp_int_ctrl::COMP_INT_CTRL_SPEC>;
#[doc = "Comparator Interrupt control."]
pub mod comp_int_ctrl;
#[doc = "COMP_INT_STATUS (rw) register accessor: an alias for `Reg<COMP_INT_STATUS_SPEC>`"]
pub type COMP_INT_STATUS = crate::Reg<comp_int_status::COMP_INT_STATUS_SPEC>;
#[doc = "Comparator Interrupt status."]
pub mod comp_int_status;
#[doc = "AUTOCLKGATEOVERRIDE (rw) register accessor: an alias for `Reg<AUTOCLKGATEOVERRIDE_SPEC>`"]
pub type AUTOCLKGATEOVERRIDE = crate::Reg<autoclkgateoverride::AUTOCLKGATEOVERRIDE_SPEC>;
#[doc = "Control automatic clock gating."]
pub mod autoclkgateoverride;
#[doc = "GPIOPSYNC (rw) register accessor: an alias for `Reg<GPIOPSYNC_SPEC>`"]
pub type GPIOPSYNC = crate::Reg<gpiopsync::GPIOPSYNC_SPEC>;
#[doc = "Enable bypass of the first stage of synchonization inside GPIO_INT module."]
pub mod gpiopsync;
#[doc = "HASHRESTHWKEY (rw) register accessor: an alias for `Reg<HASHRESTHWKEY_SPEC>`"]
pub type HASHRESTHWKEY = crate::Reg<hashresthwkey::HASHRESTHWKEY_SPEC>;
#[doc = "Controls whether the HASH AES hardware secret key is restricted to use by secure code."]
pub mod hashresthwkey;
#[doc = "DEBUG_LOCK_EN (rw) register accessor: an alias for `Reg<DEBUG_LOCK_EN_SPEC>`"]
pub type DEBUG_LOCK_EN = crate::Reg<debug_lock_en::DEBUG_LOCK_EN_SPEC>;
#[doc = "Control write access to security registers."]
pub mod debug_lock_en;
#[doc = "DEBUG_FEATURES (rw) register accessor: an alias for `Reg<DEBUG_FEATURES_SPEC>`"]
pub type DEBUG_FEATURES = crate::Reg<debug_features::DEBUG_FEATURES_SPEC>;
#[doc = "Cortex debug features control."]
pub mod debug_features;
#[doc = "DEBUG_FEATURES_DP (rw) register accessor: an alias for `Reg<DEBUG_FEATURES_DP_SPEC>`"]
pub type DEBUG_FEATURES_DP = crate::Reg<debug_features_dp::DEBUG_FEATURES_DP_SPEC>;
#[doc = "Cortex debug features control. (duplicate)"]
pub mod debug_features_dp;
#[doc = "SWD_ACCESS_CPU0 (rw) register accessor: an alias for `Reg<SWD_ACCESS_CPU0_SPEC>`"]
pub type SWD_ACCESS_CPU0 = crate::Reg<swd_access_cpu0::SWD_ACCESS_CPU0_SPEC>;
#[doc = "This register is used by ROM during DEBUG authentication mechanism to enable debug access port for CPU0."]
pub mod swd_access_cpu0;
#[doc = "KEY_BLOCK (w) register accessor: an alias for `Reg<KEY_BLOCK_SPEC>`"]
pub type KEY_BLOCK = crate::Reg<key_block::KEY_BLOCK_SPEC>;
#[doc = "block quiddikey/PUF all index."]
pub mod key_block;
#[doc = "DEBUG_AUTH_BEACON (rw) register accessor: an alias for `Reg<DEBUG_AUTH_BEACON_SPEC>`"]
pub type DEBUG_AUTH_BEACON = crate::Reg<debug_auth_beacon::DEBUG_AUTH_BEACON_SPEC>;
#[doc = "Debug authentication BEACON register."]
pub mod debug_auth_beacon;
#[doc = "DEVICE_ID0 (r) register accessor: an alias for `Reg<DEVICE_ID0_SPEC>`"]
pub type DEVICE_ID0 = crate::Reg<device_id0::DEVICE_ID0_SPEC>;
#[doc = "Device ID."]
pub mod device_id0;
#[doc = "DIEID (r) register accessor: an alias for `Reg<DIEID_SPEC>`"]
pub type DIEID = crate::Reg<dieid::DIEID_SPEC>;
#[doc = "Chip revision ID and Number."]
pub mod dieid;
