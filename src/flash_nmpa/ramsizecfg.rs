#[doc = "Register `RAMSIZECFG` reader"]
pub struct R(crate::R<RAMSIZECFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RAMSIZECFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RAMSIZECFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RAMSIZECFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RAMSIZECFG` writer"]
pub struct W(crate::W<RAMSIZECFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RAMSIZECFG_SPEC>;
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
impl From<crate::W<RAMSIZECFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RAMSIZECFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SRAM_CONFIGURATION` reader - no description available"]
pub type SRAM_CONFIGURATION_R = crate::FieldReader<u32, u32>;
#[doc = "Field `SRAM_CONFIGURATION` writer - no description available"]
pub type SRAM_CONFIGURATION_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RAMSIZECFG_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - no description available"]
    #[inline(always)]
    pub fn sram_configuration(&self) -> SRAM_CONFIGURATION_R {
        SRAM_CONFIGURATION_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - no description available"]
    #[inline(always)]
    pub fn sram_configuration(&mut self) -> SRAM_CONFIGURATION_W<0> {
        SRAM_CONFIGURATION_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "no description available\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ramsizecfg](index.html) module"]
pub struct RAMSIZECFG_SPEC;
impl crate::RegisterSpec for RAMSIZECFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ramsizecfg::R](R) reader structure"]
impl crate::Readable for RAMSIZECFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ramsizecfg::W](W) writer structure"]
impl crate::Writable for RAMSIZECFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RAMSIZECFG to value 0"]
impl crate::Resettable for RAMSIZECFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
