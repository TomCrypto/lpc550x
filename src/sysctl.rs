#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - update lock out control."]
    pub updatelckout: UPDATELCKOUT,
    _reserved1: [u8; 0x3c],
    #[doc = "0x40..0x60 - Selects the source for SCK going into Flexcomm index."]
    pub fcctrlsel: [FCCTRLSEL; 8],
    _reserved2: [u8; 0x20],
    #[doc = "0x80..0x88 - Selects sources and data combinations for shared signal set index."]
    pub sharedctrlset: [SHAREDCTRLSET; 2],
    _reserved3: [u8; 0xf8],
    #[doc = "0x180 - CODE_GRAY LSB input Register."]
    pub code_gray_lsb: CODE_GRAY_LSB,
    #[doc = "0x184 - CODE_GRAY MSB input Register."]
    pub code_gray_msb: CODE_GRAY_MSB,
    #[doc = "0x188 - CODE_BIN LSB output Register."]
    pub code_bin_lsb: CODE_BIN_LSB,
    #[doc = "0x18c - CODE_BIN MSB output Register."]
    pub code_bin_msb: CODE_BIN_MSB,
}
#[doc = "UPDATELCKOUT (rw) register accessor: an alias for `Reg<UPDATELCKOUT_SPEC>`"]
pub type UPDATELCKOUT = crate::Reg<updatelckout::UPDATELCKOUT_SPEC>;
#[doc = "update lock out control."]
pub mod updatelckout;
#[doc = "FCCTRLSEL (rw) register accessor: an alias for `Reg<FCCTRLSEL_SPEC>`"]
pub type FCCTRLSEL = crate::Reg<fcctrlsel::FCCTRLSEL_SPEC>;
#[doc = "Selects the source for SCK going into Flexcomm index."]
pub mod fcctrlsel;
#[doc = "SHAREDCTRLSET (rw) register accessor: an alias for `Reg<SHAREDCTRLSET_SPEC>`"]
pub type SHAREDCTRLSET = crate::Reg<sharedctrlset::SHAREDCTRLSET_SPEC>;
#[doc = "Selects sources and data combinations for shared signal set index."]
pub mod sharedctrlset;
#[doc = "CODE_GRAY_LSB (rw) register accessor: an alias for `Reg<CODE_GRAY_LSB_SPEC>`"]
pub type CODE_GRAY_LSB = crate::Reg<code_gray_lsb::CODE_GRAY_LSB_SPEC>;
#[doc = "CODE_GRAY LSB input Register."]
pub mod code_gray_lsb;
#[doc = "CODE_GRAY_MSB (rw) register accessor: an alias for `Reg<CODE_GRAY_MSB_SPEC>`"]
pub type CODE_GRAY_MSB = crate::Reg<code_gray_msb::CODE_GRAY_MSB_SPEC>;
#[doc = "CODE_GRAY MSB input Register."]
pub mod code_gray_msb;
#[doc = "CODE_BIN_LSB (r) register accessor: an alias for `Reg<CODE_BIN_LSB_SPEC>`"]
pub type CODE_BIN_LSB = crate::Reg<code_bin_lsb::CODE_BIN_LSB_SPEC>;
#[doc = "CODE_BIN LSB output Register."]
pub mod code_bin_lsb;
#[doc = "CODE_BIN_MSB (r) register accessor: an alias for `Reg<CODE_BIN_MSB_SPEC>`"]
pub type CODE_BIN_MSB = crate::Reg<code_bin_msb::CODE_BIN_MSB_SPEC>;
#[doc = "CODE_BIN MSB output Register."]
pub mod code_bin_msb;
