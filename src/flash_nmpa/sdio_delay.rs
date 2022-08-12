#[doc = "Register `SDIO_DELAY` reader"]
pub struct R(crate::R<SDIO_DELAY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SDIO_DELAY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SDIO_DELAY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SDIO_DELAY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SDIO_DELAY` writer"]
pub struct W(crate::W<SDIO_DELAY_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SDIO_DELAY_SPEC>;
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
impl From<crate::W<SDIO_DELAY_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SDIO_DELAY_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SDIO_0_VALID` reader - no description available."]
pub type SDIO_0_VALID_R = crate::BitReader<bool>;
#[doc = "Field `SDIO_0_VALID` writer - no description available."]
pub type SDIO_0_VALID_W<'a, const O: u8> = crate::BitWriter<'a, u32, SDIO_DELAY_SPEC, bool, O>;
#[doc = "Field `SDIO_0_DELAY` reader - SDIO_0_DELAY (unit: 100 ps)."]
pub type SDIO_0_DELAY_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SDIO_0_DELAY` writer - SDIO_0_DELAY (unit: 100 ps)."]
pub type SDIO_0_DELAY_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SDIO_DELAY_SPEC, u16, u16, 10, O>;
impl R {
    #[doc = "Bit 0 - no description available."]
    #[inline(always)]
    pub fn sdio_0_valid(&self) -> SDIO_0_VALID_R {
        SDIO_0_VALID_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:10 - SDIO_0_DELAY (unit: 100 ps)."]
    #[inline(always)]
    pub fn sdio_0_delay(&self) -> SDIO_0_DELAY_R {
        SDIO_0_DELAY_R::new(((self.bits >> 1) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - no description available."]
    #[inline(always)]
    pub fn sdio_0_valid(&mut self) -> SDIO_0_VALID_W<0> {
        SDIO_0_VALID_W::new(self)
    }
    #[doc = "Bits 1:10 - SDIO_0_DELAY (unit: 100 ps)."]
    #[inline(always)]
    pub fn sdio_0_delay(&mut self) -> SDIO_0_DELAY_W<1> {
        SDIO_0_DELAY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "no description available.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdio_delay](index.html) module"]
pub struct SDIO_DELAY_SPEC;
impl crate::RegisterSpec for SDIO_DELAY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sdio_delay::R](R) reader structure"]
impl crate::Readable for SDIO_DELAY_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sdio_delay::W](W) writer structure"]
impl crate::Writable for SDIO_DELAY_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SDIO_DELAY to value 0"]
impl crate::Resettable for SDIO_DELAY_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
