#[doc = "Register `REFFASTWKUP` reader"]
pub struct R(crate::R<REFFASTWKUP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<REFFASTWKUP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<REFFASTWKUP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<REFFASTWKUP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `REFFASTWKUP` writer"]
pub struct W(crate::W<REFFASTWKUP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<REFFASTWKUP_SPEC>;
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
impl From<crate::W<REFFASTWKUP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<REFFASTWKUP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LPWKUP` reader - Analog References fast wake-up in case of wake-up from a low power mode (DEEP SLEEP, POWER DOWN and DEEP POWER DOWN): ."]
pub type LPWKUP_R = crate::BitReader<LPWKUP_A>;
#[doc = "Analog References fast wake-up in case of wake-up from a low power mode (DEEP SLEEP, POWER DOWN and DEEP POWER DOWN): .\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPWKUP_A {
    #[doc = "0: Analog References fast wake-up feature is disabled in case of wake-up from any Low power mode."]
    DISABLE = 0,
    #[doc = "1: Analog References fast wake-up feature is enabled in case of wake-up from any Low power mode."]
    ENABLE = 1,
}
impl From<LPWKUP_A> for bool {
    #[inline(always)]
    fn from(variant: LPWKUP_A) -> Self {
        variant as u8 != 0
    }
}
impl LPWKUP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPWKUP_A {
        match self.bits {
            false => LPWKUP_A::DISABLE,
            true => LPWKUP_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == LPWKUP_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == LPWKUP_A::ENABLE
    }
}
#[doc = "Field `LPWKUP` writer - Analog References fast wake-up in case of wake-up from a low power mode (DEEP SLEEP, POWER DOWN and DEEP POWER DOWN): ."]
pub type LPWKUP_W<'a, const O: u8> = crate::BitWriter<'a, u32, REFFASTWKUP_SPEC, LPWKUP_A, O>;
impl<'a, const O: u8> LPWKUP_W<'a, O> {
    #[doc = "Analog References fast wake-up feature is disabled in case of wake-up from any Low power mode."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(LPWKUP_A::DISABLE)
    }
    #[doc = "Analog References fast wake-up feature is enabled in case of wake-up from any Low power mode."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(LPWKUP_A::ENABLE)
    }
}
#[doc = "Field `HWWKUP` reader - Analog References fast wake-up in case of Hardware Pin reset: ."]
pub type HWWKUP_R = crate::BitReader<HWWKUP_A>;
#[doc = "Analog References fast wake-up in case of Hardware Pin reset: .\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HWWKUP_A {
    #[doc = "0: Analog References fast wake-up feature is disabled in case of Hardware Pin reset."]
    DISABLE = 0,
    #[doc = "1: Analog References fast wake-up feature is enabled in case of Hardware Pin reset."]
    ENABLE = 1,
}
impl From<HWWKUP_A> for bool {
    #[inline(always)]
    fn from(variant: HWWKUP_A) -> Self {
        variant as u8 != 0
    }
}
impl HWWKUP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HWWKUP_A {
        match self.bits {
            false => HWWKUP_A::DISABLE,
            true => HWWKUP_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == HWWKUP_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == HWWKUP_A::ENABLE
    }
}
#[doc = "Field `HWWKUP` writer - Analog References fast wake-up in case of Hardware Pin reset: ."]
pub type HWWKUP_W<'a, const O: u8> = crate::BitWriter<'a, u32, REFFASTWKUP_SPEC, HWWKUP_A, O>;
impl<'a, const O: u8> HWWKUP_W<'a, O> {
    #[doc = "Analog References fast wake-up feature is disabled in case of Hardware Pin reset."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(HWWKUP_A::DISABLE)
    }
    #[doc = "Analog References fast wake-up feature is enabled in case of Hardware Pin reset."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(HWWKUP_A::ENABLE)
    }
}
impl R {
    #[doc = "Bit 0 - Analog References fast wake-up in case of wake-up from a low power mode (DEEP SLEEP, POWER DOWN and DEEP POWER DOWN): ."]
    #[inline(always)]
    pub fn lpwkup(&self) -> LPWKUP_R {
        LPWKUP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Analog References fast wake-up in case of Hardware Pin reset: ."]
    #[inline(always)]
    pub fn hwwkup(&self) -> HWWKUP_R {
        HWWKUP_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Analog References fast wake-up in case of wake-up from a low power mode (DEEP SLEEP, POWER DOWN and DEEP POWER DOWN): ."]
    #[inline(always)]
    pub fn lpwkup(&mut self) -> LPWKUP_W<0> {
        LPWKUP_W::new(self)
    }
    #[doc = "Bit 1 - Analog References fast wake-up in case of Hardware Pin reset: ."]
    #[inline(always)]
    pub fn hwwkup(&mut self) -> HWWKUP_W<1> {
        HWWKUP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Analog References fast wake-up Control register \\[Reset by: PoR\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reffastwkup](index.html) module"]
pub struct REFFASTWKUP_SPEC;
impl crate::RegisterSpec for REFFASTWKUP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [reffastwkup::R](R) reader structure"]
impl crate::Readable for REFFASTWKUP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [reffastwkup::W](W) writer structure"]
impl crate::Writable for REFFASTWKUP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets REFFASTWKUP to value 0x01"]
impl crate::Resettable for REFFASTWKUP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
