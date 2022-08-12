#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Various Analog blocks configuration (like FRO 192MHz trimmings source ...)"]
    pub analog_ctrl_cfg: ANALOG_CTRL_CFG,
    #[doc = "0x04 - Analog Macroblock Identity registers, Flash Status registers."]
    pub analog_ctrl_status: ANALOG_CTRL_STATUS,
    _reserved2: [u8; 0x04],
    #[doc = "0x0c - Frequency Measure function control register."]
    pub freq_me_ctrl: FREQ_ME_CTRL,
    #[doc = "0x10 - 192MHz Free Running OScillator (FRO) Control register."]
    pub fro192m_ctrl: FRO192M_CTRL,
    #[doc = "0x14 - 192MHz Free Running OScillator (FRO) Status register."]
    pub fro192m_status: FRO192M_STATUS,
    #[doc = "0x18 - ADC control static configuration."]
    pub adc_ctrl: ADC_CTRL,
    _reserved6: [u8; 0x04],
    #[doc = "0x20 - High speed Crystal Oscillator Control register."]
    pub xo32m_ctrl: XO32M_CTRL,
    #[doc = "0x24 - High speed Crystal Oscillator Status register."]
    pub xo32m_status: XO32M_STATUS,
    _reserved8: [u8; 0x08],
    #[doc = "0x30 - Brown Out Detectors (BoDs) & DCDC interrupts generation control register."]
    pub bod_dcdc_int_ctrl: BOD_DCDC_INT_CTRL,
    #[doc = "0x34 - BoDs & DCDC interrupts status register."]
    pub bod_dcdc_int_status: BOD_DCDC_INT_STATUS,
    _reserved10: [u8; 0x08],
    #[doc = "0x40 - First Ring Oscillator module control register."]
    pub ringo0_ctrl: RINGO0_CTRL,
    #[doc = "0x44 - Second Ring Oscillator module control register."]
    pub ringo1_ctrl: RINGO1_CTRL,
    #[doc = "0x48 - Third Ring Oscillator module control register."]
    pub ringo2_ctrl: RINGO2_CTRL,
    _reserved13: [u8; 0x64],
    #[doc = "0xb0 - High Speed Crystal Oscillator (12 MHz - 32 MHz) Voltage Source Supply Control register."]
    pub ldo_xo32m: LDO_XO32M,
    #[doc = "0xb4 - AUX_BIAS."]
    pub aux_bias: AUX_BIAS,
    _reserved15: [u8; 0x40],
    #[doc = "0xf8 - Dummy Control bus to analog modules."]
    pub dummy_ctrl: DUMMY_CTRL,
}
#[doc = "ANALOG_CTRL_CFG (rw) register accessor: an alias for `Reg<ANALOG_CTRL_CFG_SPEC>`"]
pub type ANALOG_CTRL_CFG = crate::Reg<analog_ctrl_cfg::ANALOG_CTRL_CFG_SPEC>;
#[doc = "Various Analog blocks configuration (like FRO 192MHz trimmings source ...)"]
pub mod analog_ctrl_cfg;
#[doc = "ANALOG_CTRL_STATUS (r) register accessor: an alias for `Reg<ANALOG_CTRL_STATUS_SPEC>`"]
pub type ANALOG_CTRL_STATUS = crate::Reg<analog_ctrl_status::ANALOG_CTRL_STATUS_SPEC>;
#[doc = "Analog Macroblock Identity registers, Flash Status registers."]
pub mod analog_ctrl_status;
#[doc = "FREQ_ME_CTRL (rw) register accessor: an alias for `Reg<FREQ_ME_CTRL_SPEC>`"]
pub type FREQ_ME_CTRL = crate::Reg<freq_me_ctrl::FREQ_ME_CTRL_SPEC>;
#[doc = "Frequency Measure function control register."]
pub mod freq_me_ctrl;
#[doc = "FRO192M_CTRL (rw) register accessor: an alias for `Reg<FRO192M_CTRL_SPEC>`"]
pub type FRO192M_CTRL = crate::Reg<fro192m_ctrl::FRO192M_CTRL_SPEC>;
#[doc = "192MHz Free Running OScillator (FRO) Control register."]
pub mod fro192m_ctrl;
#[doc = "FRO192M_STATUS (rw) register accessor: an alias for `Reg<FRO192M_STATUS_SPEC>`"]
pub type FRO192M_STATUS = crate::Reg<fro192m_status::FRO192M_STATUS_SPEC>;
#[doc = "192MHz Free Running OScillator (FRO) Status register."]
pub mod fro192m_status;
#[doc = "ADC_CTRL (rw) register accessor: an alias for `Reg<ADC_CTRL_SPEC>`"]
pub type ADC_CTRL = crate::Reg<adc_ctrl::ADC_CTRL_SPEC>;
#[doc = "ADC control static configuration."]
pub mod adc_ctrl;
#[doc = "XO32M_CTRL (rw) register accessor: an alias for `Reg<XO32M_CTRL_SPEC>`"]
pub type XO32M_CTRL = crate::Reg<xo32m_ctrl::XO32M_CTRL_SPEC>;
#[doc = "High speed Crystal Oscillator Control register."]
pub mod xo32m_ctrl;
#[doc = "XO32M_STATUS (r) register accessor: an alias for `Reg<XO32M_STATUS_SPEC>`"]
pub type XO32M_STATUS = crate::Reg<xo32m_status::XO32M_STATUS_SPEC>;
#[doc = "High speed Crystal Oscillator Status register."]
pub mod xo32m_status;
#[doc = "BOD_DCDC_INT_CTRL (rw) register accessor: an alias for `Reg<BOD_DCDC_INT_CTRL_SPEC>`"]
pub type BOD_DCDC_INT_CTRL = crate::Reg<bod_dcdc_int_ctrl::BOD_DCDC_INT_CTRL_SPEC>;
#[doc = "Brown Out Detectors (BoDs) & DCDC interrupts generation control register."]
pub mod bod_dcdc_int_ctrl;
#[doc = "BOD_DCDC_INT_STATUS (r) register accessor: an alias for `Reg<BOD_DCDC_INT_STATUS_SPEC>`"]
pub type BOD_DCDC_INT_STATUS = crate::Reg<bod_dcdc_int_status::BOD_DCDC_INT_STATUS_SPEC>;
#[doc = "BoDs & DCDC interrupts status register."]
pub mod bod_dcdc_int_status;
#[doc = "RINGO0_CTRL (rw) register accessor: an alias for `Reg<RINGO0_CTRL_SPEC>`"]
pub type RINGO0_CTRL = crate::Reg<ringo0_ctrl::RINGO0_CTRL_SPEC>;
#[doc = "First Ring Oscillator module control register."]
pub mod ringo0_ctrl;
#[doc = "RINGO1_CTRL (rw) register accessor: an alias for `Reg<RINGO1_CTRL_SPEC>`"]
pub type RINGO1_CTRL = crate::Reg<ringo1_ctrl::RINGO1_CTRL_SPEC>;
#[doc = "Second Ring Oscillator module control register."]
pub mod ringo1_ctrl;
#[doc = "RINGO2_CTRL (rw) register accessor: an alias for `Reg<RINGO2_CTRL_SPEC>`"]
pub type RINGO2_CTRL = crate::Reg<ringo2_ctrl::RINGO2_CTRL_SPEC>;
#[doc = "Third Ring Oscillator module control register."]
pub mod ringo2_ctrl;
#[doc = "LDO_XO32M (rw) register accessor: an alias for `Reg<LDO_XO32M_SPEC>`"]
pub type LDO_XO32M = crate::Reg<ldo_xo32m::LDO_XO32M_SPEC>;
#[doc = "High Speed Crystal Oscillator (12 MHz - 32 MHz) Voltage Source Supply Control register."]
pub mod ldo_xo32m;
#[doc = "AUX_BIAS (rw) register accessor: an alias for `Reg<AUX_BIAS_SPEC>`"]
pub type AUX_BIAS = crate::Reg<aux_bias::AUX_BIAS_SPEC>;
#[doc = "AUX_BIAS."]
pub mod aux_bias;
#[doc = "DUMMY_CTRL (rw) register accessor: an alias for `Reg<DUMMY_CTRL_SPEC>`"]
pub type DUMMY_CTRL = crate::Reg<dummy_ctrl::DUMMY_CTRL_SPEC>;
#[doc = "Dummy Control bus to analog modules."]
pub mod dummy_ctrl;
