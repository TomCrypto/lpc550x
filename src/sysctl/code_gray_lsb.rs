#[doc = "Register `CODE_GRAY_LSB` reader"]
pub struct R(crate::R<CODE_GRAY_LSB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CODE_GRAY_LSB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CODE_GRAY_LSB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CODE_GRAY_LSB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CODE_GRAY_LSB` writer"]
pub struct W(crate::W<CODE_GRAY_LSB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CODE_GRAY_LSB_SPEC>;
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
impl From<crate::W<CODE_GRAY_LSB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CODE_GRAY_LSB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CODE_GRAY_LSB` reader - Gray code (42bits) to be converted back to binary"]
pub type CODE_GRAY_LSB_R = crate::FieldReader<u32, u32>;
#[doc = "Field `CODE_GRAY_LSB` writer - Gray code (42bits) to be converted back to binary"]
pub type CODE_GRAY_LSB_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CODE_GRAY_LSB_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Gray code (42bits) to be converted back to binary"]
    #[inline(always)]
    pub fn code_gray_lsb(&self) -> CODE_GRAY_LSB_R {
        CODE_GRAY_LSB_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Gray code (42bits) to be converted back to binary"]
    #[inline(always)]
    pub fn code_gray_lsb(&mut self) -> CODE_GRAY_LSB_W<0> {
        CODE_GRAY_LSB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CODE_GRAY LSB input Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [code_gray_lsb](index.html) module"]
pub struct CODE_GRAY_LSB_SPEC;
impl crate::RegisterSpec for CODE_GRAY_LSB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [code_gray_lsb::R](R) reader structure"]
impl crate::Readable for CODE_GRAY_LSB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [code_gray_lsb::W](W) writer structure"]
impl crate::Writable for CODE_GRAY_LSB_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CODE_GRAY_LSB to value 0"]
impl crate::Resettable for CODE_GRAY_LSB_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
