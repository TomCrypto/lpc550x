#[doc = "Register `CODE_BIN_MSB` reader"]
pub struct R(crate::R<CODE_BIN_MSB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CODE_BIN_MSB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CODE_BIN_MSB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CODE_BIN_MSB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CODE_BIN_MSB` reader - Binary converted code (42bits)"]
pub type CODE_BIN_MSB_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:9 - Binary converted code (42bits)"]
    #[inline(always)]
    pub fn code_bin_msb(&self) -> CODE_BIN_MSB_R {
        CODE_BIN_MSB_R::new((self.bits & 0x03ff) as u16)
    }
}
#[doc = "CODE_BIN MSB output Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [code_bin_msb](index.html) module"]
pub struct CODE_BIN_MSB_SPEC;
impl crate::RegisterSpec for CODE_BIN_MSB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [code_bin_msb::R](R) reader structure"]
impl crate::Readable for CODE_BIN_MSB_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CODE_BIN_MSB to value 0"]
impl crate::Resettable for CODE_BIN_MSB_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
