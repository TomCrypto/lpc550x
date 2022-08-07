#[doc = "Register `CODE_BIN_LSB` reader"]
pub struct R(crate::R<CODE_BIN_LSB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CODE_BIN_LSB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CODE_BIN_LSB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CODE_BIN_LSB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CODE_BIN_LSB` reader - Binary converted code (42bits)"]
pub type CODE_BIN_LSB_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Binary converted code (42bits)"]
    #[inline(always)]
    pub fn code_bin_lsb(&self) -> CODE_BIN_LSB_R {
        CODE_BIN_LSB_R::new(self.bits)
    }
}
#[doc = "CODE_BIN LSB output Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [code_bin_lsb](index.html) module"]
pub struct CODE_BIN_LSB_SPEC;
impl crate::RegisterSpec for CODE_BIN_LSB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [code_bin_lsb::R](R) reader structure"]
impl crate::Readable for CODE_BIN_LSB_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CODE_BIN_LSB to value 0"]
impl crate::Resettable for CODE_BIN_LSB_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
