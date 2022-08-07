#[doc = "Register `ECID_BACKUP_1` reader"]
pub struct R(crate::R<ECID_BACKUP_ECID_BACKUP_1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ECID_BACKUP_ECID_BACKUP_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ECID_BACKUP_ECID_BACKUP_1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ECID_BACKUP_ECID_BACKUP_1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ECID_BACKUP_1` writer"]
pub struct W(crate::W<ECID_BACKUP_ECID_BACKUP_1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ECID_BACKUP_ECID_BACKUP_1_SPEC>;
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
impl From<crate::W<ECID_BACKUP_ECID_BACKUP_1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ECID_BACKUP_ECID_BACKUP_1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WAFER` reader - no description available"]
pub type WAFER_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WAFER` writer - no description available"]
pub type WAFER_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ECID_BACKUP_ECID_BACKUP_1_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - no description available"]
    #[inline(always)]
    pub fn wafer(&self) -> WAFER_R {
        WAFER_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - no description available"]
    #[inline(always)]
    pub fn wafer(&mut self) -> WAFER_W<0> {
        WAFER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "no description available\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ecid_backup_ecid_backup_1](index.html) module"]
pub struct ECID_BACKUP_ECID_BACKUP_1_SPEC;
impl crate::RegisterSpec for ECID_BACKUP_ECID_BACKUP_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ecid_backup_ecid_backup_1::R](R) reader structure"]
impl crate::Readable for ECID_BACKUP_ECID_BACKUP_1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ecid_backup_ecid_backup_1::W](W) writer structure"]
impl crate::Writable for ECID_BACKUP_ECID_BACKUP_1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ECID_BACKUP_1 to value 0"]
impl crate::Resettable for ECID_BACKUP_ECID_BACKUP_1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
