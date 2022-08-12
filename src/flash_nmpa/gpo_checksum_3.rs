#[doc = "Register `GPO_CHECKSUM_3` reader"]
pub struct R(crate::R<GPO_CHECKSUM_3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPO_CHECKSUM_3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPO_CHECKSUM_3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPO_CHECKSUM_3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPO_CHECKSUM_3` writer"]
pub struct W(crate::W<GPO_CHECKSUM_3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPO_CHECKSUM_3_SPEC>;
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
impl From<crate::W<GPO_CHECKSUM_3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPO_CHECKSUM_3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FIELD` reader - no description available."]
pub type FIELD_R = crate::FieldReader<u32, u32>;
#[doc = "Field `FIELD` writer - no description available."]
pub type FIELD_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GPO_CHECKSUM_3_SPEC, u32, u32, 32, O>;
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
#[doc = "checksum of the GPO data in words 3.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpo_checksum_3](index.html) module"]
pub struct GPO_CHECKSUM_3_SPEC;
impl crate::RegisterSpec for GPO_CHECKSUM_3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpo_checksum_3::R](R) reader structure"]
impl crate::Readable for GPO_CHECKSUM_3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpo_checksum_3::W](W) writer structure"]
impl crate::Writable for GPO_CHECKSUM_3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GPO_CHECKSUM_3 to value 0"]
impl crate::Resettable for GPO_CHECKSUM_3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
