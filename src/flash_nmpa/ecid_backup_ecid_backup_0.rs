#[doc = "Register `ECID_BACKUP_0` reader"]
pub struct R(crate::R<ECID_BACKUP_ECID_BACKUP_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ECID_BACKUP_ECID_BACKUP_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ECID_BACKUP_ECID_BACKUP_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ECID_BACKUP_ECID_BACKUP_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ECID_BACKUP_0` writer"]
pub struct W(crate::W<ECID_BACKUP_ECID_BACKUP_0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ECID_BACKUP_ECID_BACKUP_0_SPEC>;
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
impl From<crate::W<ECID_BACKUP_ECID_BACKUP_0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ECID_BACKUP_ECID_BACKUP_0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COORD_Y` reader - no description available"]
pub type COORD_Y_R = crate::FieldReader<u16, u16>;
#[doc = "Field `COORD_Y` writer - no description available"]
pub type COORD_Y_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ECID_BACKUP_ECID_BACKUP_0_SPEC, u16, u16, 16, O>;
#[doc = "Field `COORD_X` reader - no description available"]
pub type COORD_X_R = crate::FieldReader<u16, u16>;
#[doc = "Field `COORD_X` writer - no description available"]
pub type COORD_X_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ECID_BACKUP_ECID_BACKUP_0_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - no description available"]
    #[inline(always)]
    pub fn coord_y(&self) -> COORD_Y_R {
        COORD_Y_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - no description available"]
    #[inline(always)]
    pub fn coord_x(&self) -> COORD_X_R {
        COORD_X_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - no description available"]
    #[inline(always)]
    pub fn coord_y(&mut self) -> COORD_Y_W<0> {
        COORD_Y_W::new(self)
    }
    #[doc = "Bits 16:31 - no description available"]
    #[inline(always)]
    pub fn coord_x(&mut self) -> COORD_X_W<16> {
        COORD_X_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "no description available\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ecid_backup_ecid_backup_0](index.html) module"]
pub struct ECID_BACKUP_ECID_BACKUP_0_SPEC;
impl crate::RegisterSpec for ECID_BACKUP_ECID_BACKUP_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ecid_backup_ecid_backup_0::R](R) reader structure"]
impl crate::Readable for ECID_BACKUP_ECID_BACKUP_0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ecid_backup_ecid_backup_0::W](W) writer structure"]
impl crate::Writable for ECID_BACKUP_ECID_BACKUP_0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ECID_BACKUP_0 to value 0"]
impl crate::Resettable for ECID_BACKUP_ECID_BACKUP_0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
