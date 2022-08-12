#[doc = "Register `STOP` writer"]
pub struct W(crate::W<STOP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STOP_SPEC>;
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
impl From<crate::W<STOP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STOP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `STP` writer - Address of stop command access."]
pub type STP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, STOP_SPEC, u32, u32, 32, O>;
impl W {
    #[doc = "Bits 0:31 - Address of stop command access."]
    #[inline(always)]
    pub fn stp(&mut self) -> STP_W<0> {
        STP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Write address for issuing the STOP command.\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stop](index.html) module"]
pub struct STOP_SPEC;
impl crate::RegisterSpec for STOP_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [stop::W](W) writer structure"]
impl crate::Writable for STOP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets STOP to value 0"]
impl crate::Resettable for STOP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
