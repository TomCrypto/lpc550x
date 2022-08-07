#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved_0_gpo0_gpo0: [u8; 0x04],
    _reserved_1_gpo0_gpo0: [u8; 0x04],
    _reserved_2_gpo0_gpo0: [u8; 0x04],
    _reserved_3_gpo0_gpo0: [u8; 0x04],
    _reserved_4_gpo1_gpo1: [u8; 0x04],
    _reserved_5_gpo1_gpo1: [u8; 0x04],
    _reserved_6_gpo1_gpo1: [u8; 0x04],
    _reserved_7_gpo1_gpo1: [u8; 0x04],
    _reserved_8_gpo2_gpo2: [u8; 0x04],
    _reserved_9_gpo2_gpo2: [u8; 0x04],
    _reserved_10_gpo2_gpo2: [u8; 0x04],
    _reserved_11_gpo2_gpo2: [u8; 0x04],
    _reserved_12_gpo3_gpo3: [u8; 0x04],
    _reserved_13_gpo3_gpo3: [u8; 0x04],
    _reserved_14_gpo3_gpo3: [u8; 0x04],
    _reserved_15_gpo3_gpo3: [u8; 0x04],
    _reserved_16_gpo_checksum_gpo_checksum: [u8; 0x04],
    _reserved_17_gpo_checksum_gpo_checksum: [u8; 0x04],
    _reserved_18_gpo_checksum_gpo_checksum: [u8; 0x04],
    _reserved_19_gpo_checksum_gpo_checksum: [u8; 0x04],
    _reserved_20_final_test_batch_id_final_test_batch_id: [u8; 0x04],
    _reserved_21_final_test_batch_id_final_test_batch_id: [u8; 0x04],
    _reserved_22_final_test_batch_id_final_test_batch_id: [u8; 0x04],
    _reserved_23_final_test_batch_id_final_test_batch_id: [u8; 0x04],
    #[doc = "0x60 - no description available"]
    pub device_type: DEVICE_TYPE,
    #[doc = "0x64 - no description available"]
    pub final_test_program_version: FINAL_TEST_PROGRAM_VERSION,
    #[doc = "0x68 - no description available"]
    pub final_test_date: FINAL_TEST_DATE,
    #[doc = "0x6c - no description available"]
    pub final_test_time: FINAL_TEST_TIME,
    _reserved_28_uuid_uuid: [u8; 0x04],
    _reserved_29_uuid_uuid: [u8; 0x04],
    _reserved_30_uuid_uuid: [u8; 0x04],
    _reserved_31_uuid_uuid: [u8; 0x04],
    #[doc = "0x80 - no description available"]
    pub wafer_test1_program_version: WAFER_TEST1_PROGRAM_VERSION,
    #[doc = "0x84 - no description available"]
    pub wafer_test1_date: WAFER_TEST1_DATE,
    #[doc = "0x88 - no description available"]
    pub wafer_test1_time: WAFER_TEST1_TIME,
    _reserved35: [u8; 0x04],
    #[doc = "0x90 - no description available"]
    pub wafer_test2_program_version: WAFER_TEST2_PROGRAM_VERSION,
    #[doc = "0x94 - no description available"]
    pub wafer_test2_date: WAFER_TEST2_DATE,
    #[doc = "0x98 - no description available"]
    pub wafer_test2_time: WAFER_TEST2_TIME,
    #[doc = "0x9c - no description available"]
    pub usbcfg: USBCFG,
    #[doc = "0xa0 - no description available"]
    pub periphencfg: PERIPHENCFG,
    #[doc = "0xa4 - no description available"]
    pub ramsizecfg: RAMSIZECFG,
    #[doc = "0xa8 - no description available"]
    pub flashsizecfg: FLASHSIZECFG,
    _reserved42: [u8; 0x04],
    #[doc = "0xb0 - no description available"]
    pub ringo_0: RINGO_0,
    #[doc = "0xb4 - no description available"]
    pub ringo_1: RINGO_1,
    #[doc = "0xb8 - no description available"]
    pub ringo_2: RINGO_2,
    _reserved45: [u8; 0x04],
    #[doc = "0xc0 - no description available"]
    pub fro_192mhz: FRO_192MHZ,
    _reserved46: [u8; 0x04],
    #[doc = "0xc8 - no description available"]
    pub xo_32mhz: XO_32MHZ,
    #[doc = "0xcc - no description available"]
    pub xo_32khz: XO_32KHZ,
    #[doc = "0xd0 - no description available"]
    pub fro_1mhz: FRO_1MHZ,
    _reserved49: [u8; 0x04],
    _reserved_49_dcdc_power_profile_high_dcdc_power_profile_high: [u8; 0x04],
    _reserved_50_dcdc_power_profile_high_dcdc_power_profile_high: [u8; 0x04],
    _reserved_51_dcdc_power_profile_low_dcdc_power_profile_low: [u8; 0x04],
    _reserved_52_dcdc_power_profile_low_dcdc_power_profile_low: [u8; 0x04],
    _reserved_53_dcdc_power_profile_medium_dcdc_power_profile_medium: [u8; 0x04],
    _reserved_54_dcdc_power_profile_medium_dcdc_power_profile_medium: [u8; 0x04],
    #[doc = "0xf0 - no description available"]
    pub bod: BOD,
    #[doc = "0xf4 - no description available"]
    pub ldo_ao: LDO_AO,
    #[doc = "0xf8 - no description available"]
    pub sdio_delay: SDIO_DELAY,
    _reserved58: [u8; 0x04],
    _reserved_58_aux_bias_curve_ambient_aux_bias_curve_ambient: [u8; 0x04],
    _reserved_59_aux_bias_curve_ambient_aux_bias_curve_ambient: [u8; 0x04],
    _reserved_60_aux_bias_curve_ambient_aux_bias_curve_ambient: [u8; 0x04],
    _reserved_61_aux_bias_curve_ambient_aux_bias_curve_ambient: [u8; 0x04],
    _reserved_62_aux_bias_curve_temp_aux_bias_curve_temp: [u8; 0x04],
    _reserved_63_aux_bias_curve_temp_aux_bias_curve_temp: [u8; 0x04],
    _reserved_64_aux_bias_curve_temp_aux_bias_curve_temp: [u8; 0x04],
    _reserved_65_aux_bias_curve_temp_aux_bias_curve_temp: [u8; 0x04],
    #[doc = "0x120 - no description available"]
    pub temp_sens_vbe1vbe8_ref_1: TEMP_SENS_VBE1VBE8_REF_1,
    #[doc = "0x124 - no description available"]
    pub temp_sens_vbe1vbe8_ref_2: TEMP_SENS_VBE1VBE8_REF_2,
    #[doc = "0x128 - no description available"]
    pub temp_sens_slope: TEMP_SENS_SLOPE,
    #[doc = "0x12c - no description available"]
    pub temp_sens_offset: TEMP_SENS_OFFSET,
    _reserved_70_pvt_monitor_0_pvt_monitor_0: [u8; 0x04],
    _reserved_71_pvt_monitor_0_pvt_monitor_0: [u8; 0x04],
    _reserved_72_pvt_monitor_0_pvt_monitor_0: [u8; 0x04],
    _reserved73: [u8; 0x04],
    _reserved_73_pvt_monitor_1_pvt_monitor_1: [u8; 0x04],
    _reserved_74_pvt_monitor_1_pvt_monitor_1: [u8; 0x04],
    _reserved_75_pvt_monitor_1_pvt_monitor_1: [u8; 0x04],
    #[doc = "0x14c..0x180 - no description available"]
    pub nxp_device_private_key: [NXP_DEVICE_PRIVATE_KEY; 13],
    #[doc = "0x180..0x190 - NXP Device Certificate (ECDSA_sign - r\\[255:128\\])"]
    pub nxp_device_certificate_0: [NXP_DEVICE_CERTIFICATE_0; 4],
    #[doc = "0x190..0x1a0 - NXP Device Certificate (ECDSA_sign - r\\[127:0\\])"]
    pub nxp_device_certificate_1: [NXP_DEVICE_CERTIFICATE_1; 4],
    #[doc = "0x1a0..0x1b0 - NXP Device Certificate (ECDSA_sign - s\\[255:128\\])"]
    pub nxp_device_certificate_2: [NXP_DEVICE_CERTIFICATE_2; 4],
    #[doc = "0x1b0..0x1c0 - NXP Device Certificate (ECDSA_sign - s\\[127:0\\])"]
    pub nxp_device_certificate_3: [NXP_DEVICE_CERTIFICATE_3; 4],
    #[doc = "0x1c0..0x1e0 - SHA-256 DIGEST (9EC00 - 9FDBC) ROM Patch Area + NXP Area (IMPORTANT NOTE: Pages used for Repair (N-8 to N-3) are excluded from the computation) SHA256_DIGESTindex for DIGEST\\[((index * 32) + 31):(index * 32)\\]"]
    pub sha256_digest: [SHA256_DIGEST; 8],
    _reserved_82_ecid_backup_ecid_backup: [u8; 0x04],
    _reserved_83_ecid_backup_ecid_backup: [u8; 0x04],
    _reserved_84_ecid_backup_ecid_backup: [u8; 0x04],
    _reserved_85_ecid_backup_ecid_backup: [u8; 0x04],
    #[doc = "0x1f0..0x200 - Checksum of the whole page"]
    pub checksum: [CHECKSUM; 4],
    _reserved87: [u8; 0x0aac],
    #[doc = "0xcac - no description available"]
    pub dis_rom_hiding: DIS_ROM_HIDING,
    _reserved88: [u8; 0x0c],
    #[doc = "0xcbc - no description available"]
    pub puf_sram: PUF_SRAM,
}
impl RegisterBlock {
    #[doc = "0x00 - GPO0 array description"]
    #[inline(always)]
    pub fn gpo0_gpo0_array0(&self) -> &GPO0_GPO0_ARRAY0 {
        unsafe { &*(((self as *const Self) as *const u8).add(0usize) as *const GPO0_GPO0_ARRAY0) }
    }
    #[doc = "0x00 - GPO0 register 0 description"]
    #[inline(always)]
    pub fn gpo0_gpo0_0(&self) -> &GPO0_GPO0_0 {
        unsafe { &*(((self as *const Self) as *const u8).add(0usize) as *const GPO0_GPO0_0) }
    }
    #[doc = "0x04 - GPO0 array description"]
    #[inline(always)]
    pub fn gpo0_gpo0_array1(&self) -> &GPO0_GPO0_ARRAY1 {
        unsafe { &*(((self as *const Self) as *const u8).add(4usize) as *const GPO0_GPO0_ARRAY1) }
    }
    #[doc = "0x04 - GPO0 register 1 description"]
    #[inline(always)]
    pub fn gpo0_gpo0_1(&self) -> &GPO0_GPO0_1 {
        unsafe { &*(((self as *const Self) as *const u8).add(4usize) as *const GPO0_GPO0_1) }
    }
    #[doc = "0x08 - GPO0 array description"]
    #[inline(always)]
    pub fn gpo0_gpo0_array2(&self) -> &GPO0_GPO0_ARRAY2 {
        unsafe { &*(((self as *const Self) as *const u8).add(8usize) as *const GPO0_GPO0_ARRAY2) }
    }
    #[doc = "0x08 - GPO0 register 2 description"]
    #[inline(always)]
    pub fn gpo0_gpo0_2(&self) -> &GPO0_GPO0_2 {
        unsafe { &*(((self as *const Self) as *const u8).add(8usize) as *const GPO0_GPO0_2) }
    }
    #[doc = "0x0c - GPO0 array description"]
    #[inline(always)]
    pub fn gpo0_gpo0_array3(&self) -> &GPO0_GPO0_ARRAY3 {
        unsafe { &*(((self as *const Self) as *const u8).add(12usize) as *const GPO0_GPO0_ARRAY3) }
    }
    #[doc = "0x0c - GPO0 register 3 description"]
    #[inline(always)]
    pub fn gpo0_gpo0_3(&self) -> &GPO0_GPO0_3 {
        unsafe { &*(((self as *const Self) as *const u8).add(12usize) as *const GPO0_GPO0_3) }
    }
    #[doc = "0x10 - GPO1 array description"]
    #[inline(always)]
    pub fn gpo1_gpo1_array0(&self) -> &GPO1_GPO1_ARRAY0 {
        unsafe { &*(((self as *const Self) as *const u8).add(16usize) as *const GPO1_GPO1_ARRAY0) }
    }
    #[doc = "0x10 - GPO1 register 0 description"]
    #[inline(always)]
    pub fn gpo1_gpo1_0(&self) -> &GPO1_GPO1_0 {
        unsafe { &*(((self as *const Self) as *const u8).add(16usize) as *const GPO1_GPO1_0) }
    }
    #[doc = "0x14 - GPO1 array description"]
    #[inline(always)]
    pub fn gpo1_gpo1_array1(&self) -> &GPO1_GPO1_ARRAY1 {
        unsafe { &*(((self as *const Self) as *const u8).add(20usize) as *const GPO1_GPO1_ARRAY1) }
    }
    #[doc = "0x14 - GPO1 register 1 description"]
    #[inline(always)]
    pub fn gpo1_gpo1_1(&self) -> &GPO1_GPO1_1 {
        unsafe { &*(((self as *const Self) as *const u8).add(20usize) as *const GPO1_GPO1_1) }
    }
    #[doc = "0x18 - GPO1 array description"]
    #[inline(always)]
    pub fn gpo1_gpo1_array2(&self) -> &GPO1_GPO1_ARRAY2 {
        unsafe { &*(((self as *const Self) as *const u8).add(24usize) as *const GPO1_GPO1_ARRAY2) }
    }
    #[doc = "0x18 - GPO1 register 2 description"]
    #[inline(always)]
    pub fn gpo1_gpo1_2(&self) -> &GPO1_GPO1_2 {
        unsafe { &*(((self as *const Self) as *const u8).add(24usize) as *const GPO1_GPO1_2) }
    }
    #[doc = "0x1c - GPO1 array description"]
    #[inline(always)]
    pub fn gpo1_gpo1_array3(&self) -> &GPO1_GPO1_ARRAY3 {
        unsafe { &*(((self as *const Self) as *const u8).add(28usize) as *const GPO1_GPO1_ARRAY3) }
    }
    #[doc = "0x1c - GPO1 register 3 description"]
    #[inline(always)]
    pub fn gpo1_gpo1_3(&self) -> &GPO1_GPO1_3 {
        unsafe { &*(((self as *const Self) as *const u8).add(28usize) as *const GPO1_GPO1_3) }
    }
    #[doc = "0x20 - GPO2 array description"]
    #[inline(always)]
    pub fn gpo2_gpo2_array0(&self) -> &GPO2_GPO2_ARRAY0 {
        unsafe { &*(((self as *const Self) as *const u8).add(32usize) as *const GPO2_GPO2_ARRAY0) }
    }
    #[doc = "0x20 - GPO2 register 0 description"]
    #[inline(always)]
    pub fn gpo2_gpo2_0(&self) -> &GPO2_GPO2_0 {
        unsafe { &*(((self as *const Self) as *const u8).add(32usize) as *const GPO2_GPO2_0) }
    }
    #[doc = "0x24 - GPO2 array description"]
    #[inline(always)]
    pub fn gpo2_gpo2_array1(&self) -> &GPO2_GPO2_ARRAY1 {
        unsafe { &*(((self as *const Self) as *const u8).add(36usize) as *const GPO2_GPO2_ARRAY1) }
    }
    #[doc = "0x24 - GPO2 register 1 description"]
    #[inline(always)]
    pub fn gpo2_gpo2_1(&self) -> &GPO2_GPO2_1 {
        unsafe { &*(((self as *const Self) as *const u8).add(36usize) as *const GPO2_GPO2_1) }
    }
    #[doc = "0x28 - GPO2 array description"]
    #[inline(always)]
    pub fn gpo2_gpo2_array2(&self) -> &GPO2_GPO2_ARRAY2 {
        unsafe { &*(((self as *const Self) as *const u8).add(40usize) as *const GPO2_GPO2_ARRAY2) }
    }
    #[doc = "0x28 - GPO2 register 2 description"]
    #[inline(always)]
    pub fn gpo2_gpo2_2(&self) -> &GPO2_GPO2_2 {
        unsafe { &*(((self as *const Self) as *const u8).add(40usize) as *const GPO2_GPO2_2) }
    }
    #[doc = "0x2c - GPO2 array description"]
    #[inline(always)]
    pub fn gpo2_gpo2_array3(&self) -> &GPO2_GPO2_ARRAY3 {
        unsafe { &*(((self as *const Self) as *const u8).add(44usize) as *const GPO2_GPO2_ARRAY3) }
    }
    #[doc = "0x2c - GPO2 register 3 description"]
    #[inline(always)]
    pub fn gpo2_gpo2_3(&self) -> &GPO2_GPO2_3 {
        unsafe { &*(((self as *const Self) as *const u8).add(44usize) as *const GPO2_GPO2_3) }
    }
    #[doc = "0x30 - GPO3 array description"]
    #[inline(always)]
    pub fn gpo3_gpo3_array0(&self) -> &GPO3_GPO3_ARRAY0 {
        unsafe { &*(((self as *const Self) as *const u8).add(48usize) as *const GPO3_GPO3_ARRAY0) }
    }
    #[doc = "0x30 - GPO3 register 0 description"]
    #[inline(always)]
    pub fn gpo3_gpo3_0(&self) -> &GPO3_GPO3_0 {
        unsafe { &*(((self as *const Self) as *const u8).add(48usize) as *const GPO3_GPO3_0) }
    }
    #[doc = "0x34 - GPO3 array description"]
    #[inline(always)]
    pub fn gpo3_gpo3_array1(&self) -> &GPO3_GPO3_ARRAY1 {
        unsafe { &*(((self as *const Self) as *const u8).add(52usize) as *const GPO3_GPO3_ARRAY1) }
    }
    #[doc = "0x34 - GPO3 register 1 description"]
    #[inline(always)]
    pub fn gpo3_gpo3_1(&self) -> &GPO3_GPO3_1 {
        unsafe { &*(((self as *const Self) as *const u8).add(52usize) as *const GPO3_GPO3_1) }
    }
    #[doc = "0x38 - GPO3 array description"]
    #[inline(always)]
    pub fn gpo3_gpo3_array2(&self) -> &GPO3_GPO3_ARRAY2 {
        unsafe { &*(((self as *const Self) as *const u8).add(56usize) as *const GPO3_GPO3_ARRAY2) }
    }
    #[doc = "0x38 - GPO3 register 2 description"]
    #[inline(always)]
    pub fn gpo3_gpo3_2(&self) -> &GPO3_GPO3_2 {
        unsafe { &*(((self as *const Self) as *const u8).add(56usize) as *const GPO3_GPO3_2) }
    }
    #[doc = "0x3c - GPO3 array description"]
    #[inline(always)]
    pub fn gpo3_gpo3_array3(&self) -> &GPO3_GPO3_ARRAY3 {
        unsafe { &*(((self as *const Self) as *const u8).add(60usize) as *const GPO3_GPO3_ARRAY3) }
    }
    #[doc = "0x3c - GPO3 register 3 description"]
    #[inline(always)]
    pub fn gpo3_gpo3_3(&self) -> &GPO3_GPO3_3 {
        unsafe { &*(((self as *const Self) as *const u8).add(60usize) as *const GPO3_GPO3_3) }
    }
    #[doc = "0x40 - checksum of the GPO data in words \\[3:0\\]"]
    #[inline(always)]
    pub fn gpo_checksum_gpo_checksum_array0(&self) -> &GPO_CHECKSUM_GPO_CHECKSUM_ARRAY0 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(64usize)
                as *const GPO_CHECKSUM_GPO_CHECKSUM_ARRAY0)
        }
    }
    #[doc = "0x40 - checksum of the GPO data in words 0"]
    #[inline(always)]
    pub fn gpo_checksum_gpo_checksum_0(&self) -> &GPO_CHECKSUM_GPO_CHECKSUM_0 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(64usize)
                as *const GPO_CHECKSUM_GPO_CHECKSUM_0)
        }
    }
    #[doc = "0x44 - checksum of the GPO data in words \\[3:0\\]"]
    #[inline(always)]
    pub fn gpo_checksum_gpo_checksum_array1(&self) -> &GPO_CHECKSUM_GPO_CHECKSUM_ARRAY1 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(68usize)
                as *const GPO_CHECKSUM_GPO_CHECKSUM_ARRAY1)
        }
    }
    #[doc = "0x44 - checksum of the GPO data in words 1"]
    #[inline(always)]
    pub fn gpo_checksum_gpo_checksum_1(&self) -> &GPO_CHECKSUM_GPO_CHECKSUM_1 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(68usize)
                as *const GPO_CHECKSUM_GPO_CHECKSUM_1)
        }
    }
    #[doc = "0x48 - checksum of the GPO data in words \\[3:0\\]"]
    #[inline(always)]
    pub fn gpo_checksum_gpo_checksum_array2(&self) -> &GPO_CHECKSUM_GPO_CHECKSUM_ARRAY2 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(72usize)
                as *const GPO_CHECKSUM_GPO_CHECKSUM_ARRAY2)
        }
    }
    #[doc = "0x48 - checksum of the GPO data in words 2"]
    #[inline(always)]
    pub fn gpo_checksum_gpo_checksum_2(&self) -> &GPO_CHECKSUM_GPO_CHECKSUM_2 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(72usize)
                as *const GPO_CHECKSUM_GPO_CHECKSUM_2)
        }
    }
    #[doc = "0x4c - checksum of the GPO data in words \\[3:0\\]"]
    #[inline(always)]
    pub fn gpo_checksum_gpo_checksum_array3(&self) -> &GPO_CHECKSUM_GPO_CHECKSUM_ARRAY3 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(76usize)
                as *const GPO_CHECKSUM_GPO_CHECKSUM_ARRAY3)
        }
    }
    #[doc = "0x4c - checksum of the GPO data in words 3"]
    #[inline(always)]
    pub fn gpo_checksum_gpo_checksum_3(&self) -> &GPO_CHECKSUM_GPO_CHECKSUM_3 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(76usize)
                as *const GPO_CHECKSUM_GPO_CHECKSUM_3)
        }
    }
    #[doc = "0x50 - no description available"]
    #[inline(always)]
    pub fn final_test_batch_id_final_test_batch_id_array0(
        &self,
    ) -> &FINAL_TEST_BATCH_ID_FINAL_TEST_BATCH_ID_ARRAY0 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(80usize)
                as *const FINAL_TEST_BATCH_ID_FINAL_TEST_BATCH_ID_ARRAY0)
        }
    }
    #[doc = "0x50 - no description available"]
    #[inline(always)]
    pub fn final_test_batch_id_final_test_batch_id_0(
        &self,
    ) -> &FINAL_TEST_BATCH_ID_FINAL_TEST_BATCH_ID_0 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(80usize)
                as *const FINAL_TEST_BATCH_ID_FINAL_TEST_BATCH_ID_0)
        }
    }
    #[doc = "0x54 - no description available"]
    #[inline(always)]
    pub fn final_test_batch_id_final_test_batch_id_array1(
        &self,
    ) -> &FINAL_TEST_BATCH_ID_FINAL_TEST_BATCH_ID_ARRAY1 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(84usize)
                as *const FINAL_TEST_BATCH_ID_FINAL_TEST_BATCH_ID_ARRAY1)
        }
    }
    #[doc = "0x54 - no description available"]
    #[inline(always)]
    pub fn final_test_batch_id_final_test_batch_id_1(
        &self,
    ) -> &FINAL_TEST_BATCH_ID_FINAL_TEST_BATCH_ID_1 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(84usize)
                as *const FINAL_TEST_BATCH_ID_FINAL_TEST_BATCH_ID_1)
        }
    }
    #[doc = "0x58 - no description available"]
    #[inline(always)]
    pub fn final_test_batch_id_final_test_batch_id_array2(
        &self,
    ) -> &FINAL_TEST_BATCH_ID_FINAL_TEST_BATCH_ID_ARRAY2 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(88usize)
                as *const FINAL_TEST_BATCH_ID_FINAL_TEST_BATCH_ID_ARRAY2)
        }
    }
    #[doc = "0x58 - no description available"]
    #[inline(always)]
    pub fn final_test_batch_id_final_test_batch_id_2(
        &self,
    ) -> &FINAL_TEST_BATCH_ID_FINAL_TEST_BATCH_ID_2 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(88usize)
                as *const FINAL_TEST_BATCH_ID_FINAL_TEST_BATCH_ID_2)
        }
    }
    #[doc = "0x5c - no description available"]
    #[inline(always)]
    pub fn final_test_batch_id_final_test_batch_id_array3(
        &self,
    ) -> &FINAL_TEST_BATCH_ID_FINAL_TEST_BATCH_ID_ARRAY3 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(92usize)
                as *const FINAL_TEST_BATCH_ID_FINAL_TEST_BATCH_ID_ARRAY3)
        }
    }
    #[doc = "0x5c - no description available"]
    #[inline(always)]
    pub fn final_test_batch_id_final_test_batch_id_3(
        &self,
    ) -> &FINAL_TEST_BATCH_ID_FINAL_TEST_BATCH_ID_3 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(92usize)
                as *const FINAL_TEST_BATCH_ID_FINAL_TEST_BATCH_ID_3)
        }
    }
    #[doc = "0x70 - no description available"]
    #[inline(always)]
    pub fn uuid_uuid_array0(&self) -> &UUID_UUID_ARRAY0 {
        unsafe { &*(((self as *const Self) as *const u8).add(112usize) as *const UUID_UUID_ARRAY0) }
    }
    #[doc = "0x70 - no description available"]
    #[inline(always)]
    pub fn uuid_uuid_0(&self) -> &UUID_UUID_0 {
        unsafe { &*(((self as *const Self) as *const u8).add(112usize) as *const UUID_UUID_0) }
    }
    #[doc = "0x74 - no description available"]
    #[inline(always)]
    pub fn uuid_uuid_array1(&self) -> &UUID_UUID_ARRAY1 {
        unsafe { &*(((self as *const Self) as *const u8).add(116usize) as *const UUID_UUID_ARRAY1) }
    }
    #[doc = "0x74 - no description available"]
    #[inline(always)]
    pub fn uuid_uuid_1(&self) -> &UUID_UUID_1 {
        unsafe { &*(((self as *const Self) as *const u8).add(116usize) as *const UUID_UUID_1) }
    }
    #[doc = "0x78 - no description available"]
    #[inline(always)]
    pub fn uuid_uuid_array2(&self) -> &UUID_UUID_ARRAY2 {
        unsafe { &*(((self as *const Self) as *const u8).add(120usize) as *const UUID_UUID_ARRAY2) }
    }
    #[doc = "0x78 - no description available"]
    #[inline(always)]
    pub fn uuid_uuid_2(&self) -> &UUID_UUID_2 {
        unsafe { &*(((self as *const Self) as *const u8).add(120usize) as *const UUID_UUID_2) }
    }
    #[doc = "0x7c - no description available"]
    #[inline(always)]
    pub fn uuid_uuid_array3(&self) -> &UUID_UUID_ARRAY3 {
        unsafe { &*(((self as *const Self) as *const u8).add(124usize) as *const UUID_UUID_ARRAY3) }
    }
    #[doc = "0x7c - no description available"]
    #[inline(always)]
    pub fn uuid_uuid_3(&self) -> &UUID_UUID_3 {
        unsafe { &*(((self as *const Self) as *const u8).add(124usize) as *const UUID_UUID_3) }
    }
    #[doc = "0xd8 - no description available"]
    #[inline(always)]
    pub fn dcdc_power_profile_high_dcdc_power_profile_high_array0(
        &self,
    ) -> &DCDC_POWER_PROFILE_HIGH_DCDC_POWER_PROFILE_HIGH_ARRAY0 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(216usize)
                as *const DCDC_POWER_PROFILE_HIGH_DCDC_POWER_PROFILE_HIGH_ARRAY0)
        }
    }
    #[doc = "0xd8 - no description available"]
    #[inline(always)]
    pub fn dcdc_power_profile_high_dcdc_power_profile_high_0(
        &self,
    ) -> &DCDC_POWER_PROFILE_HIGH_DCDC_POWER_PROFILE_HIGH_0 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(216usize)
                as *const DCDC_POWER_PROFILE_HIGH_DCDC_POWER_PROFILE_HIGH_0)
        }
    }
    #[doc = "0xdc - no description available"]
    #[inline(always)]
    pub fn dcdc_power_profile_high_dcdc_power_profile_high_array1(
        &self,
    ) -> &DCDC_POWER_PROFILE_HIGH_DCDC_POWER_PROFILE_HIGH_ARRAY1 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(220usize)
                as *const DCDC_POWER_PROFILE_HIGH_DCDC_POWER_PROFILE_HIGH_ARRAY1)
        }
    }
    #[doc = "0xdc - no description available"]
    #[inline(always)]
    pub fn dcdc_power_profile_high_dcdc_power_profile_high_1(
        &self,
    ) -> &DCDC_POWER_PROFILE_HIGH_DCDC_POWER_PROFILE_HIGH_1 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(220usize)
                as *const DCDC_POWER_PROFILE_HIGH_DCDC_POWER_PROFILE_HIGH_1)
        }
    }
    #[doc = "0xe0 - no description available"]
    #[inline(always)]
    pub fn dcdc_power_profile_low_dcdc_power_profile_low_array0(
        &self,
    ) -> &DCDC_POWER_PROFILE_LOW_DCDC_POWER_PROFILE_LOW_ARRAY0 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(224usize)
                as *const DCDC_POWER_PROFILE_LOW_DCDC_POWER_PROFILE_LOW_ARRAY0)
        }
    }
    #[doc = "0xe0 - no description available"]
    #[inline(always)]
    pub fn dcdc_power_profile_low_dcdc_power_profile_low_0(
        &self,
    ) -> &DCDC_POWER_PROFILE_LOW_DCDC_POWER_PROFILE_LOW_0 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(224usize)
                as *const DCDC_POWER_PROFILE_LOW_DCDC_POWER_PROFILE_LOW_0)
        }
    }
    #[doc = "0xe4 - no description available"]
    #[inline(always)]
    pub fn dcdc_power_profile_low_dcdc_power_profile_low_array1(
        &self,
    ) -> &DCDC_POWER_PROFILE_LOW_DCDC_POWER_PROFILE_LOW_ARRAY1 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(228usize)
                as *const DCDC_POWER_PROFILE_LOW_DCDC_POWER_PROFILE_LOW_ARRAY1)
        }
    }
    #[doc = "0xe4 - no description available"]
    #[inline(always)]
    pub fn dcdc_power_profile_low_dcdc_power_profile_low_1(
        &self,
    ) -> &DCDC_POWER_PROFILE_LOW_DCDC_POWER_PROFILE_LOW_1 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(228usize)
                as *const DCDC_POWER_PROFILE_LOW_DCDC_POWER_PROFILE_LOW_1)
        }
    }
    #[doc = "0xe8 - no description available"]
    #[inline(always)]
    pub fn dcdc_power_profile_medium_dcdc_power_profile_medium_array0(
        &self,
    ) -> &DCDC_POWER_PROFILE_MEDIUM_DCDC_POWER_PROFILE_MEDIUM_ARRAY0 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(232usize)
                as *const DCDC_POWER_PROFILE_MEDIUM_DCDC_POWER_PROFILE_MEDIUM_ARRAY0)
        }
    }
    #[doc = "0xe8 - no description available"]
    #[inline(always)]
    pub fn dcdc_power_profile_medium_dcdc_power_profile_medium_0(
        &self,
    ) -> &DCDC_POWER_PROFILE_MEDIUM_DCDC_POWER_PROFILE_MEDIUM_0 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(232usize)
                as *const DCDC_POWER_PROFILE_MEDIUM_DCDC_POWER_PROFILE_MEDIUM_0)
        }
    }
    #[doc = "0xec - no description available"]
    #[inline(always)]
    pub fn dcdc_power_profile_medium_dcdc_power_profile_medium_array1(
        &self,
    ) -> &DCDC_POWER_PROFILE_MEDIUM_DCDC_POWER_PROFILE_MEDIUM_ARRAY1 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(236usize)
                as *const DCDC_POWER_PROFILE_MEDIUM_DCDC_POWER_PROFILE_MEDIUM_ARRAY1)
        }
    }
    #[doc = "0xec - no description available"]
    #[inline(always)]
    pub fn dcdc_power_profile_medium_dcdc_power_profile_medium_1(
        &self,
    ) -> &DCDC_POWER_PROFILE_MEDIUM_DCDC_POWER_PROFILE_MEDIUM_1 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(236usize)
                as *const DCDC_POWER_PROFILE_MEDIUM_DCDC_POWER_PROFILE_MEDIUM_1)
        }
    }
    #[doc = "0x100 - Aux Bias Curve Ambient (30degC)"]
    #[inline(always)]
    pub fn aux_bias_curve_ambient_aux_bias_curve_ambient_array0(
        &self,
    ) -> &AUX_BIAS_CURVE_AMBIENT_AUX_BIAS_CURVE_AMBIENT_ARRAY0 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(256usize)
                as *const AUX_BIAS_CURVE_AMBIENT_AUX_BIAS_CURVE_AMBIENT_ARRAY0)
        }
    }
    #[doc = "0x100 - no description available"]
    #[inline(always)]
    pub fn aux_bias_curve_ambient_aux_bias_curve_ambient_0(
        &self,
    ) -> &AUX_BIAS_CURVE_AMBIENT_AUX_BIAS_CURVE_AMBIENT_0 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(256usize)
                as *const AUX_BIAS_CURVE_AMBIENT_AUX_BIAS_CURVE_AMBIENT_0)
        }
    }
    #[doc = "0x104 - Aux Bias Curve Ambient (30degC)"]
    #[inline(always)]
    pub fn aux_bias_curve_ambient_aux_bias_curve_ambient_array1(
        &self,
    ) -> &AUX_BIAS_CURVE_AMBIENT_AUX_BIAS_CURVE_AMBIENT_ARRAY1 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(260usize)
                as *const AUX_BIAS_CURVE_AMBIENT_AUX_BIAS_CURVE_AMBIENT_ARRAY1)
        }
    }
    #[doc = "0x104 - no description available"]
    #[inline(always)]
    pub fn aux_bias_curve_ambient_aux_bias_curve_ambient_1(
        &self,
    ) -> &AUX_BIAS_CURVE_AMBIENT_AUX_BIAS_CURVE_AMBIENT_1 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(260usize)
                as *const AUX_BIAS_CURVE_AMBIENT_AUX_BIAS_CURVE_AMBIENT_1)
        }
    }
    #[doc = "0x108 - Aux Bias Curve Ambient (30degC)"]
    #[inline(always)]
    pub fn aux_bias_curve_ambient_aux_bias_curve_ambient_array2(
        &self,
    ) -> &AUX_BIAS_CURVE_AMBIENT_AUX_BIAS_CURVE_AMBIENT_ARRAY2 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(264usize)
                as *const AUX_BIAS_CURVE_AMBIENT_AUX_BIAS_CURVE_AMBIENT_ARRAY2)
        }
    }
    #[doc = "0x108 - no description available"]
    #[inline(always)]
    pub fn aux_bias_curve_ambient_aux_bias_curve_ambient_2(
        &self,
    ) -> &AUX_BIAS_CURVE_AMBIENT_AUX_BIAS_CURVE_AMBIENT_2 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(264usize)
                as *const AUX_BIAS_CURVE_AMBIENT_AUX_BIAS_CURVE_AMBIENT_2)
        }
    }
    #[doc = "0x10c - Aux Bias Curve Ambient (30degC)"]
    #[inline(always)]
    pub fn aux_bias_curve_ambient_aux_bias_curve_ambient_array3(
        &self,
    ) -> &AUX_BIAS_CURVE_AMBIENT_AUX_BIAS_CURVE_AMBIENT_ARRAY3 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(268usize)
                as *const AUX_BIAS_CURVE_AMBIENT_AUX_BIAS_CURVE_AMBIENT_ARRAY3)
        }
    }
    #[doc = "0x10c - no description available"]
    #[inline(always)]
    pub fn aux_bias_curve_ambient_aux_bias_curve_ambient_3(
        &self,
    ) -> &AUX_BIAS_CURVE_AMBIENT_AUX_BIAS_CURVE_AMBIENT_3 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(268usize)
                as *const AUX_BIAS_CURVE_AMBIENT_AUX_BIAS_CURVE_AMBIENT_3)
        }
    }
    #[doc = "0x110 - Aux Bias Curve TEMP (105degC)"]
    #[inline(always)]
    pub fn aux_bias_curve_temp_aux_bias_curve_temp_array0(
        &self,
    ) -> &AUX_BIAS_CURVE_TEMP_AUX_BIAS_CURVE_TEMP_ARRAY0 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(272usize)
                as *const AUX_BIAS_CURVE_TEMP_AUX_BIAS_CURVE_TEMP_ARRAY0)
        }
    }
    #[doc = "0x110 - no description available"]
    #[inline(always)]
    pub fn aux_bias_curve_temp_aux_bias_curve_temp_0(
        &self,
    ) -> &AUX_BIAS_CURVE_TEMP_AUX_BIAS_CURVE_TEMP_0 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(272usize)
                as *const AUX_BIAS_CURVE_TEMP_AUX_BIAS_CURVE_TEMP_0)
        }
    }
    #[doc = "0x114 - Aux Bias Curve TEMP (105degC)"]
    #[inline(always)]
    pub fn aux_bias_curve_temp_aux_bias_curve_temp_array1(
        &self,
    ) -> &AUX_BIAS_CURVE_TEMP_AUX_BIAS_CURVE_TEMP_ARRAY1 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(276usize)
                as *const AUX_BIAS_CURVE_TEMP_AUX_BIAS_CURVE_TEMP_ARRAY1)
        }
    }
    #[doc = "0x114 - no description available"]
    #[inline(always)]
    pub fn aux_bias_curve_temp_aux_bias_curve_temp_1(
        &self,
    ) -> &AUX_BIAS_CURVE_TEMP_AUX_BIAS_CURVE_TEMP_1 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(276usize)
                as *const AUX_BIAS_CURVE_TEMP_AUX_BIAS_CURVE_TEMP_1)
        }
    }
    #[doc = "0x118 - Aux Bias Curve TEMP (105degC)"]
    #[inline(always)]
    pub fn aux_bias_curve_temp_aux_bias_curve_temp_array2(
        &self,
    ) -> &AUX_BIAS_CURVE_TEMP_AUX_BIAS_CURVE_TEMP_ARRAY2 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(280usize)
                as *const AUX_BIAS_CURVE_TEMP_AUX_BIAS_CURVE_TEMP_ARRAY2)
        }
    }
    #[doc = "0x118 - no description available"]
    #[inline(always)]
    pub fn aux_bias_curve_temp_aux_bias_curve_temp_2(
        &self,
    ) -> &AUX_BIAS_CURVE_TEMP_AUX_BIAS_CURVE_TEMP_2 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(280usize)
                as *const AUX_BIAS_CURVE_TEMP_AUX_BIAS_CURVE_TEMP_2)
        }
    }
    #[doc = "0x11c - Aux Bias Curve TEMP (105degC)"]
    #[inline(always)]
    pub fn aux_bias_curve_temp_aux_bias_curve_temp_array3(
        &self,
    ) -> &AUX_BIAS_CURVE_TEMP_AUX_BIAS_CURVE_TEMP_ARRAY3 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(284usize)
                as *const AUX_BIAS_CURVE_TEMP_AUX_BIAS_CURVE_TEMP_ARRAY3)
        }
    }
    #[doc = "0x11c - no description available"]
    #[inline(always)]
    pub fn aux_bias_curve_temp_aux_bias_curve_temp_3(
        &self,
    ) -> &AUX_BIAS_CURVE_TEMP_AUX_BIAS_CURVE_TEMP_3 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(284usize)
                as *const AUX_BIAS_CURVE_TEMP_AUX_BIAS_CURVE_TEMP_3)
        }
    }
    #[doc = "0x130 - no description available"]
    #[inline(always)]
    pub fn pvt_monitor_0_pvt_monitor_0_ringo(&self) -> &PVT_MONITOR_0_PVT_MONITOR_0_RINGO {
        unsafe {
            &*(((self as *const Self) as *const u8).add(304usize)
                as *const PVT_MONITOR_0_PVT_MONITOR_0_RINGO)
        }
    }
    #[doc = "0x130 - no description available"]
    #[inline(always)]
    pub fn pvt_monitor_0_pvt_monitor_0_array0(&self) -> &PVT_MONITOR_0_PVT_MONITOR_0_ARRAY0 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(304usize)
                as *const PVT_MONITOR_0_PVT_MONITOR_0_ARRAY0)
        }
    }
    #[doc = "0x134 - no description available"]
    #[inline(always)]
    pub fn pvt_monitor_0_pvt_monitor_0_delays_lsb(
        &self,
    ) -> &PVT_MONITOR_0_PVT_MONITOR_0_DELAYS_LSB {
        unsafe {
            &*(((self as *const Self) as *const u8).add(308usize)
                as *const PVT_MONITOR_0_PVT_MONITOR_0_DELAYS_LSB)
        }
    }
    #[doc = "0x134 - no description available"]
    #[inline(always)]
    pub fn pvt_monitor_0_pvt_monitor_0_array1(&self) -> &PVT_MONITOR_0_PVT_MONITOR_0_ARRAY1 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(308usize)
                as *const PVT_MONITOR_0_PVT_MONITOR_0_ARRAY1)
        }
    }
    #[doc = "0x138 - no description available"]
    #[inline(always)]
    pub fn pvt_monitor_0_pvt_monitor_0_delays_msb(
        &self,
    ) -> &PVT_MONITOR_0_PVT_MONITOR_0_DELAYS_MSB {
        unsafe {
            &*(((self as *const Self) as *const u8).add(312usize)
                as *const PVT_MONITOR_0_PVT_MONITOR_0_DELAYS_MSB)
        }
    }
    #[doc = "0x138 - no description available"]
    #[inline(always)]
    pub fn pvt_monitor_0_pvt_monitor_0_array2(&self) -> &PVT_MONITOR_0_PVT_MONITOR_0_ARRAY2 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(312usize)
                as *const PVT_MONITOR_0_PVT_MONITOR_0_ARRAY2)
        }
    }
    #[doc = "0x140 - no description available"]
    #[inline(always)]
    pub fn pvt_monitor_1_pvt_monitor_1_ringo(&self) -> &PVT_MONITOR_1_PVT_MONITOR_1_RINGO {
        unsafe {
            &*(((self as *const Self) as *const u8).add(320usize)
                as *const PVT_MONITOR_1_PVT_MONITOR_1_RINGO)
        }
    }
    #[doc = "0x140 - no description available"]
    #[inline(always)]
    pub fn pvt_monitor_1_pvt_monitor_1_array0(&self) -> &PVT_MONITOR_1_PVT_MONITOR_1_ARRAY0 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(320usize)
                as *const PVT_MONITOR_1_PVT_MONITOR_1_ARRAY0)
        }
    }
    #[doc = "0x144 - no description available"]
    #[inline(always)]
    pub fn pvt_monitor_1_pvt_monitor_1_delays_lsb(
        &self,
    ) -> &PVT_MONITOR_1_PVT_MONITOR_1_DELAYS_LSB {
        unsafe {
            &*(((self as *const Self) as *const u8).add(324usize)
                as *const PVT_MONITOR_1_PVT_MONITOR_1_DELAYS_LSB)
        }
    }
    #[doc = "0x144 - no description available"]
    #[inline(always)]
    pub fn pvt_monitor_1_pvt_monitor_1_array1(&self) -> &PVT_MONITOR_1_PVT_MONITOR_1_ARRAY1 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(324usize)
                as *const PVT_MONITOR_1_PVT_MONITOR_1_ARRAY1)
        }
    }
    #[doc = "0x148 - no description available"]
    #[inline(always)]
    pub fn pvt_monitor_1_pvt_monitor_1_delays_msb(
        &self,
    ) -> &PVT_MONITOR_1_PVT_MONITOR_1_DELAYS_MSB {
        unsafe {
            &*(((self as *const Self) as *const u8).add(328usize)
                as *const PVT_MONITOR_1_PVT_MONITOR_1_DELAYS_MSB)
        }
    }
    #[doc = "0x148 - no description available"]
    #[inline(always)]
    pub fn pvt_monitor_1_pvt_monitor_1_array2(&self) -> &PVT_MONITOR_1_PVT_MONITOR_1_ARRAY2 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(328usize)
                as *const PVT_MONITOR_1_PVT_MONITOR_1_ARRAY2)
        }
    }
    #[doc = "0x1e0 - ECID backup (the original is in page n-1)"]
    #[inline(always)]
    pub fn ecid_backup_ecid_backup_array0(&self) -> &ECID_BACKUP_ECID_BACKUP_ARRAY0 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(480usize)
                as *const ECID_BACKUP_ECID_BACKUP_ARRAY0)
        }
    }
    #[doc = "0x1e0 - no description available"]
    #[inline(always)]
    pub fn ecid_backup_ecid_backup_0(&self) -> &ECID_BACKUP_ECID_BACKUP_0 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(480usize)
                as *const ECID_BACKUP_ECID_BACKUP_0)
        }
    }
    #[doc = "0x1e4 - ECID backup (the original is in page n-1)"]
    #[inline(always)]
    pub fn ecid_backup_ecid_backup_array1(&self) -> &ECID_BACKUP_ECID_BACKUP_ARRAY1 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(484usize)
                as *const ECID_BACKUP_ECID_BACKUP_ARRAY1)
        }
    }
    #[doc = "0x1e4 - no description available"]
    #[inline(always)]
    pub fn ecid_backup_ecid_backup_1(&self) -> &ECID_BACKUP_ECID_BACKUP_1 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(484usize)
                as *const ECID_BACKUP_ECID_BACKUP_1)
        }
    }
    #[doc = "0x1e8 - ECID backup (the original is in page n-1)"]
    #[inline(always)]
    pub fn ecid_backup_ecid_backup_array2(&self) -> &ECID_BACKUP_ECID_BACKUP_ARRAY2 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(488usize)
                as *const ECID_BACKUP_ECID_BACKUP_ARRAY2)
        }
    }
    #[doc = "0x1e8 - no description available"]
    #[inline(always)]
    pub fn ecid_backup_ecid_backup_2(&self) -> &ECID_BACKUP_ECID_BACKUP_2 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(488usize)
                as *const ECID_BACKUP_ECID_BACKUP_2)
        }
    }
    #[doc = "0x1ec - ECID backup (the original is in page n-1)"]
    #[inline(always)]
    pub fn ecid_backup_ecid_backup_array3(&self) -> &ECID_BACKUP_ECID_BACKUP_ARRAY3 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(492usize)
                as *const ECID_BACKUP_ECID_BACKUP_ARRAY3)
        }
    }
    #[doc = "0x1ec - no description available"]
    #[inline(always)]
    pub fn ecid_backup_ecid_backup_3(&self) -> &ECID_BACKUP_ECID_BACKUP_3 {
        unsafe {
            &*(((self as *const Self) as *const u8).add(492usize)
                as *const ECID_BACKUP_ECID_BACKUP_3)
        }
    }
}
#[doc = "GPO0_GPO0_0 (rw) register accessor: an alias for `Reg<GPO0_GPO0_0_SPEC>`"]
pub type GPO0_GPO0_0 = crate::Reg<gpo0_gpo0_0::GPO0_GPO0_0_SPEC>;
#[doc = "GPO0 register 0 description"]
pub mod gpo0_gpo0_0;
#[doc = "GPO0_GPO0_ARRAY0 (rw) register accessor: an alias for `Reg<GPO0_GPO0_ARRAY0_SPEC>`"]
pub type GPO0_GPO0_ARRAY0 = crate::Reg<gpo0_gpo0_array0::GPO0_GPO0_ARRAY0_SPEC>;
#[doc = "GPO0 array description"]
pub mod gpo0_gpo0_array0;
#[doc = "GPO0_GPO0_1 (rw) register accessor: an alias for `Reg<GPO0_GPO0_1_SPEC>`"]
pub type GPO0_GPO0_1 = crate::Reg<gpo0_gpo0_1::GPO0_GPO0_1_SPEC>;
#[doc = "GPO0 register 1 description"]
pub mod gpo0_gpo0_1;
#[doc = "GPO0_GPO0_ARRAY1 (rw) register accessor: an alias for `Reg<GPO0_GPO0_ARRAY1_SPEC>`"]
pub type GPO0_GPO0_ARRAY1 = crate::Reg<gpo0_gpo0_array1::GPO0_GPO0_ARRAY1_SPEC>;
#[doc = "GPO0 array description"]
pub mod gpo0_gpo0_array1;
#[doc = "GPO0_GPO0_2 (rw) register accessor: an alias for `Reg<GPO0_GPO0_2_SPEC>`"]
pub type GPO0_GPO0_2 = crate::Reg<gpo0_gpo0_2::GPO0_GPO0_2_SPEC>;
#[doc = "GPO0 register 2 description"]
pub mod gpo0_gpo0_2;
#[doc = "GPO0_GPO0_ARRAY2 (rw) register accessor: an alias for `Reg<GPO0_GPO0_ARRAY2_SPEC>`"]
pub type GPO0_GPO0_ARRAY2 = crate::Reg<gpo0_gpo0_array2::GPO0_GPO0_ARRAY2_SPEC>;
#[doc = "GPO0 array description"]
pub mod gpo0_gpo0_array2;
#[doc = "GPO0_GPO0_3 (rw) register accessor: an alias for `Reg<GPO0_GPO0_3_SPEC>`"]
pub type GPO0_GPO0_3 = crate::Reg<gpo0_gpo0_3::GPO0_GPO0_3_SPEC>;
#[doc = "GPO0 register 3 description"]
pub mod gpo0_gpo0_3;
#[doc = "GPO0_GPO0_ARRAY3 (rw) register accessor: an alias for `Reg<GPO0_GPO0_ARRAY3_SPEC>`"]
pub type GPO0_GPO0_ARRAY3 = crate::Reg<gpo0_gpo0_array3::GPO0_GPO0_ARRAY3_SPEC>;
#[doc = "GPO0 array description"]
pub mod gpo0_gpo0_array3;
#[doc = "GPO1_GPO1_0 (rw) register accessor: an alias for `Reg<GPO1_GPO1_0_SPEC>`"]
pub type GPO1_GPO1_0 = crate::Reg<gpo1_gpo1_0::GPO1_GPO1_0_SPEC>;
#[doc = "GPO1 register 0 description"]
pub mod gpo1_gpo1_0;
#[doc = "GPO1_GPO1_ARRAY0 (rw) register accessor: an alias for `Reg<GPO1_GPO1_ARRAY0_SPEC>`"]
pub type GPO1_GPO1_ARRAY0 = crate::Reg<gpo1_gpo1_array0::GPO1_GPO1_ARRAY0_SPEC>;
#[doc = "GPO1 array description"]
pub mod gpo1_gpo1_array0;
#[doc = "GPO1_GPO1_1 (rw) register accessor: an alias for `Reg<GPO1_GPO1_1_SPEC>`"]
pub type GPO1_GPO1_1 = crate::Reg<gpo1_gpo1_1::GPO1_GPO1_1_SPEC>;
#[doc = "GPO1 register 1 description"]
pub mod gpo1_gpo1_1;
#[doc = "GPO1_GPO1_ARRAY1 (rw) register accessor: an alias for `Reg<GPO1_GPO1_ARRAY1_SPEC>`"]
pub type GPO1_GPO1_ARRAY1 = crate::Reg<gpo1_gpo1_array1::GPO1_GPO1_ARRAY1_SPEC>;
#[doc = "GPO1 array description"]
pub mod gpo1_gpo1_array1;
#[doc = "GPO1_GPO1_2 (rw) register accessor: an alias for `Reg<GPO1_GPO1_2_SPEC>`"]
pub type GPO1_GPO1_2 = crate::Reg<gpo1_gpo1_2::GPO1_GPO1_2_SPEC>;
#[doc = "GPO1 register 2 description"]
pub mod gpo1_gpo1_2;
#[doc = "GPO1_GPO1_ARRAY2 (rw) register accessor: an alias for `Reg<GPO1_GPO1_ARRAY2_SPEC>`"]
pub type GPO1_GPO1_ARRAY2 = crate::Reg<gpo1_gpo1_array2::GPO1_GPO1_ARRAY2_SPEC>;
#[doc = "GPO1 array description"]
pub mod gpo1_gpo1_array2;
#[doc = "GPO1_GPO1_3 (rw) register accessor: an alias for `Reg<GPO1_GPO1_3_SPEC>`"]
pub type GPO1_GPO1_3 = crate::Reg<gpo1_gpo1_3::GPO1_GPO1_3_SPEC>;
#[doc = "GPO1 register 3 description"]
pub mod gpo1_gpo1_3;
#[doc = "GPO1_GPO1_ARRAY3 (rw) register accessor: an alias for `Reg<GPO1_GPO1_ARRAY3_SPEC>`"]
pub type GPO1_GPO1_ARRAY3 = crate::Reg<gpo1_gpo1_array3::GPO1_GPO1_ARRAY3_SPEC>;
#[doc = "GPO1 array description"]
pub mod gpo1_gpo1_array3;
#[doc = "GPO2_GPO2_0 (rw) register accessor: an alias for `Reg<GPO2_GPO2_0_SPEC>`"]
pub type GPO2_GPO2_0 = crate::Reg<gpo2_gpo2_0::GPO2_GPO2_0_SPEC>;
#[doc = "GPO2 register 0 description"]
pub mod gpo2_gpo2_0;
#[doc = "GPO2_GPO2_ARRAY0 (rw) register accessor: an alias for `Reg<GPO2_GPO2_ARRAY0_SPEC>`"]
pub type GPO2_GPO2_ARRAY0 = crate::Reg<gpo2_gpo2_array0::GPO2_GPO2_ARRAY0_SPEC>;
#[doc = "GPO2 array description"]
pub mod gpo2_gpo2_array0;
#[doc = "GPO2_GPO2_1 (rw) register accessor: an alias for `Reg<GPO2_GPO2_1_SPEC>`"]
pub type GPO2_GPO2_1 = crate::Reg<gpo2_gpo2_1::GPO2_GPO2_1_SPEC>;
#[doc = "GPO2 register 1 description"]
pub mod gpo2_gpo2_1;
#[doc = "GPO2_GPO2_ARRAY1 (rw) register accessor: an alias for `Reg<GPO2_GPO2_ARRAY1_SPEC>`"]
pub type GPO2_GPO2_ARRAY1 = crate::Reg<gpo2_gpo2_array1::GPO2_GPO2_ARRAY1_SPEC>;
#[doc = "GPO2 array description"]
pub mod gpo2_gpo2_array1;
#[doc = "GPO2_GPO2_2 (rw) register accessor: an alias for `Reg<GPO2_GPO2_2_SPEC>`"]
pub type GPO2_GPO2_2 = crate::Reg<gpo2_gpo2_2::GPO2_GPO2_2_SPEC>;
#[doc = "GPO2 register 2 description"]
pub mod gpo2_gpo2_2;
#[doc = "GPO2_GPO2_ARRAY2 (rw) register accessor: an alias for `Reg<GPO2_GPO2_ARRAY2_SPEC>`"]
pub type GPO2_GPO2_ARRAY2 = crate::Reg<gpo2_gpo2_array2::GPO2_GPO2_ARRAY2_SPEC>;
#[doc = "GPO2 array description"]
pub mod gpo2_gpo2_array2;
#[doc = "GPO2_GPO2_3 (rw) register accessor: an alias for `Reg<GPO2_GPO2_3_SPEC>`"]
pub type GPO2_GPO2_3 = crate::Reg<gpo2_gpo2_3::GPO2_GPO2_3_SPEC>;
#[doc = "GPO2 register 3 description"]
pub mod gpo2_gpo2_3;
#[doc = "GPO2_GPO2_ARRAY3 (rw) register accessor: an alias for `Reg<GPO2_GPO2_ARRAY3_SPEC>`"]
pub type GPO2_GPO2_ARRAY3 = crate::Reg<gpo2_gpo2_array3::GPO2_GPO2_ARRAY3_SPEC>;
#[doc = "GPO2 array description"]
pub mod gpo2_gpo2_array3;
#[doc = "GPO3_GPO3_0 (rw) register accessor: an alias for `Reg<GPO3_GPO3_0_SPEC>`"]
pub type GPO3_GPO3_0 = crate::Reg<gpo3_gpo3_0::GPO3_GPO3_0_SPEC>;
#[doc = "GPO3 register 0 description"]
pub mod gpo3_gpo3_0;
#[doc = "GPO3_GPO3_ARRAY0 (rw) register accessor: an alias for `Reg<GPO3_GPO3_ARRAY0_SPEC>`"]
pub type GPO3_GPO3_ARRAY0 = crate::Reg<gpo3_gpo3_array0::GPO3_GPO3_ARRAY0_SPEC>;
#[doc = "GPO3 array description"]
pub mod gpo3_gpo3_array0;
#[doc = "GPO3_GPO3_1 (rw) register accessor: an alias for `Reg<GPO3_GPO3_1_SPEC>`"]
pub type GPO3_GPO3_1 = crate::Reg<gpo3_gpo3_1::GPO3_GPO3_1_SPEC>;
#[doc = "GPO3 register 1 description"]
pub mod gpo3_gpo3_1;
#[doc = "GPO3_GPO3_ARRAY1 (rw) register accessor: an alias for `Reg<GPO3_GPO3_ARRAY1_SPEC>`"]
pub type GPO3_GPO3_ARRAY1 = crate::Reg<gpo3_gpo3_array1::GPO3_GPO3_ARRAY1_SPEC>;
#[doc = "GPO3 array description"]
pub mod gpo3_gpo3_array1;
#[doc = "GPO3_GPO3_2 (rw) register accessor: an alias for `Reg<GPO3_GPO3_2_SPEC>`"]
pub type GPO3_GPO3_2 = crate::Reg<gpo3_gpo3_2::GPO3_GPO3_2_SPEC>;
#[doc = "GPO3 register 2 description"]
pub mod gpo3_gpo3_2;
#[doc = "GPO3_GPO3_ARRAY2 (rw) register accessor: an alias for `Reg<GPO3_GPO3_ARRAY2_SPEC>`"]
pub type GPO3_GPO3_ARRAY2 = crate::Reg<gpo3_gpo3_array2::GPO3_GPO3_ARRAY2_SPEC>;
#[doc = "GPO3 array description"]
pub mod gpo3_gpo3_array2;
#[doc = "GPO3_GPO3_3 (rw) register accessor: an alias for `Reg<GPO3_GPO3_3_SPEC>`"]
pub type GPO3_GPO3_3 = crate::Reg<gpo3_gpo3_3::GPO3_GPO3_3_SPEC>;
#[doc = "GPO3 register 3 description"]
pub mod gpo3_gpo3_3;
#[doc = "GPO3_GPO3_ARRAY3 (rw) register accessor: an alias for `Reg<GPO3_GPO3_ARRAY3_SPEC>`"]
pub type GPO3_GPO3_ARRAY3 = crate::Reg<gpo3_gpo3_array3::GPO3_GPO3_ARRAY3_SPEC>;
#[doc = "GPO3 array description"]
pub mod gpo3_gpo3_array3;
#[doc = "GPO_CHECKSUM_GPO_CHECKSUM_0 (rw) register accessor: an alias for `Reg<GPO_CHECKSUM_GPO_CHECKSUM_0_SPEC>`"]
pub type GPO_CHECKSUM_GPO_CHECKSUM_0 =
    crate::Reg<gpo_checksum_gpo_checksum_0::GPO_CHECKSUM_GPO_CHECKSUM_0_SPEC>;
#[doc = "checksum of the GPO data in words 0"]
pub mod gpo_checksum_gpo_checksum_0;
#[doc = "GPO_CHECKSUM_GPO_CHECKSUM_ARRAY0 (rw) register accessor: an alias for `Reg<GPO_CHECKSUM_GPO_CHECKSUM_ARRAY0_SPEC>`"]
pub type GPO_CHECKSUM_GPO_CHECKSUM_ARRAY0 =
    crate::Reg<gpo_checksum_gpo_checksum_array0::GPO_CHECKSUM_GPO_CHECKSUM_ARRAY0_SPEC>;
#[doc = "checksum of the GPO data in words \\[3:0\\]"]
pub mod gpo_checksum_gpo_checksum_array0;
#[doc = "GPO_CHECKSUM_GPO_CHECKSUM_1 (rw) register accessor: an alias for `Reg<GPO_CHECKSUM_GPO_CHECKSUM_1_SPEC>`"]
pub type GPO_CHECKSUM_GPO_CHECKSUM_1 =
    crate::Reg<gpo_checksum_gpo_checksum_1::GPO_CHECKSUM_GPO_CHECKSUM_1_SPEC>;
#[doc = "checksum of the GPO data in words 1"]
pub mod gpo_checksum_gpo_checksum_1;
#[doc = "GPO_CHECKSUM_GPO_CHECKSUM_ARRAY1 (rw) register accessor: an alias for `Reg<GPO_CHECKSUM_GPO_CHECKSUM_ARRAY1_SPEC>`"]
pub type GPO_CHECKSUM_GPO_CHECKSUM_ARRAY1 =
    crate::Reg<gpo_checksum_gpo_checksum_array1::GPO_CHECKSUM_GPO_CHECKSUM_ARRAY1_SPEC>;
#[doc = "checksum of the GPO data in words \\[3:0\\]"]
pub mod gpo_checksum_gpo_checksum_array1;
#[doc = "GPO_CHECKSUM_GPO_CHECKSUM_2 (rw) register accessor: an alias for `Reg<GPO_CHECKSUM_GPO_CHECKSUM_2_SPEC>`"]
pub type GPO_CHECKSUM_GPO_CHECKSUM_2 =
    crate::Reg<gpo_checksum_gpo_checksum_2::GPO_CHECKSUM_GPO_CHECKSUM_2_SPEC>;
#[doc = "checksum of the GPO data in words 2"]
pub mod gpo_checksum_gpo_checksum_2;
#[doc = "GPO_CHECKSUM_GPO_CHECKSUM_ARRAY2 (rw) register accessor: an alias for `Reg<GPO_CHECKSUM_GPO_CHECKSUM_ARRAY2_SPEC>`"]
pub type GPO_CHECKSUM_GPO_CHECKSUM_ARRAY2 =
    crate::Reg<gpo_checksum_gpo_checksum_array2::GPO_CHECKSUM_GPO_CHECKSUM_ARRAY2_SPEC>;
#[doc = "checksum of the GPO data in words \\[3:0\\]"]
pub mod gpo_checksum_gpo_checksum_array2;
#[doc = "GPO_CHECKSUM_GPO_CHECKSUM_3 (rw) register accessor: an alias for `Reg<GPO_CHECKSUM_GPO_CHECKSUM_3_SPEC>`"]
pub type GPO_CHECKSUM_GPO_CHECKSUM_3 =
    crate::Reg<gpo_checksum_gpo_checksum_3::GPO_CHECKSUM_GPO_CHECKSUM_3_SPEC>;
#[doc = "checksum of the GPO data in words 3"]
pub mod gpo_checksum_gpo_checksum_3;
#[doc = "GPO_CHECKSUM_GPO_CHECKSUM_ARRAY3 (rw) register accessor: an alias for `Reg<GPO_CHECKSUM_GPO_CHECKSUM_ARRAY3_SPEC>`"]
pub type GPO_CHECKSUM_GPO_CHECKSUM_ARRAY3 =
    crate::Reg<gpo_checksum_gpo_checksum_array3::GPO_CHECKSUM_GPO_CHECKSUM_ARRAY3_SPEC>;
#[doc = "checksum of the GPO data in words \\[3:0\\]"]
pub mod gpo_checksum_gpo_checksum_array3;
#[doc = "FINAL_TEST_BATCH_ID_FINAL_TEST_BATCH_ID_0 (rw) register accessor: an alias for `Reg<FINAL_TEST_BATCH_ID_FINAL_TEST_BATCH_ID_0_SPEC>`"]
pub type FINAL_TEST_BATCH_ID_FINAL_TEST_BATCH_ID_0 = crate::Reg<
    final_test_batch_id_final_test_batch_id_0::FINAL_TEST_BATCH_ID_FINAL_TEST_BATCH_ID_0_SPEC,
>;
#[doc = "no description available"]
pub mod final_test_batch_id_final_test_batch_id_0;
#[doc = "FINAL_TEST_BATCH_ID_FINAL_TEST_BATCH_ID_ARRAY0 (rw) register accessor: an alias for `Reg<FINAL_TEST_BATCH_ID_FINAL_TEST_BATCH_ID_ARRAY0_SPEC>`"]
pub type FINAL_TEST_BATCH_ID_FINAL_TEST_BATCH_ID_ARRAY0 = crate :: Reg < final_test_batch_id_final_test_batch_id_array0 :: FINAL_TEST_BATCH_ID_FINAL_TEST_BATCH_ID_ARRAY0_SPEC > ;
#[doc = "no description available"]
pub mod final_test_batch_id_final_test_batch_id_array0;
#[doc = "FINAL_TEST_BATCH_ID_FINAL_TEST_BATCH_ID_1 (rw) register accessor: an alias for `Reg<FINAL_TEST_BATCH_ID_FINAL_TEST_BATCH_ID_1_SPEC>`"]
pub type FINAL_TEST_BATCH_ID_FINAL_TEST_BATCH_ID_1 = crate::Reg<
    final_test_batch_id_final_test_batch_id_1::FINAL_TEST_BATCH_ID_FINAL_TEST_BATCH_ID_1_SPEC,
>;
#[doc = "no description available"]
pub mod final_test_batch_id_final_test_batch_id_1;
#[doc = "FINAL_TEST_BATCH_ID_FINAL_TEST_BATCH_ID_ARRAY1 (rw) register accessor: an alias for `Reg<FINAL_TEST_BATCH_ID_FINAL_TEST_BATCH_ID_ARRAY1_SPEC>`"]
pub type FINAL_TEST_BATCH_ID_FINAL_TEST_BATCH_ID_ARRAY1 = crate :: Reg < final_test_batch_id_final_test_batch_id_array1 :: FINAL_TEST_BATCH_ID_FINAL_TEST_BATCH_ID_ARRAY1_SPEC > ;
#[doc = "no description available"]
pub mod final_test_batch_id_final_test_batch_id_array1;
#[doc = "FINAL_TEST_BATCH_ID_FINAL_TEST_BATCH_ID_2 (rw) register accessor: an alias for `Reg<FINAL_TEST_BATCH_ID_FINAL_TEST_BATCH_ID_2_SPEC>`"]
pub type FINAL_TEST_BATCH_ID_FINAL_TEST_BATCH_ID_2 = crate::Reg<
    final_test_batch_id_final_test_batch_id_2::FINAL_TEST_BATCH_ID_FINAL_TEST_BATCH_ID_2_SPEC,
>;
#[doc = "no description available"]
pub mod final_test_batch_id_final_test_batch_id_2;
#[doc = "FINAL_TEST_BATCH_ID_FINAL_TEST_BATCH_ID_ARRAY2 (rw) register accessor: an alias for `Reg<FINAL_TEST_BATCH_ID_FINAL_TEST_BATCH_ID_ARRAY2_SPEC>`"]
pub type FINAL_TEST_BATCH_ID_FINAL_TEST_BATCH_ID_ARRAY2 = crate :: Reg < final_test_batch_id_final_test_batch_id_array2 :: FINAL_TEST_BATCH_ID_FINAL_TEST_BATCH_ID_ARRAY2_SPEC > ;
#[doc = "no description available"]
pub mod final_test_batch_id_final_test_batch_id_array2;
#[doc = "FINAL_TEST_BATCH_ID_FINAL_TEST_BATCH_ID_3 (rw) register accessor: an alias for `Reg<FINAL_TEST_BATCH_ID_FINAL_TEST_BATCH_ID_3_SPEC>`"]
pub type FINAL_TEST_BATCH_ID_FINAL_TEST_BATCH_ID_3 = crate::Reg<
    final_test_batch_id_final_test_batch_id_3::FINAL_TEST_BATCH_ID_FINAL_TEST_BATCH_ID_3_SPEC,
>;
#[doc = "no description available"]
pub mod final_test_batch_id_final_test_batch_id_3;
#[doc = "FINAL_TEST_BATCH_ID_FINAL_TEST_BATCH_ID_ARRAY3 (rw) register accessor: an alias for `Reg<FINAL_TEST_BATCH_ID_FINAL_TEST_BATCH_ID_ARRAY3_SPEC>`"]
pub type FINAL_TEST_BATCH_ID_FINAL_TEST_BATCH_ID_ARRAY3 = crate :: Reg < final_test_batch_id_final_test_batch_id_array3 :: FINAL_TEST_BATCH_ID_FINAL_TEST_BATCH_ID_ARRAY3_SPEC > ;
#[doc = "no description available"]
pub mod final_test_batch_id_final_test_batch_id_array3;
#[doc = "DEVICE_TYPE (rw) register accessor: an alias for `Reg<DEVICE_TYPE_SPEC>`"]
pub type DEVICE_TYPE = crate::Reg<device_type::DEVICE_TYPE_SPEC>;
#[doc = "no description available"]
pub mod device_type;
#[doc = "FINAL_TEST_PROGRAM_VERSION (rw) register accessor: an alias for `Reg<FINAL_TEST_PROGRAM_VERSION_SPEC>`"]
pub type FINAL_TEST_PROGRAM_VERSION =
    crate::Reg<final_test_program_version::FINAL_TEST_PROGRAM_VERSION_SPEC>;
#[doc = "no description available"]
pub mod final_test_program_version;
#[doc = "FINAL_TEST_DATE (rw) register accessor: an alias for `Reg<FINAL_TEST_DATE_SPEC>`"]
pub type FINAL_TEST_DATE = crate::Reg<final_test_date::FINAL_TEST_DATE_SPEC>;
#[doc = "no description available"]
pub mod final_test_date;
#[doc = "FINAL_TEST_TIME (rw) register accessor: an alias for `Reg<FINAL_TEST_TIME_SPEC>`"]
pub type FINAL_TEST_TIME = crate::Reg<final_test_time::FINAL_TEST_TIME_SPEC>;
#[doc = "no description available"]
pub mod final_test_time;
#[doc = "UUID_UUID_0 (rw) register accessor: an alias for `Reg<UUID_UUID_0_SPEC>`"]
pub type UUID_UUID_0 = crate::Reg<uuid_uuid_0::UUID_UUID_0_SPEC>;
#[doc = "no description available"]
pub mod uuid_uuid_0;
#[doc = "UUID_UUID_ARRAY0 (rw) register accessor: an alias for `Reg<UUID_UUID_ARRAY0_SPEC>`"]
pub type UUID_UUID_ARRAY0 = crate::Reg<uuid_uuid_array0::UUID_UUID_ARRAY0_SPEC>;
#[doc = "no description available"]
pub mod uuid_uuid_array0;
#[doc = "UUID_UUID_1 (rw) register accessor: an alias for `Reg<UUID_UUID_1_SPEC>`"]
pub type UUID_UUID_1 = crate::Reg<uuid_uuid_1::UUID_UUID_1_SPEC>;
#[doc = "no description available"]
pub mod uuid_uuid_1;
#[doc = "UUID_UUID_ARRAY1 (rw) register accessor: an alias for `Reg<UUID_UUID_ARRAY1_SPEC>`"]
pub type UUID_UUID_ARRAY1 = crate::Reg<uuid_uuid_array1::UUID_UUID_ARRAY1_SPEC>;
#[doc = "no description available"]
pub mod uuid_uuid_array1;
#[doc = "UUID_UUID_2 (rw) register accessor: an alias for `Reg<UUID_UUID_2_SPEC>`"]
pub type UUID_UUID_2 = crate::Reg<uuid_uuid_2::UUID_UUID_2_SPEC>;
#[doc = "no description available"]
pub mod uuid_uuid_2;
#[doc = "UUID_UUID_ARRAY2 (rw) register accessor: an alias for `Reg<UUID_UUID_ARRAY2_SPEC>`"]
pub type UUID_UUID_ARRAY2 = crate::Reg<uuid_uuid_array2::UUID_UUID_ARRAY2_SPEC>;
#[doc = "no description available"]
pub mod uuid_uuid_array2;
#[doc = "UUID_UUID_3 (rw) register accessor: an alias for `Reg<UUID_UUID_3_SPEC>`"]
pub type UUID_UUID_3 = crate::Reg<uuid_uuid_3::UUID_UUID_3_SPEC>;
#[doc = "no description available"]
pub mod uuid_uuid_3;
#[doc = "UUID_UUID_ARRAY3 (rw) register accessor: an alias for `Reg<UUID_UUID_ARRAY3_SPEC>`"]
pub type UUID_UUID_ARRAY3 = crate::Reg<uuid_uuid_array3::UUID_UUID_ARRAY3_SPEC>;
#[doc = "no description available"]
pub mod uuid_uuid_array3;
#[doc = "WAFER_TEST1_PROGRAM_VERSION (rw) register accessor: an alias for `Reg<WAFER_TEST1_PROGRAM_VERSION_SPEC>`"]
pub type WAFER_TEST1_PROGRAM_VERSION =
    crate::Reg<wafer_test1_program_version::WAFER_TEST1_PROGRAM_VERSION_SPEC>;
#[doc = "no description available"]
pub mod wafer_test1_program_version;
#[doc = "WAFER_TEST1_DATE (rw) register accessor: an alias for `Reg<WAFER_TEST1_DATE_SPEC>`"]
pub type WAFER_TEST1_DATE = crate::Reg<wafer_test1_date::WAFER_TEST1_DATE_SPEC>;
#[doc = "no description available"]
pub mod wafer_test1_date;
#[doc = "WAFER_TEST1_TIME (rw) register accessor: an alias for `Reg<WAFER_TEST1_TIME_SPEC>`"]
pub type WAFER_TEST1_TIME = crate::Reg<wafer_test1_time::WAFER_TEST1_TIME_SPEC>;
#[doc = "no description available"]
pub mod wafer_test1_time;
#[doc = "WAFER_TEST2_PROGRAM_VERSION (rw) register accessor: an alias for `Reg<WAFER_TEST2_PROGRAM_VERSION_SPEC>`"]
pub type WAFER_TEST2_PROGRAM_VERSION =
    crate::Reg<wafer_test2_program_version::WAFER_TEST2_PROGRAM_VERSION_SPEC>;
#[doc = "no description available"]
pub mod wafer_test2_program_version;
#[doc = "WAFER_TEST2_DATE (rw) register accessor: an alias for `Reg<WAFER_TEST2_DATE_SPEC>`"]
pub type WAFER_TEST2_DATE = crate::Reg<wafer_test2_date::WAFER_TEST2_DATE_SPEC>;
#[doc = "no description available"]
pub mod wafer_test2_date;
#[doc = "WAFER_TEST2_TIME (rw) register accessor: an alias for `Reg<WAFER_TEST2_TIME_SPEC>`"]
pub type WAFER_TEST2_TIME = crate::Reg<wafer_test2_time::WAFER_TEST2_TIME_SPEC>;
#[doc = "no description available"]
pub mod wafer_test2_time;
#[doc = "USBCFG (rw) register accessor: an alias for `Reg<USBCFG_SPEC>`"]
pub type USBCFG = crate::Reg<usbcfg::USBCFG_SPEC>;
#[doc = "no description available"]
pub mod usbcfg;
#[doc = "PERIPHENCFG (rw) register accessor: an alias for `Reg<PERIPHENCFG_SPEC>`"]
pub type PERIPHENCFG = crate::Reg<periphencfg::PERIPHENCFG_SPEC>;
#[doc = "no description available"]
pub mod periphencfg;
#[doc = "RAMSIZECFG (rw) register accessor: an alias for `Reg<RAMSIZECFG_SPEC>`"]
pub type RAMSIZECFG = crate::Reg<ramsizecfg::RAMSIZECFG_SPEC>;
#[doc = "no description available"]
pub mod ramsizecfg;
#[doc = "FLASHSIZECFG (rw) register accessor: an alias for `Reg<FLASHSIZECFG_SPEC>`"]
pub type FLASHSIZECFG = crate::Reg<flashsizecfg::FLASHSIZECFG_SPEC>;
#[doc = "no description available"]
pub mod flashsizecfg;
#[doc = "RINGO_0 (rw) register accessor: an alias for `Reg<RINGO_0_SPEC>`"]
pub type RINGO_0 = crate::Reg<ringo_0::RINGO_0_SPEC>;
#[doc = "no description available"]
pub mod ringo_0;
#[doc = "RINGO_1 (rw) register accessor: an alias for `Reg<RINGO_1_SPEC>`"]
pub type RINGO_1 = crate::Reg<ringo_1::RINGO_1_SPEC>;
#[doc = "no description available"]
pub mod ringo_1;
#[doc = "RINGO_2 (rw) register accessor: an alias for `Reg<RINGO_2_SPEC>`"]
pub type RINGO_2 = crate::Reg<ringo_2::RINGO_2_SPEC>;
#[doc = "no description available"]
pub mod ringo_2;
#[doc = "FRO_192MHZ (rw) register accessor: an alias for `Reg<FRO_192MHZ_SPEC>`"]
pub type FRO_192MHZ = crate::Reg<fro_192mhz::FRO_192MHZ_SPEC>;
#[doc = "no description available"]
pub mod fro_192mhz;
#[doc = "XO_32MHZ (rw) register accessor: an alias for `Reg<XO_32MHZ_SPEC>`"]
pub type XO_32MHZ = crate::Reg<xo_32mhz::XO_32MHZ_SPEC>;
#[doc = "no description available"]
pub mod xo_32mhz;
#[doc = "XO_32KHZ (rw) register accessor: an alias for `Reg<XO_32KHZ_SPEC>`"]
pub type XO_32KHZ = crate::Reg<xo_32khz::XO_32KHZ_SPEC>;
#[doc = "no description available"]
pub mod xo_32khz;
#[doc = "FRO_1MHZ (rw) register accessor: an alias for `Reg<FRO_1MHZ_SPEC>`"]
pub type FRO_1MHZ = crate::Reg<fro_1mhz::FRO_1MHZ_SPEC>;
#[doc = "no description available"]
pub mod fro_1mhz;
#[doc = "DCDC_POWER_PROFILE_HIGH_DCDC_POWER_PROFILE_HIGH_0 (rw) register accessor: an alias for `Reg<DCDC_POWER_PROFILE_HIGH_DCDC_POWER_PROFILE_HIGH_0_SPEC>`"]
pub type DCDC_POWER_PROFILE_HIGH_DCDC_POWER_PROFILE_HIGH_0 = crate :: Reg < dcdc_power_profile_high_dcdc_power_profile_high_0 :: DCDC_POWER_PROFILE_HIGH_DCDC_POWER_PROFILE_HIGH_0_SPEC > ;
#[doc = "no description available"]
pub mod dcdc_power_profile_high_dcdc_power_profile_high_0;
#[doc = "DCDC_POWER_PROFILE_HIGH_DCDC_POWER_PROFILE_HIGH_ARRAY0 (rw) register accessor: an alias for `Reg<DCDC_POWER_PROFILE_HIGH_DCDC_POWER_PROFILE_HIGH_ARRAY0_SPEC>`"]
pub type DCDC_POWER_PROFILE_HIGH_DCDC_POWER_PROFILE_HIGH_ARRAY0 = crate :: Reg < dcdc_power_profile_high_dcdc_power_profile_high_array0 :: DCDC_POWER_PROFILE_HIGH_DCDC_POWER_PROFILE_HIGH_ARRAY0_SPEC > ;
#[doc = "no description available"]
pub mod dcdc_power_profile_high_dcdc_power_profile_high_array0;
#[doc = "DCDC_POWER_PROFILE_HIGH_DCDC_POWER_PROFILE_HIGH_1 (rw) register accessor: an alias for `Reg<DCDC_POWER_PROFILE_HIGH_DCDC_POWER_PROFILE_HIGH_1_SPEC>`"]
pub type DCDC_POWER_PROFILE_HIGH_DCDC_POWER_PROFILE_HIGH_1 = crate :: Reg < dcdc_power_profile_high_dcdc_power_profile_high_1 :: DCDC_POWER_PROFILE_HIGH_DCDC_POWER_PROFILE_HIGH_1_SPEC > ;
#[doc = "no description available"]
pub mod dcdc_power_profile_high_dcdc_power_profile_high_1;
#[doc = "DCDC_POWER_PROFILE_HIGH_DCDC_POWER_PROFILE_HIGH_ARRAY1 (rw) register accessor: an alias for `Reg<DCDC_POWER_PROFILE_HIGH_DCDC_POWER_PROFILE_HIGH_ARRAY1_SPEC>`"]
pub type DCDC_POWER_PROFILE_HIGH_DCDC_POWER_PROFILE_HIGH_ARRAY1 = crate :: Reg < dcdc_power_profile_high_dcdc_power_profile_high_array1 :: DCDC_POWER_PROFILE_HIGH_DCDC_POWER_PROFILE_HIGH_ARRAY1_SPEC > ;
#[doc = "no description available"]
pub mod dcdc_power_profile_high_dcdc_power_profile_high_array1;
#[doc = "DCDC_POWER_PROFILE_LOW_DCDC_POWER_PROFILE_LOW_0 (rw) register accessor: an alias for `Reg<DCDC_POWER_PROFILE_LOW_DCDC_POWER_PROFILE_LOW_0_SPEC>`"]
pub type DCDC_POWER_PROFILE_LOW_DCDC_POWER_PROFILE_LOW_0 = crate :: Reg < dcdc_power_profile_low_dcdc_power_profile_low_0 :: DCDC_POWER_PROFILE_LOW_DCDC_POWER_PROFILE_LOW_0_SPEC > ;
#[doc = "no description available"]
pub mod dcdc_power_profile_low_dcdc_power_profile_low_0;
#[doc = "DCDC_POWER_PROFILE_LOW_DCDC_POWER_PROFILE_LOW_ARRAY0 (rw) register accessor: an alias for `Reg<DCDC_POWER_PROFILE_LOW_DCDC_POWER_PROFILE_LOW_ARRAY0_SPEC>`"]
pub type DCDC_POWER_PROFILE_LOW_DCDC_POWER_PROFILE_LOW_ARRAY0 = crate :: Reg < dcdc_power_profile_low_dcdc_power_profile_low_array0 :: DCDC_POWER_PROFILE_LOW_DCDC_POWER_PROFILE_LOW_ARRAY0_SPEC > ;
#[doc = "no description available"]
pub mod dcdc_power_profile_low_dcdc_power_profile_low_array0;
#[doc = "DCDC_POWER_PROFILE_LOW_DCDC_POWER_PROFILE_LOW_1 (rw) register accessor: an alias for `Reg<DCDC_POWER_PROFILE_LOW_DCDC_POWER_PROFILE_LOW_1_SPEC>`"]
pub type DCDC_POWER_PROFILE_LOW_DCDC_POWER_PROFILE_LOW_1 = crate :: Reg < dcdc_power_profile_low_dcdc_power_profile_low_1 :: DCDC_POWER_PROFILE_LOW_DCDC_POWER_PROFILE_LOW_1_SPEC > ;
#[doc = "no description available"]
pub mod dcdc_power_profile_low_dcdc_power_profile_low_1;
#[doc = "DCDC_POWER_PROFILE_LOW_DCDC_POWER_PROFILE_LOW_ARRAY1 (rw) register accessor: an alias for `Reg<DCDC_POWER_PROFILE_LOW_DCDC_POWER_PROFILE_LOW_ARRAY1_SPEC>`"]
pub type DCDC_POWER_PROFILE_LOW_DCDC_POWER_PROFILE_LOW_ARRAY1 = crate :: Reg < dcdc_power_profile_low_dcdc_power_profile_low_array1 :: DCDC_POWER_PROFILE_LOW_DCDC_POWER_PROFILE_LOW_ARRAY1_SPEC > ;
#[doc = "no description available"]
pub mod dcdc_power_profile_low_dcdc_power_profile_low_array1;
#[doc = "DCDC_POWER_PROFILE_MEDIUM_DCDC_POWER_PROFILE_MEDIUM_0 (rw) register accessor: an alias for `Reg<DCDC_POWER_PROFILE_MEDIUM_DCDC_POWER_PROFILE_MEDIUM_0_SPEC>`"]
pub type DCDC_POWER_PROFILE_MEDIUM_DCDC_POWER_PROFILE_MEDIUM_0 = crate :: Reg < dcdc_power_profile_medium_dcdc_power_profile_medium_0 :: DCDC_POWER_PROFILE_MEDIUM_DCDC_POWER_PROFILE_MEDIUM_0_SPEC > ;
#[doc = "no description available"]
pub mod dcdc_power_profile_medium_dcdc_power_profile_medium_0;
#[doc = "DCDC_POWER_PROFILE_MEDIUM_DCDC_POWER_PROFILE_MEDIUM_ARRAY0 (rw) register accessor: an alias for `Reg<DCDC_POWER_PROFILE_MEDIUM_DCDC_POWER_PROFILE_MEDIUM_ARRAY0_SPEC>`"]
pub type DCDC_POWER_PROFILE_MEDIUM_DCDC_POWER_PROFILE_MEDIUM_ARRAY0 = crate :: Reg < dcdc_power_profile_medium_dcdc_power_profile_medium_array0 :: DCDC_POWER_PROFILE_MEDIUM_DCDC_POWER_PROFILE_MEDIUM_ARRAY0_SPEC > ;
#[doc = "no description available"]
pub mod dcdc_power_profile_medium_dcdc_power_profile_medium_array0;
#[doc = "DCDC_POWER_PROFILE_MEDIUM_DCDC_POWER_PROFILE_MEDIUM_1 (rw) register accessor: an alias for `Reg<DCDC_POWER_PROFILE_MEDIUM_DCDC_POWER_PROFILE_MEDIUM_1_SPEC>`"]
pub type DCDC_POWER_PROFILE_MEDIUM_DCDC_POWER_PROFILE_MEDIUM_1 = crate :: Reg < dcdc_power_profile_medium_dcdc_power_profile_medium_1 :: DCDC_POWER_PROFILE_MEDIUM_DCDC_POWER_PROFILE_MEDIUM_1_SPEC > ;
#[doc = "no description available"]
pub mod dcdc_power_profile_medium_dcdc_power_profile_medium_1;
#[doc = "DCDC_POWER_PROFILE_MEDIUM_DCDC_POWER_PROFILE_MEDIUM_ARRAY1 (rw) register accessor: an alias for `Reg<DCDC_POWER_PROFILE_MEDIUM_DCDC_POWER_PROFILE_MEDIUM_ARRAY1_SPEC>`"]
pub type DCDC_POWER_PROFILE_MEDIUM_DCDC_POWER_PROFILE_MEDIUM_ARRAY1 = crate :: Reg < dcdc_power_profile_medium_dcdc_power_profile_medium_array1 :: DCDC_POWER_PROFILE_MEDIUM_DCDC_POWER_PROFILE_MEDIUM_ARRAY1_SPEC > ;
#[doc = "no description available"]
pub mod dcdc_power_profile_medium_dcdc_power_profile_medium_array1;
#[doc = "BOD (rw) register accessor: an alias for `Reg<BOD_SPEC>`"]
pub type BOD = crate::Reg<bod::BOD_SPEC>;
#[doc = "no description available"]
pub mod bod;
#[doc = "LDO_AO (rw) register accessor: an alias for `Reg<LDO_AO_SPEC>`"]
pub type LDO_AO = crate::Reg<ldo_ao::LDO_AO_SPEC>;
#[doc = "no description available"]
pub mod ldo_ao;
#[doc = "SDIO_DELAY (rw) register accessor: an alias for `Reg<SDIO_DELAY_SPEC>`"]
pub type SDIO_DELAY = crate::Reg<sdio_delay::SDIO_DELAY_SPEC>;
#[doc = "no description available"]
pub mod sdio_delay;
#[doc = "AUX_BIAS_CURVE_AMBIENT_AUX_BIAS_CURVE_AMBIENT_0 (rw) register accessor: an alias for `Reg<AUX_BIAS_CURVE_AMBIENT_AUX_BIAS_CURVE_AMBIENT_0_SPEC>`"]
pub type AUX_BIAS_CURVE_AMBIENT_AUX_BIAS_CURVE_AMBIENT_0 = crate :: Reg < aux_bias_curve_ambient_aux_bias_curve_ambient_0 :: AUX_BIAS_CURVE_AMBIENT_AUX_BIAS_CURVE_AMBIENT_0_SPEC > ;
#[doc = "no description available"]
pub mod aux_bias_curve_ambient_aux_bias_curve_ambient_0;
#[doc = "AUX_BIAS_CURVE_AMBIENT_AUX_BIAS_CURVE_AMBIENT_ARRAY0 (rw) register accessor: an alias for `Reg<AUX_BIAS_CURVE_AMBIENT_AUX_BIAS_CURVE_AMBIENT_ARRAY0_SPEC>`"]
pub type AUX_BIAS_CURVE_AMBIENT_AUX_BIAS_CURVE_AMBIENT_ARRAY0 = crate :: Reg < aux_bias_curve_ambient_aux_bias_curve_ambient_array0 :: AUX_BIAS_CURVE_AMBIENT_AUX_BIAS_CURVE_AMBIENT_ARRAY0_SPEC > ;
#[doc = "Aux Bias Curve Ambient (30degC)"]
pub mod aux_bias_curve_ambient_aux_bias_curve_ambient_array0;
#[doc = "AUX_BIAS_CURVE_AMBIENT_AUX_BIAS_CURVE_AMBIENT_1 (rw) register accessor: an alias for `Reg<AUX_BIAS_CURVE_AMBIENT_AUX_BIAS_CURVE_AMBIENT_1_SPEC>`"]
pub type AUX_BIAS_CURVE_AMBIENT_AUX_BIAS_CURVE_AMBIENT_1 = crate :: Reg < aux_bias_curve_ambient_aux_bias_curve_ambient_1 :: AUX_BIAS_CURVE_AMBIENT_AUX_BIAS_CURVE_AMBIENT_1_SPEC > ;
#[doc = "no description available"]
pub mod aux_bias_curve_ambient_aux_bias_curve_ambient_1;
#[doc = "AUX_BIAS_CURVE_AMBIENT_AUX_BIAS_CURVE_AMBIENT_ARRAY1 (rw) register accessor: an alias for `Reg<AUX_BIAS_CURVE_AMBIENT_AUX_BIAS_CURVE_AMBIENT_ARRAY1_SPEC>`"]
pub type AUX_BIAS_CURVE_AMBIENT_AUX_BIAS_CURVE_AMBIENT_ARRAY1 = crate :: Reg < aux_bias_curve_ambient_aux_bias_curve_ambient_array1 :: AUX_BIAS_CURVE_AMBIENT_AUX_BIAS_CURVE_AMBIENT_ARRAY1_SPEC > ;
#[doc = "Aux Bias Curve Ambient (30degC)"]
pub mod aux_bias_curve_ambient_aux_bias_curve_ambient_array1;
#[doc = "AUX_BIAS_CURVE_AMBIENT_AUX_BIAS_CURVE_AMBIENT_2 (rw) register accessor: an alias for `Reg<AUX_BIAS_CURVE_AMBIENT_AUX_BIAS_CURVE_AMBIENT_2_SPEC>`"]
pub type AUX_BIAS_CURVE_AMBIENT_AUX_BIAS_CURVE_AMBIENT_2 = crate :: Reg < aux_bias_curve_ambient_aux_bias_curve_ambient_2 :: AUX_BIAS_CURVE_AMBIENT_AUX_BIAS_CURVE_AMBIENT_2_SPEC > ;
#[doc = "no description available"]
pub mod aux_bias_curve_ambient_aux_bias_curve_ambient_2;
#[doc = "AUX_BIAS_CURVE_AMBIENT_AUX_BIAS_CURVE_AMBIENT_ARRAY2 (rw) register accessor: an alias for `Reg<AUX_BIAS_CURVE_AMBIENT_AUX_BIAS_CURVE_AMBIENT_ARRAY2_SPEC>`"]
pub type AUX_BIAS_CURVE_AMBIENT_AUX_BIAS_CURVE_AMBIENT_ARRAY2 = crate :: Reg < aux_bias_curve_ambient_aux_bias_curve_ambient_array2 :: AUX_BIAS_CURVE_AMBIENT_AUX_BIAS_CURVE_AMBIENT_ARRAY2_SPEC > ;
#[doc = "Aux Bias Curve Ambient (30degC)"]
pub mod aux_bias_curve_ambient_aux_bias_curve_ambient_array2;
#[doc = "AUX_BIAS_CURVE_AMBIENT_AUX_BIAS_CURVE_AMBIENT_3 (rw) register accessor: an alias for `Reg<AUX_BIAS_CURVE_AMBIENT_AUX_BIAS_CURVE_AMBIENT_3_SPEC>`"]
pub type AUX_BIAS_CURVE_AMBIENT_AUX_BIAS_CURVE_AMBIENT_3 = crate :: Reg < aux_bias_curve_ambient_aux_bias_curve_ambient_3 :: AUX_BIAS_CURVE_AMBIENT_AUX_BIAS_CURVE_AMBIENT_3_SPEC > ;
#[doc = "no description available"]
pub mod aux_bias_curve_ambient_aux_bias_curve_ambient_3;
#[doc = "AUX_BIAS_CURVE_AMBIENT_AUX_BIAS_CURVE_AMBIENT_ARRAY3 (rw) register accessor: an alias for `Reg<AUX_BIAS_CURVE_AMBIENT_AUX_BIAS_CURVE_AMBIENT_ARRAY3_SPEC>`"]
pub type AUX_BIAS_CURVE_AMBIENT_AUX_BIAS_CURVE_AMBIENT_ARRAY3 = crate :: Reg < aux_bias_curve_ambient_aux_bias_curve_ambient_array3 :: AUX_BIAS_CURVE_AMBIENT_AUX_BIAS_CURVE_AMBIENT_ARRAY3_SPEC > ;
#[doc = "Aux Bias Curve Ambient (30degC)"]
pub mod aux_bias_curve_ambient_aux_bias_curve_ambient_array3;
#[doc = "AUX_BIAS_CURVE_TEMP_AUX_BIAS_CURVE_TEMP_0 (rw) register accessor: an alias for `Reg<AUX_BIAS_CURVE_TEMP_AUX_BIAS_CURVE_TEMP_0_SPEC>`"]
pub type AUX_BIAS_CURVE_TEMP_AUX_BIAS_CURVE_TEMP_0 = crate::Reg<
    aux_bias_curve_temp_aux_bias_curve_temp_0::AUX_BIAS_CURVE_TEMP_AUX_BIAS_CURVE_TEMP_0_SPEC,
>;
#[doc = "no description available"]
pub mod aux_bias_curve_temp_aux_bias_curve_temp_0;
#[doc = "AUX_BIAS_CURVE_TEMP_AUX_BIAS_CURVE_TEMP_ARRAY0 (rw) register accessor: an alias for `Reg<AUX_BIAS_CURVE_TEMP_AUX_BIAS_CURVE_TEMP_ARRAY0_SPEC>`"]
pub type AUX_BIAS_CURVE_TEMP_AUX_BIAS_CURVE_TEMP_ARRAY0 = crate :: Reg < aux_bias_curve_temp_aux_bias_curve_temp_array0 :: AUX_BIAS_CURVE_TEMP_AUX_BIAS_CURVE_TEMP_ARRAY0_SPEC > ;
#[doc = "Aux Bias Curve TEMP (105degC)"]
pub mod aux_bias_curve_temp_aux_bias_curve_temp_array0;
#[doc = "AUX_BIAS_CURVE_TEMP_AUX_BIAS_CURVE_TEMP_1 (rw) register accessor: an alias for `Reg<AUX_BIAS_CURVE_TEMP_AUX_BIAS_CURVE_TEMP_1_SPEC>`"]
pub type AUX_BIAS_CURVE_TEMP_AUX_BIAS_CURVE_TEMP_1 = crate::Reg<
    aux_bias_curve_temp_aux_bias_curve_temp_1::AUX_BIAS_CURVE_TEMP_AUX_BIAS_CURVE_TEMP_1_SPEC,
>;
#[doc = "no description available"]
pub mod aux_bias_curve_temp_aux_bias_curve_temp_1;
#[doc = "AUX_BIAS_CURVE_TEMP_AUX_BIAS_CURVE_TEMP_ARRAY1 (rw) register accessor: an alias for `Reg<AUX_BIAS_CURVE_TEMP_AUX_BIAS_CURVE_TEMP_ARRAY1_SPEC>`"]
pub type AUX_BIAS_CURVE_TEMP_AUX_BIAS_CURVE_TEMP_ARRAY1 = crate :: Reg < aux_bias_curve_temp_aux_bias_curve_temp_array1 :: AUX_BIAS_CURVE_TEMP_AUX_BIAS_CURVE_TEMP_ARRAY1_SPEC > ;
#[doc = "Aux Bias Curve TEMP (105degC)"]
pub mod aux_bias_curve_temp_aux_bias_curve_temp_array1;
#[doc = "AUX_BIAS_CURVE_TEMP_AUX_BIAS_CURVE_TEMP_2 (rw) register accessor: an alias for `Reg<AUX_BIAS_CURVE_TEMP_AUX_BIAS_CURVE_TEMP_2_SPEC>`"]
pub type AUX_BIAS_CURVE_TEMP_AUX_BIAS_CURVE_TEMP_2 = crate::Reg<
    aux_bias_curve_temp_aux_bias_curve_temp_2::AUX_BIAS_CURVE_TEMP_AUX_BIAS_CURVE_TEMP_2_SPEC,
>;
#[doc = "no description available"]
pub mod aux_bias_curve_temp_aux_bias_curve_temp_2;
#[doc = "AUX_BIAS_CURVE_TEMP_AUX_BIAS_CURVE_TEMP_ARRAY2 (rw) register accessor: an alias for `Reg<AUX_BIAS_CURVE_TEMP_AUX_BIAS_CURVE_TEMP_ARRAY2_SPEC>`"]
pub type AUX_BIAS_CURVE_TEMP_AUX_BIAS_CURVE_TEMP_ARRAY2 = crate :: Reg < aux_bias_curve_temp_aux_bias_curve_temp_array2 :: AUX_BIAS_CURVE_TEMP_AUX_BIAS_CURVE_TEMP_ARRAY2_SPEC > ;
#[doc = "Aux Bias Curve TEMP (105degC)"]
pub mod aux_bias_curve_temp_aux_bias_curve_temp_array2;
#[doc = "AUX_BIAS_CURVE_TEMP_AUX_BIAS_CURVE_TEMP_3 (rw) register accessor: an alias for `Reg<AUX_BIAS_CURVE_TEMP_AUX_BIAS_CURVE_TEMP_3_SPEC>`"]
pub type AUX_BIAS_CURVE_TEMP_AUX_BIAS_CURVE_TEMP_3 = crate::Reg<
    aux_bias_curve_temp_aux_bias_curve_temp_3::AUX_BIAS_CURVE_TEMP_AUX_BIAS_CURVE_TEMP_3_SPEC,
>;
#[doc = "no description available"]
pub mod aux_bias_curve_temp_aux_bias_curve_temp_3;
#[doc = "AUX_BIAS_CURVE_TEMP_AUX_BIAS_CURVE_TEMP_ARRAY3 (rw) register accessor: an alias for `Reg<AUX_BIAS_CURVE_TEMP_AUX_BIAS_CURVE_TEMP_ARRAY3_SPEC>`"]
pub type AUX_BIAS_CURVE_TEMP_AUX_BIAS_CURVE_TEMP_ARRAY3 = crate :: Reg < aux_bias_curve_temp_aux_bias_curve_temp_array3 :: AUX_BIAS_CURVE_TEMP_AUX_BIAS_CURVE_TEMP_ARRAY3_SPEC > ;
#[doc = "Aux Bias Curve TEMP (105degC)"]
pub mod aux_bias_curve_temp_aux_bias_curve_temp_array3;
#[doc = "TEMP_SENS_VBE1VBE8_REF_1 (rw) register accessor: an alias for `Reg<TEMP_SENS_VBE1VBE8_REF_1_SPEC>`"]
pub type TEMP_SENS_VBE1VBE8_REF_1 =
    crate::Reg<temp_sens_vbe1vbe8_ref_1::TEMP_SENS_VBE1VBE8_REF_1_SPEC>;
#[doc = "no description available"]
pub mod temp_sens_vbe1vbe8_ref_1;
#[doc = "TEMP_SENS_VBE1VBE8_REF_2 (rw) register accessor: an alias for `Reg<TEMP_SENS_VBE1VBE8_REF_2_SPEC>`"]
pub type TEMP_SENS_VBE1VBE8_REF_2 =
    crate::Reg<temp_sens_vbe1vbe8_ref_2::TEMP_SENS_VBE1VBE8_REF_2_SPEC>;
#[doc = "no description available"]
pub mod temp_sens_vbe1vbe8_ref_2;
#[doc = "TEMP_SENS_SLOPE (rw) register accessor: an alias for `Reg<TEMP_SENS_SLOPE_SPEC>`"]
pub type TEMP_SENS_SLOPE = crate::Reg<temp_sens_slope::TEMP_SENS_SLOPE_SPEC>;
#[doc = "no description available"]
pub mod temp_sens_slope;
#[doc = "TEMP_SENS_OFFSET (rw) register accessor: an alias for `Reg<TEMP_SENS_OFFSET_SPEC>`"]
pub type TEMP_SENS_OFFSET = crate::Reg<temp_sens_offset::TEMP_SENS_OFFSET_SPEC>;
#[doc = "no description available"]
pub mod temp_sens_offset;
#[doc = "PVT_MONITOR_0_PVT_MONITOR_0_ARRAY0 (rw) register accessor: an alias for `Reg<PVT_MONITOR_0_PVT_MONITOR_0_ARRAY0_SPEC>`"]
pub type PVT_MONITOR_0_PVT_MONITOR_0_ARRAY0 =
    crate::Reg<pvt_monitor_0_pvt_monitor_0_array0::PVT_MONITOR_0_PVT_MONITOR_0_ARRAY0_SPEC>;
#[doc = "no description available"]
pub mod pvt_monitor_0_pvt_monitor_0_array0;
#[doc = "PVT_MONITOR_0_PVT_MONITOR_0_RINGO (rw) register accessor: an alias for `Reg<PVT_MONITOR_0_PVT_MONITOR_0_RINGO_SPEC>`"]
pub type PVT_MONITOR_0_PVT_MONITOR_0_RINGO =
    crate::Reg<pvt_monitor_0_pvt_monitor_0_ringo::PVT_MONITOR_0_PVT_MONITOR_0_RINGO_SPEC>;
#[doc = "no description available"]
pub mod pvt_monitor_0_pvt_monitor_0_ringo;
#[doc = "PVT_MONITOR_0_PVT_MONITOR_0_ARRAY1 (rw) register accessor: an alias for `Reg<PVT_MONITOR_0_PVT_MONITOR_0_ARRAY1_SPEC>`"]
pub type PVT_MONITOR_0_PVT_MONITOR_0_ARRAY1 =
    crate::Reg<pvt_monitor_0_pvt_monitor_0_array1::PVT_MONITOR_0_PVT_MONITOR_0_ARRAY1_SPEC>;
#[doc = "no description available"]
pub mod pvt_monitor_0_pvt_monitor_0_array1;
#[doc = "PVT_MONITOR_0_PVT_MONITOR_0_DELAYS_LSB (rw) register accessor: an alias for `Reg<PVT_MONITOR_0_PVT_MONITOR_0_DELAYS_LSB_SPEC>`"]
pub type PVT_MONITOR_0_PVT_MONITOR_0_DELAYS_LSB =
    crate::Reg<pvt_monitor_0_pvt_monitor_0_delays_lsb::PVT_MONITOR_0_PVT_MONITOR_0_DELAYS_LSB_SPEC>;
#[doc = "no description available"]
pub mod pvt_monitor_0_pvt_monitor_0_delays_lsb;
#[doc = "PVT_MONITOR_0_PVT_MONITOR_0_ARRAY2 (rw) register accessor: an alias for `Reg<PVT_MONITOR_0_PVT_MONITOR_0_ARRAY2_SPEC>`"]
pub type PVT_MONITOR_0_PVT_MONITOR_0_ARRAY2 =
    crate::Reg<pvt_monitor_0_pvt_monitor_0_array2::PVT_MONITOR_0_PVT_MONITOR_0_ARRAY2_SPEC>;
#[doc = "no description available"]
pub mod pvt_monitor_0_pvt_monitor_0_array2;
#[doc = "PVT_MONITOR_0_PVT_MONITOR_0_DELAYS_MSB (rw) register accessor: an alias for `Reg<PVT_MONITOR_0_PVT_MONITOR_0_DELAYS_MSB_SPEC>`"]
pub type PVT_MONITOR_0_PVT_MONITOR_0_DELAYS_MSB =
    crate::Reg<pvt_monitor_0_pvt_monitor_0_delays_msb::PVT_MONITOR_0_PVT_MONITOR_0_DELAYS_MSB_SPEC>;
#[doc = "no description available"]
pub mod pvt_monitor_0_pvt_monitor_0_delays_msb;
#[doc = "PVT_MONITOR_1_PVT_MONITOR_1_ARRAY0 (rw) register accessor: an alias for `Reg<PVT_MONITOR_1_PVT_MONITOR_1_ARRAY0_SPEC>`"]
pub type PVT_MONITOR_1_PVT_MONITOR_1_ARRAY0 =
    crate::Reg<pvt_monitor_1_pvt_monitor_1_array0::PVT_MONITOR_1_PVT_MONITOR_1_ARRAY0_SPEC>;
#[doc = "no description available"]
pub mod pvt_monitor_1_pvt_monitor_1_array0;
#[doc = "PVT_MONITOR_1_PVT_MONITOR_1_RINGO (rw) register accessor: an alias for `Reg<PVT_MONITOR_1_PVT_MONITOR_1_RINGO_SPEC>`"]
pub type PVT_MONITOR_1_PVT_MONITOR_1_RINGO =
    crate::Reg<pvt_monitor_1_pvt_monitor_1_ringo::PVT_MONITOR_1_PVT_MONITOR_1_RINGO_SPEC>;
#[doc = "no description available"]
pub mod pvt_monitor_1_pvt_monitor_1_ringo;
#[doc = "PVT_MONITOR_1_PVT_MONITOR_1_ARRAY1 (rw) register accessor: an alias for `Reg<PVT_MONITOR_1_PVT_MONITOR_1_ARRAY1_SPEC>`"]
pub type PVT_MONITOR_1_PVT_MONITOR_1_ARRAY1 =
    crate::Reg<pvt_monitor_1_pvt_monitor_1_array1::PVT_MONITOR_1_PVT_MONITOR_1_ARRAY1_SPEC>;
#[doc = "no description available"]
pub mod pvt_monitor_1_pvt_monitor_1_array1;
#[doc = "PVT_MONITOR_1_PVT_MONITOR_1_DELAYS_LSB (rw) register accessor: an alias for `Reg<PVT_MONITOR_1_PVT_MONITOR_1_DELAYS_LSB_SPEC>`"]
pub type PVT_MONITOR_1_PVT_MONITOR_1_DELAYS_LSB =
    crate::Reg<pvt_monitor_1_pvt_monitor_1_delays_lsb::PVT_MONITOR_1_PVT_MONITOR_1_DELAYS_LSB_SPEC>;
#[doc = "no description available"]
pub mod pvt_monitor_1_pvt_monitor_1_delays_lsb;
#[doc = "PVT_MONITOR_1_PVT_MONITOR_1_ARRAY2 (rw) register accessor: an alias for `Reg<PVT_MONITOR_1_PVT_MONITOR_1_ARRAY2_SPEC>`"]
pub type PVT_MONITOR_1_PVT_MONITOR_1_ARRAY2 =
    crate::Reg<pvt_monitor_1_pvt_monitor_1_array2::PVT_MONITOR_1_PVT_MONITOR_1_ARRAY2_SPEC>;
#[doc = "no description available"]
pub mod pvt_monitor_1_pvt_monitor_1_array2;
#[doc = "PVT_MONITOR_1_PVT_MONITOR_1_DELAYS_MSB (rw) register accessor: an alias for `Reg<PVT_MONITOR_1_PVT_MONITOR_1_DELAYS_MSB_SPEC>`"]
pub type PVT_MONITOR_1_PVT_MONITOR_1_DELAYS_MSB =
    crate::Reg<pvt_monitor_1_pvt_monitor_1_delays_msb::PVT_MONITOR_1_PVT_MONITOR_1_DELAYS_MSB_SPEC>;
#[doc = "no description available"]
pub mod pvt_monitor_1_pvt_monitor_1_delays_msb;
#[doc = "NXP_DEVICE_PRIVATE_KEY (rw) register accessor: an alias for `Reg<NXP_DEVICE_PRIVATE_KEY_SPEC>`"]
pub type NXP_DEVICE_PRIVATE_KEY = crate::Reg<nxp_device_private_key::NXP_DEVICE_PRIVATE_KEY_SPEC>;
#[doc = "no description available"]
pub mod nxp_device_private_key;
#[doc = "NXP_DEVICE_CERTIFICATE_0 (rw) register accessor: an alias for `Reg<NXP_DEVICE_CERTIFICATE_0_SPEC>`"]
pub type NXP_DEVICE_CERTIFICATE_0 =
    crate::Reg<nxp_device_certificate_0::NXP_DEVICE_CERTIFICATE_0_SPEC>;
#[doc = "NXP Device Certificate (ECDSA_sign - r\\[255:128\\])"]
pub mod nxp_device_certificate_0;
#[doc = "NXP_DEVICE_CERTIFICATE_1 (rw) register accessor: an alias for `Reg<NXP_DEVICE_CERTIFICATE_1_SPEC>`"]
pub type NXP_DEVICE_CERTIFICATE_1 =
    crate::Reg<nxp_device_certificate_1::NXP_DEVICE_CERTIFICATE_1_SPEC>;
#[doc = "NXP Device Certificate (ECDSA_sign - r\\[127:0\\])"]
pub mod nxp_device_certificate_1;
#[doc = "NXP_DEVICE_CERTIFICATE_2 (rw) register accessor: an alias for `Reg<NXP_DEVICE_CERTIFICATE_2_SPEC>`"]
pub type NXP_DEVICE_CERTIFICATE_2 =
    crate::Reg<nxp_device_certificate_2::NXP_DEVICE_CERTIFICATE_2_SPEC>;
#[doc = "NXP Device Certificate (ECDSA_sign - s\\[255:128\\])"]
pub mod nxp_device_certificate_2;
#[doc = "NXP_DEVICE_CERTIFICATE_3 (rw) register accessor: an alias for `Reg<NXP_DEVICE_CERTIFICATE_3_SPEC>`"]
pub type NXP_DEVICE_CERTIFICATE_3 =
    crate::Reg<nxp_device_certificate_3::NXP_DEVICE_CERTIFICATE_3_SPEC>;
#[doc = "NXP Device Certificate (ECDSA_sign - s\\[127:0\\])"]
pub mod nxp_device_certificate_3;
#[doc = "SHA256_DIGEST (rw) register accessor: an alias for `Reg<SHA256_DIGEST_SPEC>`"]
pub type SHA256_DIGEST = crate::Reg<sha256_digest::SHA256_DIGEST_SPEC>;
#[doc = "SHA-256 DIGEST (9EC00 - 9FDBC) ROM Patch Area + NXP Area (IMPORTANT NOTE: Pages used for Repair (N-8 to N-3) are excluded from the computation) SHA256_DIGESTindex for DIGEST\\[((index * 32) + 31):(index * 32)\\]"]
pub mod sha256_digest;
#[doc = "ECID_BACKUP_ECID_BACKUP_0 (rw) register accessor: an alias for `Reg<ECID_BACKUP_ECID_BACKUP_0_SPEC>`"]
pub type ECID_BACKUP_ECID_BACKUP_0 =
    crate::Reg<ecid_backup_ecid_backup_0::ECID_BACKUP_ECID_BACKUP_0_SPEC>;
#[doc = "no description available"]
pub mod ecid_backup_ecid_backup_0;
#[doc = "ECID_BACKUP_ECID_BACKUP_ARRAY0 (rw) register accessor: an alias for `Reg<ECID_BACKUP_ECID_BACKUP_ARRAY0_SPEC>`"]
pub type ECID_BACKUP_ECID_BACKUP_ARRAY0 =
    crate::Reg<ecid_backup_ecid_backup_array0::ECID_BACKUP_ECID_BACKUP_ARRAY0_SPEC>;
#[doc = "ECID backup (the original is in page n-1)"]
pub mod ecid_backup_ecid_backup_array0;
#[doc = "ECID_BACKUP_ECID_BACKUP_1 (rw) register accessor: an alias for `Reg<ECID_BACKUP_ECID_BACKUP_1_SPEC>`"]
pub type ECID_BACKUP_ECID_BACKUP_1 =
    crate::Reg<ecid_backup_ecid_backup_1::ECID_BACKUP_ECID_BACKUP_1_SPEC>;
#[doc = "no description available"]
pub mod ecid_backup_ecid_backup_1;
#[doc = "ECID_BACKUP_ECID_BACKUP_ARRAY1 (rw) register accessor: an alias for `Reg<ECID_BACKUP_ECID_BACKUP_ARRAY1_SPEC>`"]
pub type ECID_BACKUP_ECID_BACKUP_ARRAY1 =
    crate::Reg<ecid_backup_ecid_backup_array1::ECID_BACKUP_ECID_BACKUP_ARRAY1_SPEC>;
#[doc = "ECID backup (the original is in page n-1)"]
pub mod ecid_backup_ecid_backup_array1;
#[doc = "ECID_BACKUP_ECID_BACKUP_2 (rw) register accessor: an alias for `Reg<ECID_BACKUP_ECID_BACKUP_2_SPEC>`"]
pub type ECID_BACKUP_ECID_BACKUP_2 =
    crate::Reg<ecid_backup_ecid_backup_2::ECID_BACKUP_ECID_BACKUP_2_SPEC>;
#[doc = "no description available"]
pub mod ecid_backup_ecid_backup_2;
#[doc = "ECID_BACKUP_ECID_BACKUP_ARRAY2 (rw) register accessor: an alias for `Reg<ECID_BACKUP_ECID_BACKUP_ARRAY2_SPEC>`"]
pub type ECID_BACKUP_ECID_BACKUP_ARRAY2 =
    crate::Reg<ecid_backup_ecid_backup_array2::ECID_BACKUP_ECID_BACKUP_ARRAY2_SPEC>;
#[doc = "ECID backup (the original is in page n-1)"]
pub mod ecid_backup_ecid_backup_array2;
#[doc = "ECID_BACKUP_ECID_BACKUP_3 (rw) register accessor: an alias for `Reg<ECID_BACKUP_ECID_BACKUP_3_SPEC>`"]
pub type ECID_BACKUP_ECID_BACKUP_3 =
    crate::Reg<ecid_backup_ecid_backup_3::ECID_BACKUP_ECID_BACKUP_3_SPEC>;
#[doc = "no description available"]
pub mod ecid_backup_ecid_backup_3;
#[doc = "ECID_BACKUP_ECID_BACKUP_ARRAY3 (rw) register accessor: an alias for `Reg<ECID_BACKUP_ECID_BACKUP_ARRAY3_SPEC>`"]
pub type ECID_BACKUP_ECID_BACKUP_ARRAY3 =
    crate::Reg<ecid_backup_ecid_backup_array3::ECID_BACKUP_ECID_BACKUP_ARRAY3_SPEC>;
#[doc = "ECID backup (the original is in page n-1)"]
pub mod ecid_backup_ecid_backup_array3;
#[doc = "CHECKSUM (rw) register accessor: an alias for `Reg<CHECKSUM_SPEC>`"]
pub type CHECKSUM = crate::Reg<checksum::CHECKSUM_SPEC>;
#[doc = "Checksum of the whole page"]
pub mod checksum;
#[doc = "DIS_ROM_HIDING (rw) register accessor: an alias for `Reg<DIS_ROM_HIDING_SPEC>`"]
pub type DIS_ROM_HIDING = crate::Reg<dis_rom_hiding::DIS_ROM_HIDING_SPEC>;
#[doc = "no description available"]
pub mod dis_rom_hiding;
#[doc = "PUF_SRAM (rw) register accessor: an alias for `Reg<PUF_SRAM_SPEC>`"]
pub type PUF_SRAM = crate::Reg<puf_sram::PUF_SRAM_SPEC>;
#[doc = "no description available"]
pub mod puf_sram;
