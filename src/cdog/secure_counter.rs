#[doc = "Register `SECURE_COUNTER` reader"]
pub struct R(crate::R<SECURE_COUNTER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SECURE_COUNTER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SECURE_COUNTER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SECURE_COUNTER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SECURE_COUNTER` writer"]
pub struct W(crate::W<SECURE_COUNTER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SECURE_COUNTER_SPEC>;
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
impl From<crate::W<SECURE_COUNTER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SECURE_COUNTER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SECCNT` reader - Secure Counter"]
pub type SECCNT_R = crate::FieldReader<u32, u32>;
#[doc = "Field `SECCNT` writer - Secure Counter"]
pub type SECCNT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SECURE_COUNTER_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Secure Counter"]
    #[inline(always)]
    pub fn seccnt(&self) -> SECCNT_R {
        SECCNT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Secure Counter"]
    #[inline(always)]
    pub fn seccnt(&mut self) -> SECCNT_W<0> {
        SECCNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Also known as SEC_CNT\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [secure_counter](index.html) module"]
pub struct SECURE_COUNTER_SPEC;
impl crate::RegisterSpec for SECURE_COUNTER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [secure_counter::R](R) reader structure"]
impl crate::Readable for SECURE_COUNTER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [secure_counter::W](W) writer structure"]
impl crate::Writable for SECURE_COUNTER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SECURE_COUNTER to value 0"]
impl crate::Resettable for SECURE_COUNTER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
