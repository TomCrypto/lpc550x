#[doc = "Register `FINAL_TEST_TIME` reader"]
pub struct R(crate::R<FINAL_TEST_TIME_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FINAL_TEST_TIME_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FINAL_TEST_TIME_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FINAL_TEST_TIME_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FINAL_TEST_TIME` writer"]
pub struct W(crate::W<FINAL_TEST_TIME_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FINAL_TEST_TIME_SPEC>;
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
impl From<crate::W<FINAL_TEST_TIME_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FINAL_TEST_TIME_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIME` reader - TIME \\[stored as : hour*10000+minute*100+seconde\\]"]
pub type TIME_R = crate::FieldReader<u32, u32>;
#[doc = "Field `TIME` writer - TIME \\[stored as : hour*10000+minute*100+seconde\\]"]
pub type TIME_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FINAL_TEST_TIME_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - TIME \\[stored as : hour*10000+minute*100+seconde\\]"]
    #[inline(always)]
    pub fn time(&self) -> TIME_R {
        TIME_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - TIME \\[stored as : hour*10000+minute*100+seconde\\]"]
    #[inline(always)]
    pub fn time(&mut self) -> TIME_W<0> {
        TIME_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "no description available.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [final_test_time](index.html) module"]
pub struct FINAL_TEST_TIME_SPEC;
impl crate::RegisterSpec for FINAL_TEST_TIME_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [final_test_time::R](R) reader structure"]
impl crate::Readable for FINAL_TEST_TIME_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [final_test_time::W](W) writer structure"]
impl crate::Writable for FINAL_TEST_TIME_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FINAL_TEST_TIME to value 0"]
impl crate::Resettable for FINAL_TEST_TIME_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
