#[doc = "Register `PRESETCTRL0` reader"]
pub struct R(crate::R<PRESETCTRL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRESETCTRL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRESETCTRL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRESETCTRL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PRESETCTRL0` writer"]
pub struct W(crate::W<PRESETCTRL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRESETCTRL0_SPEC>;
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
impl From<crate::W<PRESETCTRL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRESETCTRL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ROM_RST` reader - ROM reset control."]
pub type ROM_RST_R = crate::BitReader<ROM_RST_A>;
#[doc = "ROM reset control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ROM_RST_A {
    #[doc = "0: Bloc is not reset."]
    RELEASED = 0,
    #[doc = "1: Bloc is reset."]
    ASSERTED = 1,
}
impl From<ROM_RST_A> for bool {
    #[inline(always)]
    fn from(variant: ROM_RST_A) -> Self {
        variant as u8 != 0
    }
}
impl ROM_RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ROM_RST_A {
        match self.bits {
            false => ROM_RST_A::RELEASED,
            true => ROM_RST_A::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        *self == ROM_RST_A::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        *self == ROM_RST_A::ASSERTED
    }
}
#[doc = "Field `ROM_RST` writer - ROM reset control."]
pub type ROM_RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRESETCTRL0_SPEC, ROM_RST_A, O>;
impl<'a, const O: u8> ROM_RST_W<'a, O> {
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut W {
        self.variant(ROM_RST_A::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut W {
        self.variant(ROM_RST_A::ASSERTED)
    }
}
#[doc = "Field `SRAM_CTRL1_RST` reader - SRAM Controller 1 reset control."]
pub type SRAM_CTRL1_RST_R = crate::BitReader<SRAM_CTRL1_RST_A>;
#[doc = "SRAM Controller 1 reset control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM_CTRL1_RST_A {
    #[doc = "0: Bloc is not reset."]
    RELEASED = 0,
    #[doc = "1: Bloc is reset."]
    ASSERTED = 1,
}
impl From<SRAM_CTRL1_RST_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM_CTRL1_RST_A) -> Self {
        variant as u8 != 0
    }
}
impl SRAM_CTRL1_RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAM_CTRL1_RST_A {
        match self.bits {
            false => SRAM_CTRL1_RST_A::RELEASED,
            true => SRAM_CTRL1_RST_A::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        *self == SRAM_CTRL1_RST_A::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        *self == SRAM_CTRL1_RST_A::ASSERTED
    }
}
#[doc = "Field `SRAM_CTRL1_RST` writer - SRAM Controller 1 reset control."]
pub type SRAM_CTRL1_RST_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PRESETCTRL0_SPEC, SRAM_CTRL1_RST_A, O>;
impl<'a, const O: u8> SRAM_CTRL1_RST_W<'a, O> {
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut W {
        self.variant(SRAM_CTRL1_RST_A::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut W {
        self.variant(SRAM_CTRL1_RST_A::ASSERTED)
    }
}
#[doc = "Field `SRAM_CTRL2_RST` reader - SRAM Controller 2 reset control."]
pub type SRAM_CTRL2_RST_R = crate::BitReader<SRAM_CTRL2_RST_A>;
#[doc = "SRAM Controller 2 reset control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM_CTRL2_RST_A {
    #[doc = "0: Bloc is not reset."]
    RELEASED = 0,
    #[doc = "1: Bloc is reset."]
    ASSERTED = 1,
}
impl From<SRAM_CTRL2_RST_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM_CTRL2_RST_A) -> Self {
        variant as u8 != 0
    }
}
impl SRAM_CTRL2_RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAM_CTRL2_RST_A {
        match self.bits {
            false => SRAM_CTRL2_RST_A::RELEASED,
            true => SRAM_CTRL2_RST_A::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        *self == SRAM_CTRL2_RST_A::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        *self == SRAM_CTRL2_RST_A::ASSERTED
    }
}
#[doc = "Field `SRAM_CTRL2_RST` writer - SRAM Controller 2 reset control."]
pub type SRAM_CTRL2_RST_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PRESETCTRL0_SPEC, SRAM_CTRL2_RST_A, O>;
impl<'a, const O: u8> SRAM_CTRL2_RST_W<'a, O> {
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut W {
        self.variant(SRAM_CTRL2_RST_A::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut W {
        self.variant(SRAM_CTRL2_RST_A::ASSERTED)
    }
}
#[doc = "Field `FLASH_RST` reader - Flash controller reset control."]
pub type FLASH_RST_R = crate::BitReader<FLASH_RST_A>;
#[doc = "Flash controller reset control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLASH_RST_A {
    #[doc = "0: Bloc is not reset."]
    RELEASED = 0,
    #[doc = "1: Bloc is reset."]
    ASSERTED = 1,
}
impl From<FLASH_RST_A> for bool {
    #[inline(always)]
    fn from(variant: FLASH_RST_A) -> Self {
        variant as u8 != 0
    }
}
impl FLASH_RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLASH_RST_A {
        match self.bits {
            false => FLASH_RST_A::RELEASED,
            true => FLASH_RST_A::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        *self == FLASH_RST_A::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        *self == FLASH_RST_A::ASSERTED
    }
}
#[doc = "Field `FLASH_RST` writer - Flash controller reset control."]
pub type FLASH_RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRESETCTRL0_SPEC, FLASH_RST_A, O>;
impl<'a, const O: u8> FLASH_RST_W<'a, O> {
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut W {
        self.variant(FLASH_RST_A::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut W {
        self.variant(FLASH_RST_A::ASSERTED)
    }
}
#[doc = "Field `FMC_RST` reader - FMC controller reset control."]
pub type FMC_RST_R = crate::BitReader<FMC_RST_A>;
#[doc = "FMC controller reset control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FMC_RST_A {
    #[doc = "0: Bloc is not reset."]
    RELEASED = 0,
    #[doc = "1: Bloc is reset."]
    ASSERTED = 1,
}
impl From<FMC_RST_A> for bool {
    #[inline(always)]
    fn from(variant: FMC_RST_A) -> Self {
        variant as u8 != 0
    }
}
impl FMC_RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FMC_RST_A {
        match self.bits {
            false => FMC_RST_A::RELEASED,
            true => FMC_RST_A::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        *self == FMC_RST_A::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        *self == FMC_RST_A::ASSERTED
    }
}
#[doc = "Field `FMC_RST` writer - FMC controller reset control."]
pub type FMC_RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRESETCTRL0_SPEC, FMC_RST_A, O>;
impl<'a, const O: u8> FMC_RST_W<'a, O> {
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut W {
        self.variant(FMC_RST_A::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut W {
        self.variant(FMC_RST_A::ASSERTED)
    }
}
#[doc = "Field `MUX_RST` reader - Input Mux reset control."]
pub type MUX_RST_R = crate::BitReader<MUX_RST_A>;
#[doc = "Input Mux reset control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MUX_RST_A {
    #[doc = "0: Bloc is not reset."]
    RELEASED = 0,
    #[doc = "1: Bloc is reset."]
    ASSERTED = 1,
}
impl From<MUX_RST_A> for bool {
    #[inline(always)]
    fn from(variant: MUX_RST_A) -> Self {
        variant as u8 != 0
    }
}
impl MUX_RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MUX_RST_A {
        match self.bits {
            false => MUX_RST_A::RELEASED,
            true => MUX_RST_A::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        *self == MUX_RST_A::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        *self == MUX_RST_A::ASSERTED
    }
}
#[doc = "Field `MUX_RST` writer - Input Mux reset control."]
pub type MUX_RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRESETCTRL0_SPEC, MUX_RST_A, O>;
impl<'a, const O: u8> MUX_RST_W<'a, O> {
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut W {
        self.variant(MUX_RST_A::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut W {
        self.variant(MUX_RST_A::ASSERTED)
    }
}
#[doc = "Field `IOCON_RST` reader - I/O controller reset control."]
pub type IOCON_RST_R = crate::BitReader<IOCON_RST_A>;
#[doc = "I/O controller reset control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IOCON_RST_A {
    #[doc = "0: Bloc is not reset."]
    RELEASED = 0,
    #[doc = "1: Bloc is reset."]
    ASSERTED = 1,
}
impl From<IOCON_RST_A> for bool {
    #[inline(always)]
    fn from(variant: IOCON_RST_A) -> Self {
        variant as u8 != 0
    }
}
impl IOCON_RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IOCON_RST_A {
        match self.bits {
            false => IOCON_RST_A::RELEASED,
            true => IOCON_RST_A::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        *self == IOCON_RST_A::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        *self == IOCON_RST_A::ASSERTED
    }
}
#[doc = "Field `IOCON_RST` writer - I/O controller reset control."]
pub type IOCON_RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRESETCTRL0_SPEC, IOCON_RST_A, O>;
impl<'a, const O: u8> IOCON_RST_W<'a, O> {
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut W {
        self.variant(IOCON_RST_A::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut W {
        self.variant(IOCON_RST_A::ASSERTED)
    }
}
#[doc = "Field `GPIO0_RST` reader - GPIO0 reset control."]
pub type GPIO0_RST_R = crate::BitReader<GPIO0_RST_A>;
#[doc = "GPIO0 reset control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO0_RST_A {
    #[doc = "0: Bloc is not reset."]
    RELEASED = 0,
    #[doc = "1: Bloc is reset."]
    ASSERTED = 1,
}
impl From<GPIO0_RST_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO0_RST_A) -> Self {
        variant as u8 != 0
    }
}
impl GPIO0_RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO0_RST_A {
        match self.bits {
            false => GPIO0_RST_A::RELEASED,
            true => GPIO0_RST_A::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        *self == GPIO0_RST_A::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        *self == GPIO0_RST_A::ASSERTED
    }
}
#[doc = "Field `GPIO0_RST` writer - GPIO0 reset control."]
pub type GPIO0_RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRESETCTRL0_SPEC, GPIO0_RST_A, O>;
impl<'a, const O: u8> GPIO0_RST_W<'a, O> {
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut W {
        self.variant(GPIO0_RST_A::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut W {
        self.variant(GPIO0_RST_A::ASSERTED)
    }
}
#[doc = "Field `GPIO1_RST` reader - GPIO1 reset control."]
pub type GPIO1_RST_R = crate::BitReader<GPIO1_RST_A>;
#[doc = "GPIO1 reset control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO1_RST_A {
    #[doc = "0: Bloc is not reset."]
    RELEASED = 0,
    #[doc = "1: Bloc is reset."]
    ASSERTED = 1,
}
impl From<GPIO1_RST_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO1_RST_A) -> Self {
        variant as u8 != 0
    }
}
impl GPIO1_RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO1_RST_A {
        match self.bits {
            false => GPIO1_RST_A::RELEASED,
            true => GPIO1_RST_A::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        *self == GPIO1_RST_A::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        *self == GPIO1_RST_A::ASSERTED
    }
}
#[doc = "Field `GPIO1_RST` writer - GPIO1 reset control."]
pub type GPIO1_RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRESETCTRL0_SPEC, GPIO1_RST_A, O>;
impl<'a, const O: u8> GPIO1_RST_W<'a, O> {
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut W {
        self.variant(GPIO1_RST_A::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut W {
        self.variant(GPIO1_RST_A::ASSERTED)
    }
}
#[doc = "Field `PINT_RST` reader - Pin interrupt (PINT) reset control."]
pub type PINT_RST_R = crate::BitReader<PINT_RST_A>;
#[doc = "Pin interrupt (PINT) reset control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PINT_RST_A {
    #[doc = "0: Bloc is not reset."]
    RELEASED = 0,
    #[doc = "1: Bloc is reset."]
    ASSERTED = 1,
}
impl From<PINT_RST_A> for bool {
    #[inline(always)]
    fn from(variant: PINT_RST_A) -> Self {
        variant as u8 != 0
    }
}
impl PINT_RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PINT_RST_A {
        match self.bits {
            false => PINT_RST_A::RELEASED,
            true => PINT_RST_A::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        *self == PINT_RST_A::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        *self == PINT_RST_A::ASSERTED
    }
}
#[doc = "Field `PINT_RST` writer - Pin interrupt (PINT) reset control."]
pub type PINT_RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRESETCTRL0_SPEC, PINT_RST_A, O>;
impl<'a, const O: u8> PINT_RST_W<'a, O> {
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut W {
        self.variant(PINT_RST_A::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut W {
        self.variant(PINT_RST_A::ASSERTED)
    }
}
#[doc = "Field `GINT_RST` reader - Group interrupt (GINT) reset control."]
pub type GINT_RST_R = crate::BitReader<GINT_RST_A>;
#[doc = "Group interrupt (GINT) reset control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GINT_RST_A {
    #[doc = "0: Bloc is not reset."]
    RELEASED = 0,
    #[doc = "1: Bloc is reset."]
    ASSERTED = 1,
}
impl From<GINT_RST_A> for bool {
    #[inline(always)]
    fn from(variant: GINT_RST_A) -> Self {
        variant as u8 != 0
    }
}
impl GINT_RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GINT_RST_A {
        match self.bits {
            false => GINT_RST_A::RELEASED,
            true => GINT_RST_A::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        *self == GINT_RST_A::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        *self == GINT_RST_A::ASSERTED
    }
}
#[doc = "Field `GINT_RST` writer - Group interrupt (GINT) reset control."]
pub type GINT_RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRESETCTRL0_SPEC, GINT_RST_A, O>;
impl<'a, const O: u8> GINT_RST_W<'a, O> {
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut W {
        self.variant(GINT_RST_A::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut W {
        self.variant(GINT_RST_A::ASSERTED)
    }
}
#[doc = "Field `DMA0_RST` reader - DMA0 reset control."]
pub type DMA0_RST_R = crate::BitReader<DMA0_RST_A>;
#[doc = "DMA0 reset control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMA0_RST_A {
    #[doc = "0: Bloc is not reset."]
    RELEASED = 0,
    #[doc = "1: Bloc is reset."]
    ASSERTED = 1,
}
impl From<DMA0_RST_A> for bool {
    #[inline(always)]
    fn from(variant: DMA0_RST_A) -> Self {
        variant as u8 != 0
    }
}
impl DMA0_RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMA0_RST_A {
        match self.bits {
            false => DMA0_RST_A::RELEASED,
            true => DMA0_RST_A::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        *self == DMA0_RST_A::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        *self == DMA0_RST_A::ASSERTED
    }
}
#[doc = "Field `DMA0_RST` writer - DMA0 reset control."]
pub type DMA0_RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRESETCTRL0_SPEC, DMA0_RST_A, O>;
impl<'a, const O: u8> DMA0_RST_W<'a, O> {
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut W {
        self.variant(DMA0_RST_A::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut W {
        self.variant(DMA0_RST_A::ASSERTED)
    }
}
#[doc = "Field `CRCGEN_RST` reader - CRCGEN reset control."]
pub type CRCGEN_RST_R = crate::BitReader<CRCGEN_RST_A>;
#[doc = "CRCGEN reset control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CRCGEN_RST_A {
    #[doc = "0: Bloc is not reset."]
    RELEASED = 0,
    #[doc = "1: Bloc is reset."]
    ASSERTED = 1,
}
impl From<CRCGEN_RST_A> for bool {
    #[inline(always)]
    fn from(variant: CRCGEN_RST_A) -> Self {
        variant as u8 != 0
    }
}
impl CRCGEN_RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CRCGEN_RST_A {
        match self.bits {
            false => CRCGEN_RST_A::RELEASED,
            true => CRCGEN_RST_A::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        *self == CRCGEN_RST_A::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        *self == CRCGEN_RST_A::ASSERTED
    }
}
#[doc = "Field `CRCGEN_RST` writer - CRCGEN reset control."]
pub type CRCGEN_RST_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PRESETCTRL0_SPEC, CRCGEN_RST_A, O>;
impl<'a, const O: u8> CRCGEN_RST_W<'a, O> {
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut W {
        self.variant(CRCGEN_RST_A::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut W {
        self.variant(CRCGEN_RST_A::ASSERTED)
    }
}
#[doc = "Field `WWDT_RST` reader - Watchdog Timer reset control."]
pub type WWDT_RST_R = crate::BitReader<WWDT_RST_A>;
#[doc = "Watchdog Timer reset control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WWDT_RST_A {
    #[doc = "0: Bloc is not reset."]
    RELEASED = 0,
    #[doc = "1: Bloc is reset."]
    ASSERTED = 1,
}
impl From<WWDT_RST_A> for bool {
    #[inline(always)]
    fn from(variant: WWDT_RST_A) -> Self {
        variant as u8 != 0
    }
}
impl WWDT_RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WWDT_RST_A {
        match self.bits {
            false => WWDT_RST_A::RELEASED,
            true => WWDT_RST_A::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        *self == WWDT_RST_A::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        *self == WWDT_RST_A::ASSERTED
    }
}
#[doc = "Field `WWDT_RST` writer - Watchdog Timer reset control."]
pub type WWDT_RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRESETCTRL0_SPEC, WWDT_RST_A, O>;
impl<'a, const O: u8> WWDT_RST_W<'a, O> {
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut W {
        self.variant(WWDT_RST_A::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut W {
        self.variant(WWDT_RST_A::ASSERTED)
    }
}
#[doc = "Field `RTC_RST` reader - Real Time Clock (RTC) reset control."]
pub type RTC_RST_R = crate::BitReader<RTC_RST_A>;
#[doc = "Real Time Clock (RTC) reset control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTC_RST_A {
    #[doc = "0: Bloc is not reset."]
    RELEASED = 0,
    #[doc = "1: Bloc is reset."]
    ASSERTED = 1,
}
impl From<RTC_RST_A> for bool {
    #[inline(always)]
    fn from(variant: RTC_RST_A) -> Self {
        variant as u8 != 0
    }
}
impl RTC_RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTC_RST_A {
        match self.bits {
            false => RTC_RST_A::RELEASED,
            true => RTC_RST_A::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        *self == RTC_RST_A::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        *self == RTC_RST_A::ASSERTED
    }
}
#[doc = "Field `RTC_RST` writer - Real Time Clock (RTC) reset control."]
pub type RTC_RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRESETCTRL0_SPEC, RTC_RST_A, O>;
impl<'a, const O: u8> RTC_RST_W<'a, O> {
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut W {
        self.variant(RTC_RST_A::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut W {
        self.variant(RTC_RST_A::ASSERTED)
    }
}
#[doc = "Field `MAILBOX_RST` reader - Inter CPU communication Mailbox reset control."]
pub type MAILBOX_RST_R = crate::BitReader<MAILBOX_RST_A>;
#[doc = "Inter CPU communication Mailbox reset control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MAILBOX_RST_A {
    #[doc = "0: Bloc is not reset."]
    RELEASED = 0,
    #[doc = "1: Bloc is reset."]
    ASSERTED = 1,
}
impl From<MAILBOX_RST_A> for bool {
    #[inline(always)]
    fn from(variant: MAILBOX_RST_A) -> Self {
        variant as u8 != 0
    }
}
impl MAILBOX_RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MAILBOX_RST_A {
        match self.bits {
            false => MAILBOX_RST_A::RELEASED,
            true => MAILBOX_RST_A::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        *self == MAILBOX_RST_A::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        *self == MAILBOX_RST_A::ASSERTED
    }
}
#[doc = "Field `MAILBOX_RST` writer - Inter CPU communication Mailbox reset control."]
pub type MAILBOX_RST_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PRESETCTRL0_SPEC, MAILBOX_RST_A, O>;
impl<'a, const O: u8> MAILBOX_RST_W<'a, O> {
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut W {
        self.variant(MAILBOX_RST_A::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut W {
        self.variant(MAILBOX_RST_A::ASSERTED)
    }
}
#[doc = "Field `ADC_RST` reader - ADC reset control."]
pub type ADC_RST_R = crate::BitReader<ADC_RST_A>;
#[doc = "ADC reset control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADC_RST_A {
    #[doc = "0: Bloc is not reset."]
    RELEASED = 0,
    #[doc = "1: Bloc is reset."]
    ASSERTED = 1,
}
impl From<ADC_RST_A> for bool {
    #[inline(always)]
    fn from(variant: ADC_RST_A) -> Self {
        variant as u8 != 0
    }
}
impl ADC_RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC_RST_A {
        match self.bits {
            false => ADC_RST_A::RELEASED,
            true => ADC_RST_A::ASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASED`"]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        *self == ADC_RST_A::RELEASED
    }
    #[doc = "Checks if the value of the field is `ASSERTED`"]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        *self == ADC_RST_A::ASSERTED
    }
}
#[doc = "Field `ADC_RST` writer - ADC reset control."]
pub type ADC_RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRESETCTRL0_SPEC, ADC_RST_A, O>;
impl<'a, const O: u8> ADC_RST_W<'a, O> {
    #[doc = "Bloc is not reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut W {
        self.variant(ADC_RST_A::RELEASED)
    }
    #[doc = "Bloc is reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut W {
        self.variant(ADC_RST_A::ASSERTED)
    }
}
impl R {
    #[doc = "Bit 1 - ROM reset control."]
    #[inline(always)]
    pub fn rom_rst(&self) -> ROM_RST_R {
        ROM_RST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - SRAM Controller 1 reset control."]
    #[inline(always)]
    pub fn sram_ctrl1_rst(&self) -> SRAM_CTRL1_RST_R {
        SRAM_CTRL1_RST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SRAM Controller 2 reset control."]
    #[inline(always)]
    pub fn sram_ctrl2_rst(&self) -> SRAM_CTRL2_RST_R {
        SRAM_CTRL2_RST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - Flash controller reset control."]
    #[inline(always)]
    pub fn flash_rst(&self) -> FLASH_RST_R {
        FLASH_RST_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - FMC controller reset control."]
    #[inline(always)]
    pub fn fmc_rst(&self) -> FMC_RST_R {
        FMC_RST_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 11 - Input Mux reset control."]
    #[inline(always)]
    pub fn mux_rst(&self) -> MUX_RST_R {
        MUX_RST_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 13 - I/O controller reset control."]
    #[inline(always)]
    pub fn iocon_rst(&self) -> IOCON_RST_R {
        IOCON_RST_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - GPIO0 reset control."]
    #[inline(always)]
    pub fn gpio0_rst(&self) -> GPIO0_RST_R {
        GPIO0_RST_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - GPIO1 reset control."]
    #[inline(always)]
    pub fn gpio1_rst(&self) -> GPIO1_RST_R {
        GPIO1_RST_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 18 - Pin interrupt (PINT) reset control."]
    #[inline(always)]
    pub fn pint_rst(&self) -> PINT_RST_R {
        PINT_RST_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Group interrupt (GINT) reset control."]
    #[inline(always)]
    pub fn gint_rst(&self) -> GINT_RST_R {
        GINT_RST_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - DMA0 reset control."]
    #[inline(always)]
    pub fn dma0_rst(&self) -> DMA0_RST_R {
        DMA0_RST_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - CRCGEN reset control."]
    #[inline(always)]
    pub fn crcgen_rst(&self) -> CRCGEN_RST_R {
        CRCGEN_RST_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Watchdog Timer reset control."]
    #[inline(always)]
    pub fn wwdt_rst(&self) -> WWDT_RST_R {
        WWDT_RST_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Real Time Clock (RTC) reset control."]
    #[inline(always)]
    pub fn rtc_rst(&self) -> RTC_RST_R {
        RTC_RST_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 26 - Inter CPU communication Mailbox reset control."]
    #[inline(always)]
    pub fn mailbox_rst(&self) -> MAILBOX_RST_R {
        MAILBOX_RST_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - ADC reset control."]
    #[inline(always)]
    pub fn adc_rst(&self) -> ADC_RST_R {
        ADC_RST_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - ROM reset control."]
    #[inline(always)]
    pub fn rom_rst(&mut self) -> ROM_RST_W<1> {
        ROM_RST_W::new(self)
    }
    #[doc = "Bit 3 - SRAM Controller 1 reset control."]
    #[inline(always)]
    pub fn sram_ctrl1_rst(&mut self) -> SRAM_CTRL1_RST_W<3> {
        SRAM_CTRL1_RST_W::new(self)
    }
    #[doc = "Bit 4 - SRAM Controller 2 reset control."]
    #[inline(always)]
    pub fn sram_ctrl2_rst(&mut self) -> SRAM_CTRL2_RST_W<4> {
        SRAM_CTRL2_RST_W::new(self)
    }
    #[doc = "Bit 7 - Flash controller reset control."]
    #[inline(always)]
    pub fn flash_rst(&mut self) -> FLASH_RST_W<7> {
        FLASH_RST_W::new(self)
    }
    #[doc = "Bit 8 - FMC controller reset control."]
    #[inline(always)]
    pub fn fmc_rst(&mut self) -> FMC_RST_W<8> {
        FMC_RST_W::new(self)
    }
    #[doc = "Bit 11 - Input Mux reset control."]
    #[inline(always)]
    pub fn mux_rst(&mut self) -> MUX_RST_W<11> {
        MUX_RST_W::new(self)
    }
    #[doc = "Bit 13 - I/O controller reset control."]
    #[inline(always)]
    pub fn iocon_rst(&mut self) -> IOCON_RST_W<13> {
        IOCON_RST_W::new(self)
    }
    #[doc = "Bit 14 - GPIO0 reset control."]
    #[inline(always)]
    pub fn gpio0_rst(&mut self) -> GPIO0_RST_W<14> {
        GPIO0_RST_W::new(self)
    }
    #[doc = "Bit 15 - GPIO1 reset control."]
    #[inline(always)]
    pub fn gpio1_rst(&mut self) -> GPIO1_RST_W<15> {
        GPIO1_RST_W::new(self)
    }
    #[doc = "Bit 18 - Pin interrupt (PINT) reset control."]
    #[inline(always)]
    pub fn pint_rst(&mut self) -> PINT_RST_W<18> {
        PINT_RST_W::new(self)
    }
    #[doc = "Bit 19 - Group interrupt (GINT) reset control."]
    #[inline(always)]
    pub fn gint_rst(&mut self) -> GINT_RST_W<19> {
        GINT_RST_W::new(self)
    }
    #[doc = "Bit 20 - DMA0 reset control."]
    #[inline(always)]
    pub fn dma0_rst(&mut self) -> DMA0_RST_W<20> {
        DMA0_RST_W::new(self)
    }
    #[doc = "Bit 21 - CRCGEN reset control."]
    #[inline(always)]
    pub fn crcgen_rst(&mut self) -> CRCGEN_RST_W<21> {
        CRCGEN_RST_W::new(self)
    }
    #[doc = "Bit 22 - Watchdog Timer reset control."]
    #[inline(always)]
    pub fn wwdt_rst(&mut self) -> WWDT_RST_W<22> {
        WWDT_RST_W::new(self)
    }
    #[doc = "Bit 23 - Real Time Clock (RTC) reset control."]
    #[inline(always)]
    pub fn rtc_rst(&mut self) -> RTC_RST_W<23> {
        RTC_RST_W::new(self)
    }
    #[doc = "Bit 26 - Inter CPU communication Mailbox reset control."]
    #[inline(always)]
    pub fn mailbox_rst(&mut self) -> MAILBOX_RST_W<26> {
        MAILBOX_RST_W::new(self)
    }
    #[doc = "Bit 27 - ADC reset control."]
    #[inline(always)]
    pub fn adc_rst(&mut self) -> ADC_RST_W<27> {
        ADC_RST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Peripheral reset control 0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [presetctrl0](index.html) module"]
pub struct PRESETCTRL0_SPEC;
impl crate::RegisterSpec for PRESETCTRL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [presetctrl0::R](R) reader structure"]
impl crate::Readable for PRESETCTRL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [presetctrl0::W](W) writer structure"]
impl crate::Writable for PRESETCTRL0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PRESETCTRL0 to value 0"]
impl crate::Resettable for PRESETCTRL0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
