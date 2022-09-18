#[doc = "Register `DIR[%s]` reader"]
pub struct R(crate::R<DIR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DIR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DIR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DIR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DIR[%s]` writer"]
pub struct W(crate::W<DIR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DIR_SPEC>;
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
impl From<crate::W<DIR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DIR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PIO0` reader - Selects pin direction for pin PIOn_0."]
pub type PIO0_R = crate::BitReader<PIO0_A>;
#[doc = "Selects pin direction for pin PIOn_0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIO0_A {
    #[doc = "0: Pin is an input."]
    INPUT = 0,
    #[doc = "1: Pin is an output."]
    OUTPUT = 1,
}
impl From<PIO0_A> for bool {
    #[inline(always)]
    fn from(variant: PIO0_A) -> Self {
        variant as u8 != 0
    }
}
impl PIO0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIO0_A {
        match self.bits {
            false => PIO0_A::INPUT,
            true => PIO0_A::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == PIO0_A::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == PIO0_A::OUTPUT
    }
}
#[doc = "Field `PIO0` writer - Selects pin direction for pin PIOn_0."]
pub type PIO0_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIR_SPEC, PIO0_A, O>;
impl<'a, const O: u8> PIO0_W<'a, O> {
    #[doc = "Pin is an input."]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(PIO0_A::INPUT)
    }
    #[doc = "Pin is an output."]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(PIO0_A::OUTPUT)
    }
}
#[doc = "Field `PIO1` reader - Selects pin direction for pin PIOn_1."]
pub type PIO1_R = crate::BitReader<PIO1_A>;
#[doc = "Selects pin direction for pin PIOn_1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIO1_A {
    #[doc = "0: Pin is an input."]
    INPUT = 0,
    #[doc = "1: Pin is an output."]
    OUTPUT = 1,
}
impl From<PIO1_A> for bool {
    #[inline(always)]
    fn from(variant: PIO1_A) -> Self {
        variant as u8 != 0
    }
}
impl PIO1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIO1_A {
        match self.bits {
            false => PIO1_A::INPUT,
            true => PIO1_A::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == PIO1_A::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == PIO1_A::OUTPUT
    }
}
#[doc = "Field `PIO1` writer - Selects pin direction for pin PIOn_1."]
pub type PIO1_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIR_SPEC, PIO1_A, O>;
impl<'a, const O: u8> PIO1_W<'a, O> {
    #[doc = "Pin is an input."]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(PIO1_A::INPUT)
    }
    #[doc = "Pin is an output."]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(PIO1_A::OUTPUT)
    }
}
#[doc = "Field `PIO2` reader - Selects pin direction for pin PIOn_2."]
pub type PIO2_R = crate::BitReader<PIO2_A>;
#[doc = "Selects pin direction for pin PIOn_2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIO2_A {
    #[doc = "0: Pin is an input."]
    INPUT = 0,
    #[doc = "1: Pin is an output."]
    OUTPUT = 1,
}
impl From<PIO2_A> for bool {
    #[inline(always)]
    fn from(variant: PIO2_A) -> Self {
        variant as u8 != 0
    }
}
impl PIO2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIO2_A {
        match self.bits {
            false => PIO2_A::INPUT,
            true => PIO2_A::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == PIO2_A::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == PIO2_A::OUTPUT
    }
}
#[doc = "Field `PIO2` writer - Selects pin direction for pin PIOn_2."]
pub type PIO2_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIR_SPEC, PIO2_A, O>;
impl<'a, const O: u8> PIO2_W<'a, O> {
    #[doc = "Pin is an input."]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(PIO2_A::INPUT)
    }
    #[doc = "Pin is an output."]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(PIO2_A::OUTPUT)
    }
}
#[doc = "Field `PIO3` reader - Selects pin direction for pin PIOn_3."]
pub type PIO3_R = crate::BitReader<PIO3_A>;
#[doc = "Selects pin direction for pin PIOn_3.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIO3_A {
    #[doc = "0: Pin is an input."]
    INPUT = 0,
    #[doc = "1: Pin is an output."]
    OUTPUT = 1,
}
impl From<PIO3_A> for bool {
    #[inline(always)]
    fn from(variant: PIO3_A) -> Self {
        variant as u8 != 0
    }
}
impl PIO3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIO3_A {
        match self.bits {
            false => PIO3_A::INPUT,
            true => PIO3_A::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == PIO3_A::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == PIO3_A::OUTPUT
    }
}
#[doc = "Field `PIO3` writer - Selects pin direction for pin PIOn_3."]
pub type PIO3_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIR_SPEC, PIO3_A, O>;
impl<'a, const O: u8> PIO3_W<'a, O> {
    #[doc = "Pin is an input."]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(PIO3_A::INPUT)
    }
    #[doc = "Pin is an output."]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(PIO3_A::OUTPUT)
    }
}
#[doc = "Field `PIO4` reader - Selects pin direction for pin PIOn_4."]
pub type PIO4_R = crate::BitReader<PIO4_A>;
#[doc = "Selects pin direction for pin PIOn_4.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIO4_A {
    #[doc = "0: Pin is an input."]
    INPUT = 0,
    #[doc = "1: Pin is an output."]
    OUTPUT = 1,
}
impl From<PIO4_A> for bool {
    #[inline(always)]
    fn from(variant: PIO4_A) -> Self {
        variant as u8 != 0
    }
}
impl PIO4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIO4_A {
        match self.bits {
            false => PIO4_A::INPUT,
            true => PIO4_A::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == PIO4_A::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == PIO4_A::OUTPUT
    }
}
#[doc = "Field `PIO4` writer - Selects pin direction for pin PIOn_4."]
pub type PIO4_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIR_SPEC, PIO4_A, O>;
impl<'a, const O: u8> PIO4_W<'a, O> {
    #[doc = "Pin is an input."]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(PIO4_A::INPUT)
    }
    #[doc = "Pin is an output."]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(PIO4_A::OUTPUT)
    }
}
#[doc = "Field `PIO5` reader - Selects pin direction for pin PIOn_5."]
pub type PIO5_R = crate::BitReader<PIO5_A>;
#[doc = "Selects pin direction for pin PIOn_5.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIO5_A {
    #[doc = "0: Pin is an input."]
    INPUT = 0,
    #[doc = "1: Pin is an output."]
    OUTPUT = 1,
}
impl From<PIO5_A> for bool {
    #[inline(always)]
    fn from(variant: PIO5_A) -> Self {
        variant as u8 != 0
    }
}
impl PIO5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIO5_A {
        match self.bits {
            false => PIO5_A::INPUT,
            true => PIO5_A::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == PIO5_A::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == PIO5_A::OUTPUT
    }
}
#[doc = "Field `PIO5` writer - Selects pin direction for pin PIOn_5."]
pub type PIO5_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIR_SPEC, PIO5_A, O>;
impl<'a, const O: u8> PIO5_W<'a, O> {
    #[doc = "Pin is an input."]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(PIO5_A::INPUT)
    }
    #[doc = "Pin is an output."]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(PIO5_A::OUTPUT)
    }
}
#[doc = "Field `PIO6` reader - Selects pin direction for pin PIOn_6."]
pub type PIO6_R = crate::BitReader<PIO6_A>;
#[doc = "Selects pin direction for pin PIOn_6.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIO6_A {
    #[doc = "0: Pin is an input."]
    INPUT = 0,
    #[doc = "1: Pin is an output."]
    OUTPUT = 1,
}
impl From<PIO6_A> for bool {
    #[inline(always)]
    fn from(variant: PIO6_A) -> Self {
        variant as u8 != 0
    }
}
impl PIO6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIO6_A {
        match self.bits {
            false => PIO6_A::INPUT,
            true => PIO6_A::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == PIO6_A::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == PIO6_A::OUTPUT
    }
}
#[doc = "Field `PIO6` writer - Selects pin direction for pin PIOn_6."]
pub type PIO6_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIR_SPEC, PIO6_A, O>;
impl<'a, const O: u8> PIO6_W<'a, O> {
    #[doc = "Pin is an input."]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(PIO6_A::INPUT)
    }
    #[doc = "Pin is an output."]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(PIO6_A::OUTPUT)
    }
}
#[doc = "Field `PIO7` reader - Selects pin direction for pin PIOn_7."]
pub type PIO7_R = crate::BitReader<PIO7_A>;
#[doc = "Selects pin direction for pin PIOn_7.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIO7_A {
    #[doc = "0: Pin is an input."]
    INPUT = 0,
    #[doc = "1: Pin is an output."]
    OUTPUT = 1,
}
impl From<PIO7_A> for bool {
    #[inline(always)]
    fn from(variant: PIO7_A) -> Self {
        variant as u8 != 0
    }
}
impl PIO7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIO7_A {
        match self.bits {
            false => PIO7_A::INPUT,
            true => PIO7_A::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == PIO7_A::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == PIO7_A::OUTPUT
    }
}
#[doc = "Field `PIO7` writer - Selects pin direction for pin PIOn_7."]
pub type PIO7_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIR_SPEC, PIO7_A, O>;
impl<'a, const O: u8> PIO7_W<'a, O> {
    #[doc = "Pin is an input."]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(PIO7_A::INPUT)
    }
    #[doc = "Pin is an output."]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(PIO7_A::OUTPUT)
    }
}
#[doc = "Field `PIO8` reader - Selects pin direction for pin PIOn_8."]
pub type PIO8_R = crate::BitReader<PIO8_A>;
#[doc = "Selects pin direction for pin PIOn_8.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIO8_A {
    #[doc = "0: Pin is an input."]
    INPUT = 0,
    #[doc = "1: Pin is an output."]
    OUTPUT = 1,
}
impl From<PIO8_A> for bool {
    #[inline(always)]
    fn from(variant: PIO8_A) -> Self {
        variant as u8 != 0
    }
}
impl PIO8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIO8_A {
        match self.bits {
            false => PIO8_A::INPUT,
            true => PIO8_A::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == PIO8_A::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == PIO8_A::OUTPUT
    }
}
#[doc = "Field `PIO8` writer - Selects pin direction for pin PIOn_8."]
pub type PIO8_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIR_SPEC, PIO8_A, O>;
impl<'a, const O: u8> PIO8_W<'a, O> {
    #[doc = "Pin is an input."]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(PIO8_A::INPUT)
    }
    #[doc = "Pin is an output."]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(PIO8_A::OUTPUT)
    }
}
#[doc = "Field `PIO9` reader - Selects pin direction for pin PIOn_9."]
pub type PIO9_R = crate::BitReader<PIO9_A>;
#[doc = "Selects pin direction for pin PIOn_9.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIO9_A {
    #[doc = "0: Pin is an input."]
    INPUT = 0,
    #[doc = "1: Pin is an output."]
    OUTPUT = 1,
}
impl From<PIO9_A> for bool {
    #[inline(always)]
    fn from(variant: PIO9_A) -> Self {
        variant as u8 != 0
    }
}
impl PIO9_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIO9_A {
        match self.bits {
            false => PIO9_A::INPUT,
            true => PIO9_A::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == PIO9_A::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == PIO9_A::OUTPUT
    }
}
#[doc = "Field `PIO9` writer - Selects pin direction for pin PIOn_9."]
pub type PIO9_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIR_SPEC, PIO9_A, O>;
impl<'a, const O: u8> PIO9_W<'a, O> {
    #[doc = "Pin is an input."]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(PIO9_A::INPUT)
    }
    #[doc = "Pin is an output."]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(PIO9_A::OUTPUT)
    }
}
#[doc = "Field `PIO10` reader - Selects pin direction for pin PIOn_10."]
pub type PIO10_R = crate::BitReader<PIO10_A>;
#[doc = "Selects pin direction for pin PIOn_10.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIO10_A {
    #[doc = "0: Pin is an input."]
    INPUT = 0,
    #[doc = "1: Pin is an output."]
    OUTPUT = 1,
}
impl From<PIO10_A> for bool {
    #[inline(always)]
    fn from(variant: PIO10_A) -> Self {
        variant as u8 != 0
    }
}
impl PIO10_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIO10_A {
        match self.bits {
            false => PIO10_A::INPUT,
            true => PIO10_A::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == PIO10_A::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == PIO10_A::OUTPUT
    }
}
#[doc = "Field `PIO10` writer - Selects pin direction for pin PIOn_10."]
pub type PIO10_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIR_SPEC, PIO10_A, O>;
impl<'a, const O: u8> PIO10_W<'a, O> {
    #[doc = "Pin is an input."]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(PIO10_A::INPUT)
    }
    #[doc = "Pin is an output."]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(PIO10_A::OUTPUT)
    }
}
#[doc = "Field `PIO11` reader - Selects pin direction for pin PIOn_11."]
pub type PIO11_R = crate::BitReader<PIO11_A>;
#[doc = "Selects pin direction for pin PIOn_11.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIO11_A {
    #[doc = "0: Pin is an input."]
    INPUT = 0,
    #[doc = "1: Pin is an output."]
    OUTPUT = 1,
}
impl From<PIO11_A> for bool {
    #[inline(always)]
    fn from(variant: PIO11_A) -> Self {
        variant as u8 != 0
    }
}
impl PIO11_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIO11_A {
        match self.bits {
            false => PIO11_A::INPUT,
            true => PIO11_A::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == PIO11_A::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == PIO11_A::OUTPUT
    }
}
#[doc = "Field `PIO11` writer - Selects pin direction for pin PIOn_11."]
pub type PIO11_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIR_SPEC, PIO11_A, O>;
impl<'a, const O: u8> PIO11_W<'a, O> {
    #[doc = "Pin is an input."]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(PIO11_A::INPUT)
    }
    #[doc = "Pin is an output."]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(PIO11_A::OUTPUT)
    }
}
#[doc = "Field `PIO12` reader - Selects pin direction for pin PIOn_12."]
pub type PIO12_R = crate::BitReader<PIO12_A>;
#[doc = "Selects pin direction for pin PIOn_12.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIO12_A {
    #[doc = "0: Pin is an input."]
    INPUT = 0,
    #[doc = "1: Pin is an output."]
    OUTPUT = 1,
}
impl From<PIO12_A> for bool {
    #[inline(always)]
    fn from(variant: PIO12_A) -> Self {
        variant as u8 != 0
    }
}
impl PIO12_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIO12_A {
        match self.bits {
            false => PIO12_A::INPUT,
            true => PIO12_A::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == PIO12_A::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == PIO12_A::OUTPUT
    }
}
#[doc = "Field `PIO12` writer - Selects pin direction for pin PIOn_12."]
pub type PIO12_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIR_SPEC, PIO12_A, O>;
impl<'a, const O: u8> PIO12_W<'a, O> {
    #[doc = "Pin is an input."]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(PIO12_A::INPUT)
    }
    #[doc = "Pin is an output."]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(PIO12_A::OUTPUT)
    }
}
#[doc = "Field `PIO13` reader - Selects pin direction for pin PIOn_13."]
pub type PIO13_R = crate::BitReader<PIO13_A>;
#[doc = "Selects pin direction for pin PIOn_13.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIO13_A {
    #[doc = "0: Pin is an input."]
    INPUT = 0,
    #[doc = "1: Pin is an output."]
    OUTPUT = 1,
}
impl From<PIO13_A> for bool {
    #[inline(always)]
    fn from(variant: PIO13_A) -> Self {
        variant as u8 != 0
    }
}
impl PIO13_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIO13_A {
        match self.bits {
            false => PIO13_A::INPUT,
            true => PIO13_A::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == PIO13_A::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == PIO13_A::OUTPUT
    }
}
#[doc = "Field `PIO13` writer - Selects pin direction for pin PIOn_13."]
pub type PIO13_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIR_SPEC, PIO13_A, O>;
impl<'a, const O: u8> PIO13_W<'a, O> {
    #[doc = "Pin is an input."]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(PIO13_A::INPUT)
    }
    #[doc = "Pin is an output."]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(PIO13_A::OUTPUT)
    }
}
#[doc = "Field `PIO14` reader - Selects pin direction for pin PIOn_14."]
pub type PIO14_R = crate::BitReader<PIO14_A>;
#[doc = "Selects pin direction for pin PIOn_14.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIO14_A {
    #[doc = "0: Pin is an input."]
    INPUT = 0,
    #[doc = "1: Pin is an output."]
    OUTPUT = 1,
}
impl From<PIO14_A> for bool {
    #[inline(always)]
    fn from(variant: PIO14_A) -> Self {
        variant as u8 != 0
    }
}
impl PIO14_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIO14_A {
        match self.bits {
            false => PIO14_A::INPUT,
            true => PIO14_A::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == PIO14_A::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == PIO14_A::OUTPUT
    }
}
#[doc = "Field `PIO14` writer - Selects pin direction for pin PIOn_14."]
pub type PIO14_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIR_SPEC, PIO14_A, O>;
impl<'a, const O: u8> PIO14_W<'a, O> {
    #[doc = "Pin is an input."]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(PIO14_A::INPUT)
    }
    #[doc = "Pin is an output."]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(PIO14_A::OUTPUT)
    }
}
#[doc = "Field `PIO15` reader - Selects pin direction for pin PIOn_15."]
pub type PIO15_R = crate::BitReader<PIO15_A>;
#[doc = "Selects pin direction for pin PIOn_15.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIO15_A {
    #[doc = "0: Pin is an input."]
    INPUT = 0,
    #[doc = "1: Pin is an output."]
    OUTPUT = 1,
}
impl From<PIO15_A> for bool {
    #[inline(always)]
    fn from(variant: PIO15_A) -> Self {
        variant as u8 != 0
    }
}
impl PIO15_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIO15_A {
        match self.bits {
            false => PIO15_A::INPUT,
            true => PIO15_A::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == PIO15_A::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == PIO15_A::OUTPUT
    }
}
#[doc = "Field `PIO15` writer - Selects pin direction for pin PIOn_15."]
pub type PIO15_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIR_SPEC, PIO15_A, O>;
impl<'a, const O: u8> PIO15_W<'a, O> {
    #[doc = "Pin is an input."]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(PIO15_A::INPUT)
    }
    #[doc = "Pin is an output."]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(PIO15_A::OUTPUT)
    }
}
#[doc = "Field `PIO16` reader - Selects pin direction for pin PIOn_16."]
pub type PIO16_R = crate::BitReader<PIO16_A>;
#[doc = "Selects pin direction for pin PIOn_16.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIO16_A {
    #[doc = "0: Pin is an input."]
    INPUT = 0,
    #[doc = "1: Pin is an output."]
    OUTPUT = 1,
}
impl From<PIO16_A> for bool {
    #[inline(always)]
    fn from(variant: PIO16_A) -> Self {
        variant as u8 != 0
    }
}
impl PIO16_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIO16_A {
        match self.bits {
            false => PIO16_A::INPUT,
            true => PIO16_A::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == PIO16_A::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == PIO16_A::OUTPUT
    }
}
#[doc = "Field `PIO16` writer - Selects pin direction for pin PIOn_16."]
pub type PIO16_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIR_SPEC, PIO16_A, O>;
impl<'a, const O: u8> PIO16_W<'a, O> {
    #[doc = "Pin is an input."]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(PIO16_A::INPUT)
    }
    #[doc = "Pin is an output."]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(PIO16_A::OUTPUT)
    }
}
#[doc = "Field `PIO17` reader - Selects pin direction for pin PIOn_17."]
pub type PIO17_R = crate::BitReader<PIO17_A>;
#[doc = "Selects pin direction for pin PIOn_17.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIO17_A {
    #[doc = "0: Pin is an input."]
    INPUT = 0,
    #[doc = "1: Pin is an output."]
    OUTPUT = 1,
}
impl From<PIO17_A> for bool {
    #[inline(always)]
    fn from(variant: PIO17_A) -> Self {
        variant as u8 != 0
    }
}
impl PIO17_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIO17_A {
        match self.bits {
            false => PIO17_A::INPUT,
            true => PIO17_A::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == PIO17_A::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == PIO17_A::OUTPUT
    }
}
#[doc = "Field `PIO17` writer - Selects pin direction for pin PIOn_17."]
pub type PIO17_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIR_SPEC, PIO17_A, O>;
impl<'a, const O: u8> PIO17_W<'a, O> {
    #[doc = "Pin is an input."]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(PIO17_A::INPUT)
    }
    #[doc = "Pin is an output."]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(PIO17_A::OUTPUT)
    }
}
#[doc = "Field `PIO18` reader - Selects pin direction for pin PIOn_18."]
pub type PIO18_R = crate::BitReader<PIO18_A>;
#[doc = "Selects pin direction for pin PIOn_18.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIO18_A {
    #[doc = "0: Pin is an input."]
    INPUT = 0,
    #[doc = "1: Pin is an output."]
    OUTPUT = 1,
}
impl From<PIO18_A> for bool {
    #[inline(always)]
    fn from(variant: PIO18_A) -> Self {
        variant as u8 != 0
    }
}
impl PIO18_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIO18_A {
        match self.bits {
            false => PIO18_A::INPUT,
            true => PIO18_A::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == PIO18_A::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == PIO18_A::OUTPUT
    }
}
#[doc = "Field `PIO18` writer - Selects pin direction for pin PIOn_18."]
pub type PIO18_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIR_SPEC, PIO18_A, O>;
impl<'a, const O: u8> PIO18_W<'a, O> {
    #[doc = "Pin is an input."]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(PIO18_A::INPUT)
    }
    #[doc = "Pin is an output."]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(PIO18_A::OUTPUT)
    }
}
#[doc = "Field `PIO19` reader - Selects pin direction for pin PIOn_19."]
pub type PIO19_R = crate::BitReader<PIO19_A>;
#[doc = "Selects pin direction for pin PIOn_19.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIO19_A {
    #[doc = "0: Pin is an input."]
    INPUT = 0,
    #[doc = "1: Pin is an output."]
    OUTPUT = 1,
}
impl From<PIO19_A> for bool {
    #[inline(always)]
    fn from(variant: PIO19_A) -> Self {
        variant as u8 != 0
    }
}
impl PIO19_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIO19_A {
        match self.bits {
            false => PIO19_A::INPUT,
            true => PIO19_A::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == PIO19_A::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == PIO19_A::OUTPUT
    }
}
#[doc = "Field `PIO19` writer - Selects pin direction for pin PIOn_19."]
pub type PIO19_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIR_SPEC, PIO19_A, O>;
impl<'a, const O: u8> PIO19_W<'a, O> {
    #[doc = "Pin is an input."]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(PIO19_A::INPUT)
    }
    #[doc = "Pin is an output."]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(PIO19_A::OUTPUT)
    }
}
#[doc = "Field `PIO20` reader - Selects pin direction for pin PIOn_20."]
pub type PIO20_R = crate::BitReader<PIO20_A>;
#[doc = "Selects pin direction for pin PIOn_20.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIO20_A {
    #[doc = "0: Pin is an input."]
    INPUT = 0,
    #[doc = "1: Pin is an output."]
    OUTPUT = 1,
}
impl From<PIO20_A> for bool {
    #[inline(always)]
    fn from(variant: PIO20_A) -> Self {
        variant as u8 != 0
    }
}
impl PIO20_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIO20_A {
        match self.bits {
            false => PIO20_A::INPUT,
            true => PIO20_A::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == PIO20_A::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == PIO20_A::OUTPUT
    }
}
#[doc = "Field `PIO20` writer - Selects pin direction for pin PIOn_20."]
pub type PIO20_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIR_SPEC, PIO20_A, O>;
impl<'a, const O: u8> PIO20_W<'a, O> {
    #[doc = "Pin is an input."]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(PIO20_A::INPUT)
    }
    #[doc = "Pin is an output."]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(PIO20_A::OUTPUT)
    }
}
#[doc = "Field `PIO21` reader - Selects pin direction for pin PIOn_21."]
pub type PIO21_R = crate::BitReader<PIO21_A>;
#[doc = "Selects pin direction for pin PIOn_21.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIO21_A {
    #[doc = "0: Pin is an input."]
    INPUT = 0,
    #[doc = "1: Pin is an output."]
    OUTPUT = 1,
}
impl From<PIO21_A> for bool {
    #[inline(always)]
    fn from(variant: PIO21_A) -> Self {
        variant as u8 != 0
    }
}
impl PIO21_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIO21_A {
        match self.bits {
            false => PIO21_A::INPUT,
            true => PIO21_A::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == PIO21_A::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == PIO21_A::OUTPUT
    }
}
#[doc = "Field `PIO21` writer - Selects pin direction for pin PIOn_21."]
pub type PIO21_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIR_SPEC, PIO21_A, O>;
impl<'a, const O: u8> PIO21_W<'a, O> {
    #[doc = "Pin is an input."]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(PIO21_A::INPUT)
    }
    #[doc = "Pin is an output."]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(PIO21_A::OUTPUT)
    }
}
#[doc = "Field `PIO22` reader - Selects pin direction for pin PIOn_22."]
pub type PIO22_R = crate::BitReader<PIO22_A>;
#[doc = "Selects pin direction for pin PIOn_22.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIO22_A {
    #[doc = "0: Pin is an input."]
    INPUT = 0,
    #[doc = "1: Pin is an output."]
    OUTPUT = 1,
}
impl From<PIO22_A> for bool {
    #[inline(always)]
    fn from(variant: PIO22_A) -> Self {
        variant as u8 != 0
    }
}
impl PIO22_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIO22_A {
        match self.bits {
            false => PIO22_A::INPUT,
            true => PIO22_A::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == PIO22_A::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == PIO22_A::OUTPUT
    }
}
#[doc = "Field `PIO22` writer - Selects pin direction for pin PIOn_22."]
pub type PIO22_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIR_SPEC, PIO22_A, O>;
impl<'a, const O: u8> PIO22_W<'a, O> {
    #[doc = "Pin is an input."]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(PIO22_A::INPUT)
    }
    #[doc = "Pin is an output."]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(PIO22_A::OUTPUT)
    }
}
#[doc = "Field `PIO23` reader - Selects pin direction for pin PIOn_23."]
pub type PIO23_R = crate::BitReader<PIO23_A>;
#[doc = "Selects pin direction for pin PIOn_23.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIO23_A {
    #[doc = "0: Pin is an input."]
    INPUT = 0,
    #[doc = "1: Pin is an output."]
    OUTPUT = 1,
}
impl From<PIO23_A> for bool {
    #[inline(always)]
    fn from(variant: PIO23_A) -> Self {
        variant as u8 != 0
    }
}
impl PIO23_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIO23_A {
        match self.bits {
            false => PIO23_A::INPUT,
            true => PIO23_A::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == PIO23_A::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == PIO23_A::OUTPUT
    }
}
#[doc = "Field `PIO23` writer - Selects pin direction for pin PIOn_23."]
pub type PIO23_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIR_SPEC, PIO23_A, O>;
impl<'a, const O: u8> PIO23_W<'a, O> {
    #[doc = "Pin is an input."]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(PIO23_A::INPUT)
    }
    #[doc = "Pin is an output."]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(PIO23_A::OUTPUT)
    }
}
#[doc = "Field `PIO24` reader - Selects pin direction for pin PIOn_24."]
pub type PIO24_R = crate::BitReader<PIO24_A>;
#[doc = "Selects pin direction for pin PIOn_24.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIO24_A {
    #[doc = "0: Pin is an input."]
    INPUT = 0,
    #[doc = "1: Pin is an output."]
    OUTPUT = 1,
}
impl From<PIO24_A> for bool {
    #[inline(always)]
    fn from(variant: PIO24_A) -> Self {
        variant as u8 != 0
    }
}
impl PIO24_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIO24_A {
        match self.bits {
            false => PIO24_A::INPUT,
            true => PIO24_A::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == PIO24_A::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == PIO24_A::OUTPUT
    }
}
#[doc = "Field `PIO24` writer - Selects pin direction for pin PIOn_24."]
pub type PIO24_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIR_SPEC, PIO24_A, O>;
impl<'a, const O: u8> PIO24_W<'a, O> {
    #[doc = "Pin is an input."]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(PIO24_A::INPUT)
    }
    #[doc = "Pin is an output."]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(PIO24_A::OUTPUT)
    }
}
#[doc = "Field `PIO25` reader - Selects pin direction for pin PIOn_25."]
pub type PIO25_R = crate::BitReader<PIO25_A>;
#[doc = "Selects pin direction for pin PIOn_25.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIO25_A {
    #[doc = "0: Pin is an input."]
    INPUT = 0,
    #[doc = "1: Pin is an output."]
    OUTPUT = 1,
}
impl From<PIO25_A> for bool {
    #[inline(always)]
    fn from(variant: PIO25_A) -> Self {
        variant as u8 != 0
    }
}
impl PIO25_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIO25_A {
        match self.bits {
            false => PIO25_A::INPUT,
            true => PIO25_A::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == PIO25_A::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == PIO25_A::OUTPUT
    }
}
#[doc = "Field `PIO25` writer - Selects pin direction for pin PIOn_25."]
pub type PIO25_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIR_SPEC, PIO25_A, O>;
impl<'a, const O: u8> PIO25_W<'a, O> {
    #[doc = "Pin is an input."]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(PIO25_A::INPUT)
    }
    #[doc = "Pin is an output."]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(PIO25_A::OUTPUT)
    }
}
#[doc = "Field `PIO26` reader - Selects pin direction for pin PIOn_26."]
pub type PIO26_R = crate::BitReader<PIO26_A>;
#[doc = "Selects pin direction for pin PIOn_26.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIO26_A {
    #[doc = "0: Pin is an input."]
    INPUT = 0,
    #[doc = "1: Pin is an output."]
    OUTPUT = 1,
}
impl From<PIO26_A> for bool {
    #[inline(always)]
    fn from(variant: PIO26_A) -> Self {
        variant as u8 != 0
    }
}
impl PIO26_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIO26_A {
        match self.bits {
            false => PIO26_A::INPUT,
            true => PIO26_A::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == PIO26_A::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == PIO26_A::OUTPUT
    }
}
#[doc = "Field `PIO26` writer - Selects pin direction for pin PIOn_26."]
pub type PIO26_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIR_SPEC, PIO26_A, O>;
impl<'a, const O: u8> PIO26_W<'a, O> {
    #[doc = "Pin is an input."]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(PIO26_A::INPUT)
    }
    #[doc = "Pin is an output."]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(PIO26_A::OUTPUT)
    }
}
#[doc = "Field `PIO27` reader - Selects pin direction for pin PIOn_27."]
pub type PIO27_R = crate::BitReader<PIO27_A>;
#[doc = "Selects pin direction for pin PIOn_27.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIO27_A {
    #[doc = "0: Pin is an input."]
    INPUT = 0,
    #[doc = "1: Pin is an output."]
    OUTPUT = 1,
}
impl From<PIO27_A> for bool {
    #[inline(always)]
    fn from(variant: PIO27_A) -> Self {
        variant as u8 != 0
    }
}
impl PIO27_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIO27_A {
        match self.bits {
            false => PIO27_A::INPUT,
            true => PIO27_A::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == PIO27_A::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == PIO27_A::OUTPUT
    }
}
#[doc = "Field `PIO27` writer - Selects pin direction for pin PIOn_27."]
pub type PIO27_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIR_SPEC, PIO27_A, O>;
impl<'a, const O: u8> PIO27_W<'a, O> {
    #[doc = "Pin is an input."]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(PIO27_A::INPUT)
    }
    #[doc = "Pin is an output."]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(PIO27_A::OUTPUT)
    }
}
#[doc = "Field `PIO28` reader - Selects pin direction for pin PIOn_28."]
pub type PIO28_R = crate::BitReader<PIO28_A>;
#[doc = "Selects pin direction for pin PIOn_28.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIO28_A {
    #[doc = "0: Pin is an input."]
    INPUT = 0,
    #[doc = "1: Pin is an output."]
    OUTPUT = 1,
}
impl From<PIO28_A> for bool {
    #[inline(always)]
    fn from(variant: PIO28_A) -> Self {
        variant as u8 != 0
    }
}
impl PIO28_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIO28_A {
        match self.bits {
            false => PIO28_A::INPUT,
            true => PIO28_A::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == PIO28_A::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == PIO28_A::OUTPUT
    }
}
#[doc = "Field `PIO28` writer - Selects pin direction for pin PIOn_28."]
pub type PIO28_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIR_SPEC, PIO28_A, O>;
impl<'a, const O: u8> PIO28_W<'a, O> {
    #[doc = "Pin is an input."]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(PIO28_A::INPUT)
    }
    #[doc = "Pin is an output."]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(PIO28_A::OUTPUT)
    }
}
#[doc = "Field `PIO29` reader - Selects pin direction for pin PIOn_29."]
pub type PIO29_R = crate::BitReader<PIO29_A>;
#[doc = "Selects pin direction for pin PIOn_29.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIO29_A {
    #[doc = "0: Pin is an input."]
    INPUT = 0,
    #[doc = "1: Pin is an output."]
    OUTPUT = 1,
}
impl From<PIO29_A> for bool {
    #[inline(always)]
    fn from(variant: PIO29_A) -> Self {
        variant as u8 != 0
    }
}
impl PIO29_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIO29_A {
        match self.bits {
            false => PIO29_A::INPUT,
            true => PIO29_A::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == PIO29_A::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == PIO29_A::OUTPUT
    }
}
#[doc = "Field `PIO29` writer - Selects pin direction for pin PIOn_29."]
pub type PIO29_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIR_SPEC, PIO29_A, O>;
impl<'a, const O: u8> PIO29_W<'a, O> {
    #[doc = "Pin is an input."]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(PIO29_A::INPUT)
    }
    #[doc = "Pin is an output."]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(PIO29_A::OUTPUT)
    }
}
#[doc = "Field `PIO30` reader - Selects pin direction for pin PIOn_30."]
pub type PIO30_R = crate::BitReader<PIO30_A>;
#[doc = "Selects pin direction for pin PIOn_30.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIO30_A {
    #[doc = "0: Pin is an input."]
    INPUT = 0,
    #[doc = "1: Pin is an output."]
    OUTPUT = 1,
}
impl From<PIO30_A> for bool {
    #[inline(always)]
    fn from(variant: PIO30_A) -> Self {
        variant as u8 != 0
    }
}
impl PIO30_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIO30_A {
        match self.bits {
            false => PIO30_A::INPUT,
            true => PIO30_A::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == PIO30_A::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == PIO30_A::OUTPUT
    }
}
#[doc = "Field `PIO30` writer - Selects pin direction for pin PIOn_30."]
pub type PIO30_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIR_SPEC, PIO30_A, O>;
impl<'a, const O: u8> PIO30_W<'a, O> {
    #[doc = "Pin is an input."]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(PIO30_A::INPUT)
    }
    #[doc = "Pin is an output."]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(PIO30_A::OUTPUT)
    }
}
#[doc = "Field `PIO31` reader - Selects pin direction for pin PIOn_31."]
pub type PIO31_R = crate::BitReader<PIO31_A>;
#[doc = "Selects pin direction for pin PIOn_31.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIO31_A {
    #[doc = "0: Pin is an input."]
    INPUT = 0,
    #[doc = "1: Pin is an output."]
    OUTPUT = 1,
}
impl From<PIO31_A> for bool {
    #[inline(always)]
    fn from(variant: PIO31_A) -> Self {
        variant as u8 != 0
    }
}
impl PIO31_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIO31_A {
        match self.bits {
            false => PIO31_A::INPUT,
            true => PIO31_A::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == PIO31_A::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == PIO31_A::OUTPUT
    }
}
#[doc = "Field `PIO31` writer - Selects pin direction for pin PIOn_31."]
pub type PIO31_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIR_SPEC, PIO31_A, O>;
impl<'a, const O: u8> PIO31_W<'a, O> {
    #[doc = "Pin is an input."]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(PIO31_A::INPUT)
    }
    #[doc = "Pin is an output."]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(PIO31_A::OUTPUT)
    }
}
impl R {
    #[doc = "Bit 0 - Selects pin direction for pin PIOn_0."]
    #[inline(always)]
    pub fn pio0(&self) -> PIO0_R {
        PIO0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Selects pin direction for pin PIOn_1."]
    #[inline(always)]
    pub fn pio1(&self) -> PIO1_R {
        PIO1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Selects pin direction for pin PIOn_2."]
    #[inline(always)]
    pub fn pio2(&self) -> PIO2_R {
        PIO2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Selects pin direction for pin PIOn_3."]
    #[inline(always)]
    pub fn pio3(&self) -> PIO3_R {
        PIO3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Selects pin direction for pin PIOn_4."]
    #[inline(always)]
    pub fn pio4(&self) -> PIO4_R {
        PIO4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Selects pin direction for pin PIOn_5."]
    #[inline(always)]
    pub fn pio5(&self) -> PIO5_R {
        PIO5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Selects pin direction for pin PIOn_6."]
    #[inline(always)]
    pub fn pio6(&self) -> PIO6_R {
        PIO6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Selects pin direction for pin PIOn_7."]
    #[inline(always)]
    pub fn pio7(&self) -> PIO7_R {
        PIO7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Selects pin direction for pin PIOn_8."]
    #[inline(always)]
    pub fn pio8(&self) -> PIO8_R {
        PIO8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Selects pin direction for pin PIOn_9."]
    #[inline(always)]
    pub fn pio9(&self) -> PIO9_R {
        PIO9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Selects pin direction for pin PIOn_10."]
    #[inline(always)]
    pub fn pio10(&self) -> PIO10_R {
        PIO10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Selects pin direction for pin PIOn_11."]
    #[inline(always)]
    pub fn pio11(&self) -> PIO11_R {
        PIO11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Selects pin direction for pin PIOn_12."]
    #[inline(always)]
    pub fn pio12(&self) -> PIO12_R {
        PIO12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Selects pin direction for pin PIOn_13."]
    #[inline(always)]
    pub fn pio13(&self) -> PIO13_R {
        PIO13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Selects pin direction for pin PIOn_14."]
    #[inline(always)]
    pub fn pio14(&self) -> PIO14_R {
        PIO14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Selects pin direction for pin PIOn_15."]
    #[inline(always)]
    pub fn pio15(&self) -> PIO15_R {
        PIO15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Selects pin direction for pin PIOn_16."]
    #[inline(always)]
    pub fn pio16(&self) -> PIO16_R {
        PIO16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Selects pin direction for pin PIOn_17."]
    #[inline(always)]
    pub fn pio17(&self) -> PIO17_R {
        PIO17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Selects pin direction for pin PIOn_18."]
    #[inline(always)]
    pub fn pio18(&self) -> PIO18_R {
        PIO18_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Selects pin direction for pin PIOn_19."]
    #[inline(always)]
    pub fn pio19(&self) -> PIO19_R {
        PIO19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Selects pin direction for pin PIOn_20."]
    #[inline(always)]
    pub fn pio20(&self) -> PIO20_R {
        PIO20_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Selects pin direction for pin PIOn_21."]
    #[inline(always)]
    pub fn pio21(&self) -> PIO21_R {
        PIO21_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Selects pin direction for pin PIOn_22."]
    #[inline(always)]
    pub fn pio22(&self) -> PIO22_R {
        PIO22_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Selects pin direction for pin PIOn_23."]
    #[inline(always)]
    pub fn pio23(&self) -> PIO23_R {
        PIO23_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Selects pin direction for pin PIOn_24."]
    #[inline(always)]
    pub fn pio24(&self) -> PIO24_R {
        PIO24_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Selects pin direction for pin PIOn_25."]
    #[inline(always)]
    pub fn pio25(&self) -> PIO25_R {
        PIO25_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Selects pin direction for pin PIOn_26."]
    #[inline(always)]
    pub fn pio26(&self) -> PIO26_R {
        PIO26_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Selects pin direction for pin PIOn_27."]
    #[inline(always)]
    pub fn pio27(&self) -> PIO27_R {
        PIO27_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Selects pin direction for pin PIOn_28."]
    #[inline(always)]
    pub fn pio28(&self) -> PIO28_R {
        PIO28_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Selects pin direction for pin PIOn_29."]
    #[inline(always)]
    pub fn pio29(&self) -> PIO29_R {
        PIO29_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Selects pin direction for pin PIOn_30."]
    #[inline(always)]
    pub fn pio30(&self) -> PIO30_R {
        PIO30_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Selects pin direction for pin PIOn_31."]
    #[inline(always)]
    pub fn pio31(&self) -> PIO31_R {
        PIO31_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Selects pin direction for pin PIOn_0."]
    #[inline(always)]
    pub fn pio0(&mut self) -> PIO0_W<0> {
        PIO0_W::new(self)
    }
    #[doc = "Bit 1 - Selects pin direction for pin PIOn_1."]
    #[inline(always)]
    pub fn pio1(&mut self) -> PIO1_W<1> {
        PIO1_W::new(self)
    }
    #[doc = "Bit 2 - Selects pin direction for pin PIOn_2."]
    #[inline(always)]
    pub fn pio2(&mut self) -> PIO2_W<2> {
        PIO2_W::new(self)
    }
    #[doc = "Bit 3 - Selects pin direction for pin PIOn_3."]
    #[inline(always)]
    pub fn pio3(&mut self) -> PIO3_W<3> {
        PIO3_W::new(self)
    }
    #[doc = "Bit 4 - Selects pin direction for pin PIOn_4."]
    #[inline(always)]
    pub fn pio4(&mut self) -> PIO4_W<4> {
        PIO4_W::new(self)
    }
    #[doc = "Bit 5 - Selects pin direction for pin PIOn_5."]
    #[inline(always)]
    pub fn pio5(&mut self) -> PIO5_W<5> {
        PIO5_W::new(self)
    }
    #[doc = "Bit 6 - Selects pin direction for pin PIOn_6."]
    #[inline(always)]
    pub fn pio6(&mut self) -> PIO6_W<6> {
        PIO6_W::new(self)
    }
    #[doc = "Bit 7 - Selects pin direction for pin PIOn_7."]
    #[inline(always)]
    pub fn pio7(&mut self) -> PIO7_W<7> {
        PIO7_W::new(self)
    }
    #[doc = "Bit 8 - Selects pin direction for pin PIOn_8."]
    #[inline(always)]
    pub fn pio8(&mut self) -> PIO8_W<8> {
        PIO8_W::new(self)
    }
    #[doc = "Bit 9 - Selects pin direction for pin PIOn_9."]
    #[inline(always)]
    pub fn pio9(&mut self) -> PIO9_W<9> {
        PIO9_W::new(self)
    }
    #[doc = "Bit 10 - Selects pin direction for pin PIOn_10."]
    #[inline(always)]
    pub fn pio10(&mut self) -> PIO10_W<10> {
        PIO10_W::new(self)
    }
    #[doc = "Bit 11 - Selects pin direction for pin PIOn_11."]
    #[inline(always)]
    pub fn pio11(&mut self) -> PIO11_W<11> {
        PIO11_W::new(self)
    }
    #[doc = "Bit 12 - Selects pin direction for pin PIOn_12."]
    #[inline(always)]
    pub fn pio12(&mut self) -> PIO12_W<12> {
        PIO12_W::new(self)
    }
    #[doc = "Bit 13 - Selects pin direction for pin PIOn_13."]
    #[inline(always)]
    pub fn pio13(&mut self) -> PIO13_W<13> {
        PIO13_W::new(self)
    }
    #[doc = "Bit 14 - Selects pin direction for pin PIOn_14."]
    #[inline(always)]
    pub fn pio14(&mut self) -> PIO14_W<14> {
        PIO14_W::new(self)
    }
    #[doc = "Bit 15 - Selects pin direction for pin PIOn_15."]
    #[inline(always)]
    pub fn pio15(&mut self) -> PIO15_W<15> {
        PIO15_W::new(self)
    }
    #[doc = "Bit 16 - Selects pin direction for pin PIOn_16."]
    #[inline(always)]
    pub fn pio16(&mut self) -> PIO16_W<16> {
        PIO16_W::new(self)
    }
    #[doc = "Bit 17 - Selects pin direction for pin PIOn_17."]
    #[inline(always)]
    pub fn pio17(&mut self) -> PIO17_W<17> {
        PIO17_W::new(self)
    }
    #[doc = "Bit 18 - Selects pin direction for pin PIOn_18."]
    #[inline(always)]
    pub fn pio18(&mut self) -> PIO18_W<18> {
        PIO18_W::new(self)
    }
    #[doc = "Bit 19 - Selects pin direction for pin PIOn_19."]
    #[inline(always)]
    pub fn pio19(&mut self) -> PIO19_W<19> {
        PIO19_W::new(self)
    }
    #[doc = "Bit 20 - Selects pin direction for pin PIOn_20."]
    #[inline(always)]
    pub fn pio20(&mut self) -> PIO20_W<20> {
        PIO20_W::new(self)
    }
    #[doc = "Bit 21 - Selects pin direction for pin PIOn_21."]
    #[inline(always)]
    pub fn pio21(&mut self) -> PIO21_W<21> {
        PIO21_W::new(self)
    }
    #[doc = "Bit 22 - Selects pin direction for pin PIOn_22."]
    #[inline(always)]
    pub fn pio22(&mut self) -> PIO22_W<22> {
        PIO22_W::new(self)
    }
    #[doc = "Bit 23 - Selects pin direction for pin PIOn_23."]
    #[inline(always)]
    pub fn pio23(&mut self) -> PIO23_W<23> {
        PIO23_W::new(self)
    }
    #[doc = "Bit 24 - Selects pin direction for pin PIOn_24."]
    #[inline(always)]
    pub fn pio24(&mut self) -> PIO24_W<24> {
        PIO24_W::new(self)
    }
    #[doc = "Bit 25 - Selects pin direction for pin PIOn_25."]
    #[inline(always)]
    pub fn pio25(&mut self) -> PIO25_W<25> {
        PIO25_W::new(self)
    }
    #[doc = "Bit 26 - Selects pin direction for pin PIOn_26."]
    #[inline(always)]
    pub fn pio26(&mut self) -> PIO26_W<26> {
        PIO26_W::new(self)
    }
    #[doc = "Bit 27 - Selects pin direction for pin PIOn_27."]
    #[inline(always)]
    pub fn pio27(&mut self) -> PIO27_W<27> {
        PIO27_W::new(self)
    }
    #[doc = "Bit 28 - Selects pin direction for pin PIOn_28."]
    #[inline(always)]
    pub fn pio28(&mut self) -> PIO28_W<28> {
        PIO28_W::new(self)
    }
    #[doc = "Bit 29 - Selects pin direction for pin PIOn_29."]
    #[inline(always)]
    pub fn pio29(&mut self) -> PIO29_W<29> {
        PIO29_W::new(self)
    }
    #[doc = "Bit 30 - Selects pin direction for pin PIOn_30."]
    #[inline(always)]
    pub fn pio30(&mut self) -> PIO30_W<30> {
        PIO30_W::new(self)
    }
    #[doc = "Bit 31 - Selects pin direction for pin PIOn_31."]
    #[inline(always)]
    pub fn pio31(&mut self) -> PIO31_W<31> {
        PIO31_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Direction registers for all port GPIO pins. Supported pins depends on the specific device and package.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dir](index.html) module"]
pub struct DIR_SPEC;
impl crate::RegisterSpec for DIR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dir::R](R) reader structure"]
impl crate::Readable for DIR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dir::W](W) writer structure"]
impl crate::Writable for DIR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DIR[%s]
to value 0"]
impl crate::Resettable for DIR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
