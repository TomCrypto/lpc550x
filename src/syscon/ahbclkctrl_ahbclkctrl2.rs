#[doc = "Register `AHBCLKCTRL2` reader"]
pub struct R(crate::R<AHBCLKCTRL_AHBCLKCTRL2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AHBCLKCTRL_AHBCLKCTRL2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AHBCLKCTRL_AHBCLKCTRL2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AHBCLKCTRL_AHBCLKCTRL2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AHBCLKCTRL2` writer"]
pub struct W(crate::W<AHBCLKCTRL_AHBCLKCTRL2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AHBCLKCTRL_AHBCLKCTRL2_SPEC>;
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
impl From<crate::W<AHBCLKCTRL_AHBCLKCTRL2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AHBCLKCTRL_AHBCLKCTRL2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DMA1` reader - Enables the clock for the DMA1."]
pub type DMA1_R = crate::BitReader<DMA1_A>;
#[doc = "Enables the clock for the DMA1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMA1_A {
    #[doc = "0: Disable Clock."]
    DISABLE = 0,
    #[doc = "1: Enable Clock."]
    ENABLE = 1,
}
impl From<DMA1_A> for bool {
    #[inline(always)]
    fn from(variant: DMA1_A) -> Self {
        variant as u8 != 0
    }
}
impl DMA1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMA1_A {
        match self.bits {
            false => DMA1_A::DISABLE,
            true => DMA1_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == DMA1_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == DMA1_A::ENABLE
    }
}
#[doc = "Field `DMA1` writer - Enables the clock for the DMA1."]
pub type DMA1_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AHBCLKCTRL_AHBCLKCTRL2_SPEC, DMA1_A, O>;
impl<'a, const O: u8> DMA1_W<'a, O> {
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(DMA1_A::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(DMA1_A::ENABLE)
    }
}
#[doc = "Field `COMP` reader - Enables the clock for the Comparator."]
pub type COMP_R = crate::BitReader<COMP_A>;
#[doc = "Enables the clock for the Comparator.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMP_A {
    #[doc = "0: Disable Clock."]
    DISABLE = 0,
    #[doc = "1: Enable Clock."]
    ENABLE = 1,
}
impl From<COMP_A> for bool {
    #[inline(always)]
    fn from(variant: COMP_A) -> Self {
        variant as u8 != 0
    }
}
impl COMP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMP_A {
        match self.bits {
            false => COMP_A::DISABLE,
            true => COMP_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == COMP_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == COMP_A::ENABLE
    }
}
#[doc = "Field `COMP` writer - Enables the clock for the Comparator."]
pub type COMP_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AHBCLKCTRL_AHBCLKCTRL2_SPEC, COMP_A, O>;
impl<'a, const O: u8> COMP_W<'a, O> {
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(COMP_A::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(COMP_A::ENABLE)
    }
}
#[doc = "Field `SRAM_CTRL3` reader - Enables the clock for the SRAM Controller 3."]
pub type SRAM_CTRL3_R = crate::BitReader<SRAM_CTRL3_A>;
#[doc = "Enables the clock for the SRAM Controller 3.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRAM_CTRL3_A {
    #[doc = "0: Disable Clock."]
    DISABLE = 0,
    #[doc = "1: Enable Clock."]
    ENABLE = 1,
}
impl From<SRAM_CTRL3_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM_CTRL3_A) -> Self {
        variant as u8 != 0
    }
}
impl SRAM_CTRL3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAM_CTRL3_A {
        match self.bits {
            false => SRAM_CTRL3_A::DISABLE,
            true => SRAM_CTRL3_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == SRAM_CTRL3_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == SRAM_CTRL3_A::ENABLE
    }
}
#[doc = "Field `SRAM_CTRL3` writer - Enables the clock for the SRAM Controller 3."]
pub type SRAM_CTRL3_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AHBCLKCTRL_AHBCLKCTRL2_SPEC, SRAM_CTRL3_A, O>;
impl<'a, const O: u8> SRAM_CTRL3_W<'a, O> {
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SRAM_CTRL3_A::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SRAM_CTRL3_A::ENABLE)
    }
}
#[doc = "Field `FREQME` reader - Enables the clock for the Frequency meter."]
pub type FREQME_R = crate::BitReader<FREQME_A>;
#[doc = "Enables the clock for the Frequency meter.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FREQME_A {
    #[doc = "0: Disable Clock."]
    DISABLE = 0,
    #[doc = "1: Enable Clock."]
    ENABLE = 1,
}
impl From<FREQME_A> for bool {
    #[inline(always)]
    fn from(variant: FREQME_A) -> Self {
        variant as u8 != 0
    }
}
impl FREQME_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FREQME_A {
        match self.bits {
            false => FREQME_A::DISABLE,
            true => FREQME_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == FREQME_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == FREQME_A::ENABLE
    }
}
#[doc = "Field `FREQME` writer - Enables the clock for the Frequency meter."]
pub type FREQME_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AHBCLKCTRL_AHBCLKCTRL2_SPEC, FREQME_A, O>;
impl<'a, const O: u8> FREQME_W<'a, O> {
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(FREQME_A::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(FREQME_A::ENABLE)
    }
}
#[doc = "Field `CDOG` reader - Enables the clock for the code watchdog."]
pub type CDOG_R = crate::BitReader<CDOG_A>;
#[doc = "Enables the clock for the code watchdog.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CDOG_A {
    #[doc = "0: Disable Clock."]
    DISABLE = 0,
    #[doc = "1: Enable Clock."]
    ENABLE = 1,
}
impl From<CDOG_A> for bool {
    #[inline(always)]
    fn from(variant: CDOG_A) -> Self {
        variant as u8 != 0
    }
}
impl CDOG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CDOG_A {
        match self.bits {
            false => CDOG_A::DISABLE,
            true => CDOG_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CDOG_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CDOG_A::ENABLE
    }
}
#[doc = "Field `CDOG` writer - Enables the clock for the code watchdog."]
pub type CDOG_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AHBCLKCTRL_AHBCLKCTRL2_SPEC, CDOG_A, O>;
impl<'a, const O: u8> CDOG_W<'a, O> {
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CDOG_A::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CDOG_A::ENABLE)
    }
}
#[doc = "Field `RNG` reader - Enables the clock for the RNG."]
pub type RNG_R = crate::BitReader<RNG_A>;
#[doc = "Enables the clock for the RNG.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RNG_A {
    #[doc = "0: Disable Clock."]
    DISABLE = 0,
    #[doc = "1: Enable Clock."]
    ENABLE = 1,
}
impl From<RNG_A> for bool {
    #[inline(always)]
    fn from(variant: RNG_A) -> Self {
        variant as u8 != 0
    }
}
impl RNG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RNG_A {
        match self.bits {
            false => RNG_A::DISABLE,
            true => RNG_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == RNG_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == RNG_A::ENABLE
    }
}
#[doc = "Field `RNG` writer - Enables the clock for the RNG."]
pub type RNG_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHBCLKCTRL_AHBCLKCTRL2_SPEC, RNG_A, O>;
impl<'a, const O: u8> RNG_W<'a, O> {
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(RNG_A::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(RNG_A::ENABLE)
    }
}
#[doc = "Field `SYSCTL` reader - SYSCTL block clock."]
pub type SYSCTL_R = crate::BitReader<SYSCTL_A>;
#[doc = "SYSCTL block clock.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYSCTL_A {
    #[doc = "0: Disable Clock."]
    DISABLE = 0,
    #[doc = "1: Enable Clock."]
    ENABLE = 1,
}
impl From<SYSCTL_A> for bool {
    #[inline(always)]
    fn from(variant: SYSCTL_A) -> Self {
        variant as u8 != 0
    }
}
impl SYSCTL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYSCTL_A {
        match self.bits {
            false => SYSCTL_A::DISABLE,
            true => SYSCTL_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == SYSCTL_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == SYSCTL_A::ENABLE
    }
}
#[doc = "Field `SYSCTL` writer - SYSCTL block clock."]
pub type SYSCTL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AHBCLKCTRL_AHBCLKCTRL2_SPEC, SYSCTL_A, O>;
impl<'a, const O: u8> SYSCTL_W<'a, O> {
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SYSCTL_A::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SYSCTL_A::ENABLE)
    }
}
#[doc = "Field `HASH_AES` reader - Enables the clock for the HASH_AES."]
pub type HASH_AES_R = crate::BitReader<HASH_AES_A>;
#[doc = "Enables the clock for the HASH_AES.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HASH_AES_A {
    #[doc = "0: Disable Clock."]
    DISABLE = 0,
    #[doc = "1: Enable Clock."]
    ENABLE = 1,
}
impl From<HASH_AES_A> for bool {
    #[inline(always)]
    fn from(variant: HASH_AES_A) -> Self {
        variant as u8 != 0
    }
}
impl HASH_AES_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HASH_AES_A {
        match self.bits {
            false => HASH_AES_A::DISABLE,
            true => HASH_AES_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == HASH_AES_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == HASH_AES_A::ENABLE
    }
}
#[doc = "Field `HASH_AES` writer - Enables the clock for the HASH_AES."]
pub type HASH_AES_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AHBCLKCTRL_AHBCLKCTRL2_SPEC, HASH_AES_A, O>;
impl<'a, const O: u8> HASH_AES_W<'a, O> {
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(HASH_AES_A::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(HASH_AES_A::ENABLE)
    }
}
#[doc = "Field `PLULUT` reader - Enables the clock for the PLU LUT."]
pub type PLULUT_R = crate::BitReader<PLULUT_A>;
#[doc = "Enables the clock for the PLU LUT.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLULUT_A {
    #[doc = "0: Disable Clock."]
    DISABLE = 0,
    #[doc = "1: Enable Clock."]
    ENABLE = 1,
}
impl From<PLULUT_A> for bool {
    #[inline(always)]
    fn from(variant: PLULUT_A) -> Self {
        variant as u8 != 0
    }
}
impl PLULUT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLULUT_A {
        match self.bits {
            false => PLULUT_A::DISABLE,
            true => PLULUT_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == PLULUT_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == PLULUT_A::ENABLE
    }
}
#[doc = "Field `PLULUT` writer - Enables the clock for the PLU LUT."]
pub type PLULUT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AHBCLKCTRL_AHBCLKCTRL2_SPEC, PLULUT_A, O>;
impl<'a, const O: u8> PLULUT_W<'a, O> {
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(PLULUT_A::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(PLULUT_A::ENABLE)
    }
}
#[doc = "Field `TIMER3` reader - Enables the clock for the Timer 3."]
pub type TIMER3_R = crate::BitReader<TIMER3_A>;
#[doc = "Enables the clock for the Timer 3.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIMER3_A {
    #[doc = "0: Disable Clock."]
    DISABLE = 0,
    #[doc = "1: Enable Clock."]
    ENABLE = 1,
}
impl From<TIMER3_A> for bool {
    #[inline(always)]
    fn from(variant: TIMER3_A) -> Self {
        variant as u8 != 0
    }
}
impl TIMER3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIMER3_A {
        match self.bits {
            false => TIMER3_A::DISABLE,
            true => TIMER3_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == TIMER3_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == TIMER3_A::ENABLE
    }
}
#[doc = "Field `TIMER3` writer - Enables the clock for the Timer 3."]
pub type TIMER3_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AHBCLKCTRL_AHBCLKCTRL2_SPEC, TIMER3_A, O>;
impl<'a, const O: u8> TIMER3_W<'a, O> {
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(TIMER3_A::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(TIMER3_A::ENABLE)
    }
}
#[doc = "Field `TIMER4` reader - Enables the clock for the Timer 4."]
pub type TIMER4_R = crate::BitReader<TIMER4_A>;
#[doc = "Enables the clock for the Timer 4.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIMER4_A {
    #[doc = "0: Disable Clock."]
    DISABLE = 0,
    #[doc = "1: Enable Clock."]
    ENABLE = 1,
}
impl From<TIMER4_A> for bool {
    #[inline(always)]
    fn from(variant: TIMER4_A) -> Self {
        variant as u8 != 0
    }
}
impl TIMER4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIMER4_A {
        match self.bits {
            false => TIMER4_A::DISABLE,
            true => TIMER4_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == TIMER4_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == TIMER4_A::ENABLE
    }
}
#[doc = "Field `TIMER4` writer - Enables the clock for the Timer 4."]
pub type TIMER4_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AHBCLKCTRL_AHBCLKCTRL2_SPEC, TIMER4_A, O>;
impl<'a, const O: u8> TIMER4_W<'a, O> {
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(TIMER4_A::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(TIMER4_A::ENABLE)
    }
}
#[doc = "Field `PUF` reader - Enables the clock for the PUF reset control."]
pub type PUF_R = crate::BitReader<PUF_A>;
#[doc = "Enables the clock for the PUF reset control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PUF_A {
    #[doc = "0: Disable Clock."]
    DISABLE = 0,
    #[doc = "1: Enable Clock."]
    ENABLE = 1,
}
impl From<PUF_A> for bool {
    #[inline(always)]
    fn from(variant: PUF_A) -> Self {
        variant as u8 != 0
    }
}
impl PUF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PUF_A {
        match self.bits {
            false => PUF_A::DISABLE,
            true => PUF_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == PUF_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == PUF_A::ENABLE
    }
}
#[doc = "Field `PUF` writer - Enables the clock for the PUF reset control."]
pub type PUF_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHBCLKCTRL_AHBCLKCTRL2_SPEC, PUF_A, O>;
impl<'a, const O: u8> PUF_W<'a, O> {
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(PUF_A::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(PUF_A::ENABLE)
    }
}
#[doc = "Field `CASPER` reader - Enables the clock for the Casper."]
pub type CASPER_R = crate::BitReader<CASPER_A>;
#[doc = "Enables the clock for the Casper.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CASPER_A {
    #[doc = "0: Disable Clock."]
    DISABLE = 0,
    #[doc = "1: Enable Clock."]
    ENABLE = 1,
}
impl From<CASPER_A> for bool {
    #[inline(always)]
    fn from(variant: CASPER_A) -> Self {
        variant as u8 != 0
    }
}
impl CASPER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CASPER_A {
        match self.bits {
            false => CASPER_A::DISABLE,
            true => CASPER_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CASPER_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CASPER_A::ENABLE
    }
}
#[doc = "Field `CASPER` writer - Enables the clock for the Casper."]
pub type CASPER_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AHBCLKCTRL_AHBCLKCTRL2_SPEC, CASPER_A, O>;
impl<'a, const O: u8> CASPER_W<'a, O> {
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CASPER_A::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CASPER_A::ENABLE)
    }
}
#[doc = "Field `ANALOG_CTRL` reader - Enables the clock for the analog control."]
pub type ANALOG_CTRL_R = crate::BitReader<ANALOG_CTRL_A>;
#[doc = "Enables the clock for the analog control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ANALOG_CTRL_A {
    #[doc = "0: Disable Clock."]
    DISABLE = 0,
    #[doc = "1: Enable Clock."]
    ENABLE = 1,
}
impl From<ANALOG_CTRL_A> for bool {
    #[inline(always)]
    fn from(variant: ANALOG_CTRL_A) -> Self {
        variant as u8 != 0
    }
}
impl ANALOG_CTRL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ANALOG_CTRL_A {
        match self.bits {
            false => ANALOG_CTRL_A::DISABLE,
            true => ANALOG_CTRL_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ANALOG_CTRL_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ANALOG_CTRL_A::ENABLE
    }
}
#[doc = "Field `ANALOG_CTRL` writer - Enables the clock for the analog control."]
pub type ANALOG_CTRL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AHBCLKCTRL_AHBCLKCTRL2_SPEC, ANALOG_CTRL_A, O>;
impl<'a, const O: u8> ANALOG_CTRL_W<'a, O> {
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ANALOG_CTRL_A::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ANALOG_CTRL_A::ENABLE)
    }
}
#[doc = "Field `HS_LSPI` reader - Enables the clock for the HS LSPI."]
pub type HS_LSPI_R = crate::BitReader<HS_LSPI_A>;
#[doc = "Enables the clock for the HS LSPI.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HS_LSPI_A {
    #[doc = "0: Disable Clock."]
    DISABLE = 0,
    #[doc = "1: Enable Clock."]
    ENABLE = 1,
}
impl From<HS_LSPI_A> for bool {
    #[inline(always)]
    fn from(variant: HS_LSPI_A) -> Self {
        variant as u8 != 0
    }
}
impl HS_LSPI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HS_LSPI_A {
        match self.bits {
            false => HS_LSPI_A::DISABLE,
            true => HS_LSPI_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == HS_LSPI_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == HS_LSPI_A::ENABLE
    }
}
#[doc = "Field `HS_LSPI` writer - Enables the clock for the HS LSPI."]
pub type HS_LSPI_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AHBCLKCTRL_AHBCLKCTRL2_SPEC, HS_LSPI_A, O>;
impl<'a, const O: u8> HS_LSPI_W<'a, O> {
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(HS_LSPI_A::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(HS_LSPI_A::ENABLE)
    }
}
#[doc = "Field `GPIO_SEC` reader - Enables the clock for the GPIO secure."]
pub type GPIO_SEC_R = crate::BitReader<GPIO_SEC_A>;
#[doc = "Enables the clock for the GPIO secure.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO_SEC_A {
    #[doc = "0: Disable Clock."]
    DISABLE = 0,
    #[doc = "1: Enable Clock."]
    ENABLE = 1,
}
impl From<GPIO_SEC_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO_SEC_A) -> Self {
        variant as u8 != 0
    }
}
impl GPIO_SEC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO_SEC_A {
        match self.bits {
            false => GPIO_SEC_A::DISABLE,
            true => GPIO_SEC_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == GPIO_SEC_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == GPIO_SEC_A::ENABLE
    }
}
#[doc = "Field `GPIO_SEC` writer - Enables the clock for the GPIO secure."]
pub type GPIO_SEC_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AHBCLKCTRL_AHBCLKCTRL2_SPEC, GPIO_SEC_A, O>;
impl<'a, const O: u8> GPIO_SEC_W<'a, O> {
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(GPIO_SEC_A::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(GPIO_SEC_A::ENABLE)
    }
}
#[doc = "Field `GPIO_SEC_INT` reader - Enables the clock for the GPIO secure int."]
pub type GPIO_SEC_INT_R = crate::BitReader<GPIO_SEC_INT_A>;
#[doc = "Enables the clock for the GPIO secure int.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO_SEC_INT_A {
    #[doc = "0: Disable Clock."]
    DISABLE = 0,
    #[doc = "1: Enable Clock."]
    ENABLE = 1,
}
impl From<GPIO_SEC_INT_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO_SEC_INT_A) -> Self {
        variant as u8 != 0
    }
}
impl GPIO_SEC_INT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO_SEC_INT_A {
        match self.bits {
            false => GPIO_SEC_INT_A::DISABLE,
            true => GPIO_SEC_INT_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == GPIO_SEC_INT_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == GPIO_SEC_INT_A::ENABLE
    }
}
#[doc = "Field `GPIO_SEC_INT` writer - Enables the clock for the GPIO secure int."]
pub type GPIO_SEC_INT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AHBCLKCTRL_AHBCLKCTRL2_SPEC, GPIO_SEC_INT_A, O>;
impl<'a, const O: u8> GPIO_SEC_INT_W<'a, O> {
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(GPIO_SEC_INT_A::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(GPIO_SEC_INT_A::ENABLE)
    }
}
impl R {
    #[doc = "Bit 1 - Enables the clock for the DMA1."]
    #[inline(always)]
    pub fn dma1(&self) -> DMA1_R {
        DMA1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enables the clock for the Comparator."]
    #[inline(always)]
    pub fn comp(&self) -> COMP_R {
        COMP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 6 - Enables the clock for the SRAM Controller 3."]
    #[inline(always)]
    pub fn sram_ctrl3(&self) -> SRAM_CTRL3_R {
        SRAM_CTRL3_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Enables the clock for the Frequency meter."]
    #[inline(always)]
    pub fn freqme(&self) -> FREQME_R {
        FREQME_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 11 - Enables the clock for the code watchdog."]
    #[inline(always)]
    pub fn cdog(&self) -> CDOG_R {
        CDOG_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 13 - Enables the clock for the RNG."]
    #[inline(always)]
    pub fn rng(&self) -> RNG_R {
        RNG_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 15 - SYSCTL block clock."]
    #[inline(always)]
    pub fn sysctl(&self) -> SYSCTL_R {
        SYSCTL_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 18 - Enables the clock for the HASH_AES."]
    #[inline(always)]
    pub fn hash_aes(&self) -> HASH_AES_R {
        HASH_AES_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 20 - Enables the clock for the PLU LUT."]
    #[inline(always)]
    pub fn plulut(&self) -> PLULUT_R {
        PLULUT_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Enables the clock for the Timer 3."]
    #[inline(always)]
    pub fn timer3(&self) -> TIMER3_R {
        TIMER3_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Enables the clock for the Timer 4."]
    #[inline(always)]
    pub fn timer4(&self) -> TIMER4_R {
        TIMER4_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Enables the clock for the PUF reset control."]
    #[inline(always)]
    pub fn puf(&self) -> PUF_R {
        PUF_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Enables the clock for the Casper."]
    #[inline(always)]
    pub fn casper(&self) -> CASPER_R {
        CASPER_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 27 - Enables the clock for the analog control."]
    #[inline(always)]
    pub fn analog_ctrl(&self) -> ANALOG_CTRL_R {
        ANALOG_CTRL_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Enables the clock for the HS LSPI."]
    #[inline(always)]
    pub fn hs_lspi(&self) -> HS_LSPI_R {
        HS_LSPI_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Enables the clock for the GPIO secure."]
    #[inline(always)]
    pub fn gpio_sec(&self) -> GPIO_SEC_R {
        GPIO_SEC_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Enables the clock for the GPIO secure int."]
    #[inline(always)]
    pub fn gpio_sec_int(&self) -> GPIO_SEC_INT_R {
        GPIO_SEC_INT_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Enables the clock for the DMA1."]
    #[inline(always)]
    pub fn dma1(&mut self) -> DMA1_W<1> {
        DMA1_W::new(self)
    }
    #[doc = "Bit 2 - Enables the clock for the Comparator."]
    #[inline(always)]
    pub fn comp(&mut self) -> COMP_W<2> {
        COMP_W::new(self)
    }
    #[doc = "Bit 6 - Enables the clock for the SRAM Controller 3."]
    #[inline(always)]
    pub fn sram_ctrl3(&mut self) -> SRAM_CTRL3_W<6> {
        SRAM_CTRL3_W::new(self)
    }
    #[doc = "Bit 8 - Enables the clock for the Frequency meter."]
    #[inline(always)]
    pub fn freqme(&mut self) -> FREQME_W<8> {
        FREQME_W::new(self)
    }
    #[doc = "Bit 11 - Enables the clock for the code watchdog."]
    #[inline(always)]
    pub fn cdog(&mut self) -> CDOG_W<11> {
        CDOG_W::new(self)
    }
    #[doc = "Bit 13 - Enables the clock for the RNG."]
    #[inline(always)]
    pub fn rng(&mut self) -> RNG_W<13> {
        RNG_W::new(self)
    }
    #[doc = "Bit 15 - SYSCTL block clock."]
    #[inline(always)]
    pub fn sysctl(&mut self) -> SYSCTL_W<15> {
        SYSCTL_W::new(self)
    }
    #[doc = "Bit 18 - Enables the clock for the HASH_AES."]
    #[inline(always)]
    pub fn hash_aes(&mut self) -> HASH_AES_W<18> {
        HASH_AES_W::new(self)
    }
    #[doc = "Bit 20 - Enables the clock for the PLU LUT."]
    #[inline(always)]
    pub fn plulut(&mut self) -> PLULUT_W<20> {
        PLULUT_W::new(self)
    }
    #[doc = "Bit 21 - Enables the clock for the Timer 3."]
    #[inline(always)]
    pub fn timer3(&mut self) -> TIMER3_W<21> {
        TIMER3_W::new(self)
    }
    #[doc = "Bit 22 - Enables the clock for the Timer 4."]
    #[inline(always)]
    pub fn timer4(&mut self) -> TIMER4_W<22> {
        TIMER4_W::new(self)
    }
    #[doc = "Bit 23 - Enables the clock for the PUF reset control."]
    #[inline(always)]
    pub fn puf(&mut self) -> PUF_W<23> {
        PUF_W::new(self)
    }
    #[doc = "Bit 24 - Enables the clock for the Casper."]
    #[inline(always)]
    pub fn casper(&mut self) -> CASPER_W<24> {
        CASPER_W::new(self)
    }
    #[doc = "Bit 27 - Enables the clock for the analog control."]
    #[inline(always)]
    pub fn analog_ctrl(&mut self) -> ANALOG_CTRL_W<27> {
        ANALOG_CTRL_W::new(self)
    }
    #[doc = "Bit 28 - Enables the clock for the HS LSPI."]
    #[inline(always)]
    pub fn hs_lspi(&mut self) -> HS_LSPI_W<28> {
        HS_LSPI_W::new(self)
    }
    #[doc = "Bit 29 - Enables the clock for the GPIO secure."]
    #[inline(always)]
    pub fn gpio_sec(&mut self) -> GPIO_SEC_W<29> {
        GPIO_SEC_W::new(self)
    }
    #[doc = "Bit 30 - Enables the clock for the GPIO secure int."]
    #[inline(always)]
    pub fn gpio_sec_int(&mut self) -> GPIO_SEC_INT_W<30> {
        GPIO_SEC_INT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AHB Clock control 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahbclkctrl_ahbclkctrl2](index.html) module"]
pub struct AHBCLKCTRL_AHBCLKCTRL2_SPEC;
impl crate::RegisterSpec for AHBCLKCTRL_AHBCLKCTRL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ahbclkctrl_ahbclkctrl2::R](R) reader structure"]
impl crate::Readable for AHBCLKCTRL_AHBCLKCTRL2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ahbclkctrl_ahbclkctrl2::W](W) writer structure"]
impl crate::Writable for AHBCLKCTRL_AHBCLKCTRL2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AHBCLKCTRL2 to value 0"]
impl crate::Resettable for AHBCLKCTRL_AHBCLKCTRL2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
