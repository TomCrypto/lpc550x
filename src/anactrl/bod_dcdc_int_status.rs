#[doc = "Register `BOD_DCDC_INT_STATUS` reader"]
pub struct R(crate::R<BOD_DCDC_INT_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BOD_DCDC_INT_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BOD_DCDC_INT_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BOD_DCDC_INT_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `BODVBAT_STATUS` reader - BOD VBAT Interrupt status before Interrupt Enable."]
pub type BODVBAT_STATUS_R = crate::BitReader<BODVBAT_STATUS_A>;
#[doc = "BOD VBAT Interrupt status before Interrupt Enable.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BODVBAT_STATUS_A {
    #[doc = "0: No interrupt pending.."]
    NOT_PENDING = 0,
    #[doc = "1: Interrupt pending.."]
    PENDING = 1,
}
impl From<BODVBAT_STATUS_A> for bool {
    #[inline(always)]
    fn from(variant: BODVBAT_STATUS_A) -> Self {
        variant as u8 != 0
    }
}
impl BODVBAT_STATUS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BODVBAT_STATUS_A {
        match self.bits {
            false => BODVBAT_STATUS_A::NOT_PENDING,
            true => BODVBAT_STATUS_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_PENDING`"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == BODVBAT_STATUS_A::NOT_PENDING
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == BODVBAT_STATUS_A::PENDING
    }
}
#[doc = "Field `BODVBAT_INT_STATUS` reader - BOD VBAT Interrupt status after Interrupt Enable."]
pub type BODVBAT_INT_STATUS_R = crate::BitReader<BODVBAT_INT_STATUS_A>;
#[doc = "BOD VBAT Interrupt status after Interrupt Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BODVBAT_INT_STATUS_A {
    #[doc = "0: No interrupt pending.."]
    NOT_PENDING = 0,
    #[doc = "1: Interrupt pending.."]
    PENDING = 1,
}
impl From<BODVBAT_INT_STATUS_A> for bool {
    #[inline(always)]
    fn from(variant: BODVBAT_INT_STATUS_A) -> Self {
        variant as u8 != 0
    }
}
impl BODVBAT_INT_STATUS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BODVBAT_INT_STATUS_A {
        match self.bits {
            false => BODVBAT_INT_STATUS_A::NOT_PENDING,
            true => BODVBAT_INT_STATUS_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_PENDING`"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == BODVBAT_INT_STATUS_A::NOT_PENDING
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == BODVBAT_INT_STATUS_A::PENDING
    }
}
#[doc = "Field `BODVBAT_VAL` reader - Current value of BOD VBAT power status output."]
pub type BODVBAT_VAL_R = crate::BitReader<BODVBAT_VAL_A>;
#[doc = "Current value of BOD VBAT power status output.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BODVBAT_VAL_A {
    #[doc = "0: VBAT voltage level is below the threshold."]
    NOT_OK = 0,
    #[doc = "1: VBAT voltage level is above the threshold."]
    OK = 1,
}
impl From<BODVBAT_VAL_A> for bool {
    #[inline(always)]
    fn from(variant: BODVBAT_VAL_A) -> Self {
        variant as u8 != 0
    }
}
impl BODVBAT_VAL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BODVBAT_VAL_A {
        match self.bits {
            false => BODVBAT_VAL_A::NOT_OK,
            true => BODVBAT_VAL_A::OK,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_OK`"]
    #[inline(always)]
    pub fn is_not_ok(&self) -> bool {
        *self == BODVBAT_VAL_A::NOT_OK
    }
    #[doc = "Checks if the value of the field is `OK`"]
    #[inline(always)]
    pub fn is_ok(&self) -> bool {
        *self == BODVBAT_VAL_A::OK
    }
}
#[doc = "Field `BODCORE_STATUS` reader - BOD CORE Interrupt status before Interrupt Enable."]
pub type BODCORE_STATUS_R = crate::BitReader<BODCORE_STATUS_A>;
#[doc = "BOD CORE Interrupt status before Interrupt Enable.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BODCORE_STATUS_A {
    #[doc = "0: No interrupt pending.."]
    NOT_PENDING = 0,
    #[doc = "1: Interrupt pending.."]
    PENDING = 1,
}
impl From<BODCORE_STATUS_A> for bool {
    #[inline(always)]
    fn from(variant: BODCORE_STATUS_A) -> Self {
        variant as u8 != 0
    }
}
impl BODCORE_STATUS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BODCORE_STATUS_A {
        match self.bits {
            false => BODCORE_STATUS_A::NOT_PENDING,
            true => BODCORE_STATUS_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_PENDING`"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == BODCORE_STATUS_A::NOT_PENDING
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == BODCORE_STATUS_A::PENDING
    }
}
#[doc = "Field `BODCORE_INT_STATUS` reader - BOD CORE Interrupt status after Interrupt Enable."]
pub type BODCORE_INT_STATUS_R = crate::BitReader<BODCORE_INT_STATUS_A>;
#[doc = "BOD CORE Interrupt status after Interrupt Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BODCORE_INT_STATUS_A {
    #[doc = "0: No interrupt pending.."]
    NOT_PENDING = 0,
    #[doc = "1: Interrupt pending.."]
    PENDING = 1,
}
impl From<BODCORE_INT_STATUS_A> for bool {
    #[inline(always)]
    fn from(variant: BODCORE_INT_STATUS_A) -> Self {
        variant as u8 != 0
    }
}
impl BODCORE_INT_STATUS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BODCORE_INT_STATUS_A {
        match self.bits {
            false => BODCORE_INT_STATUS_A::NOT_PENDING,
            true => BODCORE_INT_STATUS_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_PENDING`"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == BODCORE_INT_STATUS_A::NOT_PENDING
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == BODCORE_INT_STATUS_A::PENDING
    }
}
#[doc = "Field `BODCORE_VAL` reader - Current value of BOD CORE power status output."]
pub type BODCORE_VAL_R = crate::BitReader<BODCORE_VAL_A>;
#[doc = "Current value of BOD CORE power status output.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BODCORE_VAL_A {
    #[doc = "0: CORE voltage level is below the threshold."]
    NOT_OK = 0,
    #[doc = "1: CORE voltage level is above the threshold."]
    OK = 1,
}
impl From<BODCORE_VAL_A> for bool {
    #[inline(always)]
    fn from(variant: BODCORE_VAL_A) -> Self {
        variant as u8 != 0
    }
}
impl BODCORE_VAL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BODCORE_VAL_A {
        match self.bits {
            false => BODCORE_VAL_A::NOT_OK,
            true => BODCORE_VAL_A::OK,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_OK`"]
    #[inline(always)]
    pub fn is_not_ok(&self) -> bool {
        *self == BODCORE_VAL_A::NOT_OK
    }
    #[doc = "Checks if the value of the field is `OK`"]
    #[inline(always)]
    pub fn is_ok(&self) -> bool {
        *self == BODCORE_VAL_A::OK
    }
}
#[doc = "Field `DCDC_STATUS` reader - DCDC Interrupt status before Interrupt Enable."]
pub type DCDC_STATUS_R = crate::BitReader<DCDC_STATUS_A>;
#[doc = "DCDC Interrupt status before Interrupt Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DCDC_STATUS_A {
    #[doc = "0: No interrupt pending.."]
    NOT_PENDING = 0,
    #[doc = "1: Interrupt pending.."]
    PENDING = 1,
}
impl From<DCDC_STATUS_A> for bool {
    #[inline(always)]
    fn from(variant: DCDC_STATUS_A) -> Self {
        variant as u8 != 0
    }
}
impl DCDC_STATUS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DCDC_STATUS_A {
        match self.bits {
            false => DCDC_STATUS_A::NOT_PENDING,
            true => DCDC_STATUS_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_PENDING`"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == DCDC_STATUS_A::NOT_PENDING
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == DCDC_STATUS_A::PENDING
    }
}
#[doc = "Field `DCDC_INT_STATUS` reader - DCDC Interrupt status after Interrupt Enable."]
pub type DCDC_INT_STATUS_R = crate::BitReader<DCDC_INT_STATUS_A>;
#[doc = "DCDC Interrupt status after Interrupt Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DCDC_INT_STATUS_A {
    #[doc = "0: No interrupt pending.."]
    NOT_PENDING = 0,
    #[doc = "1: Interrupt pending.."]
    PENDING = 1,
}
impl From<DCDC_INT_STATUS_A> for bool {
    #[inline(always)]
    fn from(variant: DCDC_INT_STATUS_A) -> Self {
        variant as u8 != 0
    }
}
impl DCDC_INT_STATUS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DCDC_INT_STATUS_A {
        match self.bits {
            false => DCDC_INT_STATUS_A::NOT_PENDING,
            true => DCDC_INT_STATUS_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_PENDING`"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == DCDC_INT_STATUS_A::NOT_PENDING
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == DCDC_INT_STATUS_A::PENDING
    }
}
#[doc = "Field `DCDC_VAL` reader - Current value of DCDC power status output."]
pub type DCDC_VAL_R = crate::BitReader<DCDC_VAL_A>;
#[doc = "Current value of DCDC power status output.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DCDC_VAL_A {
    #[doc = "0: DCDC output Voltage is below the targeted regulation level."]
    NOT_OK = 0,
    #[doc = "1: DCDC output Voltage is above the targeted regulation level."]
    OK = 1,
}
impl From<DCDC_VAL_A> for bool {
    #[inline(always)]
    fn from(variant: DCDC_VAL_A) -> Self {
        variant as u8 != 0
    }
}
impl DCDC_VAL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DCDC_VAL_A {
        match self.bits {
            false => DCDC_VAL_A::NOT_OK,
            true => DCDC_VAL_A::OK,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_OK`"]
    #[inline(always)]
    pub fn is_not_ok(&self) -> bool {
        *self == DCDC_VAL_A::NOT_OK
    }
    #[doc = "Checks if the value of the field is `OK`"]
    #[inline(always)]
    pub fn is_ok(&self) -> bool {
        *self == DCDC_VAL_A::OK
    }
}
impl R {
    #[doc = "Bit 0 - BOD VBAT Interrupt status before Interrupt Enable."]
    #[inline(always)]
    pub fn bodvbat_status(&self) -> BODVBAT_STATUS_R {
        BODVBAT_STATUS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - BOD VBAT Interrupt status after Interrupt Enable."]
    #[inline(always)]
    pub fn bodvbat_int_status(&self) -> BODVBAT_INT_STATUS_R {
        BODVBAT_INT_STATUS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Current value of BOD VBAT power status output."]
    #[inline(always)]
    pub fn bodvbat_val(&self) -> BODVBAT_VAL_R {
        BODVBAT_VAL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - BOD CORE Interrupt status before Interrupt Enable."]
    #[inline(always)]
    pub fn bodcore_status(&self) -> BODCORE_STATUS_R {
        BODCORE_STATUS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - BOD CORE Interrupt status after Interrupt Enable."]
    #[inline(always)]
    pub fn bodcore_int_status(&self) -> BODCORE_INT_STATUS_R {
        BODCORE_INT_STATUS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Current value of BOD CORE power status output."]
    #[inline(always)]
    pub fn bodcore_val(&self) -> BODCORE_VAL_R {
        BODCORE_VAL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - DCDC Interrupt status before Interrupt Enable."]
    #[inline(always)]
    pub fn dcdc_status(&self) -> DCDC_STATUS_R {
        DCDC_STATUS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - DCDC Interrupt status after Interrupt Enable."]
    #[inline(always)]
    pub fn dcdc_int_status(&self) -> DCDC_INT_STATUS_R {
        DCDC_INT_STATUS_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Current value of DCDC power status output."]
    #[inline(always)]
    pub fn dcdc_val(&self) -> DCDC_VAL_R {
        DCDC_VAL_R::new(((self.bits >> 8) & 1) != 0)
    }
}
#[doc = "BoDs & DCDC interrupts status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bod_dcdc_int_status](index.html) module"]
pub struct BOD_DCDC_INT_STATUS_SPEC;
impl crate::RegisterSpec for BOD_DCDC_INT_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bod_dcdc_int_status::R](R) reader structure"]
impl crate::Readable for BOD_DCDC_INT_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets BOD_DCDC_INT_STATUS to value 0x012d"]
impl crate::Resettable for BOD_DCDC_INT_STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x012d
    }
}
