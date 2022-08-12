#[doc = "Register `TXEFS` reader"]
pub struct R(crate::R<TXEFS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TXEFS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TXEFS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TXEFS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `EFFL` reader - Event FIFO fill level."]
pub type EFFL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EFGI` reader - Event FIFO get index."]
pub type EFGI_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EFPI` reader - Event FIFO put index."]
pub type EFPI_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EFF` reader - Event FIFO full."]
pub type EFF_R = crate::BitReader<bool>;
#[doc = "Field `TEFL` reader - Tx event FIFO element lost."]
pub type TEFL_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:5 - Event FIFO fill level."]
    #[inline(always)]
    pub fn effl(&self) -> EFFL_R {
        EFFL_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:12 - Event FIFO get index."]
    #[inline(always)]
    pub fn efgi(&self) -> EFGI_R {
        EFGI_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:21 - Event FIFO put index."]
    #[inline(always)]
    pub fn efpi(&self) -> EFPI_R {
        EFPI_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bit 24 - Event FIFO full."]
    #[inline(always)]
    pub fn eff(&self) -> EFF_R {
        EFF_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Tx event FIFO element lost."]
    #[inline(always)]
    pub fn tefl(&self) -> TEFL_R {
        TEFL_R::new(((self.bits >> 25) & 1) != 0)
    }
}
#[doc = "Tx Event FIFO Status.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txefs](index.html) module"]
pub struct TXEFS_SPEC;
impl crate::RegisterSpec for TXEFS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [txefs::R](R) reader structure"]
impl crate::Readable for TXEFS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TXEFS to value 0"]
impl crate::Resettable for TXEFS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
