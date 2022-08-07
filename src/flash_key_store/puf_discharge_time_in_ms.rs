#[doc = "Register `puf_discharge_time_in_ms` reader"]
pub struct R(crate::R<PUF_DISCHARGE_TIME_IN_MS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PUF_DISCHARGE_TIME_IN_MS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PUF_DISCHARGE_TIME_IN_MS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PUF_DISCHARGE_TIME_IN_MS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `puf_discharge_time_in_ms` writer"]
pub struct W(crate::W<PUF_DISCHARGE_TIME_IN_MS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PUF_DISCHARGE_TIME_IN_MS_SPEC>;
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
impl From<crate::W<PUF_DISCHARGE_TIME_IN_MS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PUF_DISCHARGE_TIME_IN_MS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FIELD` reader - ."]
pub type FIELD_R = crate::FieldReader<u32, u32>;
#[doc = "Field `FIELD` writer - ."]
pub type FIELD_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PUF_DISCHARGE_TIME_IN_MS_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - ."]
    #[inline(always)]
    pub fn field(&self) -> FIELD_R {
        FIELD_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - ."]
    #[inline(always)]
    pub fn field(&mut self) -> FIELD_W<0> {
        FIELD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "puf discharge time in ms.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [puf_discharge_time_in_ms](index.html) module"]
pub struct PUF_DISCHARGE_TIME_IN_MS_SPEC;
impl crate::RegisterSpec for PUF_DISCHARGE_TIME_IN_MS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [puf_discharge_time_in_ms::R](R) reader structure"]
impl crate::Readable for PUF_DISCHARGE_TIME_IN_MS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [puf_discharge_time_in_ms::W](W) writer structure"]
impl crate::Writable for PUF_DISCHARGE_TIME_IN_MS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets puf_discharge_time_in_ms to value 0"]
impl crate::Resettable for PUF_DISCHARGE_TIME_IN_MS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
