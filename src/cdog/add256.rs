#[doc = "Register `ADD256` writer"]
pub struct W(crate::W<ADD256_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADD256_SPEC>;
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
impl From<crate::W<ADD256_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADD256_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AD256` writer - Address of ADD256 command."]
pub type AD256_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADD256_SPEC, u32, u32, 32, O>;
impl W {
    #[doc = "Bits 0:31 - Address of ADD256 command."]
    #[inline(always)]
    pub fn ad256(&mut self) -> AD256_W<0> {
        AD256_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Write address for issuing the ADD16 command.\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [add256](index.html) module"]
pub struct ADD256_SPEC;
impl crate::RegisterSpec for ADD256_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [add256::W](W) writer structure"]
impl crate::Writable for ADD256_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADD256 to value 0"]
impl crate::Resettable for ADD256_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
