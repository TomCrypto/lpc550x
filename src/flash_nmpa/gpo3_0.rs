#[doc = "Register `GPO3_0` reader"]
pub struct R(crate::R<GPO3_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPO3_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPO3_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPO3_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPO3_0` writer"]
pub struct W(crate::W<GPO3_0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPO3_0_SPEC>;
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
impl From<crate::W<GPO3_0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPO3_0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AUX_BIAS_TRIM_VALID` reader - no description available."]
pub type AUX_BIAS_TRIM_VALID_R = crate::BitReader<bool>;
#[doc = "Field `AUX_BIAS_TRIM_VALID` writer - no description available."]
pub type AUX_BIAS_TRIM_VALID_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPO3_0_SPEC, bool, O>;
#[doc = "Field `AUX_BIAS_ITRIM` reader - no description available."]
pub type AUX_BIAS_ITRIM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AUX_BIAS_ITRIM` writer - no description available."]
pub type AUX_BIAS_ITRIM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GPO3_0_SPEC, u8, u8, 5, O>;
#[doc = "Field `AUX_BIAS_PTAT_ITRIM` reader - no description available."]
pub type AUX_BIAS_PTAT_ITRIM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AUX_BIAS_PTAT_ITRIM` writer - no description available."]
pub type AUX_BIAS_PTAT_ITRIM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GPO3_0_SPEC, u8, u8, 5, O>;
#[doc = "Field `AUX_BIAS_VREF1_VTRIM` reader - no description available."]
pub type AUX_BIAS_VREF1_VTRIM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AUX_BIAS_VREF1_VTRIM` writer - no description available."]
pub type AUX_BIAS_VREF1_VTRIM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GPO3_0_SPEC, u8, u8, 5, O>;
#[doc = "Field `AUX_BIAS_VREF1_VCURVE_TRIM` reader - no description available."]
pub type AUX_BIAS_VREF1_VCURVE_TRIM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AUX_BIAS_VREF1_VCURVE_TRIM` writer - no description available."]
pub type AUX_BIAS_VREF1_VCURVE_TRIM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GPO3_0_SPEC, u8, u8, 3, O>;
#[doc = "Field `FIELD` reader - no description available."]
pub type FIELD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FIELD` writer - no description available."]
pub type FIELD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GPO3_0_SPEC, u8, u8, 6, O>;
#[doc = "Field `MODELNUM_EXTENSION` reader - ModelNumber extension\\[2:0\\]"]
pub type MODELNUM_EXTENSION_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MODELNUM_EXTENSION` writer - ModelNumber extension\\[2:0\\]"]
pub type MODELNUM_EXTENSION_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GPO3_0_SPEC, u8, u8, 3, O>;
#[doc = "Field `FINAL_TEST_NOT_DONE` reader - FINAL_TEST_NOT_DONE\\[3:0\\]: 1010 : Final Test Not Done. All Other values: Final Test Done."]
pub type FINAL_TEST_NOT_DONE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FINAL_TEST_NOT_DONE` writer - FINAL_TEST_NOT_DONE\\[3:0\\]: 1010 : Final Test Not Done. All Other values: Final Test Done."]
pub type FINAL_TEST_NOT_DONE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GPO3_0_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bit 0 - no description available."]
    #[inline(always)]
    pub fn aux_bias_trim_valid(&self) -> AUX_BIAS_TRIM_VALID_R {
        AUX_BIAS_TRIM_VALID_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:5 - no description available."]
    #[inline(always)]
    pub fn aux_bias_itrim(&self) -> AUX_BIAS_ITRIM_R {
        AUX_BIAS_ITRIM_R::new(((self.bits >> 1) & 0x1f) as u8)
    }
    #[doc = "Bits 6:10 - no description available."]
    #[inline(always)]
    pub fn aux_bias_ptat_itrim(&self) -> AUX_BIAS_PTAT_ITRIM_R {
        AUX_BIAS_PTAT_ITRIM_R::new(((self.bits >> 6) & 0x1f) as u8)
    }
    #[doc = "Bits 11:15 - no description available."]
    #[inline(always)]
    pub fn aux_bias_vref1_vtrim(&self) -> AUX_BIAS_VREF1_VTRIM_R {
        AUX_BIAS_VREF1_VTRIM_R::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bits 16:18 - no description available."]
    #[inline(always)]
    pub fn aux_bias_vref1_vcurve_trim(&self) -> AUX_BIAS_VREF1_VCURVE_TRIM_R {
        AUX_BIAS_VREF1_VCURVE_TRIM_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 19:24 - no description available."]
    #[inline(always)]
    pub fn field(&self) -> FIELD_R {
        FIELD_R::new(((self.bits >> 19) & 0x3f) as u8)
    }
    #[doc = "Bits 25:27 - ModelNumber extension\\[2:0\\]"]
    #[inline(always)]
    pub fn modelnum_extension(&self) -> MODELNUM_EXTENSION_R {
        MODELNUM_EXTENSION_R::new(((self.bits >> 25) & 7) as u8)
    }
    #[doc = "Bits 28:31 - FINAL_TEST_NOT_DONE\\[3:0\\]: 1010 : Final Test Not Done. All Other values: Final Test Done."]
    #[inline(always)]
    pub fn final_test_not_done(&self) -> FINAL_TEST_NOT_DONE_R {
        FINAL_TEST_NOT_DONE_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - no description available."]
    #[inline(always)]
    pub fn aux_bias_trim_valid(&mut self) -> AUX_BIAS_TRIM_VALID_W<0> {
        AUX_BIAS_TRIM_VALID_W::new(self)
    }
    #[doc = "Bits 1:5 - no description available."]
    #[inline(always)]
    pub fn aux_bias_itrim(&mut self) -> AUX_BIAS_ITRIM_W<1> {
        AUX_BIAS_ITRIM_W::new(self)
    }
    #[doc = "Bits 6:10 - no description available."]
    #[inline(always)]
    pub fn aux_bias_ptat_itrim(&mut self) -> AUX_BIAS_PTAT_ITRIM_W<6> {
        AUX_BIAS_PTAT_ITRIM_W::new(self)
    }
    #[doc = "Bits 11:15 - no description available."]
    #[inline(always)]
    pub fn aux_bias_vref1_vtrim(&mut self) -> AUX_BIAS_VREF1_VTRIM_W<11> {
        AUX_BIAS_VREF1_VTRIM_W::new(self)
    }
    #[doc = "Bits 16:18 - no description available."]
    #[inline(always)]
    pub fn aux_bias_vref1_vcurve_trim(&mut self) -> AUX_BIAS_VREF1_VCURVE_TRIM_W<16> {
        AUX_BIAS_VREF1_VCURVE_TRIM_W::new(self)
    }
    #[doc = "Bits 19:24 - no description available."]
    #[inline(always)]
    pub fn field(&mut self) -> FIELD_W<19> {
        FIELD_W::new(self)
    }
    #[doc = "Bits 25:27 - ModelNumber extension\\[2:0\\]"]
    #[inline(always)]
    pub fn modelnum_extension(&mut self) -> MODELNUM_EXTENSION_W<25> {
        MODELNUM_EXTENSION_W::new(self)
    }
    #[doc = "Bits 28:31 - FINAL_TEST_NOT_DONE\\[3:0\\]: 1010 : Final Test Not Done. All Other values: Final Test Done."]
    #[inline(always)]
    pub fn final_test_not_done(&mut self) -> FINAL_TEST_NOT_DONE_W<28> {
        FINAL_TEST_NOT_DONE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPO3 register 0 description.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpo3_0](index.html) module"]
pub struct GPO3_0_SPEC;
impl crate::RegisterSpec for GPO3_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpo3_0::R](R) reader structure"]
impl crate::Readable for GPO3_0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpo3_0::W](W) writer structure"]
impl crate::Writable for GPO3_0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GPO3_0 to value 0"]
impl crate::Resettable for GPO3_0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
