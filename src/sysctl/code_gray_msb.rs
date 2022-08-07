#[doc = "Register `CODE_GRAY_MSB` reader"]
pub struct R(crate::R<CODE_GRAY_MSB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CODE_GRAY_MSB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CODE_GRAY_MSB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CODE_GRAY_MSB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CODE_GRAY_MSB` writer"]
pub struct W(crate::W<CODE_GRAY_MSB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CODE_GRAY_MSB_SPEC>;
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
impl From<crate::W<CODE_GRAY_MSB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CODE_GRAY_MSB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CODE_GRAY_MSB` reader - Gray code (42bits) to be converted back to binary"]
pub type CODE_GRAY_MSB_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CODE_GRAY_MSB` writer - Gray code (42bits) to be converted back to binary"]
pub type CODE_GRAY_MSB_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CODE_GRAY_MSB_SPEC, u16, u16, 10, O>;
impl R {
    #[doc = "Bits 0:9 - Gray code (42bits) to be converted back to binary"]
    #[inline(always)]
    pub fn code_gray_msb(&self) -> CODE_GRAY_MSB_R {
        CODE_GRAY_MSB_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Gray code (42bits) to be converted back to binary"]
    #[inline(always)]
    pub fn code_gray_msb(&mut self) -> CODE_GRAY_MSB_W<0> {
        CODE_GRAY_MSB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CODE_GRAY MSB input Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [code_gray_msb](index.html) module"]
pub struct CODE_GRAY_MSB_SPEC;
impl crate::RegisterSpec for CODE_GRAY_MSB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [code_gray_msb::R](R) reader structure"]
impl crate::Readable for CODE_GRAY_MSB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [code_gray_msb::W](W) writer structure"]
impl crate::Writable for CODE_GRAY_MSB_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CODE_GRAY_MSB to value 0"]
impl crate::Resettable for CODE_GRAY_MSB_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
