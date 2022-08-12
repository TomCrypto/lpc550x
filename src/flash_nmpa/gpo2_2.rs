#[doc = "Register `GPO2_2` reader"]
pub struct R(crate::R<GPO2_2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPO2_2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPO2_2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPO2_2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPO2_2` writer"]
pub struct W(crate::W<GPO2_2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPO2_2_SPEC>;
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
impl From<crate::W<GPO2_2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPO2_2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FIELD` reader - no description available."]
pub type FIELD_R = crate::FieldReader<u32, u32>;
#[doc = "Field `FIELD` writer - no description available."]
pub type FIELD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GPO2_2_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - no description available."]
    #[inline(always)]
    pub fn field(&self) -> FIELD_R {
        FIELD_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - no description available."]
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
#[doc = "GPO2 register 2 description.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpo2_2](index.html) module"]
pub struct GPO2_2_SPEC;
impl crate::RegisterSpec for GPO2_2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpo2_2::R](R) reader structure"]
impl crate::Readable for GPO2_2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpo2_2::W](W) writer structure"]
impl crate::Writable for GPO2_2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GPO2_2 to value 0"]
impl crate::Resettable for GPO2_2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
