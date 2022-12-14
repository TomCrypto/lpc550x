#[doc = "Register `ECID_BACKUP_2` reader"]
pub struct R(crate::R<ECID_BACKUP_2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ECID_BACKUP_2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ECID_BACKUP_2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ECID_BACKUP_2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ECID_BACKUP_2` writer"]
pub struct W(crate::W<ECID_BACKUP_2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ECID_BACKUP_2_SPEC>;
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
impl From<crate::W<ECID_BACKUP_2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ECID_BACKUP_2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LOTID_LSB` reader - no description available."]
pub type LOTID_LSB_R = crate::FieldReader<u32, u32>;
#[doc = "Field `LOTID_LSB` writer - no description available."]
pub type LOTID_LSB_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ECID_BACKUP_2_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - no description available."]
    #[inline(always)]
    pub fn lotid_lsb(&self) -> LOTID_LSB_R {
        LOTID_LSB_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - no description available."]
    #[inline(always)]
    pub fn lotid_lsb(&mut self) -> LOTID_LSB_W<0> {
        LOTID_LSB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "no description available.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ecid_backup_2](index.html) module"]
pub struct ECID_BACKUP_2_SPEC;
impl crate::RegisterSpec for ECID_BACKUP_2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ecid_backup_2::R](R) reader structure"]
impl crate::Readable for ECID_BACKUP_2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ecid_backup_2::W](W) writer structure"]
impl crate::Writable for ECID_BACKUP_2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ECID_BACKUP_2 to value 0"]
impl crate::Resettable for ECID_BACKUP_2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
