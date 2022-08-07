#[doc = "Register `TEMP_SENS_VBE1VBE8_REF_1` reader"]
pub struct R(crate::R<TEMP_SENS_VBE1VBE8_REF_1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TEMP_SENS_VBE1VBE8_REF_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TEMP_SENS_VBE1VBE8_REF_1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TEMP_SENS_VBE1VBE8_REF_1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TEMP_SENS_VBE1VBE8_REF_1` writer"]
pub struct W(crate::W<TEMP_SENS_VBE1VBE8_REF_1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TEMP_SENS_VBE1VBE8_REF_1_SPEC>;
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
impl From<crate::W<TEMP_SENS_VBE1VBE8_REF_1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TEMP_SENS_VBE1VBE8_REF_1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VBE1` reader - no description available"]
pub type VBE1_R = crate::FieldReader<u16, u16>;
#[doc = "Field `VBE1` writer - no description available"]
pub type VBE1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TEMP_SENS_VBE1VBE8_REF_1_SPEC, u16, u16, 16, O>;
#[doc = "Field `VBE8` reader - no description available"]
pub type VBE8_R = crate::FieldReader<u16, u16>;
#[doc = "Field `VBE8` writer - no description available"]
pub type VBE8_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TEMP_SENS_VBE1VBE8_REF_1_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - no description available"]
    #[inline(always)]
    pub fn vbe1(&self) -> VBE1_R {
        VBE1_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - no description available"]
    #[inline(always)]
    pub fn vbe8(&self) -> VBE8_R {
        VBE8_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - no description available"]
    #[inline(always)]
    pub fn vbe1(&mut self) -> VBE1_W<0> {
        VBE1_W::new(self)
    }
    #[doc = "Bits 16:31 - no description available"]
    #[inline(always)]
    pub fn vbe8(&mut self) -> VBE8_W<16> {
        VBE8_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "no description available\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [temp_sens_vbe1vbe8_ref_1](index.html) module"]
pub struct TEMP_SENS_VBE1VBE8_REF_1_SPEC;
impl crate::RegisterSpec for TEMP_SENS_VBE1VBE8_REF_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [temp_sens_vbe1vbe8_ref_1::R](R) reader structure"]
impl crate::Readable for TEMP_SENS_VBE1VBE8_REF_1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [temp_sens_vbe1vbe8_ref_1::W](W) writer structure"]
impl crate::Writable for TEMP_SENS_VBE1VBE8_REF_1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TEMP_SENS_VBE1VBE8_REF_1 to value 0"]
impl crate::Resettable for TEMP_SENS_VBE1VBE8_REF_1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
