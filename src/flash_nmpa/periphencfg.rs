#[doc = "Register `PERIPHENCFG` reader"]
pub struct R(crate::R<PERIPHENCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PERIPHENCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PERIPHENCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PERIPHENCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PERIPHENCFG` writer"]
pub struct W(crate::W<PERIPHENCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PERIPHENCFG_SPEC>;
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
impl From<crate::W<PERIPHENCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PERIPHENCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PERIPHERAL_CONFIGURATION` reader - no description available"]
pub type PERIPHERAL_CONFIGURATION_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PERIPHERAL_CONFIGURATION` writer - no description available"]
pub type PERIPHERAL_CONFIGURATION_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PERIPHENCFG_SPEC, u16, u16, 16, O>;
#[doc = "Field `CPU1_ENABLE` reader - no description available"]
pub type CPU1_ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `CPU1_ENABLE` writer - no description available"]
pub type CPU1_ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, PERIPHENCFG_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:15 - no description available"]
    #[inline(always)]
    pub fn peripheral_configuration(&self) -> PERIPHERAL_CONFIGURATION_R {
        PERIPHERAL_CONFIGURATION_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 31 - no description available"]
    #[inline(always)]
    pub fn cpu1_enable(&self) -> CPU1_ENABLE_R {
        CPU1_ENABLE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - no description available"]
    #[inline(always)]
    pub fn peripheral_configuration(&mut self) -> PERIPHERAL_CONFIGURATION_W<0> {
        PERIPHERAL_CONFIGURATION_W::new(self)
    }
    #[doc = "Bit 31 - no description available"]
    #[inline(always)]
    pub fn cpu1_enable(&mut self) -> CPU1_ENABLE_W<31> {
        CPU1_ENABLE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "no description available\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [periphencfg](index.html) module"]
pub struct PERIPHENCFG_SPEC;
impl crate::RegisterSpec for PERIPHENCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [periphencfg::R](R) reader structure"]
impl crate::Readable for PERIPHENCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [periphencfg::W](W) writer structure"]
impl crate::Writable for PERIPHENCFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PERIPHENCFG to value 0"]
impl crate::Resettable for PERIPHENCFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
