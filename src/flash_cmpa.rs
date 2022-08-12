#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - no description available."]
    pub boot_cfg: BOOT_CFG,
    #[doc = "0x04 - no description available."]
    pub spi_flash_cfg: SPI_FLASH_CFG,
    #[doc = "0x08 - no description available."]
    pub usb_id: USB_ID,
    #[doc = "0x0c - no description available."]
    pub sdio_cfg: SDIO_CFG,
    #[doc = "0x10 - no description available."]
    pub cc_socu_pin: CC_SOCU_PIN,
    #[doc = "0x14 - no description available."]
    pub cc_socu_dflt: CC_SOCU_DFLT,
    #[doc = "0x18 - no description available."]
    pub vendor_usage: VENDOR_USAGE,
    #[doc = "0x1c - Secure boot configuration flags."]
    pub secure_boot_cfg: SECURE_BOOT_CFG,
    #[doc = "0x20 - no description available."]
    pub prince_base_addr: PRINCE_BASE_ADDR,
    #[doc = "0x24 - Region 0, sub-region enable."]
    pub prince_sr_0: PRINCE_SR_0,
    #[doc = "0x28 - Region 1, sub-region enable."]
    pub prince_sr_1: PRINCE_SR_1,
    #[doc = "0x2c - Region 2, sub-region enable."]
    pub prince_sr_2: PRINCE_SR_2,
    #[doc = "0x30 - Xtal 32kHz capabank triming."]
    pub xtal_32khz_capabank_trim: XTAL_32KHZ_CAPABANK_TRIM,
    #[doc = "0x34 - Xtal 16MHz capabank triming."]
    pub xtal_16mhz_capabank_trim: XTAL_16MHZ_CAPABANK_TRIM,
    #[doc = "0x38 - This 32-bit register contains the size of the image to remap, in bytes. The 12 LSBs are ignored, so the size granularity is 4KB."]
    pub flash_remap_size: FLASH_REMAP_SIZE,
    #[doc = "0x3c - This 32-bit register contains the offset by which the image is to be remapped. The 12 LSBs are ignored, so the remap granularity is 4KB."]
    pub flash_remap_offset: FLASH_REMAP_OFFSET,
    _reserved16: [u8; 0x10],
    #[doc = "0x50..0x70 - ROTKHindex for Root of Trust Keys Table hash\\[(((7 - index) * 32) + 31):((7 - index) * 32)\\]"]
    pub rotkh: [ROTKH; 8],
    _reserved17: [u8; 0x90],
    #[doc = "0x100..0x1e0 - Customer Defined (Programable through ROM API)"]
    pub customer_defined: [CUSTOMER_DEFINED; 56],
    #[doc = "0x1e0..0x200 - SHA256_DIGESTindex for DIGEST\\[((index * 32) + 31):(index * 32)\\]"]
    pub sha256_digest: [SHA256_DIGEST; 8],
}
#[doc = "BOOT_CFG (rw) register accessor: an alias for `Reg<BOOT_CFG_SPEC>`"]
pub type BOOT_CFG = crate::Reg<boot_cfg::BOOT_CFG_SPEC>;
#[doc = "no description available."]
pub mod boot_cfg;
#[doc = "SPI_FLASH_CFG (rw) register accessor: an alias for `Reg<SPI_FLASH_CFG_SPEC>`"]
pub type SPI_FLASH_CFG = crate::Reg<spi_flash_cfg::SPI_FLASH_CFG_SPEC>;
#[doc = "no description available."]
pub mod spi_flash_cfg;
#[doc = "USB_ID (rw) register accessor: an alias for `Reg<USB_ID_SPEC>`"]
pub type USB_ID = crate::Reg<usb_id::USB_ID_SPEC>;
#[doc = "no description available."]
pub mod usb_id;
#[doc = "SDIO_CFG (rw) register accessor: an alias for `Reg<SDIO_CFG_SPEC>`"]
pub type SDIO_CFG = crate::Reg<sdio_cfg::SDIO_CFG_SPEC>;
#[doc = "no description available."]
pub mod sdio_cfg;
#[doc = "CC_SOCU_PIN (rw) register accessor: an alias for `Reg<CC_SOCU_PIN_SPEC>`"]
pub type CC_SOCU_PIN = crate::Reg<cc_socu_pin::CC_SOCU_PIN_SPEC>;
#[doc = "no description available."]
pub mod cc_socu_pin;
#[doc = "CC_SOCU_DFLT (rw) register accessor: an alias for `Reg<CC_SOCU_DFLT_SPEC>`"]
pub type CC_SOCU_DFLT = crate::Reg<cc_socu_dflt::CC_SOCU_DFLT_SPEC>;
#[doc = "no description available."]
pub mod cc_socu_dflt;
#[doc = "VENDOR_USAGE (rw) register accessor: an alias for `Reg<VENDOR_USAGE_SPEC>`"]
pub type VENDOR_USAGE = crate::Reg<vendor_usage::VENDOR_USAGE_SPEC>;
#[doc = "no description available."]
pub mod vendor_usage;
#[doc = "SECURE_BOOT_CFG (rw) register accessor: an alias for `Reg<SECURE_BOOT_CFG_SPEC>`"]
pub type SECURE_BOOT_CFG = crate::Reg<secure_boot_cfg::SECURE_BOOT_CFG_SPEC>;
#[doc = "Secure boot configuration flags."]
pub mod secure_boot_cfg;
#[doc = "PRINCE_BASE_ADDR (rw) register accessor: an alias for `Reg<PRINCE_BASE_ADDR_SPEC>`"]
pub type PRINCE_BASE_ADDR = crate::Reg<prince_base_addr::PRINCE_BASE_ADDR_SPEC>;
#[doc = "no description available."]
pub mod prince_base_addr;
#[doc = "PRINCE_SR_0 (rw) register accessor: an alias for `Reg<PRINCE_SR_0_SPEC>`"]
pub type PRINCE_SR_0 = crate::Reg<prince_sr_0::PRINCE_SR_0_SPEC>;
#[doc = "Region 0, sub-region enable."]
pub mod prince_sr_0;
#[doc = "PRINCE_SR_1 (rw) register accessor: an alias for `Reg<PRINCE_SR_1_SPEC>`"]
pub type PRINCE_SR_1 = crate::Reg<prince_sr_1::PRINCE_SR_1_SPEC>;
#[doc = "Region 1, sub-region enable."]
pub mod prince_sr_1;
#[doc = "PRINCE_SR_2 (rw) register accessor: an alias for `Reg<PRINCE_SR_2_SPEC>`"]
pub type PRINCE_SR_2 = crate::Reg<prince_sr_2::PRINCE_SR_2_SPEC>;
#[doc = "Region 2, sub-region enable."]
pub mod prince_sr_2;
#[doc = "XTAL_32KHZ_CAPABANK_TRIM (rw) register accessor: an alias for `Reg<XTAL_32KHZ_CAPABANK_TRIM_SPEC>`"]
pub type XTAL_32KHZ_CAPABANK_TRIM =
    crate::Reg<xtal_32khz_capabank_trim::XTAL_32KHZ_CAPABANK_TRIM_SPEC>;
#[doc = "Xtal 32kHz capabank triming."]
pub mod xtal_32khz_capabank_trim;
#[doc = "XTAL_16MHZ_CAPABANK_TRIM (rw) register accessor: an alias for `Reg<XTAL_16MHZ_CAPABANK_TRIM_SPEC>`"]
pub type XTAL_16MHZ_CAPABANK_TRIM =
    crate::Reg<xtal_16mhz_capabank_trim::XTAL_16MHZ_CAPABANK_TRIM_SPEC>;
#[doc = "Xtal 16MHz capabank triming."]
pub mod xtal_16mhz_capabank_trim;
#[doc = "FLASH_REMAP_SIZE (rw) register accessor: an alias for `Reg<FLASH_REMAP_SIZE_SPEC>`"]
pub type FLASH_REMAP_SIZE = crate::Reg<flash_remap_size::FLASH_REMAP_SIZE_SPEC>;
#[doc = "This 32-bit register contains the size of the image to remap, in bytes. The 12 LSBs are ignored, so the size granularity is 4KB."]
pub mod flash_remap_size;
#[doc = "FLASH_REMAP_OFFSET (rw) register accessor: an alias for `Reg<FLASH_REMAP_OFFSET_SPEC>`"]
pub type FLASH_REMAP_OFFSET = crate::Reg<flash_remap_offset::FLASH_REMAP_OFFSET_SPEC>;
#[doc = "This 32-bit register contains the offset by which the image is to be remapped. The 12 LSBs are ignored, so the remap granularity is 4KB."]
pub mod flash_remap_offset;
#[doc = "ROTKH (rw) register accessor: an alias for `Reg<ROTKH_SPEC>`"]
pub type ROTKH = crate::Reg<rotkh::ROTKH_SPEC>;
#[doc = "ROTKHindex for Root of Trust Keys Table hash\\[(((7 - index) * 32) + 31):((7 - index) * 32)\\]"]
pub mod rotkh;
#[doc = "CUSTOMER_DEFINED (rw) register accessor: an alias for `Reg<CUSTOMER_DEFINED_SPEC>`"]
pub type CUSTOMER_DEFINED = crate::Reg<customer_defined::CUSTOMER_DEFINED_SPEC>;
#[doc = "Customer Defined (Programable through ROM API)"]
pub mod customer_defined;
#[doc = "SHA256_DIGEST (rw) register accessor: an alias for `Reg<SHA256_DIGEST_SPEC>`"]
pub type SHA256_DIGEST = crate::Reg<sha256_digest::SHA256_DIGEST_SPEC>;
#[doc = "SHA256_DIGESTindex for DIGEST\\[((index * 32) + 31):(index * 32)\\]"]
pub mod sha256_digest;
