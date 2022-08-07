#[doc = "Register `RESETCTRL` reader"]
pub struct R(crate::R<RESETCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RESETCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RESETCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RESETCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RESETCTRL` writer"]
pub struct W(crate::W<RESETCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RESETCTRL_SPEC>;
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
impl From<crate::W<RESETCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RESETCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DPDWAKEUPRESETENABLE` reader - Wake-up from DEEP POWER DOWN reset event (either from wake up I/O or RTC or OS Event Timer)."]
pub type DPDWAKEUPRESETENABLE_R = crate::BitReader<DPDWAKEUPRESETENABLE_A>;
#[doc = "Wake-up from DEEP POWER DOWN reset event (either from wake up I/O or RTC or OS Event Timer).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DPDWAKEUPRESETENABLE_A {
    #[doc = "0: Reset event from DEEP POWER DOWN mode is disable."]
    DISABLE = 0,
    #[doc = "1: Reset event from DEEP POWER DOWN mode is enable."]
    ENABLE = 1,
}
impl From<DPDWAKEUPRESETENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: DPDWAKEUPRESETENABLE_A) -> Self {
        variant as u8 != 0
    }
}
impl DPDWAKEUPRESETENABLE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DPDWAKEUPRESETENABLE_A {
        match self.bits {
            false => DPDWAKEUPRESETENABLE_A::DISABLE,
            true => DPDWAKEUPRESETENABLE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == DPDWAKEUPRESETENABLE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == DPDWAKEUPRESETENABLE_A::ENABLE
    }
}
#[doc = "Field `DPDWAKEUPRESETENABLE` writer - Wake-up from DEEP POWER DOWN reset event (either from wake up I/O or RTC or OS Event Timer)."]
pub type DPDWAKEUPRESETENABLE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RESETCTRL_SPEC, DPDWAKEUPRESETENABLE_A, O>;
impl<'a, const O: u8> DPDWAKEUPRESETENABLE_W<'a, O> {
    #[doc = "Reset event from DEEP POWER DOWN mode is disable."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(DPDWAKEUPRESETENABLE_A::DISABLE)
    }
    #[doc = "Reset event from DEEP POWER DOWN mode is enable."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(DPDWAKEUPRESETENABLE_A::ENABLE)
    }
}
#[doc = "Field `SWRRESETENABLE` reader - Software reset enable."]
pub type SWRRESETENABLE_R = crate::BitReader<SWRRESETENABLE_A>;
#[doc = "Software reset enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWRRESETENABLE_A {
    #[doc = "0: Software reset is disable."]
    DISABLE = 0,
    #[doc = "1: Software reset is enable."]
    ENABLE = 1,
}
impl From<SWRRESETENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: SWRRESETENABLE_A) -> Self {
        variant as u8 != 0
    }
}
impl SWRRESETENABLE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SWRRESETENABLE_A {
        match self.bits {
            false => SWRRESETENABLE_A::DISABLE,
            true => SWRRESETENABLE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == SWRRESETENABLE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == SWRRESETENABLE_A::ENABLE
    }
}
#[doc = "Field `SWRRESETENABLE` writer - Software reset enable."]
pub type SWRRESETENABLE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RESETCTRL_SPEC, SWRRESETENABLE_A, O>;
impl<'a, const O: u8> SWRRESETENABLE_W<'a, O> {
    #[doc = "Software reset is disable."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SWRRESETENABLE_A::DISABLE)
    }
    #[doc = "Software reset is enable."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SWRRESETENABLE_A::ENABLE)
    }
}
#[doc = "Field `BODVBATRESETENA_SECURE` reader - BOD VBAT reset enable."]
pub type BODVBATRESETENA_SECURE_R = crate::FieldReader<u8, BODVBATRESETENA_SECURE_A>;
#[doc = "BOD VBAT reset enable.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BODVBATRESETENA_SECURE_A {
    #[doc = "1: Any other value than b10, BOD VBAT reset is enable."]
    ENABLE = 1,
    #[doc = "2: BOD VBAT reset is disable."]
    DISABLE = 2,
}
impl From<BODVBATRESETENA_SECURE_A> for u8 {
    #[inline(always)]
    fn from(variant: BODVBATRESETENA_SECURE_A) -> Self {
        variant as _
    }
}
impl BODVBATRESETENA_SECURE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<BODVBATRESETENA_SECURE_A> {
        match self.bits {
            1 => Some(BODVBATRESETENA_SECURE_A::ENABLE),
            2 => Some(BODVBATRESETENA_SECURE_A::DISABLE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == BODVBATRESETENA_SECURE_A::ENABLE
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == BODVBATRESETENA_SECURE_A::DISABLE
    }
}
#[doc = "Field `BODVBATRESETENA_SECURE` writer - BOD VBAT reset enable."]
pub type BODVBATRESETENA_SECURE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RESETCTRL_SPEC, u8, BODVBATRESETENA_SECURE_A, 2, O>;
impl<'a, const O: u8> BODVBATRESETENA_SECURE_W<'a, O> {
    #[doc = "Any other value than b10, BOD VBAT reset is enable."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(BODVBATRESETENA_SECURE_A::ENABLE)
    }
    #[doc = "BOD VBAT reset is disable."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(BODVBATRESETENA_SECURE_A::DISABLE)
    }
}
#[doc = "Field `BODCORERESETENA_SECURE` reader - BOD Core reset enable."]
pub type BODCORERESETENA_SECURE_R = crate::FieldReader<u8, BODCORERESETENA_SECURE_A>;
#[doc = "BOD Core reset enable.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BODCORERESETENA_SECURE_A {
    #[doc = "1: Any other value than b10, BOD Core reset is enable."]
    ENABLE = 1,
    #[doc = "2: BOD Core reset is disable."]
    DISABLE = 2,
}
impl From<BODCORERESETENA_SECURE_A> for u8 {
    #[inline(always)]
    fn from(variant: BODCORERESETENA_SECURE_A) -> Self {
        variant as _
    }
}
impl BODCORERESETENA_SECURE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<BODCORERESETENA_SECURE_A> {
        match self.bits {
            1 => Some(BODCORERESETENA_SECURE_A::ENABLE),
            2 => Some(BODCORERESETENA_SECURE_A::DISABLE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == BODCORERESETENA_SECURE_A::ENABLE
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == BODCORERESETENA_SECURE_A::DISABLE
    }
}
#[doc = "Field `BODCORERESETENA_SECURE` writer - BOD Core reset enable."]
pub type BODCORERESETENA_SECURE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RESETCTRL_SPEC, u8, BODCORERESETENA_SECURE_A, 2, O>;
impl<'a, const O: u8> BODCORERESETENA_SECURE_W<'a, O> {
    #[doc = "Any other value than b10, BOD Core reset is enable."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(BODCORERESETENA_SECURE_A::ENABLE)
    }
    #[doc = "BOD Core reset is disable."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(BODCORERESETENA_SECURE_A::DISABLE)
    }
}
#[doc = "Field `BODVBATRESETENA_SECURE_DP` reader - BOD VBAT reset enable."]
pub type BODVBATRESETENA_SECURE_DP_R = crate::FieldReader<u8, BODVBATRESETENA_SECURE_DP_A>;
#[doc = "BOD VBAT reset enable.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BODVBATRESETENA_SECURE_DP_A {
    #[doc = "1: Any other value than b10, BOD VBAT reset is enable."]
    ENABLE = 1,
    #[doc = "2: BOD VBAT reset is disable."]
    DISABLE = 2,
}
impl From<BODVBATRESETENA_SECURE_DP_A> for u8 {
    #[inline(always)]
    fn from(variant: BODVBATRESETENA_SECURE_DP_A) -> Self {
        variant as _
    }
}
impl BODVBATRESETENA_SECURE_DP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<BODVBATRESETENA_SECURE_DP_A> {
        match self.bits {
            1 => Some(BODVBATRESETENA_SECURE_DP_A::ENABLE),
            2 => Some(BODVBATRESETENA_SECURE_DP_A::DISABLE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == BODVBATRESETENA_SECURE_DP_A::ENABLE
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == BODVBATRESETENA_SECURE_DP_A::DISABLE
    }
}
#[doc = "Field `BODVBATRESETENA_SECURE_DP` writer - BOD VBAT reset enable."]
pub type BODVBATRESETENA_SECURE_DP_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RESETCTRL_SPEC, u8, BODVBATRESETENA_SECURE_DP_A, 2, O>;
impl<'a, const O: u8> BODVBATRESETENA_SECURE_DP_W<'a, O> {
    #[doc = "Any other value than b10, BOD VBAT reset is enable."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(BODVBATRESETENA_SECURE_DP_A::ENABLE)
    }
    #[doc = "BOD VBAT reset is disable."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(BODVBATRESETENA_SECURE_DP_A::DISABLE)
    }
}
#[doc = "Field `BODCORERESETENA_SECURE_DP` reader - BOD Core reset enable."]
pub type BODCORERESETENA_SECURE_DP_R = crate::FieldReader<u8, BODCORERESETENA_SECURE_DP_A>;
#[doc = "BOD Core reset enable.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BODCORERESETENA_SECURE_DP_A {
    #[doc = "1: Any other value than b10, BOD Core reset is enable."]
    ENABLE = 1,
    #[doc = "2: BOD Core reset is disable."]
    DISABLE = 2,
}
impl From<BODCORERESETENA_SECURE_DP_A> for u8 {
    #[inline(always)]
    fn from(variant: BODCORERESETENA_SECURE_DP_A) -> Self {
        variant as _
    }
}
impl BODCORERESETENA_SECURE_DP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<BODCORERESETENA_SECURE_DP_A> {
        match self.bits {
            1 => Some(BODCORERESETENA_SECURE_DP_A::ENABLE),
            2 => Some(BODCORERESETENA_SECURE_DP_A::DISABLE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == BODCORERESETENA_SECURE_DP_A::ENABLE
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == BODCORERESETENA_SECURE_DP_A::DISABLE
    }
}
#[doc = "Field `BODCORERESETENA_SECURE_DP` writer - BOD Core reset enable."]
pub type BODCORERESETENA_SECURE_DP_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RESETCTRL_SPEC, u8, BODCORERESETENA_SECURE_DP_A, 2, O>;
impl<'a, const O: u8> BODCORERESETENA_SECURE_DP_W<'a, O> {
    #[doc = "Any other value than b10, BOD Core reset is enable."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(BODCORERESETENA_SECURE_DP_A::ENABLE)
    }
    #[doc = "BOD Core reset is disable."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(BODCORERESETENA_SECURE_DP_A::DISABLE)
    }
}
impl R {
    #[doc = "Bit 0 - Wake-up from DEEP POWER DOWN reset event (either from wake up I/O or RTC or OS Event Timer)."]
    #[inline(always)]
    pub fn dpdwakeupresetenable(&self) -> DPDWAKEUPRESETENABLE_R {
        DPDWAKEUPRESETENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 3 - Software reset enable."]
    #[inline(always)]
    pub fn swrresetenable(&self) -> SWRRESETENABLE_R {
        SWRRESETENABLE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - BOD VBAT reset enable."]
    #[inline(always)]
    pub fn bodvbatresetena_secure(&self) -> BODVBATRESETENA_SECURE_R {
        BODVBATRESETENA_SECURE_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - BOD Core reset enable."]
    #[inline(always)]
    pub fn bodcoreresetena_secure(&self) -> BODCORERESETENA_SECURE_R {
        BODCORERESETENA_SECURE_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 28:29 - BOD VBAT reset enable."]
    #[inline(always)]
    pub fn bodvbatresetena_secure_dp(&self) -> BODVBATRESETENA_SECURE_DP_R {
        BODVBATRESETENA_SECURE_DP_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - BOD Core reset enable."]
    #[inline(always)]
    pub fn bodcoreresetena_secure_dp(&self) -> BODCORERESETENA_SECURE_DP_R {
        BODCORERESETENA_SECURE_DP_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Wake-up from DEEP POWER DOWN reset event (either from wake up I/O or RTC or OS Event Timer)."]
    #[inline(always)]
    pub fn dpdwakeupresetenable(&mut self) -> DPDWAKEUPRESETENABLE_W<0> {
        DPDWAKEUPRESETENABLE_W::new(self)
    }
    #[doc = "Bit 3 - Software reset enable."]
    #[inline(always)]
    pub fn swrresetenable(&mut self) -> SWRRESETENABLE_W<3> {
        SWRRESETENABLE_W::new(self)
    }
    #[doc = "Bits 4:5 - BOD VBAT reset enable."]
    #[inline(always)]
    pub fn bodvbatresetena_secure(&mut self) -> BODVBATRESETENA_SECURE_W<4> {
        BODVBATRESETENA_SECURE_W::new(self)
    }
    #[doc = "Bits 6:7 - BOD Core reset enable."]
    #[inline(always)]
    pub fn bodcoreresetena_secure(&mut self) -> BODCORERESETENA_SECURE_W<6> {
        BODCORERESETENA_SECURE_W::new(self)
    }
    #[doc = "Bits 28:29 - BOD VBAT reset enable."]
    #[inline(always)]
    pub fn bodvbatresetena_secure_dp(&mut self) -> BODVBATRESETENA_SECURE_DP_W<28> {
        BODVBATRESETENA_SECURE_DP_W::new(self)
    }
    #[doc = "Bits 30:31 - BOD Core reset enable."]
    #[inline(always)]
    pub fn bodcoreresetena_secure_dp(&mut self) -> BODCORERESETENA_SECURE_DP_W<30> {
        BODCORERESETENA_SECURE_DP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Reset Control \\[Reset by: PoR, Pin Reset, Brown Out Detectors Reset, Deep Power Down Reset, Software Reset\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [resetctrl](index.html) module"]
pub struct RESETCTRL_SPEC;
impl crate::RegisterSpec for RESETCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [resetctrl::R](R) reader structure"]
impl crate::Readable for RESETCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [resetctrl::W](W) writer structure"]
impl crate::Writable for RESETCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RESETCTRL to value 0x5000_0050"]
impl crate::Resettable for RESETCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x5000_0050
    }
}
