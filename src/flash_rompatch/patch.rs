#[doc = "Register `PATCH[%s]` reader"]
pub struct R(crate::R<PATCH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PATCH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PATCH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PATCH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PATCH[%s]` writer"]
pub struct W(crate::W<PATCH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PATCH_SPEC>;
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
impl From<crate::W<PATCH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PATCH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PATCH` reader - ."]
pub type PATCH_R = crate::FieldReader<u32, u32>;
#[doc = "Field `PATCH` writer - ."]
pub type PATCH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PATCH_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - ."]
    #[inline(always)]
    pub fn patch(&self) -> PATCH_R {
        PATCH_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - ."]
    #[inline(always)]
    pub fn patch(&mut self) -> PATCH_W<0> {
        PATCH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [patch](index.html) module"]
pub struct PATCH_SPEC;
impl crate::RegisterSpec for PATCH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [patch::R](R) reader structure"]
impl crate::Readable for PATCH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [patch::W](W) writer structure"]
impl crate::Writable for PATCH_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PATCH[%s]
to value 0"]
impl crate::Resettable for PATCH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
