#[doc = "Register `STATUS` reader"]
pub struct R(crate::R<STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `BOOTMODE` reader - Latest IC Boot cause:."]
pub type BOOTMODE_R = crate::FieldReader<u8, BOOTMODE_A>;
#[doc = "Latest IC Boot cause:.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BOOTMODE_A {
    #[doc = "0: Latest IC boot was a Full power cycle boot sequence (PoR, Pin Reset, Brown Out Detectors Reset, Software Reset)."]
    POWERUP = 0,
    #[doc = "1: Latest IC boot was from DEEP SLEEP low power mode."]
    DEEPSLEEP = 1,
    #[doc = "2: Latest IC boot was from POWER DOWN low power mode."]
    POWERDOWN = 2,
    #[doc = "3: Latest IC boot was from DEEP POWER DOWN low power mode."]
    DEEPPOWERDOWN = 3,
}
impl From<BOOTMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: BOOTMODE_A) -> Self {
        variant as _
    }
}
impl BOOTMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BOOTMODE_A {
        match self.bits {
            0 => BOOTMODE_A::POWERUP,
            1 => BOOTMODE_A::DEEPSLEEP,
            2 => BOOTMODE_A::POWERDOWN,
            3 => BOOTMODE_A::DEEPPOWERDOWN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `POWERUP`"]
    #[inline(always)]
    pub fn is_powerup(&self) -> bool {
        *self == BOOTMODE_A::POWERUP
    }
    #[doc = "Checks if the value of the field is `DEEPSLEEP`"]
    #[inline(always)]
    pub fn is_deepsleep(&self) -> bool {
        *self == BOOTMODE_A::DEEPSLEEP
    }
    #[doc = "Checks if the value of the field is `POWERDOWN`"]
    #[inline(always)]
    pub fn is_powerdown(&self) -> bool {
        *self == BOOTMODE_A::POWERDOWN
    }
    #[doc = "Checks if the value of the field is `DEEPPOWERDOWN`"]
    #[inline(always)]
    pub fn is_deeppowerdown(&self) -> bool {
        *self == BOOTMODE_A::DEEPPOWERDOWN
    }
}
impl R {
    #[doc = "Bits 18:19 - Latest IC Boot cause:."]
    #[inline(always)]
    pub fn bootmode(&self) -> BOOTMODE_R {
        BOOTMODE_R::new(((self.bits >> 18) & 3) as u8)
    }
}
#[doc = "Power Management Controller FSM (Finite State Machines) status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](index.html) module"]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [status::R](R) reader structure"]
impl crate::Readable for STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
