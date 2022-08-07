#[doc = "Register `HMAC_REG1` reader"]
pub struct R(crate::R<HMAC_REG1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HMAC_REG1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HMAC_REG1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HMAC_REG1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HMAC_REG1` writer"]
pub struct W(crate::W<HMAC_REG1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HMAC_REG1_SPEC>;
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
impl From<crate::W<HMAC_REG1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HMAC_REG1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HMAC_REG1` reader - no description available"]
pub type HMAC_REG1_R = crate::FieldReader<u32, u32>;
#[doc = "Field `HMAC_REG1` writer - no description available"]
pub type HMAC_REG1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, HMAC_REG1_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - no description available"]
    #[inline(always)]
    pub fn hmac_reg1(&self) -> HMAC_REG1_R {
        HMAC_REG1_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - no description available"]
    #[inline(always)]
    pub fn hmac_reg1(&mut self) -> HMAC_REG1_W<0> {
        HMAC_REG1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HMAC\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hmac_reg1](index.html) module"]
pub struct HMAC_REG1_SPEC;
impl crate::RegisterSpec for HMAC_REG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hmac_reg1::R](R) reader structure"]
impl crate::Readable for HMAC_REG1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hmac_reg1::W](W) writer structure"]
impl crate::Writable for HMAC_REG1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HMAC_REG1 to value 0"]
impl crate::Resettable for HMAC_REG1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
