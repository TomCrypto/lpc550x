#[doc = "Register `WAKEUPIOCTRL` reader"]
pub struct R(crate::R<WAKEUPIOCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WAKEUPIOCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WAKEUPIOCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WAKEUPIOCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WAKEUPIOCTRL` writer"]
pub struct W(crate::W<WAKEUPIOCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WAKEUPIOCTRL_SPEC>;
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
impl From<crate::W<WAKEUPIOCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WAKEUPIOCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RISINGEDGEWAKEUP0` reader - Enable / disable detection of rising edge events on Wake Up 0 pin in Deep Power Down modes:."]
pub type RISINGEDGEWAKEUP0_R = crate::BitReader<RISINGEDGEWAKEUP0_A>;
#[doc = "Enable / disable detection of rising edge events on Wake Up 0 pin in Deep Power Down modes:.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RISINGEDGEWAKEUP0_A {
    #[doc = "0: Rising edge detection is disable."]
    DISABLE = 0,
    #[doc = "1: Rising edge detection is enable."]
    ENABLE = 1,
}
impl From<RISINGEDGEWAKEUP0_A> for bool {
    #[inline(always)]
    fn from(variant: RISINGEDGEWAKEUP0_A) -> Self {
        variant as u8 != 0
    }
}
impl RISINGEDGEWAKEUP0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RISINGEDGEWAKEUP0_A {
        match self.bits {
            false => RISINGEDGEWAKEUP0_A::DISABLE,
            true => RISINGEDGEWAKEUP0_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == RISINGEDGEWAKEUP0_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == RISINGEDGEWAKEUP0_A::ENABLE
    }
}
#[doc = "Field `RISINGEDGEWAKEUP0` writer - Enable / disable detection of rising edge events on Wake Up 0 pin in Deep Power Down modes:."]
pub type RISINGEDGEWAKEUP0_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, WAKEUPIOCTRL_SPEC, RISINGEDGEWAKEUP0_A, O>;
impl<'a, const O: u8> RISINGEDGEWAKEUP0_W<'a, O> {
    #[doc = "Rising edge detection is disable."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(RISINGEDGEWAKEUP0_A::DISABLE)
    }
    #[doc = "Rising edge detection is enable."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(RISINGEDGEWAKEUP0_A::ENABLE)
    }
}
#[doc = "Field `FALLINGEDGEWAKEUP0` reader - Enable / disable detection of falling edge events on Wake Up 0 pin in Deep Power Down modes:."]
pub type FALLINGEDGEWAKEUP0_R = crate::BitReader<FALLINGEDGEWAKEUP0_A>;
#[doc = "Enable / disable detection of falling edge events on Wake Up 0 pin in Deep Power Down modes:.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FALLINGEDGEWAKEUP0_A {
    #[doc = "0: Falling edge detection is disable."]
    DISABLE = 0,
    #[doc = "1: Falling edge detection is enable."]
    ENABLE = 1,
}
impl From<FALLINGEDGEWAKEUP0_A> for bool {
    #[inline(always)]
    fn from(variant: FALLINGEDGEWAKEUP0_A) -> Self {
        variant as u8 != 0
    }
}
impl FALLINGEDGEWAKEUP0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FALLINGEDGEWAKEUP0_A {
        match self.bits {
            false => FALLINGEDGEWAKEUP0_A::DISABLE,
            true => FALLINGEDGEWAKEUP0_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == FALLINGEDGEWAKEUP0_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == FALLINGEDGEWAKEUP0_A::ENABLE
    }
}
#[doc = "Field `FALLINGEDGEWAKEUP0` writer - Enable / disable detection of falling edge events on Wake Up 0 pin in Deep Power Down modes:."]
pub type FALLINGEDGEWAKEUP0_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, WAKEUPIOCTRL_SPEC, FALLINGEDGEWAKEUP0_A, O>;
impl<'a, const O: u8> FALLINGEDGEWAKEUP0_W<'a, O> {
    #[doc = "Falling edge detection is disable."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(FALLINGEDGEWAKEUP0_A::DISABLE)
    }
    #[doc = "Falling edge detection is enable."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(FALLINGEDGEWAKEUP0_A::ENABLE)
    }
}
#[doc = "Field `RISINGEDGEWAKEUP1` reader - Enable / disable detection of rising edge events on Wake Up 1 pin in Deep Power Down modes:."]
pub type RISINGEDGEWAKEUP1_R = crate::BitReader<RISINGEDGEWAKEUP1_A>;
#[doc = "Enable / disable detection of rising edge events on Wake Up 1 pin in Deep Power Down modes:.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RISINGEDGEWAKEUP1_A {
    #[doc = "0: Rising edge detection is disable."]
    DISABLE = 0,
    #[doc = "1: Rising edge detection is enable."]
    ENABLE = 1,
}
impl From<RISINGEDGEWAKEUP1_A> for bool {
    #[inline(always)]
    fn from(variant: RISINGEDGEWAKEUP1_A) -> Self {
        variant as u8 != 0
    }
}
impl RISINGEDGEWAKEUP1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RISINGEDGEWAKEUP1_A {
        match self.bits {
            false => RISINGEDGEWAKEUP1_A::DISABLE,
            true => RISINGEDGEWAKEUP1_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == RISINGEDGEWAKEUP1_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == RISINGEDGEWAKEUP1_A::ENABLE
    }
}
#[doc = "Field `RISINGEDGEWAKEUP1` writer - Enable / disable detection of rising edge events on Wake Up 1 pin in Deep Power Down modes:."]
pub type RISINGEDGEWAKEUP1_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, WAKEUPIOCTRL_SPEC, RISINGEDGEWAKEUP1_A, O>;
impl<'a, const O: u8> RISINGEDGEWAKEUP1_W<'a, O> {
    #[doc = "Rising edge detection is disable."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(RISINGEDGEWAKEUP1_A::DISABLE)
    }
    #[doc = "Rising edge detection is enable."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(RISINGEDGEWAKEUP1_A::ENABLE)
    }
}
#[doc = "Field `FALLINGEDGEWAKEUP1` reader - Enable / disable detection of falling edge events on Wake Up 1 pin in Deep Power Down modes:."]
pub type FALLINGEDGEWAKEUP1_R = crate::BitReader<FALLINGEDGEWAKEUP1_A>;
#[doc = "Enable / disable detection of falling edge events on Wake Up 1 pin in Deep Power Down modes:.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FALLINGEDGEWAKEUP1_A {
    #[doc = "0: Falling edge detection is disable."]
    DISABLE = 0,
    #[doc = "1: Falling edge detection is enable."]
    ENABLE = 1,
}
impl From<FALLINGEDGEWAKEUP1_A> for bool {
    #[inline(always)]
    fn from(variant: FALLINGEDGEWAKEUP1_A) -> Self {
        variant as u8 != 0
    }
}
impl FALLINGEDGEWAKEUP1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FALLINGEDGEWAKEUP1_A {
        match self.bits {
            false => FALLINGEDGEWAKEUP1_A::DISABLE,
            true => FALLINGEDGEWAKEUP1_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == FALLINGEDGEWAKEUP1_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == FALLINGEDGEWAKEUP1_A::ENABLE
    }
}
#[doc = "Field `FALLINGEDGEWAKEUP1` writer - Enable / disable detection of falling edge events on Wake Up 1 pin in Deep Power Down modes:."]
pub type FALLINGEDGEWAKEUP1_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, WAKEUPIOCTRL_SPEC, FALLINGEDGEWAKEUP1_A, O>;
impl<'a, const O: u8> FALLINGEDGEWAKEUP1_W<'a, O> {
    #[doc = "Falling edge detection is disable."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(FALLINGEDGEWAKEUP1_A::DISABLE)
    }
    #[doc = "Falling edge detection is enable."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(FALLINGEDGEWAKEUP1_A::ENABLE)
    }
}
#[doc = "Field `RISINGEDGEWAKEUP2` reader - Enable / disable detection of rising edge events on Wake Up 2 pin in Deep Power Down modes:."]
pub type RISINGEDGEWAKEUP2_R = crate::BitReader<RISINGEDGEWAKEUP2_A>;
#[doc = "Enable / disable detection of rising edge events on Wake Up 2 pin in Deep Power Down modes:.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RISINGEDGEWAKEUP2_A {
    #[doc = "0: Rising edge detection is disable."]
    DISABLE = 0,
    #[doc = "1: Rising edge detection is enable."]
    ENABLE = 1,
}
impl From<RISINGEDGEWAKEUP2_A> for bool {
    #[inline(always)]
    fn from(variant: RISINGEDGEWAKEUP2_A) -> Self {
        variant as u8 != 0
    }
}
impl RISINGEDGEWAKEUP2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RISINGEDGEWAKEUP2_A {
        match self.bits {
            false => RISINGEDGEWAKEUP2_A::DISABLE,
            true => RISINGEDGEWAKEUP2_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == RISINGEDGEWAKEUP2_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == RISINGEDGEWAKEUP2_A::ENABLE
    }
}
#[doc = "Field `RISINGEDGEWAKEUP2` writer - Enable / disable detection of rising edge events on Wake Up 2 pin in Deep Power Down modes:."]
pub type RISINGEDGEWAKEUP2_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, WAKEUPIOCTRL_SPEC, RISINGEDGEWAKEUP2_A, O>;
impl<'a, const O: u8> RISINGEDGEWAKEUP2_W<'a, O> {
    #[doc = "Rising edge detection is disable."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(RISINGEDGEWAKEUP2_A::DISABLE)
    }
    #[doc = "Rising edge detection is enable."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(RISINGEDGEWAKEUP2_A::ENABLE)
    }
}
#[doc = "Field `FALLINGEDGEWAKEUP2` reader - Enable / disable detection of falling edge events on Wake Up 2 pin in Deep Power Down modes:."]
pub type FALLINGEDGEWAKEUP2_R = crate::BitReader<FALLINGEDGEWAKEUP2_A>;
#[doc = "Enable / disable detection of falling edge events on Wake Up 2 pin in Deep Power Down modes:.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FALLINGEDGEWAKEUP2_A {
    #[doc = "0: Falling edge detection is disable."]
    DISABLE = 0,
    #[doc = "1: Falling edge detection is enable."]
    ENABLE = 1,
}
impl From<FALLINGEDGEWAKEUP2_A> for bool {
    #[inline(always)]
    fn from(variant: FALLINGEDGEWAKEUP2_A) -> Self {
        variant as u8 != 0
    }
}
impl FALLINGEDGEWAKEUP2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FALLINGEDGEWAKEUP2_A {
        match self.bits {
            false => FALLINGEDGEWAKEUP2_A::DISABLE,
            true => FALLINGEDGEWAKEUP2_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == FALLINGEDGEWAKEUP2_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == FALLINGEDGEWAKEUP2_A::ENABLE
    }
}
#[doc = "Field `FALLINGEDGEWAKEUP2` writer - Enable / disable detection of falling edge events on Wake Up 2 pin in Deep Power Down modes:."]
pub type FALLINGEDGEWAKEUP2_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, WAKEUPIOCTRL_SPEC, FALLINGEDGEWAKEUP2_A, O>;
impl<'a, const O: u8> FALLINGEDGEWAKEUP2_W<'a, O> {
    #[doc = "Falling edge detection is disable."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(FALLINGEDGEWAKEUP2_A::DISABLE)
    }
    #[doc = "Falling edge detection is enable."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(FALLINGEDGEWAKEUP2_A::ENABLE)
    }
}
#[doc = "Field `RISINGEDGEWAKEUP3` reader - Enable / disable detection of rising edge events on Wake Up 3 pin in Deep Power Down modes:."]
pub type RISINGEDGEWAKEUP3_R = crate::BitReader<RISINGEDGEWAKEUP3_A>;
#[doc = "Enable / disable detection of rising edge events on Wake Up 3 pin in Deep Power Down modes:.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RISINGEDGEWAKEUP3_A {
    #[doc = "0: Rising edge detection is disable."]
    DISABLE = 0,
    #[doc = "1: Rising edge detection is enable."]
    ENABLE = 1,
}
impl From<RISINGEDGEWAKEUP3_A> for bool {
    #[inline(always)]
    fn from(variant: RISINGEDGEWAKEUP3_A) -> Self {
        variant as u8 != 0
    }
}
impl RISINGEDGEWAKEUP3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RISINGEDGEWAKEUP3_A {
        match self.bits {
            false => RISINGEDGEWAKEUP3_A::DISABLE,
            true => RISINGEDGEWAKEUP3_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == RISINGEDGEWAKEUP3_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == RISINGEDGEWAKEUP3_A::ENABLE
    }
}
#[doc = "Field `RISINGEDGEWAKEUP3` writer - Enable / disable detection of rising edge events on Wake Up 3 pin in Deep Power Down modes:."]
pub type RISINGEDGEWAKEUP3_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, WAKEUPIOCTRL_SPEC, RISINGEDGEWAKEUP3_A, O>;
impl<'a, const O: u8> RISINGEDGEWAKEUP3_W<'a, O> {
    #[doc = "Rising edge detection is disable."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(RISINGEDGEWAKEUP3_A::DISABLE)
    }
    #[doc = "Rising edge detection is enable."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(RISINGEDGEWAKEUP3_A::ENABLE)
    }
}
#[doc = "Field `FALLINGEDGEWAKEUP3` reader - Enable / disable detection of falling edge events on Wake Up 3 pin in Deep Power Down modes:."]
pub type FALLINGEDGEWAKEUP3_R = crate::BitReader<FALLINGEDGEWAKEUP3_A>;
#[doc = "Enable / disable detection of falling edge events on Wake Up 3 pin in Deep Power Down modes:.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FALLINGEDGEWAKEUP3_A {
    #[doc = "0: Falling edge detection is disable."]
    DISABLE = 0,
    #[doc = "1: Falling edge detection is enable."]
    ENABLE = 1,
}
impl From<FALLINGEDGEWAKEUP3_A> for bool {
    #[inline(always)]
    fn from(variant: FALLINGEDGEWAKEUP3_A) -> Self {
        variant as u8 != 0
    }
}
impl FALLINGEDGEWAKEUP3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FALLINGEDGEWAKEUP3_A {
        match self.bits {
            false => FALLINGEDGEWAKEUP3_A::DISABLE,
            true => FALLINGEDGEWAKEUP3_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == FALLINGEDGEWAKEUP3_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == FALLINGEDGEWAKEUP3_A::ENABLE
    }
}
#[doc = "Field `FALLINGEDGEWAKEUP3` writer - Enable / disable detection of falling edge events on Wake Up 3 pin in Deep Power Down modes:."]
pub type FALLINGEDGEWAKEUP3_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, WAKEUPIOCTRL_SPEC, FALLINGEDGEWAKEUP3_A, O>;
impl<'a, const O: u8> FALLINGEDGEWAKEUP3_W<'a, O> {
    #[doc = "Falling edge detection is disable."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(FALLINGEDGEWAKEUP3_A::DISABLE)
    }
    #[doc = "Falling edge detection is enable."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(FALLINGEDGEWAKEUP3_A::ENABLE)
    }
}
#[doc = "Field `MODEWAKEUPIOPAD0` reader - Selects function mode (on-chip pull-up/pull-down resistor control)."]
pub type MODEWAKEUPIOPAD0_R = crate::FieldReader<u8, MODEWAKEUPIOPAD0_A>;
#[doc = "Selects function mode (on-chip pull-up/pull-down resistor control).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODEWAKEUPIOPAD0_A {
    #[doc = "0: Inactive. Inactive (no pull-down/pull-up resistor enabled)."]
    INACTIVE = 0,
    #[doc = "1: Pull-down. Pull-down resistor enabled."]
    PULL_DOWN = 1,
    #[doc = "2: Pull-up. Pull-up resistor enabled."]
    PULL_UP = 2,
    #[doc = "3: Repeater. Repeater mode."]
    REPEATER = 3,
}
impl From<MODEWAKEUPIOPAD0_A> for u8 {
    #[inline(always)]
    fn from(variant: MODEWAKEUPIOPAD0_A) -> Self {
        variant as _
    }
}
impl MODEWAKEUPIOPAD0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODEWAKEUPIOPAD0_A {
        match self.bits {
            0 => MODEWAKEUPIOPAD0_A::INACTIVE,
            1 => MODEWAKEUPIOPAD0_A::PULL_DOWN,
            2 => MODEWAKEUPIOPAD0_A::PULL_UP,
            3 => MODEWAKEUPIOPAD0_A::REPEATER,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `INACTIVE`"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == MODEWAKEUPIOPAD0_A::INACTIVE
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == MODEWAKEUPIOPAD0_A::PULL_DOWN
    }
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == MODEWAKEUPIOPAD0_A::PULL_UP
    }
    #[doc = "Checks if the value of the field is `REPEATER`"]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == MODEWAKEUPIOPAD0_A::REPEATER
    }
}
#[doc = "Field `MODEWAKEUPIOPAD0` writer - Selects function mode (on-chip pull-up/pull-down resistor control)."]
pub type MODEWAKEUPIOPAD0_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, WAKEUPIOCTRL_SPEC, u8, MODEWAKEUPIOPAD0_A, 2, O>;
impl<'a, const O: u8> MODEWAKEUPIOPAD0_W<'a, O> {
    #[doc = "Inactive. Inactive (no pull-down/pull-up resistor enabled)."]
    #[inline(always)]
    pub fn inactive(self) -> &'a mut W {
        self.variant(MODEWAKEUPIOPAD0_A::INACTIVE)
    }
    #[doc = "Pull-down. Pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(MODEWAKEUPIOPAD0_A::PULL_DOWN)
    }
    #[doc = "Pull-up. Pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(MODEWAKEUPIOPAD0_A::PULL_UP)
    }
    #[doc = "Repeater. Repeater mode."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut W {
        self.variant(MODEWAKEUPIOPAD0_A::REPEATER)
    }
}
#[doc = "Field `MODEWAKEUPIOPAD1` reader - Selects function mode (on-chip pull-up/pull-down resistor control)."]
pub type MODEWAKEUPIOPAD1_R = crate::FieldReader<u8, MODEWAKEUPIOPAD1_A>;
#[doc = "Selects function mode (on-chip pull-up/pull-down resistor control).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODEWAKEUPIOPAD1_A {
    #[doc = "0: Inactive. Inactive (no pull-down/pull-up resistor enabled)."]
    INACTIVE = 0,
    #[doc = "1: Pull-down. Pull-down resistor enabled."]
    PULL_DOWN = 1,
    #[doc = "2: Pull-up. Pull-up resistor enabled."]
    PULL_UP = 2,
    #[doc = "3: Repeater. Repeater mode."]
    REPEATER = 3,
}
impl From<MODEWAKEUPIOPAD1_A> for u8 {
    #[inline(always)]
    fn from(variant: MODEWAKEUPIOPAD1_A) -> Self {
        variant as _
    }
}
impl MODEWAKEUPIOPAD1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODEWAKEUPIOPAD1_A {
        match self.bits {
            0 => MODEWAKEUPIOPAD1_A::INACTIVE,
            1 => MODEWAKEUPIOPAD1_A::PULL_DOWN,
            2 => MODEWAKEUPIOPAD1_A::PULL_UP,
            3 => MODEWAKEUPIOPAD1_A::REPEATER,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `INACTIVE`"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == MODEWAKEUPIOPAD1_A::INACTIVE
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == MODEWAKEUPIOPAD1_A::PULL_DOWN
    }
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == MODEWAKEUPIOPAD1_A::PULL_UP
    }
    #[doc = "Checks if the value of the field is `REPEATER`"]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == MODEWAKEUPIOPAD1_A::REPEATER
    }
}
#[doc = "Field `MODEWAKEUPIOPAD1` writer - Selects function mode (on-chip pull-up/pull-down resistor control)."]
pub type MODEWAKEUPIOPAD1_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, WAKEUPIOCTRL_SPEC, u8, MODEWAKEUPIOPAD1_A, 2, O>;
impl<'a, const O: u8> MODEWAKEUPIOPAD1_W<'a, O> {
    #[doc = "Inactive. Inactive (no pull-down/pull-up resistor enabled)."]
    #[inline(always)]
    pub fn inactive(self) -> &'a mut W {
        self.variant(MODEWAKEUPIOPAD1_A::INACTIVE)
    }
    #[doc = "Pull-down. Pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(MODEWAKEUPIOPAD1_A::PULL_DOWN)
    }
    #[doc = "Pull-up. Pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(MODEWAKEUPIOPAD1_A::PULL_UP)
    }
    #[doc = "Repeater. Repeater mode."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut W {
        self.variant(MODEWAKEUPIOPAD1_A::REPEATER)
    }
}
#[doc = "Field `MODEWAKEUPIOPAD2` reader - Selects function mode (on-chip pull-up/pull-down resistor control)."]
pub type MODEWAKEUPIOPAD2_R = crate::FieldReader<u8, MODEWAKEUPIOPAD2_A>;
#[doc = "Selects function mode (on-chip pull-up/pull-down resistor control).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODEWAKEUPIOPAD2_A {
    #[doc = "0: Inactive. Inactive (no pull-down/pull-up resistor enabled)."]
    INACTIVE = 0,
    #[doc = "1: Pull-down. Pull-down resistor enabled."]
    PULL_DOWN = 1,
    #[doc = "2: Pull-up. Pull-up resistor enabled."]
    PULL_UP = 2,
    #[doc = "3: Repeater. Repeater mode."]
    REPEATER = 3,
}
impl From<MODEWAKEUPIOPAD2_A> for u8 {
    #[inline(always)]
    fn from(variant: MODEWAKEUPIOPAD2_A) -> Self {
        variant as _
    }
}
impl MODEWAKEUPIOPAD2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODEWAKEUPIOPAD2_A {
        match self.bits {
            0 => MODEWAKEUPIOPAD2_A::INACTIVE,
            1 => MODEWAKEUPIOPAD2_A::PULL_DOWN,
            2 => MODEWAKEUPIOPAD2_A::PULL_UP,
            3 => MODEWAKEUPIOPAD2_A::REPEATER,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `INACTIVE`"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == MODEWAKEUPIOPAD2_A::INACTIVE
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == MODEWAKEUPIOPAD2_A::PULL_DOWN
    }
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == MODEWAKEUPIOPAD2_A::PULL_UP
    }
    #[doc = "Checks if the value of the field is `REPEATER`"]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == MODEWAKEUPIOPAD2_A::REPEATER
    }
}
#[doc = "Field `MODEWAKEUPIOPAD2` writer - Selects function mode (on-chip pull-up/pull-down resistor control)."]
pub type MODEWAKEUPIOPAD2_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, WAKEUPIOCTRL_SPEC, u8, MODEWAKEUPIOPAD2_A, 2, O>;
impl<'a, const O: u8> MODEWAKEUPIOPAD2_W<'a, O> {
    #[doc = "Inactive. Inactive (no pull-down/pull-up resistor enabled)."]
    #[inline(always)]
    pub fn inactive(self) -> &'a mut W {
        self.variant(MODEWAKEUPIOPAD2_A::INACTIVE)
    }
    #[doc = "Pull-down. Pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(MODEWAKEUPIOPAD2_A::PULL_DOWN)
    }
    #[doc = "Pull-up. Pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(MODEWAKEUPIOPAD2_A::PULL_UP)
    }
    #[doc = "Repeater. Repeater mode."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut W {
        self.variant(MODEWAKEUPIOPAD2_A::REPEATER)
    }
}
#[doc = "Field `MODEWAKEUPIOPAD3` reader - Selects function mode (on-chip pull-up/pull-down resistor control)."]
pub type MODEWAKEUPIOPAD3_R = crate::FieldReader<u8, MODEWAKEUPIOPAD3_A>;
#[doc = "Selects function mode (on-chip pull-up/pull-down resistor control).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODEWAKEUPIOPAD3_A {
    #[doc = "0: Inactive. Inactive (no pull-down/pull-up resistor enabled)."]
    INACTIVE = 0,
    #[doc = "1: Pull-down. Pull-down resistor enabled."]
    PULL_DOWN = 1,
    #[doc = "2: Pull-up. Pull-up resistor enabled."]
    PULL_UP = 2,
    #[doc = "3: Repeater. Repeater mode."]
    REPEATER = 3,
}
impl From<MODEWAKEUPIOPAD3_A> for u8 {
    #[inline(always)]
    fn from(variant: MODEWAKEUPIOPAD3_A) -> Self {
        variant as _
    }
}
impl MODEWAKEUPIOPAD3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODEWAKEUPIOPAD3_A {
        match self.bits {
            0 => MODEWAKEUPIOPAD3_A::INACTIVE,
            1 => MODEWAKEUPIOPAD3_A::PULL_DOWN,
            2 => MODEWAKEUPIOPAD3_A::PULL_UP,
            3 => MODEWAKEUPIOPAD3_A::REPEATER,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `INACTIVE`"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == MODEWAKEUPIOPAD3_A::INACTIVE
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == MODEWAKEUPIOPAD3_A::PULL_DOWN
    }
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == MODEWAKEUPIOPAD3_A::PULL_UP
    }
    #[doc = "Checks if the value of the field is `REPEATER`"]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == MODEWAKEUPIOPAD3_A::REPEATER
    }
}
#[doc = "Field `MODEWAKEUPIOPAD3` writer - Selects function mode (on-chip pull-up/pull-down resistor control)."]
pub type MODEWAKEUPIOPAD3_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, WAKEUPIOCTRL_SPEC, u8, MODEWAKEUPIOPAD3_A, 2, O>;
impl<'a, const O: u8> MODEWAKEUPIOPAD3_W<'a, O> {
    #[doc = "Inactive. Inactive (no pull-down/pull-up resistor enabled)."]
    #[inline(always)]
    pub fn inactive(self) -> &'a mut W {
        self.variant(MODEWAKEUPIOPAD3_A::INACTIVE)
    }
    #[doc = "Pull-down. Pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(MODEWAKEUPIOPAD3_A::PULL_DOWN)
    }
    #[doc = "Pull-up. Pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(MODEWAKEUPIOPAD3_A::PULL_UP)
    }
    #[doc = "Repeater. Repeater mode."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut W {
        self.variant(MODEWAKEUPIOPAD3_A::REPEATER)
    }
}
#[doc = "Field `WAKEUPIO_ENABLE_CTRL` reader - Enable WAKEUP IO PAD control from MODEWAKEUPIOPAD (bits 12 to 19)."]
pub type WAKEUPIO_ENABLE_CTRL_R = crate::BitReader<WAKEUPIO_ENABLE_CTRL_A>;
#[doc = "Enable WAKEUP IO PAD control from MODEWAKEUPIOPAD (bits 12 to 19).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WAKEUPIO_ENABLE_CTRL_A {
    #[doc = "0: WAKEUP IO PAD mode control comes from IOCON."]
    DISABLE = 0,
    #[doc = "1: WAKEUP IO PAD mode control comes from MODEWAKEUPIOPAD (bits 12 to 19)."]
    ENABLE = 1,
}
impl From<WAKEUPIO_ENABLE_CTRL_A> for bool {
    #[inline(always)]
    fn from(variant: WAKEUPIO_ENABLE_CTRL_A) -> Self {
        variant as u8 != 0
    }
}
impl WAKEUPIO_ENABLE_CTRL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WAKEUPIO_ENABLE_CTRL_A {
        match self.bits {
            false => WAKEUPIO_ENABLE_CTRL_A::DISABLE,
            true => WAKEUPIO_ENABLE_CTRL_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == WAKEUPIO_ENABLE_CTRL_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == WAKEUPIO_ENABLE_CTRL_A::ENABLE
    }
}
#[doc = "Field `WAKEUPIO_ENABLE_CTRL` writer - Enable WAKEUP IO PAD control from MODEWAKEUPIOPAD (bits 12 to 19)."]
pub type WAKEUPIO_ENABLE_CTRL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, WAKEUPIOCTRL_SPEC, WAKEUPIO_ENABLE_CTRL_A, O>;
impl<'a, const O: u8> WAKEUPIO_ENABLE_CTRL_W<'a, O> {
    #[doc = "WAKEUP IO PAD mode control comes from IOCON."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(WAKEUPIO_ENABLE_CTRL_A::DISABLE)
    }
    #[doc = "WAKEUP IO PAD mode control comes from MODEWAKEUPIOPAD (bits 12 to 19)."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(WAKEUPIO_ENABLE_CTRL_A::ENABLE)
    }
}
#[doc = "Field `WAKEUPIO_RSTN` reader - WAKEUP IO event detector reset control."]
pub type WAKEUPIO_RSTN_R = crate::BitReader<WAKEUPIO_RSTN_A>;
#[doc = "WAKEUP IO event detector reset control.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WAKEUPIO_RSTN_A {
    #[doc = "0: Bloc is reset."]
    ASSERTED = 0,
    #[doc = "1: Bloc is not reset."]
    RELEASED = 1,
}
impl From<WAKEUPIO_RSTN_A> for bool {
    #[inline(always)]
    fn from(variant: WAKEUPIO_RSTN_A) -> Self {
        variant as u8 != 0
    }
}
impl WAKEUPIO_RSTN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WAKEUPIO_RSTN_A {
        match self.bits {
            false => WAKEUPIO_RSTN_A::ASSERTED,
            true => WAKEUPIO_RSTN_A::RELEASED,
        }
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        *self == WAKEUPIO_RSTN_A::ASSERTED
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        *self == WAKEUPIO_RSTN_A::RELEASED
    }
}
#[doc = "Field `WAKEUPIO_RSTN` writer - WAKEUP IO event detector reset control."]
pub type WAKEUPIO_RSTN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, WAKEUPIOCTRL_SPEC, WAKEUPIO_RSTN_A, O>;
impl<'a, const O: u8> WAKEUPIO_RSTN_W<'a, O> {
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut W {
        self.variant(WAKEUPIO_RSTN_A::ASSERTED)
    }
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut W {
        self.variant(WAKEUPIO_RSTN_A::RELEASED)
    }
}
impl R {
    #[doc = "Bit 0 - Enable / disable detection of rising edge events on Wake Up 0 pin in Deep Power Down modes:."]
    #[inline(always)]
    pub fn risingedgewakeup0(&self) -> RISINGEDGEWAKEUP0_R {
        RISINGEDGEWAKEUP0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable / disable detection of falling edge events on Wake Up 0 pin in Deep Power Down modes:."]
    #[inline(always)]
    pub fn fallingedgewakeup0(&self) -> FALLINGEDGEWAKEUP0_R {
        FALLINGEDGEWAKEUP0_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable / disable detection of rising edge events on Wake Up 1 pin in Deep Power Down modes:."]
    #[inline(always)]
    pub fn risingedgewakeup1(&self) -> RISINGEDGEWAKEUP1_R {
        RISINGEDGEWAKEUP1_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable / disable detection of falling edge events on Wake Up 1 pin in Deep Power Down modes:."]
    #[inline(always)]
    pub fn fallingedgewakeup1(&self) -> FALLINGEDGEWAKEUP1_R {
        FALLINGEDGEWAKEUP1_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable / disable detection of rising edge events on Wake Up 2 pin in Deep Power Down modes:."]
    #[inline(always)]
    pub fn risingedgewakeup2(&self) -> RISINGEDGEWAKEUP2_R {
        RISINGEDGEWAKEUP2_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable / disable detection of falling edge events on Wake Up 2 pin in Deep Power Down modes:."]
    #[inline(always)]
    pub fn fallingedgewakeup2(&self) -> FALLINGEDGEWAKEUP2_R {
        FALLINGEDGEWAKEUP2_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable / disable detection of rising edge events on Wake Up 3 pin in Deep Power Down modes:."]
    #[inline(always)]
    pub fn risingedgewakeup3(&self) -> RISINGEDGEWAKEUP3_R {
        RISINGEDGEWAKEUP3_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable / disable detection of falling edge events on Wake Up 3 pin in Deep Power Down modes:."]
    #[inline(always)]
    pub fn fallingedgewakeup3(&self) -> FALLINGEDGEWAKEUP3_R {
        FALLINGEDGEWAKEUP3_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 12:13 - Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub fn modewakeupiopad0(&self) -> MODEWAKEUPIOPAD0_R {
        MODEWAKEUPIOPAD0_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub fn modewakeupiopad1(&self) -> MODEWAKEUPIOPAD1_R {
        MODEWAKEUPIOPAD1_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub fn modewakeupiopad2(&self) -> MODEWAKEUPIOPAD2_R {
        MODEWAKEUPIOPAD2_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub fn modewakeupiopad3(&self) -> MODEWAKEUPIOPAD3_R {
        MODEWAKEUPIOPAD3_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 20 - Enable WAKEUP IO PAD control from MODEWAKEUPIOPAD (bits 12 to 19)."]
    #[inline(always)]
    pub fn wakeupio_enable_ctrl(&self) -> WAKEUPIO_ENABLE_CTRL_R {
        WAKEUPIO_ENABLE_CTRL_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - WAKEUP IO event detector reset control."]
    #[inline(always)]
    pub fn wakeupio_rstn(&self) -> WAKEUPIO_RSTN_R {
        WAKEUPIO_RSTN_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable / disable detection of rising edge events on Wake Up 0 pin in Deep Power Down modes:."]
    #[inline(always)]
    pub fn risingedgewakeup0(&mut self) -> RISINGEDGEWAKEUP0_W<0> {
        RISINGEDGEWAKEUP0_W::new(self)
    }
    #[doc = "Bit 1 - Enable / disable detection of falling edge events on Wake Up 0 pin in Deep Power Down modes:."]
    #[inline(always)]
    pub fn fallingedgewakeup0(&mut self) -> FALLINGEDGEWAKEUP0_W<1> {
        FALLINGEDGEWAKEUP0_W::new(self)
    }
    #[doc = "Bit 2 - Enable / disable detection of rising edge events on Wake Up 1 pin in Deep Power Down modes:."]
    #[inline(always)]
    pub fn risingedgewakeup1(&mut self) -> RISINGEDGEWAKEUP1_W<2> {
        RISINGEDGEWAKEUP1_W::new(self)
    }
    #[doc = "Bit 3 - Enable / disable detection of falling edge events on Wake Up 1 pin in Deep Power Down modes:."]
    #[inline(always)]
    pub fn fallingedgewakeup1(&mut self) -> FALLINGEDGEWAKEUP1_W<3> {
        FALLINGEDGEWAKEUP1_W::new(self)
    }
    #[doc = "Bit 4 - Enable / disable detection of rising edge events on Wake Up 2 pin in Deep Power Down modes:."]
    #[inline(always)]
    pub fn risingedgewakeup2(&mut self) -> RISINGEDGEWAKEUP2_W<4> {
        RISINGEDGEWAKEUP2_W::new(self)
    }
    #[doc = "Bit 5 - Enable / disable detection of falling edge events on Wake Up 2 pin in Deep Power Down modes:."]
    #[inline(always)]
    pub fn fallingedgewakeup2(&mut self) -> FALLINGEDGEWAKEUP2_W<5> {
        FALLINGEDGEWAKEUP2_W::new(self)
    }
    #[doc = "Bit 6 - Enable / disable detection of rising edge events on Wake Up 3 pin in Deep Power Down modes:."]
    #[inline(always)]
    pub fn risingedgewakeup3(&mut self) -> RISINGEDGEWAKEUP3_W<6> {
        RISINGEDGEWAKEUP3_W::new(self)
    }
    #[doc = "Bit 7 - Enable / disable detection of falling edge events on Wake Up 3 pin in Deep Power Down modes:."]
    #[inline(always)]
    pub fn fallingedgewakeup3(&mut self) -> FALLINGEDGEWAKEUP3_W<7> {
        FALLINGEDGEWAKEUP3_W::new(self)
    }
    #[doc = "Bits 12:13 - Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub fn modewakeupiopad0(&mut self) -> MODEWAKEUPIOPAD0_W<12> {
        MODEWAKEUPIOPAD0_W::new(self)
    }
    #[doc = "Bits 14:15 - Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub fn modewakeupiopad1(&mut self) -> MODEWAKEUPIOPAD1_W<14> {
        MODEWAKEUPIOPAD1_W::new(self)
    }
    #[doc = "Bits 16:17 - Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub fn modewakeupiopad2(&mut self) -> MODEWAKEUPIOPAD2_W<16> {
        MODEWAKEUPIOPAD2_W::new(self)
    }
    #[doc = "Bits 18:19 - Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub fn modewakeupiopad3(&mut self) -> MODEWAKEUPIOPAD3_W<18> {
        MODEWAKEUPIOPAD3_W::new(self)
    }
    #[doc = "Bit 20 - Enable WAKEUP IO PAD control from MODEWAKEUPIOPAD (bits 12 to 19)."]
    #[inline(always)]
    pub fn wakeupio_enable_ctrl(&mut self) -> WAKEUPIO_ENABLE_CTRL_W<20> {
        WAKEUPIO_ENABLE_CTRL_W::new(self)
    }
    #[doc = "Bit 21 - WAKEUP IO event detector reset control."]
    #[inline(always)]
    pub fn wakeupio_rstn(&mut self) -> WAKEUPIO_RSTN_W<21> {
        WAKEUPIO_RSTN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Deep Power Down wake-up source \\[Reset by: PoR, Pin Reset, Software Reset\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wakeupioctrl](index.html) module"]
pub struct WAKEUPIOCTRL_SPEC;
impl crate::RegisterSpec for WAKEUPIOCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wakeupioctrl::R](R) reader structure"]
impl crate::Readable for WAKEUPIOCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wakeupioctrl::W](W) writer structure"]
impl crate::Writable for WAKEUPIOCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WAKEUPIOCTRL to value 0x0020_0000"]
impl crate::Resettable for WAKEUPIOCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0020_0000
    }
}
