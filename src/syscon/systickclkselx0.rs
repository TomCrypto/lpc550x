#[doc = "Register `SYSTICKCLKSELX0` reader"]
pub struct R(crate::R<SYSTICKCLKSELX0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYSTICKCLKSELX0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYSTICKCLKSELX0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYSTICKCLKSELX0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYSTICKCLKSELX0` writer"]
pub struct W(crate::W<SYSTICKCLKSELX0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYSTICKCLKSELX0_SPEC>;
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
impl From<crate::W<SYSTICKCLKSELX0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYSTICKCLKSELX0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATA` reader - Data array value."]
pub type DATA_R = crate::FieldReader<u32, u32>;
#[doc = "Field `DATA` writer - Data array value."]
pub type DATA_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SYSTICKCLKSELX0_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Data array value."]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Data array value."]
    #[inline(always)]
    pub fn data(&mut self) -> DATA_W<0> {
        DATA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Peripheral reset control register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [systickclkselx0](index.html) module"]
pub struct SYSTICKCLKSELX0_SPEC;
impl crate::RegisterSpec for SYSTICKCLKSELX0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [systickclkselx0::R](R) reader structure"]
impl crate::Readable for SYSTICKCLKSELX0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [systickclkselx0::W](W) writer structure"]
impl crate::Writable for SYSTICKCLKSELX0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SYSTICKCLKSELX0 to value 0"]
impl crate::Resettable for SYSTICKCLKSELX0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
