#[doc = "Register `FLASHREMAP_SIZE_DP` reader"]
pub struct R(crate::R<FLASHREMAP_SIZE_DP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLASHREMAP_SIZE_DP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLASHREMAP_SIZE_DP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLASHREMAP_SIZE_DP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FLASHREMAP_SIZE_DP` writer"]
pub struct W(crate::W<FLASHREMAP_SIZE_DP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLASHREMAP_SIZE_DP_SPEC>;
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
impl From<crate::W<FLASHREMAP_SIZE_DP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLASHREMAP_SIZE_DP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FLASHREMAP_SIZE` reader - no description available."]
pub type FLASHREMAP_SIZE_R = crate::FieldReader<u32, u32>;
#[doc = "Field `FLASHREMAP_SIZE` writer - no description available."]
pub type FLASHREMAP_SIZE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FLASHREMAP_SIZE_DP_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - no description available."]
    #[inline(always)]
    pub fn flashremap_size(&self) -> FLASHREMAP_SIZE_R {
        FLASHREMAP_SIZE_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - no description available."]
    #[inline(always)]
    pub fn flashremap_size(&mut self) -> FLASHREMAP_SIZE_W<0> {
        FLASHREMAP_SIZE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This 32-bit register is a duplicate of FLASHREMAPSIZE for increased security.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flashremap_size_dp](index.html) module"]
pub struct FLASHREMAP_SIZE_DP_SPEC;
impl crate::RegisterSpec for FLASHREMAP_SIZE_DP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [flashremap_size_dp::R](R) reader structure"]
impl crate::Readable for FLASHREMAP_SIZE_DP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [flashremap_size_dp::W](W) writer structure"]
impl crate::Writable for FLASHREMAP_SIZE_DP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FLASHREMAP_SIZE_DP to value 0"]
impl crate::Resettable for FLASHREMAP_SIZE_DP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
