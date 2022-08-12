#[doc = "Register `GPO1_0` reader"]
pub struct R(crate::R<GPO1_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPO1_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPO1_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPO1_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPO1_0` writer"]
pub struct W(crate::W<GPO1_0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPO1_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<GPO1_0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPO1_0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FINAL_TEST_NOT_DONE` reader - FINAL_TEST_NOT_DONE\\[3:0\\]: 1010 : Final Test Not Done. All Other values: Final Test Done."]
pub type FINAL_TEST_NOT_DONE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FINAL_TEST_NOT_DONE` writer - FINAL_TEST_NOT_DONE\\[3:0\\]: 1010 : Final Test Not Done. All Other values: Final Test Done."]
pub type FINAL_TEST_NOT_DONE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GPO1_0_SPEC, u8, u8, 4, O>;
#[doc = "Field `PARTCONFIG` reader - Device type number. (E.g : LPC5569 stored as 69 decimal)"]
pub type PARTCONFIG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PARTCONFIG` writer - Device type number. (E.g : LPC5569 stored as 69 decimal)"]
pub type PARTCONFIG_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GPO1_0_SPEC, u8, u8, 7, O>;
#[doc = "Field `DEVICE_TYPE_SEC` reader - Security device type: 0: LPC55xxx (Non Secure Familly) 1: LPC55Sxxx (Secure Familly)"]
pub type DEVICE_TYPE_SEC_R = crate::BitReader<bool>;
#[doc = "Field `DEVICE_TYPE_SEC` writer - Security device type: 0: LPC55xxx (Non Secure Familly) 1: LPC55Sxxx (Secure Familly)"]
pub type DEVICE_TYPE_SEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPO1_0_SPEC, bool, O>;
#[doc = "Field `SRAM_SIZE` reader - SRAM_SIZE\\[3:0\\]: (For Niobe4) 0000 : 320 KB 0001 : 256 KB 0010 : 144 KB 0011 : 80 KB (For Niobe4 Mini) 0100 : 96 KB 0101 : 80 KB 0110 : 64 KB 0111 : 48 KB All others : RESERVED."]
pub type SRAM_SIZE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SRAM_SIZE` writer - SRAM_SIZE\\[3:0\\]: (For Niobe4) 0000 : 320 KB 0001 : 256 KB 0010 : 144 KB 0011 : 80 KB (For Niobe4 Mini) 0100 : 96 KB 0101 : 80 KB 0110 : 64 KB 0111 : 48 KB All others : RESERVED."]
pub type SRAM_SIZE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GPO1_0_SPEC, u8, u8, 4, O>;
#[doc = "Field `CPU0_SECURITY_EXTENSION_DISABLE` reader - CPU0_SECURITY_EXTENSION_DISABLE\\[3:0\\]: 1010 : CPU0 Security Extension is disabled. All Other values: CPU0 Security Extension is enabled."]
pub type CPU0_SECURITY_EXTENSION_DISABLE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CPU0_SECURITY_EXTENSION_DISABLE` writer - CPU0_SECURITY_EXTENSION_DISABLE\\[3:0\\]: 1010 : CPU0 Security Extension is disabled. All Other values: CPU0 Security Extension is enabled."]
pub type CPU0_SECURITY_EXTENSION_DISABLE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GPO1_0_SPEC, u8, u8, 4, O>;
#[doc = "Field `FIELD` reader - no description available."]
pub type FIELD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FIELD` writer - no description available."]
pub type FIELD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GPO1_0_SPEC, u8, u8, 4, O>;
#[doc = "Field `ROM_REVISION_MINOR` reader - ROM Revision-Minor \\[3:0\\]"]
pub type ROM_REVISION_MINOR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ROM_REVISION_MINOR` writer - ROM Revision-Minor \\[3:0\\]"]
pub type ROM_REVISION_MINOR_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GPO1_0_SPEC, u8, u8, 4, O>;
#[doc = "Field `METAL_REVISION_ID` reader - METAL REVISION ID\\[3:0\\]"]
pub type METAL_REVISION_ID_R = crate::FieldReader<u8, u8>;
#[doc = "Field `METAL_REVISION_ID` writer - METAL REVISION ID\\[3:0\\]"]
pub type METAL_REVISION_ID_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GPO1_0_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3 - FINAL_TEST_NOT_DONE\\[3:0\\]: 1010 : Final Test Not Done. All Other values: Final Test Done."]
    #[inline(always)]
    pub fn final_test_not_done(&self) -> FINAL_TEST_NOT_DONE_R {
        FINAL_TEST_NOT_DONE_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:10 - Device type number. (E.g : LPC5569 stored as 69 decimal)"]
    #[inline(always)]
    pub fn partconfig(&self) -> PARTCONFIG_R {
        PARTCONFIG_R::new(((self.bits >> 4) & 0x7f) as u8)
    }
    #[doc = "Bit 11 - Security device type: 0: LPC55xxx (Non Secure Familly) 1: LPC55Sxxx (Secure Familly)"]
    #[inline(always)]
    pub fn device_type_sec(&self) -> DEVICE_TYPE_SEC_R {
        DEVICE_TYPE_SEC_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:15 - SRAM_SIZE\\[3:0\\]: (For Niobe4) 0000 : 320 KB 0001 : 256 KB 0010 : 144 KB 0011 : 80 KB (For Niobe4 Mini) 0100 : 96 KB 0101 : 80 KB 0110 : 64 KB 0111 : 48 KB All others : RESERVED."]
    #[inline(always)]
    pub fn sram_size(&self) -> SRAM_SIZE_R {
        SRAM_SIZE_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - CPU0_SECURITY_EXTENSION_DISABLE\\[3:0\\]: 1010 : CPU0 Security Extension is disabled. All Other values: CPU0 Security Extension is enabled."]
    #[inline(always)]
    pub fn cpu0_security_extension_disable(&self) -> CPU0_SECURITY_EXTENSION_DISABLE_R {
        CPU0_SECURITY_EXTENSION_DISABLE_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - no description available."]
    #[inline(always)]
    pub fn field(&self) -> FIELD_R {
        FIELD_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - ROM Revision-Minor \\[3:0\\]"]
    #[inline(always)]
    pub fn rom_revision_minor(&self) -> ROM_REVISION_MINOR_R {
        ROM_REVISION_MINOR_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - METAL REVISION ID\\[3:0\\]"]
    #[inline(always)]
    pub fn metal_revision_id(&self) -> METAL_REVISION_ID_R {
        METAL_REVISION_ID_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - FINAL_TEST_NOT_DONE\\[3:0\\]: 1010 : Final Test Not Done. All Other values: Final Test Done."]
    #[inline(always)]
    pub fn final_test_not_done(&mut self) -> FINAL_TEST_NOT_DONE_W<0> {
        FINAL_TEST_NOT_DONE_W::new(self)
    }
    #[doc = "Bits 4:10 - Device type number. (E.g : LPC5569 stored as 69 decimal)"]
    #[inline(always)]
    pub fn partconfig(&mut self) -> PARTCONFIG_W<4> {
        PARTCONFIG_W::new(self)
    }
    #[doc = "Bit 11 - Security device type: 0: LPC55xxx (Non Secure Familly) 1: LPC55Sxxx (Secure Familly)"]
    #[inline(always)]
    pub fn device_type_sec(&mut self) -> DEVICE_TYPE_SEC_W<11> {
        DEVICE_TYPE_SEC_W::new(self)
    }
    #[doc = "Bits 12:15 - SRAM_SIZE\\[3:0\\]: (For Niobe4) 0000 : 320 KB 0001 : 256 KB 0010 : 144 KB 0011 : 80 KB (For Niobe4 Mini) 0100 : 96 KB 0101 : 80 KB 0110 : 64 KB 0111 : 48 KB All others : RESERVED."]
    #[inline(always)]
    pub fn sram_size(&mut self) -> SRAM_SIZE_W<12> {
        SRAM_SIZE_W::new(self)
    }
    #[doc = "Bits 16:19 - CPU0_SECURITY_EXTENSION_DISABLE\\[3:0\\]: 1010 : CPU0 Security Extension is disabled. All Other values: CPU0 Security Extension is enabled."]
    #[inline(always)]
    pub fn cpu0_security_extension_disable(&mut self) -> CPU0_SECURITY_EXTENSION_DISABLE_W<16> {
        CPU0_SECURITY_EXTENSION_DISABLE_W::new(self)
    }
    #[doc = "Bits 20:23 - no description available."]
    #[inline(always)]
    pub fn field(&mut self) -> FIELD_W<20> {
        FIELD_W::new(self)
    }
    #[doc = "Bits 24:27 - ROM Revision-Minor \\[3:0\\]"]
    #[inline(always)]
    pub fn rom_revision_minor(&mut self) -> ROM_REVISION_MINOR_W<24> {
        ROM_REVISION_MINOR_W::new(self)
    }
    #[doc = "Bits 28:31 - METAL REVISION ID\\[3:0\\]"]
    #[inline(always)]
    pub fn metal_revision_id(&mut self) -> METAL_REVISION_ID_W<28> {
        METAL_REVISION_ID_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPO1 register 0 description.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpo1_0](index.html) module"]
pub struct GPO1_0_SPEC;
impl crate::RegisterSpec for GPO1_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpo1_0::R](R) reader structure"]
impl crate::Readable for GPO1_0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpo1_0::W](W) writer structure"]
impl crate::Writable for GPO1_0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GPO1_0 to value 0"]
impl crate::Resettable for GPO1_0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
