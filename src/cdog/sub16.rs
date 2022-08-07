#[doc = "Register `SUB16` writer"]
pub struct W(crate::W<SUB16_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SUB16_SPEC>;
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
impl From<crate::W<SUB16_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SUB16_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SB16` writer - Address of SUB16 command."]
pub type SB16_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SUB16_SPEC, u32, u32, 32, O>;
impl W {
    #[doc = "Bits 0:31 - Address of SUB16 command."]
    #[inline(always)]
    pub fn sb16(&mut self) -> SB16_W<0> {
        SB16_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Write address for issuing the SUB16 command.\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sub16](index.html) module"]
pub struct SUB16_SPEC;
impl crate::RegisterSpec for SUB16_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [sub16::W](W) writer structure"]
impl crate::Writable for SUB16_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SUB16 to value 0"]
impl crate::Resettable for SUB16_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
