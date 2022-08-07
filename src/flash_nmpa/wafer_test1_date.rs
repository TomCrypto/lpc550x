#[doc = "Register `WAFER_TEST1_DATE` reader"]
pub struct R(crate::R<WAFER_TEST1_DATE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WAFER_TEST1_DATE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WAFER_TEST1_DATE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WAFER_TEST1_DATE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WAFER_TEST1_DATE` writer"]
pub struct W(crate::W<WAFER_TEST1_DATE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WAFER_TEST1_DATE_SPEC>;
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
impl From<crate::W<WAFER_TEST1_DATE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WAFER_TEST1_DATE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WT1_DATE` reader - WT1_DATE \\[stored as : year*10000+month*100+day\\]"]
pub type WT1_DATE_R = crate::FieldReader<u32, u32>;
#[doc = "Field `WT1_DATE` writer - WT1_DATE \\[stored as : year*10000+month*100+day\\]"]
pub type WT1_DATE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, WAFER_TEST1_DATE_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - WT1_DATE \\[stored as : year*10000+month*100+day\\]"]
    #[inline(always)]
    pub fn wt1_date(&self) -> WT1_DATE_R {
        WT1_DATE_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - WT1_DATE \\[stored as : year*10000+month*100+day\\]"]
    #[inline(always)]
    pub fn wt1_date(&mut self) -> WT1_DATE_W<0> {
        WT1_DATE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "no description available\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wafer_test1_date](index.html) module"]
pub struct WAFER_TEST1_DATE_SPEC;
impl crate::RegisterSpec for WAFER_TEST1_DATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wafer_test1_date::R](R) reader structure"]
impl crate::Readable for WAFER_TEST1_DATE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wafer_test1_date::W](W) writer structure"]
impl crate::Writable for WAFER_TEST1_DATE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WAFER_TEST1_DATE to value 0"]
impl crate::Resettable for WAFER_TEST1_DATE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
