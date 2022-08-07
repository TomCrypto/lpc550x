#[doc = "Register `RESTART` writer"]
pub struct W(crate::W<RESTART_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RESTART_SPEC>;
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
impl From<crate::W<RESTART_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RESTART_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RSTRT` writer - Write address for issuing the RESTART command."]
pub type RSTRT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RESTART_SPEC, u32, u32, 32, O>;
impl W {
    #[doc = "Bits 0:31 - Write address for issuing the RESTART command."]
    #[inline(always)]
    pub fn rstrt(&mut self) -> RSTRT_W<0> {
        RSTRT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Write address for issuing the RESTART command.\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [restart](index.html) module"]
pub struct RESTART_SPEC;
impl crate::RegisterSpec for RESTART_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [restart::W](W) writer structure"]
impl crate::Writable for RESTART_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RESTART to value 0"]
impl crate::Resettable for RESTART_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
