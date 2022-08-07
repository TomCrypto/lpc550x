#[doc = "Register `PRESETCTRL2` reader"]
pub struct R(crate::R<PRESETCTRL_PRESETCTRL2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRESETCTRL_PRESETCTRL2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRESETCTRL_PRESETCTRL2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRESETCTRL_PRESETCTRL2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PRESETCTRL2` writer"]
pub struct W(crate::W<PRESETCTRL_PRESETCTRL2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRESETCTRL_PRESETCTRL2_SPEC>;
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
impl From<crate::W<PRESETCTRL_PRESETCTRL2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRESETCTRL_PRESETCTRL2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DMA1_RST` reader - DMA1 reset control."]
pub type DMA1_RST_R = crate::BitReader<DMA1_RST_A>;
#[doc = "DMA1 reset control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMA1_RST_A {
    #[doc = "0: Bloc is not reset."]
    RELEASED = 0,
    #[doc = "1: Bloc is reset."]
    ASSERTED = 1,
}
impl From<DMA1_RST_A> for bool {
    #[inline(always)]
    fn from(variant: DMA1_RST_A) -> Self {
        variant as u8 != 0
    }
}
impl DMA1_RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMA1_RST_A {
        match self.bits {
            false => DMA1_RST_A::RELEASED,
            true => DMA1_RST_A::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        *self == DMA1_RST_A::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        *self == DMA1_RST_A::ASSERTED
    }
}
#[doc = "Field `DMA1_RST` writer - DMA1 reset control."]
pub type DMA1_RST_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PRESETCTRL_PRESETCTRL2_SPEC, DMA1_RST_A, O>;
impl<'a, const O: u8> DMA1_RST_W<'a, O> {
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut W {
        self.variant(DMA1_RST_A::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut W {
        self.variant(DMA1_RST_A::ASSERTED)
    }
}
#[doc = "Field `COMP_RST` reader - Comparator reset control."]
pub type COMP_RST_R = crate::BitReader<COMP_RST_A>;
#[doc = "Comparator reset control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMP_RST_A {
    #[doc = "0: Bloc is not reset."]
    RELEASED = 0,
    #[doc = "1: Bloc is reset."]
    ASSERTED = 1,
}
impl From<COMP_RST_A> for bool {
    #[inline(always)]
    fn from(variant: COMP_RST_A) -> Self {
        variant as u8 != 0
    }
}
impl COMP_RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMP_RST_A {
        match self.bits {
            false => COMP_RST_A::RELEASED,
            true => COMP_RST_A::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        *self == COMP_RST_A::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        *self == COMP_RST_A::ASSERTED
    }
}
#[doc = "Field `COMP_RST` writer - Comparator reset control."]
pub type COMP_RST_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PRESETCTRL_PRESETCTRL2_SPEC, COMP_RST_A, O>;
impl<'a, const O: u8> COMP_RST_W<'a, O> {
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut W {
        self.variant(COMP_RST_A::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut W {
        self.variant(COMP_RST_A::ASSERTED)
    }
}
#[doc = "Field `SRAM_CTRL3_RST` reader - SRAM Controller 3 reset control."]
pub type SRAM_CTRL3_RST_R = crate::BitReader<SRAM_CTRL3_RST_A>;
#[doc = "SRAM Controller 3 reset control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRAM_CTRL3_RST_A {
    #[doc = "0: Bloc is not reset."]
    RELEASED = 0,
    #[doc = "1: Bloc is reset."]
    ASSERTED = 1,
}
impl From<SRAM_CTRL3_RST_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM_CTRL3_RST_A) -> Self {
        variant as u8 != 0
    }
}
impl SRAM_CTRL3_RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAM_CTRL3_RST_A {
        match self.bits {
            false => SRAM_CTRL3_RST_A::RELEASED,
            true => SRAM_CTRL3_RST_A::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        *self == SRAM_CTRL3_RST_A::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        *self == SRAM_CTRL3_RST_A::ASSERTED
    }
}
#[doc = "Field `SRAM_CTRL3_RST` writer - SRAM Controller 3 reset control."]
pub type SRAM_CTRL3_RST_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PRESETCTRL_PRESETCTRL2_SPEC, SRAM_CTRL3_RST_A, O>;
impl<'a, const O: u8> SRAM_CTRL3_RST_W<'a, O> {
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut W {
        self.variant(SRAM_CTRL3_RST_A::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut W {
        self.variant(SRAM_CTRL3_RST_A::ASSERTED)
    }
}
#[doc = "Field `FREQME_RST` reader - Frequency meter reset control."]
pub type FREQME_RST_R = crate::BitReader<FREQME_RST_A>;
#[doc = "Frequency meter reset control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FREQME_RST_A {
    #[doc = "0: Bloc is not reset."]
    RELEASED = 0,
    #[doc = "1: Bloc is reset."]
    ASSERTED = 1,
}
impl From<FREQME_RST_A> for bool {
    #[inline(always)]
    fn from(variant: FREQME_RST_A) -> Self {
        variant as u8 != 0
    }
}
impl FREQME_RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FREQME_RST_A {
        match self.bits {
            false => FREQME_RST_A::RELEASED,
            true => FREQME_RST_A::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        *self == FREQME_RST_A::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        *self == FREQME_RST_A::ASSERTED
    }
}
#[doc = "Field `FREQME_RST` writer - Frequency meter reset control."]
pub type FREQME_RST_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PRESETCTRL_PRESETCTRL2_SPEC, FREQME_RST_A, O>;
impl<'a, const O: u8> FREQME_RST_W<'a, O> {
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut W {
        self.variant(FREQME_RST_A::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut W {
        self.variant(FREQME_RST_A::ASSERTED)
    }
}
#[doc = "Field `CDOG_RST` reader - Code Watchdog reset control."]
pub type CDOG_RST_R = crate::BitReader<CDOG_RST_A>;
#[doc = "Code Watchdog reset control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CDOG_RST_A {
    #[doc = "0: Bloc is not reset."]
    RELEASED = 0,
    #[doc = "1: Bloc is reset."]
    ASSERTED = 1,
}
impl From<CDOG_RST_A> for bool {
    #[inline(always)]
    fn from(variant: CDOG_RST_A) -> Self {
        variant as u8 != 0
    }
}
impl CDOG_RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CDOG_RST_A {
        match self.bits {
            false => CDOG_RST_A::RELEASED,
            true => CDOG_RST_A::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        *self == CDOG_RST_A::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        *self == CDOG_RST_A::ASSERTED
    }
}
#[doc = "Field `CDOG_RST` writer - Code Watchdog reset control."]
pub type CDOG_RST_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PRESETCTRL_PRESETCTRL2_SPEC, CDOG_RST_A, O>;
impl<'a, const O: u8> CDOG_RST_W<'a, O> {
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut W {
        self.variant(CDOG_RST_A::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut W {
        self.variant(CDOG_RST_A::ASSERTED)
    }
}
#[doc = "Field `RNG_RST` reader - RNG reset control."]
pub type RNG_RST_R = crate::BitReader<RNG_RST_A>;
#[doc = "RNG reset control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RNG_RST_A {
    #[doc = "0: Bloc is not reset."]
    RELEASED = 0,
    #[doc = "1: Bloc is reset."]
    ASSERTED = 1,
}
impl From<RNG_RST_A> for bool {
    #[inline(always)]
    fn from(variant: RNG_RST_A) -> Self {
        variant as u8 != 0
    }
}
impl RNG_RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RNG_RST_A {
        match self.bits {
            false => RNG_RST_A::RELEASED,
            true => RNG_RST_A::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        *self == RNG_RST_A::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        *self == RNG_RST_A::ASSERTED
    }
}
#[doc = "Field `RNG_RST` writer - RNG reset control."]
pub type RNG_RST_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PRESETCTRL_PRESETCTRL2_SPEC, RNG_RST_A, O>;
impl<'a, const O: u8> RNG_RST_W<'a, O> {
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut W {
        self.variant(RNG_RST_A::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut W {
        self.variant(RNG_RST_A::ASSERTED)
    }
}
#[doc = "Field `SYSCTL_RST` reader - SYSCTL Block reset."]
pub type SYSCTL_RST_R = crate::BitReader<SYSCTL_RST_A>;
#[doc = "SYSCTL Block reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYSCTL_RST_A {
    #[doc = "0: Bloc is not reset."]
    RELEASED = 0,
    #[doc = "1: Bloc is reset."]
    ASSERTED = 1,
}
impl From<SYSCTL_RST_A> for bool {
    #[inline(always)]
    fn from(variant: SYSCTL_RST_A) -> Self {
        variant as u8 != 0
    }
}
impl SYSCTL_RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYSCTL_RST_A {
        match self.bits {
            false => SYSCTL_RST_A::RELEASED,
            true => SYSCTL_RST_A::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        *self == SYSCTL_RST_A::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        *self == SYSCTL_RST_A::ASSERTED
    }
}
#[doc = "Field `SYSCTL_RST` writer - SYSCTL Block reset."]
pub type SYSCTL_RST_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PRESETCTRL_PRESETCTRL2_SPEC, SYSCTL_RST_A, O>;
impl<'a, const O: u8> SYSCTL_RST_W<'a, O> {
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut W {
        self.variant(SYSCTL_RST_A::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut W {
        self.variant(SYSCTL_RST_A::ASSERTED)
    }
}
#[doc = "Field `HASH_AES_RST` reader - HASH_AES reset control."]
pub type HASH_AES_RST_R = crate::BitReader<HASH_AES_RST_A>;
#[doc = "HASH_AES reset control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HASH_AES_RST_A {
    #[doc = "0: Bloc is not reset."]
    RELEASED = 0,
    #[doc = "1: Bloc is reset."]
    ASSERTED = 1,
}
impl From<HASH_AES_RST_A> for bool {
    #[inline(always)]
    fn from(variant: HASH_AES_RST_A) -> Self {
        variant as u8 != 0
    }
}
impl HASH_AES_RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HASH_AES_RST_A {
        match self.bits {
            false => HASH_AES_RST_A::RELEASED,
            true => HASH_AES_RST_A::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        *self == HASH_AES_RST_A::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        *self == HASH_AES_RST_A::ASSERTED
    }
}
#[doc = "Field `HASH_AES_RST` writer - HASH_AES reset control."]
pub type HASH_AES_RST_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PRESETCTRL_PRESETCTRL2_SPEC, HASH_AES_RST_A, O>;
impl<'a, const O: u8> HASH_AES_RST_W<'a, O> {
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut W {
        self.variant(HASH_AES_RST_A::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut W {
        self.variant(HASH_AES_RST_A::ASSERTED)
    }
}
#[doc = "Field `PLULUT_RST` reader - PLU LUT reset control."]
pub type PLULUT_RST_R = crate::BitReader<PLULUT_RST_A>;
#[doc = "PLU LUT reset control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLULUT_RST_A {
    #[doc = "0: Bloc is not reset."]
    RELEASED = 0,
    #[doc = "1: Bloc is reset."]
    ASSERTED = 1,
}
impl From<PLULUT_RST_A> for bool {
    #[inline(always)]
    fn from(variant: PLULUT_RST_A) -> Self {
        variant as u8 != 0
    }
}
impl PLULUT_RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLULUT_RST_A {
        match self.bits {
            false => PLULUT_RST_A::RELEASED,
            true => PLULUT_RST_A::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        *self == PLULUT_RST_A::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        *self == PLULUT_RST_A::ASSERTED
    }
}
#[doc = "Field `PLULUT_RST` writer - PLU LUT reset control."]
pub type PLULUT_RST_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PRESETCTRL_PRESETCTRL2_SPEC, PLULUT_RST_A, O>;
impl<'a, const O: u8> PLULUT_RST_W<'a, O> {
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut W {
        self.variant(PLULUT_RST_A::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut W {
        self.variant(PLULUT_RST_A::ASSERTED)
    }
}
#[doc = "Field `TIMER3_RST` reader - Timer 3 reset control."]
pub type TIMER3_RST_R = crate::BitReader<TIMER3_RST_A>;
#[doc = "Timer 3 reset control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIMER3_RST_A {
    #[doc = "0: Bloc is not reset."]
    RELEASED = 0,
    #[doc = "1: Bloc is reset."]
    ASSERTED = 1,
}
impl From<TIMER3_RST_A> for bool {
    #[inline(always)]
    fn from(variant: TIMER3_RST_A) -> Self {
        variant as u8 != 0
    }
}
impl TIMER3_RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIMER3_RST_A {
        match self.bits {
            false => TIMER3_RST_A::RELEASED,
            true => TIMER3_RST_A::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        *self == TIMER3_RST_A::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        *self == TIMER3_RST_A::ASSERTED
    }
}
#[doc = "Field `TIMER3_RST` writer - Timer 3 reset control."]
pub type TIMER3_RST_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PRESETCTRL_PRESETCTRL2_SPEC, TIMER3_RST_A, O>;
impl<'a, const O: u8> TIMER3_RST_W<'a, O> {
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut W {
        self.variant(TIMER3_RST_A::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut W {
        self.variant(TIMER3_RST_A::ASSERTED)
    }
}
#[doc = "Field `TIMER4_RST` reader - Timer 4 reset control."]
pub type TIMER4_RST_R = crate::BitReader<TIMER4_RST_A>;
#[doc = "Timer 4 reset control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIMER4_RST_A {
    #[doc = "0: Bloc is not reset."]
    RELEASED = 0,
    #[doc = "1: Bloc is reset."]
    ASSERTED = 1,
}
impl From<TIMER4_RST_A> for bool {
    #[inline(always)]
    fn from(variant: TIMER4_RST_A) -> Self {
        variant as u8 != 0
    }
}
impl TIMER4_RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIMER4_RST_A {
        match self.bits {
            false => TIMER4_RST_A::RELEASED,
            true => TIMER4_RST_A::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        *self == TIMER4_RST_A::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        *self == TIMER4_RST_A::ASSERTED
    }
}
#[doc = "Field `TIMER4_RST` writer - Timer 4 reset control."]
pub type TIMER4_RST_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PRESETCTRL_PRESETCTRL2_SPEC, TIMER4_RST_A, O>;
impl<'a, const O: u8> TIMER4_RST_W<'a, O> {
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut W {
        self.variant(TIMER4_RST_A::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut W {
        self.variant(TIMER4_RST_A::ASSERTED)
    }
}
#[doc = "Field `PUF_RST` reader - PUF reset control reset control."]
pub type PUF_RST_R = crate::BitReader<PUF_RST_A>;
#[doc = "PUF reset control reset control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PUF_RST_A {
    #[doc = "0: Bloc is not reset."]
    RELEASED = 0,
    #[doc = "1: Bloc is reset."]
    ASSERTED = 1,
}
impl From<PUF_RST_A> for bool {
    #[inline(always)]
    fn from(variant: PUF_RST_A) -> Self {
        variant as u8 != 0
    }
}
impl PUF_RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PUF_RST_A {
        match self.bits {
            false => PUF_RST_A::RELEASED,
            true => PUF_RST_A::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        *self == PUF_RST_A::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        *self == PUF_RST_A::ASSERTED
    }
}
#[doc = "Field `PUF_RST` writer - PUF reset control reset control."]
pub type PUF_RST_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PRESETCTRL_PRESETCTRL2_SPEC, PUF_RST_A, O>;
impl<'a, const O: u8> PUF_RST_W<'a, O> {
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut W {
        self.variant(PUF_RST_A::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut W {
        self.variant(PUF_RST_A::ASSERTED)
    }
}
#[doc = "Field `CASPER_RST` reader - Casper reset control."]
pub type CASPER_RST_R = crate::BitReader<CASPER_RST_A>;
#[doc = "Casper reset control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CASPER_RST_A {
    #[doc = "0: Bloc is not reset."]
    RELEASED = 0,
    #[doc = "1: Bloc is reset."]
    ASSERTED = 1,
}
impl From<CASPER_RST_A> for bool {
    #[inline(always)]
    fn from(variant: CASPER_RST_A) -> Self {
        variant as u8 != 0
    }
}
impl CASPER_RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CASPER_RST_A {
        match self.bits {
            false => CASPER_RST_A::RELEASED,
            true => CASPER_RST_A::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        *self == CASPER_RST_A::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        *self == CASPER_RST_A::ASSERTED
    }
}
#[doc = "Field `CASPER_RST` writer - Casper reset control."]
pub type CASPER_RST_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PRESETCTRL_PRESETCTRL2_SPEC, CASPER_RST_A, O>;
impl<'a, const O: u8> CASPER_RST_W<'a, O> {
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut W {
        self.variant(CASPER_RST_A::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut W {
        self.variant(CASPER_RST_A::ASSERTED)
    }
}
#[doc = "Field `ANALOG_CTRL_RST` reader - analog control reset control."]
pub type ANALOG_CTRL_RST_R = crate::BitReader<ANALOG_CTRL_RST_A>;
#[doc = "analog control reset control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ANALOG_CTRL_RST_A {
    #[doc = "0: Bloc is not reset."]
    RELEASED = 0,
    #[doc = "1: Bloc is reset."]
    ASSERTED = 1,
}
impl From<ANALOG_CTRL_RST_A> for bool {
    #[inline(always)]
    fn from(variant: ANALOG_CTRL_RST_A) -> Self {
        variant as u8 != 0
    }
}
impl ANALOG_CTRL_RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ANALOG_CTRL_RST_A {
        match self.bits {
            false => ANALOG_CTRL_RST_A::RELEASED,
            true => ANALOG_CTRL_RST_A::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        *self == ANALOG_CTRL_RST_A::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        *self == ANALOG_CTRL_RST_A::ASSERTED
    }
}
#[doc = "Field `ANALOG_CTRL_RST` writer - analog control reset control."]
pub type ANALOG_CTRL_RST_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PRESETCTRL_PRESETCTRL2_SPEC, ANALOG_CTRL_RST_A, O>;
impl<'a, const O: u8> ANALOG_CTRL_RST_W<'a, O> {
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut W {
        self.variant(ANALOG_CTRL_RST_A::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut W {
        self.variant(ANALOG_CTRL_RST_A::ASSERTED)
    }
}
#[doc = "Field `HS_LSPI_RST` reader - HS LSPI reset control."]
pub type HS_LSPI_RST_R = crate::BitReader<HS_LSPI_RST_A>;
#[doc = "HS LSPI reset control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HS_LSPI_RST_A {
    #[doc = "0: Bloc is not reset."]
    RELEASED = 0,
    #[doc = "1: Bloc is reset."]
    ASSERTED = 1,
}
impl From<HS_LSPI_RST_A> for bool {
    #[inline(always)]
    fn from(variant: HS_LSPI_RST_A) -> Self {
        variant as u8 != 0
    }
}
impl HS_LSPI_RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HS_LSPI_RST_A {
        match self.bits {
            false => HS_LSPI_RST_A::RELEASED,
            true => HS_LSPI_RST_A::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        *self == HS_LSPI_RST_A::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        *self == HS_LSPI_RST_A::ASSERTED
    }
}
#[doc = "Field `HS_LSPI_RST` writer - HS LSPI reset control."]
pub type HS_LSPI_RST_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PRESETCTRL_PRESETCTRL2_SPEC, HS_LSPI_RST_A, O>;
impl<'a, const O: u8> HS_LSPI_RST_W<'a, O> {
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut W {
        self.variant(HS_LSPI_RST_A::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut W {
        self.variant(HS_LSPI_RST_A::ASSERTED)
    }
}
#[doc = "Field `GPIO_SEC_RST` reader - GPIO secure reset control."]
pub type GPIO_SEC_RST_R = crate::BitReader<GPIO_SEC_RST_A>;
#[doc = "GPIO secure reset control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO_SEC_RST_A {
    #[doc = "0: Bloc is not reset."]
    RELEASED = 0,
    #[doc = "1: Bloc is reset."]
    ASSERTED = 1,
}
impl From<GPIO_SEC_RST_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO_SEC_RST_A) -> Self {
        variant as u8 != 0
    }
}
impl GPIO_SEC_RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO_SEC_RST_A {
        match self.bits {
            false => GPIO_SEC_RST_A::RELEASED,
            true => GPIO_SEC_RST_A::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        *self == GPIO_SEC_RST_A::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        *self == GPIO_SEC_RST_A::ASSERTED
    }
}
#[doc = "Field `GPIO_SEC_RST` writer - GPIO secure reset control."]
pub type GPIO_SEC_RST_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PRESETCTRL_PRESETCTRL2_SPEC, GPIO_SEC_RST_A, O>;
impl<'a, const O: u8> GPIO_SEC_RST_W<'a, O> {
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut W {
        self.variant(GPIO_SEC_RST_A::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut W {
        self.variant(GPIO_SEC_RST_A::ASSERTED)
    }
}
#[doc = "Field `GPIO_SEC_INT_RST` reader - GPIO secure int reset control."]
pub type GPIO_SEC_INT_RST_R = crate::BitReader<GPIO_SEC_INT_RST_A>;
#[doc = "GPIO secure int reset control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO_SEC_INT_RST_A {
    #[doc = "0: Bloc is not reset."]
    RELEASED = 0,
    #[doc = "1: Bloc is reset."]
    ASSERTED = 1,
}
impl From<GPIO_SEC_INT_RST_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO_SEC_INT_RST_A) -> Self {
        variant as u8 != 0
    }
}
impl GPIO_SEC_INT_RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO_SEC_INT_RST_A {
        match self.bits {
            false => GPIO_SEC_INT_RST_A::RELEASED,
            true => GPIO_SEC_INT_RST_A::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        *self == GPIO_SEC_INT_RST_A::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        *self == GPIO_SEC_INT_RST_A::ASSERTED
    }
}
#[doc = "Field `GPIO_SEC_INT_RST` writer - GPIO secure int reset control."]
pub type GPIO_SEC_INT_RST_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PRESETCTRL_PRESETCTRL2_SPEC, GPIO_SEC_INT_RST_A, O>;
impl<'a, const O: u8> GPIO_SEC_INT_RST_W<'a, O> {
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut W {
        self.variant(GPIO_SEC_INT_RST_A::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut W {
        self.variant(GPIO_SEC_INT_RST_A::ASSERTED)
    }
}
impl R {
    #[doc = "Bit 1 - DMA1 reset control."]
    #[inline(always)]
    pub fn dma1_rst(&self) -> DMA1_RST_R {
        DMA1_RST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Comparator reset control."]
    #[inline(always)]
    pub fn comp_rst(&self) -> COMP_RST_R {
        COMP_RST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 6 - SRAM Controller 3 reset control."]
    #[inline(always)]
    pub fn sram_ctrl3_rst(&self) -> SRAM_CTRL3_RST_R {
        SRAM_CTRL3_RST_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Frequency meter reset control."]
    #[inline(always)]
    pub fn freqme_rst(&self) -> FREQME_RST_R {
        FREQME_RST_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 11 - Code Watchdog reset control."]
    #[inline(always)]
    pub fn cdog_rst(&self) -> CDOG_RST_R {
        CDOG_RST_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 13 - RNG reset control."]
    #[inline(always)]
    pub fn rng_rst(&self) -> RNG_RST_R {
        RNG_RST_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 15 - SYSCTL Block reset."]
    #[inline(always)]
    pub fn sysctl_rst(&self) -> SYSCTL_RST_R {
        SYSCTL_RST_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 18 - HASH_AES reset control."]
    #[inline(always)]
    pub fn hash_aes_rst(&self) -> HASH_AES_RST_R {
        HASH_AES_RST_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 20 - PLU LUT reset control."]
    #[inline(always)]
    pub fn plulut_rst(&self) -> PLULUT_RST_R {
        PLULUT_RST_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Timer 3 reset control."]
    #[inline(always)]
    pub fn timer3_rst(&self) -> TIMER3_RST_R {
        TIMER3_RST_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Timer 4 reset control."]
    #[inline(always)]
    pub fn timer4_rst(&self) -> TIMER4_RST_R {
        TIMER4_RST_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - PUF reset control reset control."]
    #[inline(always)]
    pub fn puf_rst(&self) -> PUF_RST_R {
        PUF_RST_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Casper reset control."]
    #[inline(always)]
    pub fn casper_rst(&self) -> CASPER_RST_R {
        CASPER_RST_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 27 - analog control reset control."]
    #[inline(always)]
    pub fn analog_ctrl_rst(&self) -> ANALOG_CTRL_RST_R {
        ANALOG_CTRL_RST_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - HS LSPI reset control."]
    #[inline(always)]
    pub fn hs_lspi_rst(&self) -> HS_LSPI_RST_R {
        HS_LSPI_RST_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - GPIO secure reset control."]
    #[inline(always)]
    pub fn gpio_sec_rst(&self) -> GPIO_SEC_RST_R {
        GPIO_SEC_RST_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - GPIO secure int reset control."]
    #[inline(always)]
    pub fn gpio_sec_int_rst(&self) -> GPIO_SEC_INT_RST_R {
        GPIO_SEC_INT_RST_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - DMA1 reset control."]
    #[inline(always)]
    pub fn dma1_rst(&mut self) -> DMA1_RST_W<1> {
        DMA1_RST_W::new(self)
    }
    #[doc = "Bit 2 - Comparator reset control."]
    #[inline(always)]
    pub fn comp_rst(&mut self) -> COMP_RST_W<2> {
        COMP_RST_W::new(self)
    }
    #[doc = "Bit 6 - SRAM Controller 3 reset control."]
    #[inline(always)]
    pub fn sram_ctrl3_rst(&mut self) -> SRAM_CTRL3_RST_W<6> {
        SRAM_CTRL3_RST_W::new(self)
    }
    #[doc = "Bit 8 - Frequency meter reset control."]
    #[inline(always)]
    pub fn freqme_rst(&mut self) -> FREQME_RST_W<8> {
        FREQME_RST_W::new(self)
    }
    #[doc = "Bit 11 - Code Watchdog reset control."]
    #[inline(always)]
    pub fn cdog_rst(&mut self) -> CDOG_RST_W<11> {
        CDOG_RST_W::new(self)
    }
    #[doc = "Bit 13 - RNG reset control."]
    #[inline(always)]
    pub fn rng_rst(&mut self) -> RNG_RST_W<13> {
        RNG_RST_W::new(self)
    }
    #[doc = "Bit 15 - SYSCTL Block reset."]
    #[inline(always)]
    pub fn sysctl_rst(&mut self) -> SYSCTL_RST_W<15> {
        SYSCTL_RST_W::new(self)
    }
    #[doc = "Bit 18 - HASH_AES reset control."]
    #[inline(always)]
    pub fn hash_aes_rst(&mut self) -> HASH_AES_RST_W<18> {
        HASH_AES_RST_W::new(self)
    }
    #[doc = "Bit 20 - PLU LUT reset control."]
    #[inline(always)]
    pub fn plulut_rst(&mut self) -> PLULUT_RST_W<20> {
        PLULUT_RST_W::new(self)
    }
    #[doc = "Bit 21 - Timer 3 reset control."]
    #[inline(always)]
    pub fn timer3_rst(&mut self) -> TIMER3_RST_W<21> {
        TIMER3_RST_W::new(self)
    }
    #[doc = "Bit 22 - Timer 4 reset control."]
    #[inline(always)]
    pub fn timer4_rst(&mut self) -> TIMER4_RST_W<22> {
        TIMER4_RST_W::new(self)
    }
    #[doc = "Bit 23 - PUF reset control reset control."]
    #[inline(always)]
    pub fn puf_rst(&mut self) -> PUF_RST_W<23> {
        PUF_RST_W::new(self)
    }
    #[doc = "Bit 24 - Casper reset control."]
    #[inline(always)]
    pub fn casper_rst(&mut self) -> CASPER_RST_W<24> {
        CASPER_RST_W::new(self)
    }
    #[doc = "Bit 27 - analog control reset control."]
    #[inline(always)]
    pub fn analog_ctrl_rst(&mut self) -> ANALOG_CTRL_RST_W<27> {
        ANALOG_CTRL_RST_W::new(self)
    }
    #[doc = "Bit 28 - HS LSPI reset control."]
    #[inline(always)]
    pub fn hs_lspi_rst(&mut self) -> HS_LSPI_RST_W<28> {
        HS_LSPI_RST_W::new(self)
    }
    #[doc = "Bit 29 - GPIO secure reset control."]
    #[inline(always)]
    pub fn gpio_sec_rst(&mut self) -> GPIO_SEC_RST_W<29> {
        GPIO_SEC_RST_W::new(self)
    }
    #[doc = "Bit 30 - GPIO secure int reset control."]
    #[inline(always)]
    pub fn gpio_sec_int_rst(&mut self) -> GPIO_SEC_INT_RST_W<30> {
        GPIO_SEC_INT_RST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Peripheral reset control 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [presetctrl_presetctrl2](index.html) module"]
pub struct PRESETCTRL_PRESETCTRL2_SPEC;
impl crate::RegisterSpec for PRESETCTRL_PRESETCTRL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [presetctrl_presetctrl2::R](R) reader structure"]
impl crate::Readable for PRESETCTRL_PRESETCTRL2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [presetctrl_presetctrl2::W](W) writer structure"]
impl crate::Writable for PRESETCTRL_PRESETCTRL2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PRESETCTRL2 to value 0"]
impl crate::Resettable for PRESETCTRL_PRESETCTRL2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
