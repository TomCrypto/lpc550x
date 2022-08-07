#[doc = "Register `SUB` writer"]
pub struct W(crate::W<SUB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SUB_SPEC>;
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
impl From<crate::W<SUB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SUB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `S0B` writer - Address of SUB command."]
pub type S0B_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SUB_SPEC, u32, u32, 32, O>;
impl W {
    #[doc = "Bits 0:31 - Address of SUB command."]
    #[inline(always)]
    pub fn s0b(&mut self) -> S0B_W<0> {
        S0B_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Write address for issuing the SUB command.\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sub](index.html) module"]
pub struct SUB_SPEC;
impl crate::RegisterSpec for SUB_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [sub::W](W) writer structure"]
impl crate::Writable for SUB_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SUB to value 0"]
impl crate::Resettable for SUB_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
