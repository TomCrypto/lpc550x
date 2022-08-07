#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - ."]
    pub header: HEADER,
    #[doc = "0x04..0x400 - ."]
    pub patch: [PATCH; 255],
}
#[doc = "HEADER (rw) register accessor: an alias for `Reg<HEADER_SPEC>`"]
pub type HEADER = crate::Reg<header::HEADER_SPEC>;
#[doc = "."]
pub mod header;
#[doc = "PATCH (rw) register accessor: an alias for `Reg<PATCH_SPEC>`"]
pub type PATCH = crate::Reg<patch::PATCH_SPEC>;
#[doc = "."]
pub mod patch;
