#[doc = "Register `STATUS2` reader"]
pub struct R(crate::R<STATUS2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATUS2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATUS2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATUS2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `NUMCNTF` reader - Number (of) control faults"]
pub type NUMCNTF_R = crate::FieldReader<u8, u8>;
#[doc = "Field `NUMILLSTF` reader - Number (of) state faults"]
pub type NUMILLSTF_R = crate::FieldReader<u8, u8>;
#[doc = "Field `NUMILLA` reader - Number of (illegal) address faults"]
pub type NUMILLA_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Number (of) control faults"]
    #[inline(always)]
    pub fn numcntf(&self) -> NUMCNTF_R {
        NUMCNTF_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Number (of) state faults"]
    #[inline(always)]
    pub fn numillstf(&self) -> NUMILLSTF_R {
        NUMILLSTF_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Number of (illegal) address faults"]
    #[inline(always)]
    pub fn numilla(&self) -> NUMILLA_R {
        NUMILLA_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
#[doc = "STATUS register (2 of 2)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status2](index.html) module"]
pub struct STATUS2_SPEC;
impl crate::RegisterSpec for STATUS2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [status2::R](R) reader structure"]
impl crate::Readable for STATUS2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STATUS2 to value 0"]
impl crate::Resettable for STATUS2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
