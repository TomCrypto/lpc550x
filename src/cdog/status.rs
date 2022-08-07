#[doc = "Register `STATUS` reader"]
pub struct R(crate::R<STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `NUMTOF` reader - Number of Timeout Faults"]
pub type NUMTOF_R = crate::FieldReader<u8, u8>;
#[doc = "Field `NUMMISCOMPF` reader - Number of Miscompare Faults"]
pub type NUMMISCOMPF_R = crate::FieldReader<u8, u8>;
#[doc = "Field `NUMILSEQF` reader - Number of illegal sequence faults"]
pub type NUMILSEQF_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CURST` reader - Current State"]
pub type CURST_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Number of Timeout Faults"]
    #[inline(always)]
    pub fn numtof(&self) -> NUMTOF_R {
        NUMTOF_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Number of Miscompare Faults"]
    #[inline(always)]
    pub fn nummiscompf(&self) -> NUMMISCOMPF_R {
        NUMMISCOMPF_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Number of illegal sequence faults"]
    #[inline(always)]
    pub fn numilseqf(&self) -> NUMILSEQF_R {
        NUMILSEQF_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:27 - Current State"]
    #[inline(always)]
    pub fn curst(&self) -> CURST_R {
        CURST_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
#[doc = "Status register (1 of 2)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](index.html) module"]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [status::R](R) reader structure"]
impl crate::Readable for STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
