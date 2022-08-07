#[doc = "Register `FLASHREMAP_OFFSET` reader"]
pub struct R(crate::R<FLASHREMAP_OFFSET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLASHREMAP_OFFSET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLASHREMAP_OFFSET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLASHREMAP_OFFSET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FLASHREMAP_OFFSET` writer"]
pub struct W(crate::W<FLASHREMAP_OFFSET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLASHREMAP_OFFSET_SPEC>;
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
impl From<crate::W<FLASHREMAP_OFFSET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLASHREMAP_OFFSET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FLASHREMAP_OFFSET` reader - no description available"]
pub type FLASHREMAP_OFFSET_R = crate::FieldReader<u32, u32>;
#[doc = "Field `FLASHREMAP_OFFSET` writer - no description available"]
pub type FLASHREMAP_OFFSET_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FLASHREMAP_OFFSET_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - no description available"]
    #[inline(always)]
    pub fn flashremap_offset(&self) -> FLASHREMAP_OFFSET_R {
        FLASHREMAP_OFFSET_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - no description available"]
    #[inline(always)]
    pub fn flashremap_offset(&mut self) -> FLASHREMAP_OFFSET_W<0> {
        FLASHREMAP_OFFSET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This 32-bit register contains the offset by which the image is to be remapped. The 12 LSBs are ignored, so the remap granularity is 4KB.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flashremap_offset](index.html) module"]
pub struct FLASHREMAP_OFFSET_SPEC;
impl crate::RegisterSpec for FLASHREMAP_OFFSET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [flashremap_offset::R](R) reader structure"]
impl crate::Readable for FLASHREMAP_OFFSET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [flashremap_offset::W](W) writer structure"]
impl crate::Writable for FLASHREMAP_OFFSET_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FLASHREMAP_OFFSET to value 0"]
impl crate::Resettable for FLASHREMAP_OFFSET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
