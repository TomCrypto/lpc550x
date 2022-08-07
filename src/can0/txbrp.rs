#[doc = "Register `TXBRP` reader"]
pub struct R(crate::R<TXBRP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TXBRP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TXBRP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TXBRP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TRP` reader - Transmission request pending."]
pub type TRP_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Transmission request pending."]
    #[inline(always)]
    pub fn trp(&self) -> TRP_R {
        TRP_R::new(self.bits)
    }
}
#[doc = "Tx Buffer Request Pending\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txbrp](index.html) module"]
pub struct TXBRP_SPEC;
impl crate::RegisterSpec for TXBRP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [txbrp::R](R) reader structure"]
impl crate::Readable for TXBRP_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TXBRP to value 0"]
impl crate::Resettable for TXBRP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
