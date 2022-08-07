#[doc = "Register `SEC_GPIO_MASK1` reader"]
pub struct R(crate::R<SEC_GPIO_MASK1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SEC_GPIO_MASK1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SEC_GPIO_MASK1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SEC_GPIO_MASK1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SEC_GPIO_MASK1` writer"]
pub struct W(crate::W<SEC_GPIO_MASK1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SEC_GPIO_MASK1_SPEC>;
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
impl From<crate::W<SEC_GPIO_MASK1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SEC_GPIO_MASK1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PIO1_PIN0_SEC_MASK` reader - Secure mask for pin P1_0"]
pub type PIO1_PIN0_SEC_MASK_R = crate::BitReader<PIO1_PIN0_SEC_MASK_A>;
#[doc = "Secure mask for pin P1_0\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIO1_PIN0_SEC_MASK_A {
    #[doc = "0: Pin state is blocked to non-secure world."]
    BLOCKED = 0,
    #[doc = "1: Pin state is readable by non-secure world."]
    READABLE = 1,
}
impl From<PIO1_PIN0_SEC_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: PIO1_PIN0_SEC_MASK_A) -> Self {
        variant as u8 != 0
    }
}
impl PIO1_PIN0_SEC_MASK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIO1_PIN0_SEC_MASK_A {
        match self.bits {
            false => PIO1_PIN0_SEC_MASK_A::BLOCKED,
            true => PIO1_PIN0_SEC_MASK_A::READABLE,
        }
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == PIO1_PIN0_SEC_MASK_A::BLOCKED
    }
    #[doc = "Checks if the value of the field is `READABLE`"]
    #[inline(always)]
    pub fn is_readable(&self) -> bool {
        *self == PIO1_PIN0_SEC_MASK_A::READABLE
    }
}
#[doc = "Field `PIO1_PIN0_SEC_MASK` writer - Secure mask for pin P1_0"]
pub type PIO1_PIN0_SEC_MASK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SEC_GPIO_MASK1_SPEC, PIO1_PIN0_SEC_MASK_A, O>;
impl<'a, const O: u8> PIO1_PIN0_SEC_MASK_W<'a, O> {
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut W {
        self.variant(PIO1_PIN0_SEC_MASK_A::BLOCKED)
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn readable(self) -> &'a mut W {
        self.variant(PIO1_PIN0_SEC_MASK_A::READABLE)
    }
}
#[doc = "Field `PIO1_PIN1_SEC_MASK` reader - Secure mask for pin P1_1"]
pub type PIO1_PIN1_SEC_MASK_R = crate::BitReader<PIO1_PIN1_SEC_MASK_A>;
#[doc = "Secure mask for pin P1_1\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIO1_PIN1_SEC_MASK_A {
    #[doc = "0: Pin state is blocked to non-secure world."]
    BLOCKED = 0,
    #[doc = "1: Pin state is readable by non-secure world."]
    READABLE = 1,
}
impl From<PIO1_PIN1_SEC_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: PIO1_PIN1_SEC_MASK_A) -> Self {
        variant as u8 != 0
    }
}
impl PIO1_PIN1_SEC_MASK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIO1_PIN1_SEC_MASK_A {
        match self.bits {
            false => PIO1_PIN1_SEC_MASK_A::BLOCKED,
            true => PIO1_PIN1_SEC_MASK_A::READABLE,
        }
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == PIO1_PIN1_SEC_MASK_A::BLOCKED
    }
    #[doc = "Checks if the value of the field is `READABLE`"]
    #[inline(always)]
    pub fn is_readable(&self) -> bool {
        *self == PIO1_PIN1_SEC_MASK_A::READABLE
    }
}
#[doc = "Field `PIO1_PIN1_SEC_MASK` writer - Secure mask for pin P1_1"]
pub type PIO1_PIN1_SEC_MASK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SEC_GPIO_MASK1_SPEC, PIO1_PIN1_SEC_MASK_A, O>;
impl<'a, const O: u8> PIO1_PIN1_SEC_MASK_W<'a, O> {
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut W {
        self.variant(PIO1_PIN1_SEC_MASK_A::BLOCKED)
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn readable(self) -> &'a mut W {
        self.variant(PIO1_PIN1_SEC_MASK_A::READABLE)
    }
}
#[doc = "Field `PIO1_PIN2_SEC_MASK` reader - Secure mask for pin P1_2"]
pub type PIO1_PIN2_SEC_MASK_R = crate::BitReader<PIO1_PIN2_SEC_MASK_A>;
#[doc = "Secure mask for pin P1_2\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIO1_PIN2_SEC_MASK_A {
    #[doc = "0: Pin state is blocked to non-secure world."]
    BLOCKED = 0,
    #[doc = "1: Pin state is readable by non-secure world."]
    READABLE = 1,
}
impl From<PIO1_PIN2_SEC_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: PIO1_PIN2_SEC_MASK_A) -> Self {
        variant as u8 != 0
    }
}
impl PIO1_PIN2_SEC_MASK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIO1_PIN2_SEC_MASK_A {
        match self.bits {
            false => PIO1_PIN2_SEC_MASK_A::BLOCKED,
            true => PIO1_PIN2_SEC_MASK_A::READABLE,
        }
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == PIO1_PIN2_SEC_MASK_A::BLOCKED
    }
    #[doc = "Checks if the value of the field is `READABLE`"]
    #[inline(always)]
    pub fn is_readable(&self) -> bool {
        *self == PIO1_PIN2_SEC_MASK_A::READABLE
    }
}
#[doc = "Field `PIO1_PIN2_SEC_MASK` writer - Secure mask for pin P1_2"]
pub type PIO1_PIN2_SEC_MASK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SEC_GPIO_MASK1_SPEC, PIO1_PIN2_SEC_MASK_A, O>;
impl<'a, const O: u8> PIO1_PIN2_SEC_MASK_W<'a, O> {
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut W {
        self.variant(PIO1_PIN2_SEC_MASK_A::BLOCKED)
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn readable(self) -> &'a mut W {
        self.variant(PIO1_PIN2_SEC_MASK_A::READABLE)
    }
}
#[doc = "Field `PIO1_PIN3_SEC_MASK` reader - Secure mask for pin P1_3"]
pub type PIO1_PIN3_SEC_MASK_R = crate::BitReader<PIO1_PIN3_SEC_MASK_A>;
#[doc = "Secure mask for pin P1_3\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIO1_PIN3_SEC_MASK_A {
    #[doc = "0: Pin state is blocked to non-secure world."]
    BLOCKED = 0,
    #[doc = "1: Pin state is readable by non-secure world."]
    READABLE = 1,
}
impl From<PIO1_PIN3_SEC_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: PIO1_PIN3_SEC_MASK_A) -> Self {
        variant as u8 != 0
    }
}
impl PIO1_PIN3_SEC_MASK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIO1_PIN3_SEC_MASK_A {
        match self.bits {
            false => PIO1_PIN3_SEC_MASK_A::BLOCKED,
            true => PIO1_PIN3_SEC_MASK_A::READABLE,
        }
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == PIO1_PIN3_SEC_MASK_A::BLOCKED
    }
    #[doc = "Checks if the value of the field is `READABLE`"]
    #[inline(always)]
    pub fn is_readable(&self) -> bool {
        *self == PIO1_PIN3_SEC_MASK_A::READABLE
    }
}
#[doc = "Field `PIO1_PIN3_SEC_MASK` writer - Secure mask for pin P1_3"]
pub type PIO1_PIN3_SEC_MASK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SEC_GPIO_MASK1_SPEC, PIO1_PIN3_SEC_MASK_A, O>;
impl<'a, const O: u8> PIO1_PIN3_SEC_MASK_W<'a, O> {
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut W {
        self.variant(PIO1_PIN3_SEC_MASK_A::BLOCKED)
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn readable(self) -> &'a mut W {
        self.variant(PIO1_PIN3_SEC_MASK_A::READABLE)
    }
}
#[doc = "Field `PIO1_PIN4_SEC_MASK` reader - Secure mask for pin P1_4"]
pub type PIO1_PIN4_SEC_MASK_R = crate::BitReader<PIO1_PIN4_SEC_MASK_A>;
#[doc = "Secure mask for pin P1_4\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIO1_PIN4_SEC_MASK_A {
    #[doc = "0: Pin state is blocked to non-secure world."]
    BLOCKED = 0,
    #[doc = "1: Pin state is readable by non-secure world."]
    READABLE = 1,
}
impl From<PIO1_PIN4_SEC_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: PIO1_PIN4_SEC_MASK_A) -> Self {
        variant as u8 != 0
    }
}
impl PIO1_PIN4_SEC_MASK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIO1_PIN4_SEC_MASK_A {
        match self.bits {
            false => PIO1_PIN4_SEC_MASK_A::BLOCKED,
            true => PIO1_PIN4_SEC_MASK_A::READABLE,
        }
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == PIO1_PIN4_SEC_MASK_A::BLOCKED
    }
    #[doc = "Checks if the value of the field is `READABLE`"]
    #[inline(always)]
    pub fn is_readable(&self) -> bool {
        *self == PIO1_PIN4_SEC_MASK_A::READABLE
    }
}
#[doc = "Field `PIO1_PIN4_SEC_MASK` writer - Secure mask for pin P1_4"]
pub type PIO1_PIN4_SEC_MASK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SEC_GPIO_MASK1_SPEC, PIO1_PIN4_SEC_MASK_A, O>;
impl<'a, const O: u8> PIO1_PIN4_SEC_MASK_W<'a, O> {
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut W {
        self.variant(PIO1_PIN4_SEC_MASK_A::BLOCKED)
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn readable(self) -> &'a mut W {
        self.variant(PIO1_PIN4_SEC_MASK_A::READABLE)
    }
}
#[doc = "Field `PIO1_PIN5_SEC_MASK` reader - Secure mask for pin P1_5"]
pub type PIO1_PIN5_SEC_MASK_R = crate::BitReader<PIO1_PIN5_SEC_MASK_A>;
#[doc = "Secure mask for pin P1_5\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIO1_PIN5_SEC_MASK_A {
    #[doc = "0: Pin state is blocked to non-secure world."]
    BLOCKED = 0,
    #[doc = "1: Pin state is readable by non-secure world."]
    READABLE = 1,
}
impl From<PIO1_PIN5_SEC_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: PIO1_PIN5_SEC_MASK_A) -> Self {
        variant as u8 != 0
    }
}
impl PIO1_PIN5_SEC_MASK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIO1_PIN5_SEC_MASK_A {
        match self.bits {
            false => PIO1_PIN5_SEC_MASK_A::BLOCKED,
            true => PIO1_PIN5_SEC_MASK_A::READABLE,
        }
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == PIO1_PIN5_SEC_MASK_A::BLOCKED
    }
    #[doc = "Checks if the value of the field is `READABLE`"]
    #[inline(always)]
    pub fn is_readable(&self) -> bool {
        *self == PIO1_PIN5_SEC_MASK_A::READABLE
    }
}
#[doc = "Field `PIO1_PIN5_SEC_MASK` writer - Secure mask for pin P1_5"]
pub type PIO1_PIN5_SEC_MASK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SEC_GPIO_MASK1_SPEC, PIO1_PIN5_SEC_MASK_A, O>;
impl<'a, const O: u8> PIO1_PIN5_SEC_MASK_W<'a, O> {
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut W {
        self.variant(PIO1_PIN5_SEC_MASK_A::BLOCKED)
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn readable(self) -> &'a mut W {
        self.variant(PIO1_PIN5_SEC_MASK_A::READABLE)
    }
}
#[doc = "Field `PIO1_PIN6_SEC_MASK` reader - Secure mask for pin P1_6"]
pub type PIO1_PIN6_SEC_MASK_R = crate::BitReader<PIO1_PIN6_SEC_MASK_A>;
#[doc = "Secure mask for pin P1_6\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIO1_PIN6_SEC_MASK_A {
    #[doc = "0: Pin state is blocked to non-secure world."]
    BLOCKED = 0,
    #[doc = "1: Pin state is readable by non-secure world."]
    READABLE = 1,
}
impl From<PIO1_PIN6_SEC_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: PIO1_PIN6_SEC_MASK_A) -> Self {
        variant as u8 != 0
    }
}
impl PIO1_PIN6_SEC_MASK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIO1_PIN6_SEC_MASK_A {
        match self.bits {
            false => PIO1_PIN6_SEC_MASK_A::BLOCKED,
            true => PIO1_PIN6_SEC_MASK_A::READABLE,
        }
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == PIO1_PIN6_SEC_MASK_A::BLOCKED
    }
    #[doc = "Checks if the value of the field is `READABLE`"]
    #[inline(always)]
    pub fn is_readable(&self) -> bool {
        *self == PIO1_PIN6_SEC_MASK_A::READABLE
    }
}
#[doc = "Field `PIO1_PIN6_SEC_MASK` writer - Secure mask for pin P1_6"]
pub type PIO1_PIN6_SEC_MASK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SEC_GPIO_MASK1_SPEC, PIO1_PIN6_SEC_MASK_A, O>;
impl<'a, const O: u8> PIO1_PIN6_SEC_MASK_W<'a, O> {
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut W {
        self.variant(PIO1_PIN6_SEC_MASK_A::BLOCKED)
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn readable(self) -> &'a mut W {
        self.variant(PIO1_PIN6_SEC_MASK_A::READABLE)
    }
}
#[doc = "Field `PIO1_PIN7_SEC_MASK` reader - Secure mask for pin P1_7"]
pub type PIO1_PIN7_SEC_MASK_R = crate::BitReader<PIO1_PIN7_SEC_MASK_A>;
#[doc = "Secure mask for pin P1_7\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIO1_PIN7_SEC_MASK_A {
    #[doc = "0: Pin state is blocked to non-secure world."]
    BLOCKED = 0,
    #[doc = "1: Pin state is readable by non-secure world."]
    READABLE = 1,
}
impl From<PIO1_PIN7_SEC_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: PIO1_PIN7_SEC_MASK_A) -> Self {
        variant as u8 != 0
    }
}
impl PIO1_PIN7_SEC_MASK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIO1_PIN7_SEC_MASK_A {
        match self.bits {
            false => PIO1_PIN7_SEC_MASK_A::BLOCKED,
            true => PIO1_PIN7_SEC_MASK_A::READABLE,
        }
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == PIO1_PIN7_SEC_MASK_A::BLOCKED
    }
    #[doc = "Checks if the value of the field is `READABLE`"]
    #[inline(always)]
    pub fn is_readable(&self) -> bool {
        *self == PIO1_PIN7_SEC_MASK_A::READABLE
    }
}
#[doc = "Field `PIO1_PIN7_SEC_MASK` writer - Secure mask for pin P1_7"]
pub type PIO1_PIN7_SEC_MASK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SEC_GPIO_MASK1_SPEC, PIO1_PIN7_SEC_MASK_A, O>;
impl<'a, const O: u8> PIO1_PIN7_SEC_MASK_W<'a, O> {
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut W {
        self.variant(PIO1_PIN7_SEC_MASK_A::BLOCKED)
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn readable(self) -> &'a mut W {
        self.variant(PIO1_PIN7_SEC_MASK_A::READABLE)
    }
}
#[doc = "Field `PIO1_PIN8_SEC_MASK` reader - Secure mask for pin P1_8"]
pub type PIO1_PIN8_SEC_MASK_R = crate::BitReader<PIO1_PIN8_SEC_MASK_A>;
#[doc = "Secure mask for pin P1_8\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIO1_PIN8_SEC_MASK_A {
    #[doc = "0: Pin state is blocked to non-secure world."]
    BLOCKED = 0,
    #[doc = "1: Pin state is readable by non-secure world."]
    READABLE = 1,
}
impl From<PIO1_PIN8_SEC_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: PIO1_PIN8_SEC_MASK_A) -> Self {
        variant as u8 != 0
    }
}
impl PIO1_PIN8_SEC_MASK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIO1_PIN8_SEC_MASK_A {
        match self.bits {
            false => PIO1_PIN8_SEC_MASK_A::BLOCKED,
            true => PIO1_PIN8_SEC_MASK_A::READABLE,
        }
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == PIO1_PIN8_SEC_MASK_A::BLOCKED
    }
    #[doc = "Checks if the value of the field is `READABLE`"]
    #[inline(always)]
    pub fn is_readable(&self) -> bool {
        *self == PIO1_PIN8_SEC_MASK_A::READABLE
    }
}
#[doc = "Field `PIO1_PIN8_SEC_MASK` writer - Secure mask for pin P1_8"]
pub type PIO1_PIN8_SEC_MASK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SEC_GPIO_MASK1_SPEC, PIO1_PIN8_SEC_MASK_A, O>;
impl<'a, const O: u8> PIO1_PIN8_SEC_MASK_W<'a, O> {
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut W {
        self.variant(PIO1_PIN8_SEC_MASK_A::BLOCKED)
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn readable(self) -> &'a mut W {
        self.variant(PIO1_PIN8_SEC_MASK_A::READABLE)
    }
}
#[doc = "Field `PIO1_PIN9_SEC_MASK` reader - Secure mask for pin P1_9"]
pub type PIO1_PIN9_SEC_MASK_R = crate::BitReader<PIO1_PIN9_SEC_MASK_A>;
#[doc = "Secure mask for pin P1_9\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIO1_PIN9_SEC_MASK_A {
    #[doc = "0: Pin state is blocked to non-secure world."]
    BLOCKED = 0,
    #[doc = "1: Pin state is readable by non-secure world."]
    READABLE = 1,
}
impl From<PIO1_PIN9_SEC_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: PIO1_PIN9_SEC_MASK_A) -> Self {
        variant as u8 != 0
    }
}
impl PIO1_PIN9_SEC_MASK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIO1_PIN9_SEC_MASK_A {
        match self.bits {
            false => PIO1_PIN9_SEC_MASK_A::BLOCKED,
            true => PIO1_PIN9_SEC_MASK_A::READABLE,
        }
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == PIO1_PIN9_SEC_MASK_A::BLOCKED
    }
    #[doc = "Checks if the value of the field is `READABLE`"]
    #[inline(always)]
    pub fn is_readable(&self) -> bool {
        *self == PIO1_PIN9_SEC_MASK_A::READABLE
    }
}
#[doc = "Field `PIO1_PIN9_SEC_MASK` writer - Secure mask for pin P1_9"]
pub type PIO1_PIN9_SEC_MASK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SEC_GPIO_MASK1_SPEC, PIO1_PIN9_SEC_MASK_A, O>;
impl<'a, const O: u8> PIO1_PIN9_SEC_MASK_W<'a, O> {
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut W {
        self.variant(PIO1_PIN9_SEC_MASK_A::BLOCKED)
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn readable(self) -> &'a mut W {
        self.variant(PIO1_PIN9_SEC_MASK_A::READABLE)
    }
}
#[doc = "Field `PIO1_PIN10_SEC_MASK` reader - Secure mask for pin P1_10"]
pub type PIO1_PIN10_SEC_MASK_R = crate::BitReader<PIO1_PIN10_SEC_MASK_A>;
#[doc = "Secure mask for pin P1_10\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIO1_PIN10_SEC_MASK_A {
    #[doc = "0: Pin state is blocked to non-secure world."]
    BLOCKED = 0,
    #[doc = "1: Pin state is readable by non-secure world."]
    READABLE = 1,
}
impl From<PIO1_PIN10_SEC_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: PIO1_PIN10_SEC_MASK_A) -> Self {
        variant as u8 != 0
    }
}
impl PIO1_PIN10_SEC_MASK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIO1_PIN10_SEC_MASK_A {
        match self.bits {
            false => PIO1_PIN10_SEC_MASK_A::BLOCKED,
            true => PIO1_PIN10_SEC_MASK_A::READABLE,
        }
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == PIO1_PIN10_SEC_MASK_A::BLOCKED
    }
    #[doc = "Checks if the value of the field is `READABLE`"]
    #[inline(always)]
    pub fn is_readable(&self) -> bool {
        *self == PIO1_PIN10_SEC_MASK_A::READABLE
    }
}
#[doc = "Field `PIO1_PIN10_SEC_MASK` writer - Secure mask for pin P1_10"]
pub type PIO1_PIN10_SEC_MASK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SEC_GPIO_MASK1_SPEC, PIO1_PIN10_SEC_MASK_A, O>;
impl<'a, const O: u8> PIO1_PIN10_SEC_MASK_W<'a, O> {
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut W {
        self.variant(PIO1_PIN10_SEC_MASK_A::BLOCKED)
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn readable(self) -> &'a mut W {
        self.variant(PIO1_PIN10_SEC_MASK_A::READABLE)
    }
}
#[doc = "Field `PIO1_PIN11_SEC_MASK` reader - Secure mask for pin P1_11"]
pub type PIO1_PIN11_SEC_MASK_R = crate::BitReader<PIO1_PIN11_SEC_MASK_A>;
#[doc = "Secure mask for pin P1_11\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIO1_PIN11_SEC_MASK_A {
    #[doc = "0: Pin state is blocked to non-secure world."]
    BLOCKED = 0,
    #[doc = "1: Pin state is readable by non-secure world."]
    READABLE = 1,
}
impl From<PIO1_PIN11_SEC_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: PIO1_PIN11_SEC_MASK_A) -> Self {
        variant as u8 != 0
    }
}
impl PIO1_PIN11_SEC_MASK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIO1_PIN11_SEC_MASK_A {
        match self.bits {
            false => PIO1_PIN11_SEC_MASK_A::BLOCKED,
            true => PIO1_PIN11_SEC_MASK_A::READABLE,
        }
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == PIO1_PIN11_SEC_MASK_A::BLOCKED
    }
    #[doc = "Checks if the value of the field is `READABLE`"]
    #[inline(always)]
    pub fn is_readable(&self) -> bool {
        *self == PIO1_PIN11_SEC_MASK_A::READABLE
    }
}
#[doc = "Field `PIO1_PIN11_SEC_MASK` writer - Secure mask for pin P1_11"]
pub type PIO1_PIN11_SEC_MASK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SEC_GPIO_MASK1_SPEC, PIO1_PIN11_SEC_MASK_A, O>;
impl<'a, const O: u8> PIO1_PIN11_SEC_MASK_W<'a, O> {
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut W {
        self.variant(PIO1_PIN11_SEC_MASK_A::BLOCKED)
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn readable(self) -> &'a mut W {
        self.variant(PIO1_PIN11_SEC_MASK_A::READABLE)
    }
}
#[doc = "Field `PIO1_PIN12_SEC_MASK` reader - Secure mask for pin P1_12"]
pub type PIO1_PIN12_SEC_MASK_R = crate::BitReader<PIO1_PIN12_SEC_MASK_A>;
#[doc = "Secure mask for pin P1_12\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIO1_PIN12_SEC_MASK_A {
    #[doc = "0: Pin state is blocked to non-secure world."]
    BLOCKED = 0,
    #[doc = "1: Pin state is readable by non-secure world."]
    READABLE = 1,
}
impl From<PIO1_PIN12_SEC_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: PIO1_PIN12_SEC_MASK_A) -> Self {
        variant as u8 != 0
    }
}
impl PIO1_PIN12_SEC_MASK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIO1_PIN12_SEC_MASK_A {
        match self.bits {
            false => PIO1_PIN12_SEC_MASK_A::BLOCKED,
            true => PIO1_PIN12_SEC_MASK_A::READABLE,
        }
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == PIO1_PIN12_SEC_MASK_A::BLOCKED
    }
    #[doc = "Checks if the value of the field is `READABLE`"]
    #[inline(always)]
    pub fn is_readable(&self) -> bool {
        *self == PIO1_PIN12_SEC_MASK_A::READABLE
    }
}
#[doc = "Field `PIO1_PIN12_SEC_MASK` writer - Secure mask for pin P1_12"]
pub type PIO1_PIN12_SEC_MASK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SEC_GPIO_MASK1_SPEC, PIO1_PIN12_SEC_MASK_A, O>;
impl<'a, const O: u8> PIO1_PIN12_SEC_MASK_W<'a, O> {
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut W {
        self.variant(PIO1_PIN12_SEC_MASK_A::BLOCKED)
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn readable(self) -> &'a mut W {
        self.variant(PIO1_PIN12_SEC_MASK_A::READABLE)
    }
}
#[doc = "Field `PIO1_PIN13_SEC_MASK` reader - Secure mask for pin P1_13"]
pub type PIO1_PIN13_SEC_MASK_R = crate::BitReader<PIO1_PIN13_SEC_MASK_A>;
#[doc = "Secure mask for pin P1_13\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIO1_PIN13_SEC_MASK_A {
    #[doc = "0: Pin state is blocked to non-secure world."]
    BLOCKED = 0,
    #[doc = "1: Pin state is readable by non-secure world."]
    READABLE = 1,
}
impl From<PIO1_PIN13_SEC_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: PIO1_PIN13_SEC_MASK_A) -> Self {
        variant as u8 != 0
    }
}
impl PIO1_PIN13_SEC_MASK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIO1_PIN13_SEC_MASK_A {
        match self.bits {
            false => PIO1_PIN13_SEC_MASK_A::BLOCKED,
            true => PIO1_PIN13_SEC_MASK_A::READABLE,
        }
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == PIO1_PIN13_SEC_MASK_A::BLOCKED
    }
    #[doc = "Checks if the value of the field is `READABLE`"]
    #[inline(always)]
    pub fn is_readable(&self) -> bool {
        *self == PIO1_PIN13_SEC_MASK_A::READABLE
    }
}
#[doc = "Field `PIO1_PIN13_SEC_MASK` writer - Secure mask for pin P1_13"]
pub type PIO1_PIN13_SEC_MASK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SEC_GPIO_MASK1_SPEC, PIO1_PIN13_SEC_MASK_A, O>;
impl<'a, const O: u8> PIO1_PIN13_SEC_MASK_W<'a, O> {
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut W {
        self.variant(PIO1_PIN13_SEC_MASK_A::BLOCKED)
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn readable(self) -> &'a mut W {
        self.variant(PIO1_PIN13_SEC_MASK_A::READABLE)
    }
}
#[doc = "Field `PIO1_PIN14_SEC_MASK` reader - Secure mask for pin P1_14"]
pub type PIO1_PIN14_SEC_MASK_R = crate::BitReader<PIO1_PIN14_SEC_MASK_A>;
#[doc = "Secure mask for pin P1_14\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIO1_PIN14_SEC_MASK_A {
    #[doc = "0: Pin state is blocked to non-secure world."]
    BLOCKED = 0,
    #[doc = "1: Pin state is readable by non-secure world."]
    READABLE = 1,
}
impl From<PIO1_PIN14_SEC_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: PIO1_PIN14_SEC_MASK_A) -> Self {
        variant as u8 != 0
    }
}
impl PIO1_PIN14_SEC_MASK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIO1_PIN14_SEC_MASK_A {
        match self.bits {
            false => PIO1_PIN14_SEC_MASK_A::BLOCKED,
            true => PIO1_PIN14_SEC_MASK_A::READABLE,
        }
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == PIO1_PIN14_SEC_MASK_A::BLOCKED
    }
    #[doc = "Checks if the value of the field is `READABLE`"]
    #[inline(always)]
    pub fn is_readable(&self) -> bool {
        *self == PIO1_PIN14_SEC_MASK_A::READABLE
    }
}
#[doc = "Field `PIO1_PIN14_SEC_MASK` writer - Secure mask for pin P1_14"]
pub type PIO1_PIN14_SEC_MASK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SEC_GPIO_MASK1_SPEC, PIO1_PIN14_SEC_MASK_A, O>;
impl<'a, const O: u8> PIO1_PIN14_SEC_MASK_W<'a, O> {
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut W {
        self.variant(PIO1_PIN14_SEC_MASK_A::BLOCKED)
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn readable(self) -> &'a mut W {
        self.variant(PIO1_PIN14_SEC_MASK_A::READABLE)
    }
}
#[doc = "Field `PIO1_PIN15_SEC_MASK` reader - Secure mask for pin P1_15"]
pub type PIO1_PIN15_SEC_MASK_R = crate::BitReader<PIO1_PIN15_SEC_MASK_A>;
#[doc = "Secure mask for pin P1_15\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIO1_PIN15_SEC_MASK_A {
    #[doc = "0: Pin state is blocked to non-secure world."]
    BLOCKED = 0,
    #[doc = "1: Pin state is readable by non-secure world."]
    READABLE = 1,
}
impl From<PIO1_PIN15_SEC_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: PIO1_PIN15_SEC_MASK_A) -> Self {
        variant as u8 != 0
    }
}
impl PIO1_PIN15_SEC_MASK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIO1_PIN15_SEC_MASK_A {
        match self.bits {
            false => PIO1_PIN15_SEC_MASK_A::BLOCKED,
            true => PIO1_PIN15_SEC_MASK_A::READABLE,
        }
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == PIO1_PIN15_SEC_MASK_A::BLOCKED
    }
    #[doc = "Checks if the value of the field is `READABLE`"]
    #[inline(always)]
    pub fn is_readable(&self) -> bool {
        *self == PIO1_PIN15_SEC_MASK_A::READABLE
    }
}
#[doc = "Field `PIO1_PIN15_SEC_MASK` writer - Secure mask for pin P1_15"]
pub type PIO1_PIN15_SEC_MASK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SEC_GPIO_MASK1_SPEC, PIO1_PIN15_SEC_MASK_A, O>;
impl<'a, const O: u8> PIO1_PIN15_SEC_MASK_W<'a, O> {
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut W {
        self.variant(PIO1_PIN15_SEC_MASK_A::BLOCKED)
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn readable(self) -> &'a mut W {
        self.variant(PIO1_PIN15_SEC_MASK_A::READABLE)
    }
}
#[doc = "Field `PIO1_PIN16_SEC_MASK` reader - Secure mask for pin P1_16"]
pub type PIO1_PIN16_SEC_MASK_R = crate::BitReader<PIO1_PIN16_SEC_MASK_A>;
#[doc = "Secure mask for pin P1_16\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIO1_PIN16_SEC_MASK_A {
    #[doc = "0: Pin state is blocked to non-secure world."]
    BLOCKED = 0,
    #[doc = "1: Pin state is readable by non-secure world."]
    READABLE = 1,
}
impl From<PIO1_PIN16_SEC_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: PIO1_PIN16_SEC_MASK_A) -> Self {
        variant as u8 != 0
    }
}
impl PIO1_PIN16_SEC_MASK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIO1_PIN16_SEC_MASK_A {
        match self.bits {
            false => PIO1_PIN16_SEC_MASK_A::BLOCKED,
            true => PIO1_PIN16_SEC_MASK_A::READABLE,
        }
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == PIO1_PIN16_SEC_MASK_A::BLOCKED
    }
    #[doc = "Checks if the value of the field is `READABLE`"]
    #[inline(always)]
    pub fn is_readable(&self) -> bool {
        *self == PIO1_PIN16_SEC_MASK_A::READABLE
    }
}
#[doc = "Field `PIO1_PIN16_SEC_MASK` writer - Secure mask for pin P1_16"]
pub type PIO1_PIN16_SEC_MASK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SEC_GPIO_MASK1_SPEC, PIO1_PIN16_SEC_MASK_A, O>;
impl<'a, const O: u8> PIO1_PIN16_SEC_MASK_W<'a, O> {
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut W {
        self.variant(PIO1_PIN16_SEC_MASK_A::BLOCKED)
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn readable(self) -> &'a mut W {
        self.variant(PIO1_PIN16_SEC_MASK_A::READABLE)
    }
}
#[doc = "Field `PIO1_PIN17_SEC_MASK` reader - Secure mask for pin P1_17"]
pub type PIO1_PIN17_SEC_MASK_R = crate::BitReader<PIO1_PIN17_SEC_MASK_A>;
#[doc = "Secure mask for pin P1_17\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIO1_PIN17_SEC_MASK_A {
    #[doc = "0: Pin state is blocked to non-secure world."]
    BLOCKED = 0,
    #[doc = "1: Pin state is readable by non-secure world."]
    READABLE = 1,
}
impl From<PIO1_PIN17_SEC_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: PIO1_PIN17_SEC_MASK_A) -> Self {
        variant as u8 != 0
    }
}
impl PIO1_PIN17_SEC_MASK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIO1_PIN17_SEC_MASK_A {
        match self.bits {
            false => PIO1_PIN17_SEC_MASK_A::BLOCKED,
            true => PIO1_PIN17_SEC_MASK_A::READABLE,
        }
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == PIO1_PIN17_SEC_MASK_A::BLOCKED
    }
    #[doc = "Checks if the value of the field is `READABLE`"]
    #[inline(always)]
    pub fn is_readable(&self) -> bool {
        *self == PIO1_PIN17_SEC_MASK_A::READABLE
    }
}
#[doc = "Field `PIO1_PIN17_SEC_MASK` writer - Secure mask for pin P1_17"]
pub type PIO1_PIN17_SEC_MASK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SEC_GPIO_MASK1_SPEC, PIO1_PIN17_SEC_MASK_A, O>;
impl<'a, const O: u8> PIO1_PIN17_SEC_MASK_W<'a, O> {
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut W {
        self.variant(PIO1_PIN17_SEC_MASK_A::BLOCKED)
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn readable(self) -> &'a mut W {
        self.variant(PIO1_PIN17_SEC_MASK_A::READABLE)
    }
}
#[doc = "Field `PIO1_PIN18_SEC_MASK` reader - Secure mask for pin P1_18"]
pub type PIO1_PIN18_SEC_MASK_R = crate::BitReader<PIO1_PIN18_SEC_MASK_A>;
#[doc = "Secure mask for pin P1_18\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIO1_PIN18_SEC_MASK_A {
    #[doc = "0: Pin state is blocked to non-secure world."]
    BLOCKED = 0,
    #[doc = "1: Pin state is readable by non-secure world."]
    READABLE = 1,
}
impl From<PIO1_PIN18_SEC_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: PIO1_PIN18_SEC_MASK_A) -> Self {
        variant as u8 != 0
    }
}
impl PIO1_PIN18_SEC_MASK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIO1_PIN18_SEC_MASK_A {
        match self.bits {
            false => PIO1_PIN18_SEC_MASK_A::BLOCKED,
            true => PIO1_PIN18_SEC_MASK_A::READABLE,
        }
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == PIO1_PIN18_SEC_MASK_A::BLOCKED
    }
    #[doc = "Checks if the value of the field is `READABLE`"]
    #[inline(always)]
    pub fn is_readable(&self) -> bool {
        *self == PIO1_PIN18_SEC_MASK_A::READABLE
    }
}
#[doc = "Field `PIO1_PIN18_SEC_MASK` writer - Secure mask for pin P1_18"]
pub type PIO1_PIN18_SEC_MASK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SEC_GPIO_MASK1_SPEC, PIO1_PIN18_SEC_MASK_A, O>;
impl<'a, const O: u8> PIO1_PIN18_SEC_MASK_W<'a, O> {
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut W {
        self.variant(PIO1_PIN18_SEC_MASK_A::BLOCKED)
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn readable(self) -> &'a mut W {
        self.variant(PIO1_PIN18_SEC_MASK_A::READABLE)
    }
}
#[doc = "Field `PIO1_PIN19_SEC_MASK` reader - Secure mask for pin P1_19"]
pub type PIO1_PIN19_SEC_MASK_R = crate::BitReader<PIO1_PIN19_SEC_MASK_A>;
#[doc = "Secure mask for pin P1_19\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIO1_PIN19_SEC_MASK_A {
    #[doc = "0: Pin state is blocked to non-secure world."]
    BLOCKED = 0,
    #[doc = "1: Pin state is readable by non-secure world."]
    READABLE = 1,
}
impl From<PIO1_PIN19_SEC_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: PIO1_PIN19_SEC_MASK_A) -> Self {
        variant as u8 != 0
    }
}
impl PIO1_PIN19_SEC_MASK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIO1_PIN19_SEC_MASK_A {
        match self.bits {
            false => PIO1_PIN19_SEC_MASK_A::BLOCKED,
            true => PIO1_PIN19_SEC_MASK_A::READABLE,
        }
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == PIO1_PIN19_SEC_MASK_A::BLOCKED
    }
    #[doc = "Checks if the value of the field is `READABLE`"]
    #[inline(always)]
    pub fn is_readable(&self) -> bool {
        *self == PIO1_PIN19_SEC_MASK_A::READABLE
    }
}
#[doc = "Field `PIO1_PIN19_SEC_MASK` writer - Secure mask for pin P1_19"]
pub type PIO1_PIN19_SEC_MASK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SEC_GPIO_MASK1_SPEC, PIO1_PIN19_SEC_MASK_A, O>;
impl<'a, const O: u8> PIO1_PIN19_SEC_MASK_W<'a, O> {
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut W {
        self.variant(PIO1_PIN19_SEC_MASK_A::BLOCKED)
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn readable(self) -> &'a mut W {
        self.variant(PIO1_PIN19_SEC_MASK_A::READABLE)
    }
}
#[doc = "Field `PIO1_PIN20_SEC_MASK` reader - Secure mask for pin P1_20"]
pub type PIO1_PIN20_SEC_MASK_R = crate::BitReader<PIO1_PIN20_SEC_MASK_A>;
#[doc = "Secure mask for pin P1_20\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIO1_PIN20_SEC_MASK_A {
    #[doc = "0: Pin state is blocked to non-secure world."]
    BLOCKED = 0,
    #[doc = "1: Pin state is readable by non-secure world."]
    READABLE = 1,
}
impl From<PIO1_PIN20_SEC_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: PIO1_PIN20_SEC_MASK_A) -> Self {
        variant as u8 != 0
    }
}
impl PIO1_PIN20_SEC_MASK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIO1_PIN20_SEC_MASK_A {
        match self.bits {
            false => PIO1_PIN20_SEC_MASK_A::BLOCKED,
            true => PIO1_PIN20_SEC_MASK_A::READABLE,
        }
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == PIO1_PIN20_SEC_MASK_A::BLOCKED
    }
    #[doc = "Checks if the value of the field is `READABLE`"]
    #[inline(always)]
    pub fn is_readable(&self) -> bool {
        *self == PIO1_PIN20_SEC_MASK_A::READABLE
    }
}
#[doc = "Field `PIO1_PIN20_SEC_MASK` writer - Secure mask for pin P1_20"]
pub type PIO1_PIN20_SEC_MASK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SEC_GPIO_MASK1_SPEC, PIO1_PIN20_SEC_MASK_A, O>;
impl<'a, const O: u8> PIO1_PIN20_SEC_MASK_W<'a, O> {
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut W {
        self.variant(PIO1_PIN20_SEC_MASK_A::BLOCKED)
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn readable(self) -> &'a mut W {
        self.variant(PIO1_PIN20_SEC_MASK_A::READABLE)
    }
}
#[doc = "Field `PIO1_PIN21_SEC_MASK` reader - Secure mask for pin P1_21"]
pub type PIO1_PIN21_SEC_MASK_R = crate::BitReader<PIO1_PIN21_SEC_MASK_A>;
#[doc = "Secure mask for pin P1_21\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIO1_PIN21_SEC_MASK_A {
    #[doc = "0: Pin state is blocked to non-secure world."]
    BLOCKED = 0,
    #[doc = "1: Pin state is readable by non-secure world."]
    READABLE = 1,
}
impl From<PIO1_PIN21_SEC_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: PIO1_PIN21_SEC_MASK_A) -> Self {
        variant as u8 != 0
    }
}
impl PIO1_PIN21_SEC_MASK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIO1_PIN21_SEC_MASK_A {
        match self.bits {
            false => PIO1_PIN21_SEC_MASK_A::BLOCKED,
            true => PIO1_PIN21_SEC_MASK_A::READABLE,
        }
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == PIO1_PIN21_SEC_MASK_A::BLOCKED
    }
    #[doc = "Checks if the value of the field is `READABLE`"]
    #[inline(always)]
    pub fn is_readable(&self) -> bool {
        *self == PIO1_PIN21_SEC_MASK_A::READABLE
    }
}
#[doc = "Field `PIO1_PIN21_SEC_MASK` writer - Secure mask for pin P1_21"]
pub type PIO1_PIN21_SEC_MASK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SEC_GPIO_MASK1_SPEC, PIO1_PIN21_SEC_MASK_A, O>;
impl<'a, const O: u8> PIO1_PIN21_SEC_MASK_W<'a, O> {
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut W {
        self.variant(PIO1_PIN21_SEC_MASK_A::BLOCKED)
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn readable(self) -> &'a mut W {
        self.variant(PIO1_PIN21_SEC_MASK_A::READABLE)
    }
}
#[doc = "Field `PIO1_PIN22_SEC_MASK` reader - Secure mask for pin P1_22"]
pub type PIO1_PIN22_SEC_MASK_R = crate::BitReader<PIO1_PIN22_SEC_MASK_A>;
#[doc = "Secure mask for pin P1_22\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIO1_PIN22_SEC_MASK_A {
    #[doc = "0: Pin state is blocked to non-secure world."]
    BLOCKED = 0,
    #[doc = "1: Pin state is readable by non-secure world."]
    READABLE = 1,
}
impl From<PIO1_PIN22_SEC_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: PIO1_PIN22_SEC_MASK_A) -> Self {
        variant as u8 != 0
    }
}
impl PIO1_PIN22_SEC_MASK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIO1_PIN22_SEC_MASK_A {
        match self.bits {
            false => PIO1_PIN22_SEC_MASK_A::BLOCKED,
            true => PIO1_PIN22_SEC_MASK_A::READABLE,
        }
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == PIO1_PIN22_SEC_MASK_A::BLOCKED
    }
    #[doc = "Checks if the value of the field is `READABLE`"]
    #[inline(always)]
    pub fn is_readable(&self) -> bool {
        *self == PIO1_PIN22_SEC_MASK_A::READABLE
    }
}
#[doc = "Field `PIO1_PIN22_SEC_MASK` writer - Secure mask for pin P1_22"]
pub type PIO1_PIN22_SEC_MASK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SEC_GPIO_MASK1_SPEC, PIO1_PIN22_SEC_MASK_A, O>;
impl<'a, const O: u8> PIO1_PIN22_SEC_MASK_W<'a, O> {
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut W {
        self.variant(PIO1_PIN22_SEC_MASK_A::BLOCKED)
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn readable(self) -> &'a mut W {
        self.variant(PIO1_PIN22_SEC_MASK_A::READABLE)
    }
}
#[doc = "Field `PIO1_PIN23_SEC_MASK` reader - Secure mask for pin P1_23"]
pub type PIO1_PIN23_SEC_MASK_R = crate::BitReader<PIO1_PIN23_SEC_MASK_A>;
#[doc = "Secure mask for pin P1_23\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIO1_PIN23_SEC_MASK_A {
    #[doc = "0: Pin state is blocked to non-secure world."]
    BLOCKED = 0,
    #[doc = "1: Pin state is readable by non-secure world."]
    READABLE = 1,
}
impl From<PIO1_PIN23_SEC_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: PIO1_PIN23_SEC_MASK_A) -> Self {
        variant as u8 != 0
    }
}
impl PIO1_PIN23_SEC_MASK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIO1_PIN23_SEC_MASK_A {
        match self.bits {
            false => PIO1_PIN23_SEC_MASK_A::BLOCKED,
            true => PIO1_PIN23_SEC_MASK_A::READABLE,
        }
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == PIO1_PIN23_SEC_MASK_A::BLOCKED
    }
    #[doc = "Checks if the value of the field is `READABLE`"]
    #[inline(always)]
    pub fn is_readable(&self) -> bool {
        *self == PIO1_PIN23_SEC_MASK_A::READABLE
    }
}
#[doc = "Field `PIO1_PIN23_SEC_MASK` writer - Secure mask for pin P1_23"]
pub type PIO1_PIN23_SEC_MASK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SEC_GPIO_MASK1_SPEC, PIO1_PIN23_SEC_MASK_A, O>;
impl<'a, const O: u8> PIO1_PIN23_SEC_MASK_W<'a, O> {
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut W {
        self.variant(PIO1_PIN23_SEC_MASK_A::BLOCKED)
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn readable(self) -> &'a mut W {
        self.variant(PIO1_PIN23_SEC_MASK_A::READABLE)
    }
}
#[doc = "Field `PIO1_PIN24_SEC_MASK` reader - Secure mask for pin P1_24"]
pub type PIO1_PIN24_SEC_MASK_R = crate::BitReader<PIO1_PIN24_SEC_MASK_A>;
#[doc = "Secure mask for pin P1_24\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIO1_PIN24_SEC_MASK_A {
    #[doc = "0: Pin state is blocked to non-secure world."]
    BLOCKED = 0,
    #[doc = "1: Pin state is readable by non-secure world."]
    READABLE = 1,
}
impl From<PIO1_PIN24_SEC_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: PIO1_PIN24_SEC_MASK_A) -> Self {
        variant as u8 != 0
    }
}
impl PIO1_PIN24_SEC_MASK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIO1_PIN24_SEC_MASK_A {
        match self.bits {
            false => PIO1_PIN24_SEC_MASK_A::BLOCKED,
            true => PIO1_PIN24_SEC_MASK_A::READABLE,
        }
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == PIO1_PIN24_SEC_MASK_A::BLOCKED
    }
    #[doc = "Checks if the value of the field is `READABLE`"]
    #[inline(always)]
    pub fn is_readable(&self) -> bool {
        *self == PIO1_PIN24_SEC_MASK_A::READABLE
    }
}
#[doc = "Field `PIO1_PIN24_SEC_MASK` writer - Secure mask for pin P1_24"]
pub type PIO1_PIN24_SEC_MASK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SEC_GPIO_MASK1_SPEC, PIO1_PIN24_SEC_MASK_A, O>;
impl<'a, const O: u8> PIO1_PIN24_SEC_MASK_W<'a, O> {
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut W {
        self.variant(PIO1_PIN24_SEC_MASK_A::BLOCKED)
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn readable(self) -> &'a mut W {
        self.variant(PIO1_PIN24_SEC_MASK_A::READABLE)
    }
}
#[doc = "Field `PIO1_PIN25_SEC_MASK` reader - Secure mask for pin P1_25"]
pub type PIO1_PIN25_SEC_MASK_R = crate::BitReader<PIO1_PIN25_SEC_MASK_A>;
#[doc = "Secure mask for pin P1_25\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIO1_PIN25_SEC_MASK_A {
    #[doc = "0: Pin state is blocked to non-secure world."]
    BLOCKED = 0,
    #[doc = "1: Pin state is readable by non-secure world."]
    READABLE = 1,
}
impl From<PIO1_PIN25_SEC_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: PIO1_PIN25_SEC_MASK_A) -> Self {
        variant as u8 != 0
    }
}
impl PIO1_PIN25_SEC_MASK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIO1_PIN25_SEC_MASK_A {
        match self.bits {
            false => PIO1_PIN25_SEC_MASK_A::BLOCKED,
            true => PIO1_PIN25_SEC_MASK_A::READABLE,
        }
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == PIO1_PIN25_SEC_MASK_A::BLOCKED
    }
    #[doc = "Checks if the value of the field is `READABLE`"]
    #[inline(always)]
    pub fn is_readable(&self) -> bool {
        *self == PIO1_PIN25_SEC_MASK_A::READABLE
    }
}
#[doc = "Field `PIO1_PIN25_SEC_MASK` writer - Secure mask for pin P1_25"]
pub type PIO1_PIN25_SEC_MASK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SEC_GPIO_MASK1_SPEC, PIO1_PIN25_SEC_MASK_A, O>;
impl<'a, const O: u8> PIO1_PIN25_SEC_MASK_W<'a, O> {
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut W {
        self.variant(PIO1_PIN25_SEC_MASK_A::BLOCKED)
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn readable(self) -> &'a mut W {
        self.variant(PIO1_PIN25_SEC_MASK_A::READABLE)
    }
}
#[doc = "Field `PIO1_PIN26_SEC_MASK` reader - Secure mask for pin P1_26"]
pub type PIO1_PIN26_SEC_MASK_R = crate::BitReader<PIO1_PIN26_SEC_MASK_A>;
#[doc = "Secure mask for pin P1_26\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIO1_PIN26_SEC_MASK_A {
    #[doc = "0: Pin state is blocked to non-secure world."]
    BLOCKED = 0,
    #[doc = "1: Pin state is readable by non-secure world."]
    READABLE = 1,
}
impl From<PIO1_PIN26_SEC_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: PIO1_PIN26_SEC_MASK_A) -> Self {
        variant as u8 != 0
    }
}
impl PIO1_PIN26_SEC_MASK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIO1_PIN26_SEC_MASK_A {
        match self.bits {
            false => PIO1_PIN26_SEC_MASK_A::BLOCKED,
            true => PIO1_PIN26_SEC_MASK_A::READABLE,
        }
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == PIO1_PIN26_SEC_MASK_A::BLOCKED
    }
    #[doc = "Checks if the value of the field is `READABLE`"]
    #[inline(always)]
    pub fn is_readable(&self) -> bool {
        *self == PIO1_PIN26_SEC_MASK_A::READABLE
    }
}
#[doc = "Field `PIO1_PIN26_SEC_MASK` writer - Secure mask for pin P1_26"]
pub type PIO1_PIN26_SEC_MASK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SEC_GPIO_MASK1_SPEC, PIO1_PIN26_SEC_MASK_A, O>;
impl<'a, const O: u8> PIO1_PIN26_SEC_MASK_W<'a, O> {
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut W {
        self.variant(PIO1_PIN26_SEC_MASK_A::BLOCKED)
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn readable(self) -> &'a mut W {
        self.variant(PIO1_PIN26_SEC_MASK_A::READABLE)
    }
}
#[doc = "Field `PIO1_PIN27_SEC_MASK` reader - Secure mask for pin P1_27"]
pub type PIO1_PIN27_SEC_MASK_R = crate::BitReader<PIO1_PIN27_SEC_MASK_A>;
#[doc = "Secure mask for pin P1_27\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIO1_PIN27_SEC_MASK_A {
    #[doc = "0: Pin state is blocked to non-secure world."]
    BLOCKED = 0,
    #[doc = "1: Pin state is readable by non-secure world."]
    READABLE = 1,
}
impl From<PIO1_PIN27_SEC_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: PIO1_PIN27_SEC_MASK_A) -> Self {
        variant as u8 != 0
    }
}
impl PIO1_PIN27_SEC_MASK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIO1_PIN27_SEC_MASK_A {
        match self.bits {
            false => PIO1_PIN27_SEC_MASK_A::BLOCKED,
            true => PIO1_PIN27_SEC_MASK_A::READABLE,
        }
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == PIO1_PIN27_SEC_MASK_A::BLOCKED
    }
    #[doc = "Checks if the value of the field is `READABLE`"]
    #[inline(always)]
    pub fn is_readable(&self) -> bool {
        *self == PIO1_PIN27_SEC_MASK_A::READABLE
    }
}
#[doc = "Field `PIO1_PIN27_SEC_MASK` writer - Secure mask for pin P1_27"]
pub type PIO1_PIN27_SEC_MASK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SEC_GPIO_MASK1_SPEC, PIO1_PIN27_SEC_MASK_A, O>;
impl<'a, const O: u8> PIO1_PIN27_SEC_MASK_W<'a, O> {
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut W {
        self.variant(PIO1_PIN27_SEC_MASK_A::BLOCKED)
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn readable(self) -> &'a mut W {
        self.variant(PIO1_PIN27_SEC_MASK_A::READABLE)
    }
}
#[doc = "Field `PIO1_PIN28_SEC_MASK` reader - Secure mask for pin P1_28"]
pub type PIO1_PIN28_SEC_MASK_R = crate::BitReader<PIO1_PIN28_SEC_MASK_A>;
#[doc = "Secure mask for pin P1_28\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIO1_PIN28_SEC_MASK_A {
    #[doc = "0: Pin state is blocked to non-secure world."]
    BLOCKED = 0,
    #[doc = "1: Pin state is readable by non-secure world."]
    READABLE = 1,
}
impl From<PIO1_PIN28_SEC_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: PIO1_PIN28_SEC_MASK_A) -> Self {
        variant as u8 != 0
    }
}
impl PIO1_PIN28_SEC_MASK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIO1_PIN28_SEC_MASK_A {
        match self.bits {
            false => PIO1_PIN28_SEC_MASK_A::BLOCKED,
            true => PIO1_PIN28_SEC_MASK_A::READABLE,
        }
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == PIO1_PIN28_SEC_MASK_A::BLOCKED
    }
    #[doc = "Checks if the value of the field is `READABLE`"]
    #[inline(always)]
    pub fn is_readable(&self) -> bool {
        *self == PIO1_PIN28_SEC_MASK_A::READABLE
    }
}
#[doc = "Field `PIO1_PIN28_SEC_MASK` writer - Secure mask for pin P1_28"]
pub type PIO1_PIN28_SEC_MASK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SEC_GPIO_MASK1_SPEC, PIO1_PIN28_SEC_MASK_A, O>;
impl<'a, const O: u8> PIO1_PIN28_SEC_MASK_W<'a, O> {
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut W {
        self.variant(PIO1_PIN28_SEC_MASK_A::BLOCKED)
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn readable(self) -> &'a mut W {
        self.variant(PIO1_PIN28_SEC_MASK_A::READABLE)
    }
}
#[doc = "Field `PIO1_PIN29_SEC_MASK` reader - Secure mask for pin P1_29"]
pub type PIO1_PIN29_SEC_MASK_R = crate::BitReader<PIO1_PIN29_SEC_MASK_A>;
#[doc = "Secure mask for pin P1_29\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIO1_PIN29_SEC_MASK_A {
    #[doc = "0: Pin state is blocked to non-secure world."]
    BLOCKED = 0,
    #[doc = "1: Pin state is readable by non-secure world."]
    READABLE = 1,
}
impl From<PIO1_PIN29_SEC_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: PIO1_PIN29_SEC_MASK_A) -> Self {
        variant as u8 != 0
    }
}
impl PIO1_PIN29_SEC_MASK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIO1_PIN29_SEC_MASK_A {
        match self.bits {
            false => PIO1_PIN29_SEC_MASK_A::BLOCKED,
            true => PIO1_PIN29_SEC_MASK_A::READABLE,
        }
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == PIO1_PIN29_SEC_MASK_A::BLOCKED
    }
    #[doc = "Checks if the value of the field is `READABLE`"]
    #[inline(always)]
    pub fn is_readable(&self) -> bool {
        *self == PIO1_PIN29_SEC_MASK_A::READABLE
    }
}
#[doc = "Field `PIO1_PIN29_SEC_MASK` writer - Secure mask for pin P1_29"]
pub type PIO1_PIN29_SEC_MASK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SEC_GPIO_MASK1_SPEC, PIO1_PIN29_SEC_MASK_A, O>;
impl<'a, const O: u8> PIO1_PIN29_SEC_MASK_W<'a, O> {
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut W {
        self.variant(PIO1_PIN29_SEC_MASK_A::BLOCKED)
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn readable(self) -> &'a mut W {
        self.variant(PIO1_PIN29_SEC_MASK_A::READABLE)
    }
}
#[doc = "Field `PIO1_PIN30_SEC_MASK` reader - Secure mask for pin P1_30"]
pub type PIO1_PIN30_SEC_MASK_R = crate::BitReader<PIO1_PIN30_SEC_MASK_A>;
#[doc = "Secure mask for pin P1_30\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIO1_PIN30_SEC_MASK_A {
    #[doc = "0: Pin state is blocked to non-secure world."]
    BLOCKED = 0,
    #[doc = "1: Pin state is readable by non-secure world."]
    READABLE = 1,
}
impl From<PIO1_PIN30_SEC_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: PIO1_PIN30_SEC_MASK_A) -> Self {
        variant as u8 != 0
    }
}
impl PIO1_PIN30_SEC_MASK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIO1_PIN30_SEC_MASK_A {
        match self.bits {
            false => PIO1_PIN30_SEC_MASK_A::BLOCKED,
            true => PIO1_PIN30_SEC_MASK_A::READABLE,
        }
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == PIO1_PIN30_SEC_MASK_A::BLOCKED
    }
    #[doc = "Checks if the value of the field is `READABLE`"]
    #[inline(always)]
    pub fn is_readable(&self) -> bool {
        *self == PIO1_PIN30_SEC_MASK_A::READABLE
    }
}
#[doc = "Field `PIO1_PIN30_SEC_MASK` writer - Secure mask for pin P1_30"]
pub type PIO1_PIN30_SEC_MASK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SEC_GPIO_MASK1_SPEC, PIO1_PIN30_SEC_MASK_A, O>;
impl<'a, const O: u8> PIO1_PIN30_SEC_MASK_W<'a, O> {
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut W {
        self.variant(PIO1_PIN30_SEC_MASK_A::BLOCKED)
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn readable(self) -> &'a mut W {
        self.variant(PIO1_PIN30_SEC_MASK_A::READABLE)
    }
}
#[doc = "Field `PIO1_PIN31_SEC_MASK` reader - Secure mask for pin P1_31"]
pub type PIO1_PIN31_SEC_MASK_R = crate::BitReader<PIO1_PIN31_SEC_MASK_A>;
#[doc = "Secure mask for pin P1_31\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIO1_PIN31_SEC_MASK_A {
    #[doc = "0: Pin state is blocked to non-secure world."]
    BLOCKED = 0,
    #[doc = "1: Pin state is readable by non-secure world."]
    READABLE = 1,
}
impl From<PIO1_PIN31_SEC_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: PIO1_PIN31_SEC_MASK_A) -> Self {
        variant as u8 != 0
    }
}
impl PIO1_PIN31_SEC_MASK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIO1_PIN31_SEC_MASK_A {
        match self.bits {
            false => PIO1_PIN31_SEC_MASK_A::BLOCKED,
            true => PIO1_PIN31_SEC_MASK_A::READABLE,
        }
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == PIO1_PIN31_SEC_MASK_A::BLOCKED
    }
    #[doc = "Checks if the value of the field is `READABLE`"]
    #[inline(always)]
    pub fn is_readable(&self) -> bool {
        *self == PIO1_PIN31_SEC_MASK_A::READABLE
    }
}
#[doc = "Field `PIO1_PIN31_SEC_MASK` writer - Secure mask for pin P1_31"]
pub type PIO1_PIN31_SEC_MASK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SEC_GPIO_MASK1_SPEC, PIO1_PIN31_SEC_MASK_A, O>;
impl<'a, const O: u8> PIO1_PIN31_SEC_MASK_W<'a, O> {
    #[doc = "Pin state is blocked to non-secure world."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut W {
        self.variant(PIO1_PIN31_SEC_MASK_A::BLOCKED)
    }
    #[doc = "Pin state is readable by non-secure world."]
    #[inline(always)]
    pub fn readable(self) -> &'a mut W {
        self.variant(PIO1_PIN31_SEC_MASK_A::READABLE)
    }
}
impl R {
    #[doc = "Bit 0 - Secure mask for pin P1_0"]
    #[inline(always)]
    pub fn pio1_pin0_sec_mask(&self) -> PIO1_PIN0_SEC_MASK_R {
        PIO1_PIN0_SEC_MASK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Secure mask for pin P1_1"]
    #[inline(always)]
    pub fn pio1_pin1_sec_mask(&self) -> PIO1_PIN1_SEC_MASK_R {
        PIO1_PIN1_SEC_MASK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Secure mask for pin P1_2"]
    #[inline(always)]
    pub fn pio1_pin2_sec_mask(&self) -> PIO1_PIN2_SEC_MASK_R {
        PIO1_PIN2_SEC_MASK_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Secure mask for pin P1_3"]
    #[inline(always)]
    pub fn pio1_pin3_sec_mask(&self) -> PIO1_PIN3_SEC_MASK_R {
        PIO1_PIN3_SEC_MASK_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Secure mask for pin P1_4"]
    #[inline(always)]
    pub fn pio1_pin4_sec_mask(&self) -> PIO1_PIN4_SEC_MASK_R {
        PIO1_PIN4_SEC_MASK_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Secure mask for pin P1_5"]
    #[inline(always)]
    pub fn pio1_pin5_sec_mask(&self) -> PIO1_PIN5_SEC_MASK_R {
        PIO1_PIN5_SEC_MASK_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Secure mask for pin P1_6"]
    #[inline(always)]
    pub fn pio1_pin6_sec_mask(&self) -> PIO1_PIN6_SEC_MASK_R {
        PIO1_PIN6_SEC_MASK_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Secure mask for pin P1_7"]
    #[inline(always)]
    pub fn pio1_pin7_sec_mask(&self) -> PIO1_PIN7_SEC_MASK_R {
        PIO1_PIN7_SEC_MASK_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Secure mask for pin P1_8"]
    #[inline(always)]
    pub fn pio1_pin8_sec_mask(&self) -> PIO1_PIN8_SEC_MASK_R {
        PIO1_PIN8_SEC_MASK_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Secure mask for pin P1_9"]
    #[inline(always)]
    pub fn pio1_pin9_sec_mask(&self) -> PIO1_PIN9_SEC_MASK_R {
        PIO1_PIN9_SEC_MASK_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Secure mask for pin P1_10"]
    #[inline(always)]
    pub fn pio1_pin10_sec_mask(&self) -> PIO1_PIN10_SEC_MASK_R {
        PIO1_PIN10_SEC_MASK_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Secure mask for pin P1_11"]
    #[inline(always)]
    pub fn pio1_pin11_sec_mask(&self) -> PIO1_PIN11_SEC_MASK_R {
        PIO1_PIN11_SEC_MASK_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Secure mask for pin P1_12"]
    #[inline(always)]
    pub fn pio1_pin12_sec_mask(&self) -> PIO1_PIN12_SEC_MASK_R {
        PIO1_PIN12_SEC_MASK_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Secure mask for pin P1_13"]
    #[inline(always)]
    pub fn pio1_pin13_sec_mask(&self) -> PIO1_PIN13_SEC_MASK_R {
        PIO1_PIN13_SEC_MASK_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Secure mask for pin P1_14"]
    #[inline(always)]
    pub fn pio1_pin14_sec_mask(&self) -> PIO1_PIN14_SEC_MASK_R {
        PIO1_PIN14_SEC_MASK_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Secure mask for pin P1_15"]
    #[inline(always)]
    pub fn pio1_pin15_sec_mask(&self) -> PIO1_PIN15_SEC_MASK_R {
        PIO1_PIN15_SEC_MASK_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Secure mask for pin P1_16"]
    #[inline(always)]
    pub fn pio1_pin16_sec_mask(&self) -> PIO1_PIN16_SEC_MASK_R {
        PIO1_PIN16_SEC_MASK_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Secure mask for pin P1_17"]
    #[inline(always)]
    pub fn pio1_pin17_sec_mask(&self) -> PIO1_PIN17_SEC_MASK_R {
        PIO1_PIN17_SEC_MASK_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Secure mask for pin P1_18"]
    #[inline(always)]
    pub fn pio1_pin18_sec_mask(&self) -> PIO1_PIN18_SEC_MASK_R {
        PIO1_PIN18_SEC_MASK_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Secure mask for pin P1_19"]
    #[inline(always)]
    pub fn pio1_pin19_sec_mask(&self) -> PIO1_PIN19_SEC_MASK_R {
        PIO1_PIN19_SEC_MASK_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Secure mask for pin P1_20"]
    #[inline(always)]
    pub fn pio1_pin20_sec_mask(&self) -> PIO1_PIN20_SEC_MASK_R {
        PIO1_PIN20_SEC_MASK_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Secure mask for pin P1_21"]
    #[inline(always)]
    pub fn pio1_pin21_sec_mask(&self) -> PIO1_PIN21_SEC_MASK_R {
        PIO1_PIN21_SEC_MASK_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Secure mask for pin P1_22"]
    #[inline(always)]
    pub fn pio1_pin22_sec_mask(&self) -> PIO1_PIN22_SEC_MASK_R {
        PIO1_PIN22_SEC_MASK_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Secure mask for pin P1_23"]
    #[inline(always)]
    pub fn pio1_pin23_sec_mask(&self) -> PIO1_PIN23_SEC_MASK_R {
        PIO1_PIN23_SEC_MASK_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Secure mask for pin P1_24"]
    #[inline(always)]
    pub fn pio1_pin24_sec_mask(&self) -> PIO1_PIN24_SEC_MASK_R {
        PIO1_PIN24_SEC_MASK_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Secure mask for pin P1_25"]
    #[inline(always)]
    pub fn pio1_pin25_sec_mask(&self) -> PIO1_PIN25_SEC_MASK_R {
        PIO1_PIN25_SEC_MASK_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Secure mask for pin P1_26"]
    #[inline(always)]
    pub fn pio1_pin26_sec_mask(&self) -> PIO1_PIN26_SEC_MASK_R {
        PIO1_PIN26_SEC_MASK_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Secure mask for pin P1_27"]
    #[inline(always)]
    pub fn pio1_pin27_sec_mask(&self) -> PIO1_PIN27_SEC_MASK_R {
        PIO1_PIN27_SEC_MASK_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Secure mask for pin P1_28"]
    #[inline(always)]
    pub fn pio1_pin28_sec_mask(&self) -> PIO1_PIN28_SEC_MASK_R {
        PIO1_PIN28_SEC_MASK_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Secure mask for pin P1_29"]
    #[inline(always)]
    pub fn pio1_pin29_sec_mask(&self) -> PIO1_PIN29_SEC_MASK_R {
        PIO1_PIN29_SEC_MASK_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Secure mask for pin P1_30"]
    #[inline(always)]
    pub fn pio1_pin30_sec_mask(&self) -> PIO1_PIN30_SEC_MASK_R {
        PIO1_PIN30_SEC_MASK_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Secure mask for pin P1_31"]
    #[inline(always)]
    pub fn pio1_pin31_sec_mask(&self) -> PIO1_PIN31_SEC_MASK_R {
        PIO1_PIN31_SEC_MASK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Secure mask for pin P1_0"]
    #[inline(always)]
    pub fn pio1_pin0_sec_mask(&mut self) -> PIO1_PIN0_SEC_MASK_W<0> {
        PIO1_PIN0_SEC_MASK_W::new(self)
    }
    #[doc = "Bit 1 - Secure mask for pin P1_1"]
    #[inline(always)]
    pub fn pio1_pin1_sec_mask(&mut self) -> PIO1_PIN1_SEC_MASK_W<1> {
        PIO1_PIN1_SEC_MASK_W::new(self)
    }
    #[doc = "Bit 2 - Secure mask for pin P1_2"]
    #[inline(always)]
    pub fn pio1_pin2_sec_mask(&mut self) -> PIO1_PIN2_SEC_MASK_W<2> {
        PIO1_PIN2_SEC_MASK_W::new(self)
    }
    #[doc = "Bit 3 - Secure mask for pin P1_3"]
    #[inline(always)]
    pub fn pio1_pin3_sec_mask(&mut self) -> PIO1_PIN3_SEC_MASK_W<3> {
        PIO1_PIN3_SEC_MASK_W::new(self)
    }
    #[doc = "Bit 4 - Secure mask for pin P1_4"]
    #[inline(always)]
    pub fn pio1_pin4_sec_mask(&mut self) -> PIO1_PIN4_SEC_MASK_W<4> {
        PIO1_PIN4_SEC_MASK_W::new(self)
    }
    #[doc = "Bit 5 - Secure mask for pin P1_5"]
    #[inline(always)]
    pub fn pio1_pin5_sec_mask(&mut self) -> PIO1_PIN5_SEC_MASK_W<5> {
        PIO1_PIN5_SEC_MASK_W::new(self)
    }
    #[doc = "Bit 6 - Secure mask for pin P1_6"]
    #[inline(always)]
    pub fn pio1_pin6_sec_mask(&mut self) -> PIO1_PIN6_SEC_MASK_W<6> {
        PIO1_PIN6_SEC_MASK_W::new(self)
    }
    #[doc = "Bit 7 - Secure mask for pin P1_7"]
    #[inline(always)]
    pub fn pio1_pin7_sec_mask(&mut self) -> PIO1_PIN7_SEC_MASK_W<7> {
        PIO1_PIN7_SEC_MASK_W::new(self)
    }
    #[doc = "Bit 8 - Secure mask for pin P1_8"]
    #[inline(always)]
    pub fn pio1_pin8_sec_mask(&mut self) -> PIO1_PIN8_SEC_MASK_W<8> {
        PIO1_PIN8_SEC_MASK_W::new(self)
    }
    #[doc = "Bit 9 - Secure mask for pin P1_9"]
    #[inline(always)]
    pub fn pio1_pin9_sec_mask(&mut self) -> PIO1_PIN9_SEC_MASK_W<9> {
        PIO1_PIN9_SEC_MASK_W::new(self)
    }
    #[doc = "Bit 10 - Secure mask for pin P1_10"]
    #[inline(always)]
    pub fn pio1_pin10_sec_mask(&mut self) -> PIO1_PIN10_SEC_MASK_W<10> {
        PIO1_PIN10_SEC_MASK_W::new(self)
    }
    #[doc = "Bit 11 - Secure mask for pin P1_11"]
    #[inline(always)]
    pub fn pio1_pin11_sec_mask(&mut self) -> PIO1_PIN11_SEC_MASK_W<11> {
        PIO1_PIN11_SEC_MASK_W::new(self)
    }
    #[doc = "Bit 12 - Secure mask for pin P1_12"]
    #[inline(always)]
    pub fn pio1_pin12_sec_mask(&mut self) -> PIO1_PIN12_SEC_MASK_W<12> {
        PIO1_PIN12_SEC_MASK_W::new(self)
    }
    #[doc = "Bit 13 - Secure mask for pin P1_13"]
    #[inline(always)]
    pub fn pio1_pin13_sec_mask(&mut self) -> PIO1_PIN13_SEC_MASK_W<13> {
        PIO1_PIN13_SEC_MASK_W::new(self)
    }
    #[doc = "Bit 14 - Secure mask for pin P1_14"]
    #[inline(always)]
    pub fn pio1_pin14_sec_mask(&mut self) -> PIO1_PIN14_SEC_MASK_W<14> {
        PIO1_PIN14_SEC_MASK_W::new(self)
    }
    #[doc = "Bit 15 - Secure mask for pin P1_15"]
    #[inline(always)]
    pub fn pio1_pin15_sec_mask(&mut self) -> PIO1_PIN15_SEC_MASK_W<15> {
        PIO1_PIN15_SEC_MASK_W::new(self)
    }
    #[doc = "Bit 16 - Secure mask for pin P1_16"]
    #[inline(always)]
    pub fn pio1_pin16_sec_mask(&mut self) -> PIO1_PIN16_SEC_MASK_W<16> {
        PIO1_PIN16_SEC_MASK_W::new(self)
    }
    #[doc = "Bit 17 - Secure mask for pin P1_17"]
    #[inline(always)]
    pub fn pio1_pin17_sec_mask(&mut self) -> PIO1_PIN17_SEC_MASK_W<17> {
        PIO1_PIN17_SEC_MASK_W::new(self)
    }
    #[doc = "Bit 18 - Secure mask for pin P1_18"]
    #[inline(always)]
    pub fn pio1_pin18_sec_mask(&mut self) -> PIO1_PIN18_SEC_MASK_W<18> {
        PIO1_PIN18_SEC_MASK_W::new(self)
    }
    #[doc = "Bit 19 - Secure mask for pin P1_19"]
    #[inline(always)]
    pub fn pio1_pin19_sec_mask(&mut self) -> PIO1_PIN19_SEC_MASK_W<19> {
        PIO1_PIN19_SEC_MASK_W::new(self)
    }
    #[doc = "Bit 20 - Secure mask for pin P1_20"]
    #[inline(always)]
    pub fn pio1_pin20_sec_mask(&mut self) -> PIO1_PIN20_SEC_MASK_W<20> {
        PIO1_PIN20_SEC_MASK_W::new(self)
    }
    #[doc = "Bit 21 - Secure mask for pin P1_21"]
    #[inline(always)]
    pub fn pio1_pin21_sec_mask(&mut self) -> PIO1_PIN21_SEC_MASK_W<21> {
        PIO1_PIN21_SEC_MASK_W::new(self)
    }
    #[doc = "Bit 22 - Secure mask for pin P1_22"]
    #[inline(always)]
    pub fn pio1_pin22_sec_mask(&mut self) -> PIO1_PIN22_SEC_MASK_W<22> {
        PIO1_PIN22_SEC_MASK_W::new(self)
    }
    #[doc = "Bit 23 - Secure mask for pin P1_23"]
    #[inline(always)]
    pub fn pio1_pin23_sec_mask(&mut self) -> PIO1_PIN23_SEC_MASK_W<23> {
        PIO1_PIN23_SEC_MASK_W::new(self)
    }
    #[doc = "Bit 24 - Secure mask for pin P1_24"]
    #[inline(always)]
    pub fn pio1_pin24_sec_mask(&mut self) -> PIO1_PIN24_SEC_MASK_W<24> {
        PIO1_PIN24_SEC_MASK_W::new(self)
    }
    #[doc = "Bit 25 - Secure mask for pin P1_25"]
    #[inline(always)]
    pub fn pio1_pin25_sec_mask(&mut self) -> PIO1_PIN25_SEC_MASK_W<25> {
        PIO1_PIN25_SEC_MASK_W::new(self)
    }
    #[doc = "Bit 26 - Secure mask for pin P1_26"]
    #[inline(always)]
    pub fn pio1_pin26_sec_mask(&mut self) -> PIO1_PIN26_SEC_MASK_W<26> {
        PIO1_PIN26_SEC_MASK_W::new(self)
    }
    #[doc = "Bit 27 - Secure mask for pin P1_27"]
    #[inline(always)]
    pub fn pio1_pin27_sec_mask(&mut self) -> PIO1_PIN27_SEC_MASK_W<27> {
        PIO1_PIN27_SEC_MASK_W::new(self)
    }
    #[doc = "Bit 28 - Secure mask for pin P1_28"]
    #[inline(always)]
    pub fn pio1_pin28_sec_mask(&mut self) -> PIO1_PIN28_SEC_MASK_W<28> {
        PIO1_PIN28_SEC_MASK_W::new(self)
    }
    #[doc = "Bit 29 - Secure mask for pin P1_29"]
    #[inline(always)]
    pub fn pio1_pin29_sec_mask(&mut self) -> PIO1_PIN29_SEC_MASK_W<29> {
        PIO1_PIN29_SEC_MASK_W::new(self)
    }
    #[doc = "Bit 30 - Secure mask for pin P1_30"]
    #[inline(always)]
    pub fn pio1_pin30_sec_mask(&mut self) -> PIO1_PIN30_SEC_MASK_W<30> {
        PIO1_PIN30_SEC_MASK_W::new(self)
    }
    #[doc = "Bit 31 - Secure mask for pin P1_31"]
    #[inline(always)]
    pub fn pio1_pin31_sec_mask(&mut self) -> PIO1_PIN31_SEC_MASK_W<31> {
        PIO1_PIN31_SEC_MASK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Secure GPIO mask for port 1 pins.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sec_gpio_mask1](index.html) module"]
pub struct SEC_GPIO_MASK1_SPEC;
impl crate::RegisterSpec for SEC_GPIO_MASK1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sec_gpio_mask1::R](R) reader structure"]
impl crate::Readable for SEC_GPIO_MASK1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sec_gpio_mask1::W](W) writer structure"]
impl crate::Writable for SEC_GPIO_MASK1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SEC_GPIO_MASK1 to value 0xffff_ffff"]
impl crate::Resettable for SEC_GPIO_MASK1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
