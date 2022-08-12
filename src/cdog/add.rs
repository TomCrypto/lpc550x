#[doc = "Register `ADD` writer"]
pub struct W(crate::W<ADD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADD_SPEC>;
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
impl From<crate::W<ADD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AD` writer - Address of ADD command."]
pub type AD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADD_SPEC, u32, u32, 32, O>;
impl W {
    #[doc = "Bits 0:31 - Address of ADD command."]
    #[inline(always)]
    pub fn ad(&mut self) -> AD_W<0> {
        AD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Write address for issuing the ADD command.\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [add](index.html) module"]
pub struct ADD_SPEC;
impl crate::RegisterSpec for ADD_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [add::W](W) writer structure"]
impl crate::Writable for ADD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADD to value 0"]
impl crate::Resettable for ADD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
