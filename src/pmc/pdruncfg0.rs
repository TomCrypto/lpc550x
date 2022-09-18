#[doc = "Register `PDRUNCFG0` reader"]
pub struct R(crate::R<PDRUNCFG0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDRUNCFG0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PDRUNCFG0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PDRUNCFG0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PDRUNCFG0` writer"]
pub struct W(crate::W<PDRUNCFG0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDRUNCFG0_SPEC>;
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
impl From<crate::W<PDRUNCFG0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PDRUNCFG0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PDEN_BODVBAT` reader - Controls power to VBAT Brown Out Detector (BOD)."]
pub type PDEN_BODVBAT_R = crate::BitReader<PDEN_BODVBAT_A>;
#[doc = "Controls power to VBAT Brown Out Detector (BOD).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDEN_BODVBAT_A {
    #[doc = "0: BOD VBAT is powered."]
    POWEREDON = 0,
    #[doc = "1: BOD VBAT is powered down."]
    POWEREDOFF = 1,
}
impl From<PDEN_BODVBAT_A> for bool {
    #[inline(always)]
    fn from(variant: PDEN_BODVBAT_A) -> Self {
        variant as u8 != 0
    }
}
impl PDEN_BODVBAT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDEN_BODVBAT_A {
        match self.bits {
            false => PDEN_BODVBAT_A::POWEREDON,
            true => PDEN_BODVBAT_A::POWEREDOFF,
        }
    }
    #[doc = "Checks if the value of the field is `POWEREDON`"]
    #[inline(always)]
    pub fn is_poweredon(&self) -> bool {
        *self == PDEN_BODVBAT_A::POWEREDON
    }
    #[doc = "Checks if the value of the field is `POWEREDOFF`"]
    #[inline(always)]
    pub fn is_poweredoff(&self) -> bool {
        *self == PDEN_BODVBAT_A::POWEREDOFF
    }
}
#[doc = "Field `PDEN_BODVBAT` writer - Controls power to VBAT Brown Out Detector (BOD)."]
pub type PDEN_BODVBAT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PDRUNCFG0_SPEC, PDEN_BODVBAT_A, O>;
impl<'a, const O: u8> PDEN_BODVBAT_W<'a, O> {
    #[doc = "BOD VBAT is powered."]
    #[inline(always)]
    pub fn poweredon(self) -> &'a mut W {
        self.variant(PDEN_BODVBAT_A::POWEREDON)
    }
    #[doc = "BOD VBAT is powered down."]
    #[inline(always)]
    pub fn poweredoff(self) -> &'a mut W {
        self.variant(PDEN_BODVBAT_A::POWEREDOFF)
    }
}
#[doc = "Field `PDEN_FRO32K` reader - Controls power to the Free Running Oscillator (FRO) 32 KHz."]
pub type PDEN_FRO32K_R = crate::BitReader<PDEN_FRO32K_A>;
#[doc = "Controls power to the Free Running Oscillator (FRO) 32 KHz.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDEN_FRO32K_A {
    #[doc = "0: FRO32KHz is powered."]
    POWEREDON = 0,
    #[doc = "1: FRO32KHz is powered down."]
    POWEREDOFF = 1,
}
impl From<PDEN_FRO32K_A> for bool {
    #[inline(always)]
    fn from(variant: PDEN_FRO32K_A) -> Self {
        variant as u8 != 0
    }
}
impl PDEN_FRO32K_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDEN_FRO32K_A {
        match self.bits {
            false => PDEN_FRO32K_A::POWEREDON,
            true => PDEN_FRO32K_A::POWEREDOFF,
        }
    }
    #[doc = "Checks if the value of the field is `POWEREDON`"]
    #[inline(always)]
    pub fn is_poweredon(&self) -> bool {
        *self == PDEN_FRO32K_A::POWEREDON
    }
    #[doc = "Checks if the value of the field is `POWEREDOFF`"]
    #[inline(always)]
    pub fn is_poweredoff(&self) -> bool {
        *self == PDEN_FRO32K_A::POWEREDOFF
    }
}
#[doc = "Field `PDEN_FRO32K` writer - Controls power to the Free Running Oscillator (FRO) 32 KHz."]
pub type PDEN_FRO32K_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PDRUNCFG0_SPEC, PDEN_FRO32K_A, O>;
impl<'a, const O: u8> PDEN_FRO32K_W<'a, O> {
    #[doc = "FRO32KHz is powered."]
    #[inline(always)]
    pub fn poweredon(self) -> &'a mut W {
        self.variant(PDEN_FRO32K_A::POWEREDON)
    }
    #[doc = "FRO32KHz is powered down."]
    #[inline(always)]
    pub fn poweredoff(self) -> &'a mut W {
        self.variant(PDEN_FRO32K_A::POWEREDOFF)
    }
}
#[doc = "Field `PDEN_XTAL32K` reader - Controls power to crystal 32 KHz."]
pub type PDEN_XTAL32K_R = crate::BitReader<PDEN_XTAL32K_A>;
#[doc = "Controls power to crystal 32 KHz.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDEN_XTAL32K_A {
    #[doc = "0: Crystal 32KHz is powered."]
    POWEREDON = 0,
    #[doc = "1: Crystal 32KHz is powered down."]
    POWEREDOFF = 1,
}
impl From<PDEN_XTAL32K_A> for bool {
    #[inline(always)]
    fn from(variant: PDEN_XTAL32K_A) -> Self {
        variant as u8 != 0
    }
}
impl PDEN_XTAL32K_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDEN_XTAL32K_A {
        match self.bits {
            false => PDEN_XTAL32K_A::POWEREDON,
            true => PDEN_XTAL32K_A::POWEREDOFF,
        }
    }
    #[doc = "Checks if the value of the field is `POWEREDON`"]
    #[inline(always)]
    pub fn is_poweredon(&self) -> bool {
        *self == PDEN_XTAL32K_A::POWEREDON
    }
    #[doc = "Checks if the value of the field is `POWEREDOFF`"]
    #[inline(always)]
    pub fn is_poweredoff(&self) -> bool {
        *self == PDEN_XTAL32K_A::POWEREDOFF
    }
}
#[doc = "Field `PDEN_XTAL32K` writer - Controls power to crystal 32 KHz."]
pub type PDEN_XTAL32K_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PDRUNCFG0_SPEC, PDEN_XTAL32K_A, O>;
impl<'a, const O: u8> PDEN_XTAL32K_W<'a, O> {
    #[doc = "Crystal 32KHz is powered."]
    #[inline(always)]
    pub fn poweredon(self) -> &'a mut W {
        self.variant(PDEN_XTAL32K_A::POWEREDON)
    }
    #[doc = "Crystal 32KHz is powered down."]
    #[inline(always)]
    pub fn poweredoff(self) -> &'a mut W {
        self.variant(PDEN_XTAL32K_A::POWEREDOFF)
    }
}
#[doc = "Field `PDEN_XTAL32M` reader - Controls power to high speed crystal."]
pub type PDEN_XTAL32M_R = crate::BitReader<PDEN_XTAL32M_A>;
#[doc = "Controls power to high speed crystal.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDEN_XTAL32M_A {
    #[doc = "0: High speed crystal is powered."]
    POWEREDON = 0,
    #[doc = "1: High speed crystal is powered down."]
    POWEREDOFF = 1,
}
impl From<PDEN_XTAL32M_A> for bool {
    #[inline(always)]
    fn from(variant: PDEN_XTAL32M_A) -> Self {
        variant as u8 != 0
    }
}
impl PDEN_XTAL32M_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDEN_XTAL32M_A {
        match self.bits {
            false => PDEN_XTAL32M_A::POWEREDON,
            true => PDEN_XTAL32M_A::POWEREDOFF,
        }
    }
    #[doc = "Checks if the value of the field is `POWEREDON`"]
    #[inline(always)]
    pub fn is_poweredon(&self) -> bool {
        *self == PDEN_XTAL32M_A::POWEREDON
    }
    #[doc = "Checks if the value of the field is `POWEREDOFF`"]
    #[inline(always)]
    pub fn is_poweredoff(&self) -> bool {
        *self == PDEN_XTAL32M_A::POWEREDOFF
    }
}
#[doc = "Field `PDEN_XTAL32M` writer - Controls power to high speed crystal."]
pub type PDEN_XTAL32M_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PDRUNCFG0_SPEC, PDEN_XTAL32M_A, O>;
impl<'a, const O: u8> PDEN_XTAL32M_W<'a, O> {
    #[doc = "High speed crystal is powered."]
    #[inline(always)]
    pub fn poweredon(self) -> &'a mut W {
        self.variant(PDEN_XTAL32M_A::POWEREDON)
    }
    #[doc = "High speed crystal is powered down."]
    #[inline(always)]
    pub fn poweredoff(self) -> &'a mut W {
        self.variant(PDEN_XTAL32M_A::POWEREDOFF)
    }
}
#[doc = "Field `PDEN_PLL0` reader - Controls power to PLL0."]
pub type PDEN_PLL0_R = crate::BitReader<PDEN_PLL0_A>;
#[doc = "Controls power to PLL0.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDEN_PLL0_A {
    #[doc = "0: PLL0 is powered."]
    POWEREDON = 0,
    #[doc = "1: PLL0 is powered down."]
    POWEREDOFF = 1,
}
impl From<PDEN_PLL0_A> for bool {
    #[inline(always)]
    fn from(variant: PDEN_PLL0_A) -> Self {
        variant as u8 != 0
    }
}
impl PDEN_PLL0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDEN_PLL0_A {
        match self.bits {
            false => PDEN_PLL0_A::POWEREDON,
            true => PDEN_PLL0_A::POWEREDOFF,
        }
    }
    #[doc = "Checks if the value of the field is `POWEREDON`"]
    #[inline(always)]
    pub fn is_poweredon(&self) -> bool {
        *self == PDEN_PLL0_A::POWEREDON
    }
    #[doc = "Checks if the value of the field is `POWEREDOFF`"]
    #[inline(always)]
    pub fn is_poweredoff(&self) -> bool {
        *self == PDEN_PLL0_A::POWEREDOFF
    }
}
#[doc = "Field `PDEN_PLL0` writer - Controls power to PLL0."]
pub type PDEN_PLL0_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDRUNCFG0_SPEC, PDEN_PLL0_A, O>;
impl<'a, const O: u8> PDEN_PLL0_W<'a, O> {
    #[doc = "PLL0 is powered."]
    #[inline(always)]
    pub fn poweredon(self) -> &'a mut W {
        self.variant(PDEN_PLL0_A::POWEREDON)
    }
    #[doc = "PLL0 is powered down."]
    #[inline(always)]
    pub fn poweredoff(self) -> &'a mut W {
        self.variant(PDEN_PLL0_A::POWEREDOFF)
    }
}
#[doc = "Field `PDEN_PLL1` reader - Controls power to PLL1."]
pub type PDEN_PLL1_R = crate::BitReader<PDEN_PLL1_A>;
#[doc = "Controls power to PLL1.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDEN_PLL1_A {
    #[doc = "0: PLL1 is powered."]
    POWEREDON = 0,
    #[doc = "1: PLL1 is powered down."]
    POWEREDOFF = 1,
}
impl From<PDEN_PLL1_A> for bool {
    #[inline(always)]
    fn from(variant: PDEN_PLL1_A) -> Self {
        variant as u8 != 0
    }
}
impl PDEN_PLL1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDEN_PLL1_A {
        match self.bits {
            false => PDEN_PLL1_A::POWEREDON,
            true => PDEN_PLL1_A::POWEREDOFF,
        }
    }
    #[doc = "Checks if the value of the field is `POWEREDON`"]
    #[inline(always)]
    pub fn is_poweredon(&self) -> bool {
        *self == PDEN_PLL1_A::POWEREDON
    }
    #[doc = "Checks if the value of the field is `POWEREDOFF`"]
    #[inline(always)]
    pub fn is_poweredoff(&self) -> bool {
        *self == PDEN_PLL1_A::POWEREDOFF
    }
}
#[doc = "Field `PDEN_PLL1` writer - Controls power to PLL1."]
pub type PDEN_PLL1_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDRUNCFG0_SPEC, PDEN_PLL1_A, O>;
impl<'a, const O: u8> PDEN_PLL1_W<'a, O> {
    #[doc = "PLL1 is powered."]
    #[inline(always)]
    pub fn poweredon(self) -> &'a mut W {
        self.variant(PDEN_PLL1_A::POWEREDON)
    }
    #[doc = "PLL1 is powered down."]
    #[inline(always)]
    pub fn poweredoff(self) -> &'a mut W {
        self.variant(PDEN_PLL1_A::POWEREDOFF)
    }
}
#[doc = "Field `PDEN_COMP` reader - Controls power to Analog Comparator."]
pub type PDEN_COMP_R = crate::BitReader<PDEN_COMP_A>;
#[doc = "Controls power to Analog Comparator.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDEN_COMP_A {
    #[doc = "0: Analog Comparator is powered."]
    POWEREDON = 0,
    #[doc = "1: Analog Comparator is powered down."]
    POWEREDOFF = 1,
}
impl From<PDEN_COMP_A> for bool {
    #[inline(always)]
    fn from(variant: PDEN_COMP_A) -> Self {
        variant as u8 != 0
    }
}
impl PDEN_COMP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDEN_COMP_A {
        match self.bits {
            false => PDEN_COMP_A::POWEREDON,
            true => PDEN_COMP_A::POWEREDOFF,
        }
    }
    #[doc = "Checks if the value of the field is `POWEREDON`"]
    #[inline(always)]
    pub fn is_poweredon(&self) -> bool {
        *self == PDEN_COMP_A::POWEREDON
    }
    #[doc = "Checks if the value of the field is `POWEREDOFF`"]
    #[inline(always)]
    pub fn is_poweredoff(&self) -> bool {
        *self == PDEN_COMP_A::POWEREDOFF
    }
}
#[doc = "Field `PDEN_COMP` writer - Controls power to Analog Comparator."]
pub type PDEN_COMP_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDRUNCFG0_SPEC, PDEN_COMP_A, O>;
impl<'a, const O: u8> PDEN_COMP_W<'a, O> {
    #[doc = "Analog Comparator is powered."]
    #[inline(always)]
    pub fn poweredon(self) -> &'a mut W {
        self.variant(PDEN_COMP_A::POWEREDON)
    }
    #[doc = "Analog Comparator is powered down."]
    #[inline(always)]
    pub fn poweredoff(self) -> &'a mut W {
        self.variant(PDEN_COMP_A::POWEREDOFF)
    }
}
#[doc = "Field `PDEN_AUXBIAS` reader - Controls power to auxiliary biasing (AUXBIAS)"]
pub type PDEN_AUXBIAS_R = crate::BitReader<PDEN_AUXBIAS_A>;
#[doc = "Controls power to auxiliary biasing (AUXBIAS)\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDEN_AUXBIAS_A {
    #[doc = "0: auxiliary biasing is powered."]
    POWEREDON = 0,
    #[doc = "1: auxiliary biasing is powered down."]
    POWEREDOFF = 1,
}
impl From<PDEN_AUXBIAS_A> for bool {
    #[inline(always)]
    fn from(variant: PDEN_AUXBIAS_A) -> Self {
        variant as u8 != 0
    }
}
impl PDEN_AUXBIAS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDEN_AUXBIAS_A {
        match self.bits {
            false => PDEN_AUXBIAS_A::POWEREDON,
            true => PDEN_AUXBIAS_A::POWEREDOFF,
        }
    }
    #[doc = "Checks if the value of the field is `POWEREDON`"]
    #[inline(always)]
    pub fn is_poweredon(&self) -> bool {
        *self == PDEN_AUXBIAS_A::POWEREDON
    }
    #[doc = "Checks if the value of the field is `POWEREDOFF`"]
    #[inline(always)]
    pub fn is_poweredoff(&self) -> bool {
        *self == PDEN_AUXBIAS_A::POWEREDOFF
    }
}
#[doc = "Field `PDEN_AUXBIAS` writer - Controls power to auxiliary biasing (AUXBIAS)"]
pub type PDEN_AUXBIAS_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PDRUNCFG0_SPEC, PDEN_AUXBIAS_A, O>;
impl<'a, const O: u8> PDEN_AUXBIAS_W<'a, O> {
    #[doc = "auxiliary biasing is powered."]
    #[inline(always)]
    pub fn poweredon(self) -> &'a mut W {
        self.variant(PDEN_AUXBIAS_A::POWEREDON)
    }
    #[doc = "auxiliary biasing is powered down."]
    #[inline(always)]
    pub fn poweredoff(self) -> &'a mut W {
        self.variant(PDEN_AUXBIAS_A::POWEREDOFF)
    }
}
#[doc = "Field `PDEN_LDOXO32M` reader - Controls power to high speed crystal LDO."]
pub type PDEN_LDOXO32M_R = crate::BitReader<PDEN_LDOXO32M_A>;
#[doc = "Controls power to high speed crystal LDO.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDEN_LDOXO32M_A {
    #[doc = "0: High speed crystal LDO is powered."]
    POWEREDON = 0,
    #[doc = "1: High speed crystal LDO is powered down."]
    POWEREDOFF = 1,
}
impl From<PDEN_LDOXO32M_A> for bool {
    #[inline(always)]
    fn from(variant: PDEN_LDOXO32M_A) -> Self {
        variant as u8 != 0
    }
}
impl PDEN_LDOXO32M_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDEN_LDOXO32M_A {
        match self.bits {
            false => PDEN_LDOXO32M_A::POWEREDON,
            true => PDEN_LDOXO32M_A::POWEREDOFF,
        }
    }
    #[doc = "Checks if the value of the field is `POWEREDON`"]
    #[inline(always)]
    pub fn is_poweredon(&self) -> bool {
        *self == PDEN_LDOXO32M_A::POWEREDON
    }
    #[doc = "Checks if the value of the field is `POWEREDOFF`"]
    #[inline(always)]
    pub fn is_poweredoff(&self) -> bool {
        *self == PDEN_LDOXO32M_A::POWEREDOFF
    }
}
#[doc = "Field `PDEN_LDOXO32M` writer - Controls power to high speed crystal LDO."]
pub type PDEN_LDOXO32M_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PDRUNCFG0_SPEC, PDEN_LDOXO32M_A, O>;
impl<'a, const O: u8> PDEN_LDOXO32M_W<'a, O> {
    #[doc = "High speed crystal LDO is powered."]
    #[inline(always)]
    pub fn poweredon(self) -> &'a mut W {
        self.variant(PDEN_LDOXO32M_A::POWEREDON)
    }
    #[doc = "High speed crystal LDO is powered down."]
    #[inline(always)]
    pub fn poweredoff(self) -> &'a mut W {
        self.variant(PDEN_LDOXO32M_A::POWEREDOFF)
    }
}
#[doc = "Field `PDEN_RNG` reader - Controls power to all True Random Number Genetaor (TRNG) clock sources."]
pub type PDEN_RNG_R = crate::BitReader<PDEN_RNG_A>;
#[doc = "Controls power to all True Random Number Genetaor (TRNG) clock sources.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDEN_RNG_A {
    #[doc = "0: TRNG clocks are powered."]
    POWEREDON = 0,
    #[doc = "1: TRNG clocks are powered down."]
    POWEREDOFF = 1,
}
impl From<PDEN_RNG_A> for bool {
    #[inline(always)]
    fn from(variant: PDEN_RNG_A) -> Self {
        variant as u8 != 0
    }
}
impl PDEN_RNG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDEN_RNG_A {
        match self.bits {
            false => PDEN_RNG_A::POWEREDON,
            true => PDEN_RNG_A::POWEREDOFF,
        }
    }
    #[doc = "Checks if the value of the field is `POWEREDON`"]
    #[inline(always)]
    pub fn is_poweredon(&self) -> bool {
        *self == PDEN_RNG_A::POWEREDON
    }
    #[doc = "Checks if the value of the field is `POWEREDOFF`"]
    #[inline(always)]
    pub fn is_poweredoff(&self) -> bool {
        *self == PDEN_RNG_A::POWEREDOFF
    }
}
#[doc = "Field `PDEN_RNG` writer - Controls power to all True Random Number Genetaor (TRNG) clock sources."]
pub type PDEN_RNG_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDRUNCFG0_SPEC, PDEN_RNG_A, O>;
impl<'a, const O: u8> PDEN_RNG_W<'a, O> {
    #[doc = "TRNG clocks are powered."]
    #[inline(always)]
    pub fn poweredon(self) -> &'a mut W {
        self.variant(PDEN_RNG_A::POWEREDON)
    }
    #[doc = "TRNG clocks are powered down."]
    #[inline(always)]
    pub fn poweredoff(self) -> &'a mut W {
        self.variant(PDEN_RNG_A::POWEREDOFF)
    }
}
#[doc = "Field `PDEN_PLL0_SSCG` reader - Controls power to PLL0 Spread Spectrum module."]
pub type PDEN_PLL0_SSCG_R = crate::BitReader<PDEN_PLL0_SSCG_A>;
#[doc = "Controls power to PLL0 Spread Spectrum module.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDEN_PLL0_SSCG_A {
    #[doc = "0: PLL0 Sread spectrum module is powered."]
    POWEREDON = 0,
    #[doc = "1: PLL0 Sread spectrum module is powered down."]
    POWEREDOFF = 1,
}
impl From<PDEN_PLL0_SSCG_A> for bool {
    #[inline(always)]
    fn from(variant: PDEN_PLL0_SSCG_A) -> Self {
        variant as u8 != 0
    }
}
impl PDEN_PLL0_SSCG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDEN_PLL0_SSCG_A {
        match self.bits {
            false => PDEN_PLL0_SSCG_A::POWEREDON,
            true => PDEN_PLL0_SSCG_A::POWEREDOFF,
        }
    }
    #[doc = "Checks if the value of the field is `POWEREDON`"]
    #[inline(always)]
    pub fn is_poweredon(&self) -> bool {
        *self == PDEN_PLL0_SSCG_A::POWEREDON
    }
    #[doc = "Checks if the value of the field is `POWEREDOFF`"]
    #[inline(always)]
    pub fn is_poweredoff(&self) -> bool {
        *self == PDEN_PLL0_SSCG_A::POWEREDOFF
    }
}
#[doc = "Field `PDEN_PLL0_SSCG` writer - Controls power to PLL0 Spread Spectrum module."]
pub type PDEN_PLL0_SSCG_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PDRUNCFG0_SPEC, PDEN_PLL0_SSCG_A, O>;
impl<'a, const O: u8> PDEN_PLL0_SSCG_W<'a, O> {
    #[doc = "PLL0 Sread spectrum module is powered."]
    #[inline(always)]
    pub fn poweredon(self) -> &'a mut W {
        self.variant(PDEN_PLL0_SSCG_A::POWEREDON)
    }
    #[doc = "PLL0 Sread spectrum module is powered down."]
    #[inline(always)]
    pub fn poweredoff(self) -> &'a mut W {
        self.variant(PDEN_PLL0_SSCG_A::POWEREDOFF)
    }
}
impl R {
    #[doc = "Bit 3 - Controls power to VBAT Brown Out Detector (BOD)."]
    #[inline(always)]
    pub fn pden_bodvbat(&self) -> PDEN_BODVBAT_R {
        PDEN_BODVBAT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 6 - Controls power to the Free Running Oscillator (FRO) 32 KHz."]
    #[inline(always)]
    pub fn pden_fro32k(&self) -> PDEN_FRO32K_R {
        PDEN_FRO32K_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Controls power to crystal 32 KHz."]
    #[inline(always)]
    pub fn pden_xtal32k(&self) -> PDEN_XTAL32K_R {
        PDEN_XTAL32K_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Controls power to high speed crystal."]
    #[inline(always)]
    pub fn pden_xtal32m(&self) -> PDEN_XTAL32M_R {
        PDEN_XTAL32M_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Controls power to PLL0."]
    #[inline(always)]
    pub fn pden_pll0(&self) -> PDEN_PLL0_R {
        PDEN_PLL0_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Controls power to PLL1."]
    #[inline(always)]
    pub fn pden_pll1(&self) -> PDEN_PLL1_R {
        PDEN_PLL1_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 13 - Controls power to Analog Comparator."]
    #[inline(always)]
    pub fn pden_comp(&self) -> PDEN_COMP_R {
        PDEN_COMP_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 19 - Controls power to auxiliary biasing (AUXBIAS)"]
    #[inline(always)]
    pub fn pden_auxbias(&self) -> PDEN_AUXBIAS_R {
        PDEN_AUXBIAS_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Controls power to high speed crystal LDO."]
    #[inline(always)]
    pub fn pden_ldoxo32m(&self) -> PDEN_LDOXO32M_R {
        PDEN_LDOXO32M_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 22 - Controls power to all True Random Number Genetaor (TRNG) clock sources."]
    #[inline(always)]
    pub fn pden_rng(&self) -> PDEN_RNG_R {
        PDEN_RNG_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Controls power to PLL0 Spread Spectrum module."]
    #[inline(always)]
    pub fn pden_pll0_sscg(&self) -> PDEN_PLL0_SSCG_R {
        PDEN_PLL0_SSCG_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - Controls power to VBAT Brown Out Detector (BOD)."]
    #[inline(always)]
    pub fn pden_bodvbat(&mut self) -> PDEN_BODVBAT_W<3> {
        PDEN_BODVBAT_W::new(self)
    }
    #[doc = "Bit 6 - Controls power to the Free Running Oscillator (FRO) 32 KHz."]
    #[inline(always)]
    pub fn pden_fro32k(&mut self) -> PDEN_FRO32K_W<6> {
        PDEN_FRO32K_W::new(self)
    }
    #[doc = "Bit 7 - Controls power to crystal 32 KHz."]
    #[inline(always)]
    pub fn pden_xtal32k(&mut self) -> PDEN_XTAL32K_W<7> {
        PDEN_XTAL32K_W::new(self)
    }
    #[doc = "Bit 8 - Controls power to high speed crystal."]
    #[inline(always)]
    pub fn pden_xtal32m(&mut self) -> PDEN_XTAL32M_W<8> {
        PDEN_XTAL32M_W::new(self)
    }
    #[doc = "Bit 9 - Controls power to PLL0."]
    #[inline(always)]
    pub fn pden_pll0(&mut self) -> PDEN_PLL0_W<9> {
        PDEN_PLL0_W::new(self)
    }
    #[doc = "Bit 10 - Controls power to PLL1."]
    #[inline(always)]
    pub fn pden_pll1(&mut self) -> PDEN_PLL1_W<10> {
        PDEN_PLL1_W::new(self)
    }
    #[doc = "Bit 13 - Controls power to Analog Comparator."]
    #[inline(always)]
    pub fn pden_comp(&mut self) -> PDEN_COMP_W<13> {
        PDEN_COMP_W::new(self)
    }
    #[doc = "Bit 19 - Controls power to auxiliary biasing (AUXBIAS)"]
    #[inline(always)]
    pub fn pden_auxbias(&mut self) -> PDEN_AUXBIAS_W<19> {
        PDEN_AUXBIAS_W::new(self)
    }
    #[doc = "Bit 20 - Controls power to high speed crystal LDO."]
    #[inline(always)]
    pub fn pden_ldoxo32m(&mut self) -> PDEN_LDOXO32M_W<20> {
        PDEN_LDOXO32M_W::new(self)
    }
    #[doc = "Bit 22 - Controls power to all True Random Number Genetaor (TRNG) clock sources."]
    #[inline(always)]
    pub fn pden_rng(&mut self) -> PDEN_RNG_W<22> {
        PDEN_RNG_W::new(self)
    }
    #[doc = "Bit 23 - Controls power to PLL0 Spread Spectrum module."]
    #[inline(always)]
    pub fn pden_pll0_sscg(&mut self) -> PDEN_PLL0_SSCG_W<23> {
        PDEN_PLL0_SSCG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Controls the power to various analog blocks \\[Reset by: PoR, Pin Reset, Brown Out Detectors Reset, Deep Power Down Reset, Software Reset\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdruncfg0](index.html) module"]
pub struct PDRUNCFG0_SPEC;
impl crate::RegisterSpec for PDRUNCFG0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pdruncfg0::R](R) reader structure"]
impl crate::Readable for PDRUNCFG0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pdruncfg0::W](W) writer structure"]
impl crate::Writable for PDRUNCFG0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PDRUNCFG0 to value 0x00de_ffc0"]
impl crate::Resettable for PDRUNCFG0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x00de_ffc0
    }
}
