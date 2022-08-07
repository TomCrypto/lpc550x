#[doc = "Register `WAFER_TEST1_PROGRAM_VERSION` reader"]
pub struct R(crate::R<WAFER_TEST1_PROGRAM_VERSION_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WAFER_TEST1_PROGRAM_VERSION_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WAFER_TEST1_PROGRAM_VERSION_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WAFER_TEST1_PROGRAM_VERSION_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WAFER_TEST1_PROGRAM_VERSION` writer"]
pub struct W(crate::W<WAFER_TEST1_PROGRAM_VERSION_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WAFER_TEST1_PROGRAM_VERSION_SPEC>;
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
impl From<crate::W<WAFER_TEST1_PROGRAM_VERSION_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WAFER_TEST1_PROGRAM_VERSION_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WT1_PROGRAM_VERSION` reader - WT1_PROGRAM_VERSION \\[xx.yy stored as : 100*x+y\\]"]
pub type WT1_PROGRAM_VERSION_R = crate::FieldReader<u32, u32>;
#[doc = "Field `WT1_PROGRAM_VERSION` writer - WT1_PROGRAM_VERSION \\[xx.yy stored as : 100*x+y\\]"]
pub type WT1_PROGRAM_VERSION_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, WAFER_TEST1_PROGRAM_VERSION_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - WT1_PROGRAM_VERSION \\[xx.yy stored as : 100*x+y\\]"]
    #[inline(always)]
    pub fn wt1_program_version(&self) -> WT1_PROGRAM_VERSION_R {
        WT1_PROGRAM_VERSION_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - WT1_PROGRAM_VERSION \\[xx.yy stored as : 100*x+y\\]"]
    #[inline(always)]
    pub fn wt1_program_version(&mut self) -> WT1_PROGRAM_VERSION_W<0> {
        WT1_PROGRAM_VERSION_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "no description available\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wafer_test1_program_version](index.html) module"]
pub struct WAFER_TEST1_PROGRAM_VERSION_SPEC;
impl crate::RegisterSpec for WAFER_TEST1_PROGRAM_VERSION_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wafer_test1_program_version::R](R) reader structure"]
impl crate::Readable for WAFER_TEST1_PROGRAM_VERSION_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wafer_test1_program_version::W](W) writer structure"]
impl crate::Writable for WAFER_TEST1_PROGRAM_VERSION_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WAFER_TEST1_PROGRAM_VERSION to value 0"]
impl crate::Resettable for WAFER_TEST1_PROGRAM_VERSION_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
