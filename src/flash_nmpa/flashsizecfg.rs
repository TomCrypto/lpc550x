#[doc = "Register `FLASHSIZECFG` reader"]
pub struct R(crate::R<FLASHSIZECFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLASHSIZECFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLASHSIZECFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLASHSIZECFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FLASHSIZECFG` writer"]
pub struct W(crate::W<FLASHSIZECFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLASHSIZECFG_SPEC>;
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
impl From<crate::W<FLASHSIZECFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLASHSIZECFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FLASH_CONFIGURATION` reader - no description available."]
pub type FLASH_CONFIGURATION_R = crate::FieldReader<u32, u32>;
#[doc = "Field `FLASH_CONFIGURATION` writer - no description available."]
pub type FLASH_CONFIGURATION_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FLASHSIZECFG_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - no description available."]
    #[inline(always)]
    pub fn flash_configuration(&self) -> FLASH_CONFIGURATION_R {
        FLASH_CONFIGURATION_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - no description available."]
    #[inline(always)]
    pub fn flash_configuration(&mut self) -> FLASH_CONFIGURATION_W<0> {
        FLASH_CONFIGURATION_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "no description available.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flashsizecfg](index.html) module"]
pub struct FLASHSIZECFG_SPEC;
impl crate::RegisterSpec for FLASHSIZECFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [flashsizecfg::R](R) reader structure"]
impl crate::Readable for FLASHSIZECFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [flashsizecfg::W](W) writer structure"]
impl crate::Writable for FLASHSIZECFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FLASHSIZECFG to value 0"]
impl crate::Resettable for FLASHSIZECFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
