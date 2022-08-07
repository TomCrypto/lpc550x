#[doc = "Register `COMP` reader"]
pub struct R(crate::R<COMP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COMP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<COMP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<COMP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `COMP` writer"]
pub struct W(crate::W<COMP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<COMP_SPEC>;
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
impl From<crate::W<COMP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<COMP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HYST` reader - Hysteris when hyst = '1'."]
pub type HYST_R = crate::BitReader<HYST_A>;
#[doc = "Hysteris when hyst = '1'.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HYST_A {
    #[doc = "0: Hysteresis is disable."]
    DISABLE = 0,
    #[doc = "1: Hysteresis is enable."]
    ENABLE = 1,
}
impl From<HYST_A> for bool {
    #[inline(always)]
    fn from(variant: HYST_A) -> Self {
        variant as u8 != 0
    }
}
impl HYST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HYST_A {
        match self.bits {
            false => HYST_A::DISABLE,
            true => HYST_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == HYST_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == HYST_A::ENABLE
    }
}
#[doc = "Field `HYST` writer - Hysteris when hyst = '1'."]
pub type HYST_W<'a, const O: u8> = crate::BitWriter<'a, u32, COMP_SPEC, HYST_A, O>;
impl<'a, const O: u8> HYST_W<'a, O> {
    #[doc = "Hysteresis is disable."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(HYST_A::DISABLE)
    }
    #[doc = "Hysteresis is enable."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(HYST_A::ENABLE)
    }
}
#[doc = "Field `VREFINPUT` reader - Dedicated control bit to select between internal VREF and VDDA (for the resistive ladder)."]
pub type VREFINPUT_R = crate::BitReader<VREFINPUT_A>;
#[doc = "Dedicated control bit to select between internal VREF and VDDA (for the resistive ladder).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VREFINPUT_A {
    #[doc = "0: Select internal VREF."]
    INTERNALREF = 0,
    #[doc = "1: Select VDDA."]
    VDDA = 1,
}
impl From<VREFINPUT_A> for bool {
    #[inline(always)]
    fn from(variant: VREFINPUT_A) -> Self {
        variant as u8 != 0
    }
}
impl VREFINPUT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VREFINPUT_A {
        match self.bits {
            false => VREFINPUT_A::INTERNALREF,
            true => VREFINPUT_A::VDDA,
        }
    }
    #[doc = "Checks if the value of the field is `INTERNALREF`"]
    #[inline(always)]
    pub fn is_internalref(&self) -> bool {
        *self == VREFINPUT_A::INTERNALREF
    }
    #[doc = "Checks if the value of the field is `VDDA`"]
    #[inline(always)]
    pub fn is_vdda(&self) -> bool {
        *self == VREFINPUT_A::VDDA
    }
}
#[doc = "Field `VREFINPUT` writer - Dedicated control bit to select between internal VREF and VDDA (for the resistive ladder)."]
pub type VREFINPUT_W<'a, const O: u8> = crate::BitWriter<'a, u32, COMP_SPEC, VREFINPUT_A, O>;
impl<'a, const O: u8> VREFINPUT_W<'a, O> {
    #[doc = "Select internal VREF."]
    #[inline(always)]
    pub fn internalref(self) -> &'a mut W {
        self.variant(VREFINPUT_A::INTERNALREF)
    }
    #[doc = "Select VDDA."]
    #[inline(always)]
    pub fn vdda(self) -> &'a mut W {
        self.variant(VREFINPUT_A::VDDA)
    }
}
#[doc = "Field `LOWPOWER` reader - Low power mode."]
pub type LOWPOWER_R = crate::BitReader<LOWPOWER_A>;
#[doc = "Low power mode.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOWPOWER_A {
    #[doc = "0: High speed mode."]
    HIGHSPEED = 0,
    #[doc = "1: Low power mode (Low speed)."]
    LOWSPEED = 1,
}
impl From<LOWPOWER_A> for bool {
    #[inline(always)]
    fn from(variant: LOWPOWER_A) -> Self {
        variant as u8 != 0
    }
}
impl LOWPOWER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOWPOWER_A {
        match self.bits {
            false => LOWPOWER_A::HIGHSPEED,
            true => LOWPOWER_A::LOWSPEED,
        }
    }
    #[doc = "Checks if the value of the field is `HIGHSPEED`"]
    #[inline(always)]
    pub fn is_highspeed(&self) -> bool {
        *self == LOWPOWER_A::HIGHSPEED
    }
    #[doc = "Checks if the value of the field is `LOWSPEED`"]
    #[inline(always)]
    pub fn is_lowspeed(&self) -> bool {
        *self == LOWPOWER_A::LOWSPEED
    }
}
#[doc = "Field `LOWPOWER` writer - Low power mode."]
pub type LOWPOWER_W<'a, const O: u8> = crate::BitWriter<'a, u32, COMP_SPEC, LOWPOWER_A, O>;
impl<'a, const O: u8> LOWPOWER_W<'a, O> {
    #[doc = "High speed mode."]
    #[inline(always)]
    pub fn highspeed(self) -> &'a mut W {
        self.variant(LOWPOWER_A::HIGHSPEED)
    }
    #[doc = "Low power mode (Low speed)."]
    #[inline(always)]
    pub fn lowspeed(self) -> &'a mut W {
        self.variant(LOWPOWER_A::LOWSPEED)
    }
}
#[doc = "Field `PMUX` reader - Control word for P multiplexer:."]
pub type PMUX_R = crate::FieldReader<u8, PMUX_A>;
#[doc = "Control word for P multiplexer:.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PMUX_A {
    #[doc = "0: VREF (See fiedl VREFINPUT)."]
    VREF = 0,
    #[doc = "1: Pin P0_0."]
    CMP0_A = 1,
    #[doc = "2: Pin P0_9."]
    CMP0_B = 2,
    #[doc = "3: Pin P0_18."]
    CMP0_C = 3,
    #[doc = "4: Pin P1_14."]
    CMP0_D = 4,
    #[doc = "5: Pin P2_23."]
    CMP0_E = 5,
}
impl From<PMUX_A> for u8 {
    #[inline(always)]
    fn from(variant: PMUX_A) -> Self {
        variant as _
    }
}
impl PMUX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PMUX_A> {
        match self.bits {
            0 => Some(PMUX_A::VREF),
            1 => Some(PMUX_A::CMP0_A),
            2 => Some(PMUX_A::CMP0_B),
            3 => Some(PMUX_A::CMP0_C),
            4 => Some(PMUX_A::CMP0_D),
            5 => Some(PMUX_A::CMP0_E),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VREF`"]
    #[inline(always)]
    pub fn is_vref(&self) -> bool {
        *self == PMUX_A::VREF
    }
    #[doc = "Checks if the value of the field is `CMP0_A`"]
    #[inline(always)]
    pub fn is_cmp0_a(&self) -> bool {
        *self == PMUX_A::CMP0_A
    }
    #[doc = "Checks if the value of the field is `CMP0_B`"]
    #[inline(always)]
    pub fn is_cmp0_b(&self) -> bool {
        *self == PMUX_A::CMP0_B
    }
    #[doc = "Checks if the value of the field is `CMP0_C`"]
    #[inline(always)]
    pub fn is_cmp0_c(&self) -> bool {
        *self == PMUX_A::CMP0_C
    }
    #[doc = "Checks if the value of the field is `CMP0_D`"]
    #[inline(always)]
    pub fn is_cmp0_d(&self) -> bool {
        *self == PMUX_A::CMP0_D
    }
    #[doc = "Checks if the value of the field is `CMP0_E`"]
    #[inline(always)]
    pub fn is_cmp0_e(&self) -> bool {
        *self == PMUX_A::CMP0_E
    }
}
#[doc = "Field `PMUX` writer - Control word for P multiplexer:."]
pub type PMUX_W<'a, const O: u8> = crate::FieldWriter<'a, u32, COMP_SPEC, u8, PMUX_A, 3, O>;
impl<'a, const O: u8> PMUX_W<'a, O> {
    #[doc = "VREF (See fiedl VREFINPUT)."]
    #[inline(always)]
    pub fn vref(self) -> &'a mut W {
        self.variant(PMUX_A::VREF)
    }
    #[doc = "Pin P0_0."]
    #[inline(always)]
    pub fn cmp0_a(self) -> &'a mut W {
        self.variant(PMUX_A::CMP0_A)
    }
    #[doc = "Pin P0_9."]
    #[inline(always)]
    pub fn cmp0_b(self) -> &'a mut W {
        self.variant(PMUX_A::CMP0_B)
    }
    #[doc = "Pin P0_18."]
    #[inline(always)]
    pub fn cmp0_c(self) -> &'a mut W {
        self.variant(PMUX_A::CMP0_C)
    }
    #[doc = "Pin P1_14."]
    #[inline(always)]
    pub fn cmp0_d(self) -> &'a mut W {
        self.variant(PMUX_A::CMP0_D)
    }
    #[doc = "Pin P2_23."]
    #[inline(always)]
    pub fn cmp0_e(self) -> &'a mut W {
        self.variant(PMUX_A::CMP0_E)
    }
}
#[doc = "Field `NMUX` reader - Control word for N multiplexer:."]
pub type NMUX_R = crate::FieldReader<u8, NMUX_A>;
#[doc = "Control word for N multiplexer:.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum NMUX_A {
    #[doc = "0: VREF (See field VREFINPUT)."]
    VREF = 0,
    #[doc = "1: Pin P0_0."]
    CMP0_A = 1,
    #[doc = "2: Pin P0_9."]
    CMP0_B = 2,
    #[doc = "3: Pin P0_18."]
    CMP0_C = 3,
    #[doc = "4: Pin P1_14."]
    CMP0_D = 4,
    #[doc = "5: Pin P2_23."]
    CMP0_E = 5,
}
impl From<NMUX_A> for u8 {
    #[inline(always)]
    fn from(variant: NMUX_A) -> Self {
        variant as _
    }
}
impl NMUX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<NMUX_A> {
        match self.bits {
            0 => Some(NMUX_A::VREF),
            1 => Some(NMUX_A::CMP0_A),
            2 => Some(NMUX_A::CMP0_B),
            3 => Some(NMUX_A::CMP0_C),
            4 => Some(NMUX_A::CMP0_D),
            5 => Some(NMUX_A::CMP0_E),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VREF`"]
    #[inline(always)]
    pub fn is_vref(&self) -> bool {
        *self == NMUX_A::VREF
    }
    #[doc = "Checks if the value of the field is `CMP0_A`"]
    #[inline(always)]
    pub fn is_cmp0_a(&self) -> bool {
        *self == NMUX_A::CMP0_A
    }
    #[doc = "Checks if the value of the field is `CMP0_B`"]
    #[inline(always)]
    pub fn is_cmp0_b(&self) -> bool {
        *self == NMUX_A::CMP0_B
    }
    #[doc = "Checks if the value of the field is `CMP0_C`"]
    #[inline(always)]
    pub fn is_cmp0_c(&self) -> bool {
        *self == NMUX_A::CMP0_C
    }
    #[doc = "Checks if the value of the field is `CMP0_D`"]
    #[inline(always)]
    pub fn is_cmp0_d(&self) -> bool {
        *self == NMUX_A::CMP0_D
    }
    #[doc = "Checks if the value of the field is `CMP0_E`"]
    #[inline(always)]
    pub fn is_cmp0_e(&self) -> bool {
        *self == NMUX_A::CMP0_E
    }
}
#[doc = "Field `NMUX` writer - Control word for N multiplexer:."]
pub type NMUX_W<'a, const O: u8> = crate::FieldWriter<'a, u32, COMP_SPEC, u8, NMUX_A, 3, O>;
impl<'a, const O: u8> NMUX_W<'a, O> {
    #[doc = "VREF (See field VREFINPUT)."]
    #[inline(always)]
    pub fn vref(self) -> &'a mut W {
        self.variant(NMUX_A::VREF)
    }
    #[doc = "Pin P0_0."]
    #[inline(always)]
    pub fn cmp0_a(self) -> &'a mut W {
        self.variant(NMUX_A::CMP0_A)
    }
    #[doc = "Pin P0_9."]
    #[inline(always)]
    pub fn cmp0_b(self) -> &'a mut W {
        self.variant(NMUX_A::CMP0_B)
    }
    #[doc = "Pin P0_18."]
    #[inline(always)]
    pub fn cmp0_c(self) -> &'a mut W {
        self.variant(NMUX_A::CMP0_C)
    }
    #[doc = "Pin P1_14."]
    #[inline(always)]
    pub fn cmp0_d(self) -> &'a mut W {
        self.variant(NMUX_A::CMP0_D)
    }
    #[doc = "Pin P2_23."]
    #[inline(always)]
    pub fn cmp0_e(self) -> &'a mut W {
        self.variant(NMUX_A::CMP0_E)
    }
}
#[doc = "Field `VREF` reader - Control reference voltage step, per steps of (VREFINPUT/31)."]
pub type VREF_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VREF` writer - Control reference voltage step, per steps of (VREFINPUT/31)."]
pub type VREF_W<'a, const O: u8> = crate::FieldWriter<'a, u32, COMP_SPEC, u8, u8, 5, O>;
#[doc = "Field `FILTERCGF_SAMPLEMODE` reader - Control the filtering of the Analog Comparator output."]
pub type FILTERCGF_SAMPLEMODE_R = crate::FieldReader<u8, FILTERCGF_SAMPLEMODE_A>;
#[doc = "Control the filtering of the Analog Comparator output.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FILTERCGF_SAMPLEMODE_A {
    #[doc = "0: Bypass mode."]
    BYPASS = 0,
    #[doc = "1: Filter 1 clock period."]
    FILTER1CLK = 1,
    #[doc = "2: Filter 2 clock period."]
    FILTER2CLK = 2,
    #[doc = "3: Filter 3 clock period."]
    FILTER3CLK = 3,
}
impl From<FILTERCGF_SAMPLEMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: FILTERCGF_SAMPLEMODE_A) -> Self {
        variant as _
    }
}
impl FILTERCGF_SAMPLEMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FILTERCGF_SAMPLEMODE_A {
        match self.bits {
            0 => FILTERCGF_SAMPLEMODE_A::BYPASS,
            1 => FILTERCGF_SAMPLEMODE_A::FILTER1CLK,
            2 => FILTERCGF_SAMPLEMODE_A::FILTER2CLK,
            3 => FILTERCGF_SAMPLEMODE_A::FILTER3CLK,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `BYPASS`"]
    #[inline(always)]
    pub fn is_bypass(&self) -> bool {
        *self == FILTERCGF_SAMPLEMODE_A::BYPASS
    }
    #[doc = "Checks if the value of the field is `FILTER1CLK`"]
    #[inline(always)]
    pub fn is_filter1clk(&self) -> bool {
        *self == FILTERCGF_SAMPLEMODE_A::FILTER1CLK
    }
    #[doc = "Checks if the value of the field is `FILTER2CLK`"]
    #[inline(always)]
    pub fn is_filter2clk(&self) -> bool {
        *self == FILTERCGF_SAMPLEMODE_A::FILTER2CLK
    }
    #[doc = "Checks if the value of the field is `FILTER3CLK`"]
    #[inline(always)]
    pub fn is_filter3clk(&self) -> bool {
        *self == FILTERCGF_SAMPLEMODE_A::FILTER3CLK
    }
}
#[doc = "Field `FILTERCGF_SAMPLEMODE` writer - Control the filtering of the Analog Comparator output."]
pub type FILTERCGF_SAMPLEMODE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, COMP_SPEC, u8, FILTERCGF_SAMPLEMODE_A, 2, O>;
impl<'a, const O: u8> FILTERCGF_SAMPLEMODE_W<'a, O> {
    #[doc = "Bypass mode."]
    #[inline(always)]
    pub fn bypass(self) -> &'a mut W {
        self.variant(FILTERCGF_SAMPLEMODE_A::BYPASS)
    }
    #[doc = "Filter 1 clock period."]
    #[inline(always)]
    pub fn filter1clk(self) -> &'a mut W {
        self.variant(FILTERCGF_SAMPLEMODE_A::FILTER1CLK)
    }
    #[doc = "Filter 2 clock period."]
    #[inline(always)]
    pub fn filter2clk(self) -> &'a mut W {
        self.variant(FILTERCGF_SAMPLEMODE_A::FILTER2CLK)
    }
    #[doc = "Filter 3 clock period."]
    #[inline(always)]
    pub fn filter3clk(self) -> &'a mut W {
        self.variant(FILTERCGF_SAMPLEMODE_A::FILTER3CLK)
    }
}
#[doc = "Field `FILTERCGF_CLKDIV` reader - Filter Clock divider."]
pub type FILTERCGF_CLKDIV_R = crate::FieldReader<u8, FILTERCGF_CLKDIV_A>;
#[doc = "Filter Clock divider.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FILTERCGF_CLKDIV_A {
    #[doc = "0: Filter clock period duration equals 1 Analog Comparator clock period."]
    FILTER_1CLK_PERIOD = 0,
    #[doc = "1: Filter clock period duration equals 2 Analog Comparator clock period."]
    FILTER_2CLK_PERIOD = 1,
    #[doc = "2: Filter clock period duration equals 4 Analog Comparator clock period."]
    FILTER_4CLK_PERIOD = 2,
    #[doc = "3: Filter clock period duration equals 8 Analog Comparator clock period."]
    FILTER_8CLK_PERIOD = 3,
    #[doc = "4: Filter clock period duration equals 16 Analog Comparator clock period."]
    FILTER_16CLK_PERIOD = 4,
    #[doc = "5: Filter clock period duration equals 32 Analog Comparator clock period."]
    FILTER_32CLK_PERIOD = 5,
    #[doc = "6: Filter clock period duration equals 64 Analog Comparator clock period."]
    FILTER_64CLK_PERIOD = 6,
    #[doc = "7: Filter clock period duration equals 128 Analog Comparator clock period."]
    FILTER_128CLK_PERIOD = 7,
}
impl From<FILTERCGF_CLKDIV_A> for u8 {
    #[inline(always)]
    fn from(variant: FILTERCGF_CLKDIV_A) -> Self {
        variant as _
    }
}
impl FILTERCGF_CLKDIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FILTERCGF_CLKDIV_A {
        match self.bits {
            0 => FILTERCGF_CLKDIV_A::FILTER_1CLK_PERIOD,
            1 => FILTERCGF_CLKDIV_A::FILTER_2CLK_PERIOD,
            2 => FILTERCGF_CLKDIV_A::FILTER_4CLK_PERIOD,
            3 => FILTERCGF_CLKDIV_A::FILTER_8CLK_PERIOD,
            4 => FILTERCGF_CLKDIV_A::FILTER_16CLK_PERIOD,
            5 => FILTERCGF_CLKDIV_A::FILTER_32CLK_PERIOD,
            6 => FILTERCGF_CLKDIV_A::FILTER_64CLK_PERIOD,
            7 => FILTERCGF_CLKDIV_A::FILTER_128CLK_PERIOD,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `FILTER_1CLK_PERIOD`"]
    #[inline(always)]
    pub fn is_filter_1clk_period(&self) -> bool {
        *self == FILTERCGF_CLKDIV_A::FILTER_1CLK_PERIOD
    }
    #[doc = "Checks if the value of the field is `FILTER_2CLK_PERIOD`"]
    #[inline(always)]
    pub fn is_filter_2clk_period(&self) -> bool {
        *self == FILTERCGF_CLKDIV_A::FILTER_2CLK_PERIOD
    }
    #[doc = "Checks if the value of the field is `FILTER_4CLK_PERIOD`"]
    #[inline(always)]
    pub fn is_filter_4clk_period(&self) -> bool {
        *self == FILTERCGF_CLKDIV_A::FILTER_4CLK_PERIOD
    }
    #[doc = "Checks if the value of the field is `FILTER_8CLK_PERIOD`"]
    #[inline(always)]
    pub fn is_filter_8clk_period(&self) -> bool {
        *self == FILTERCGF_CLKDIV_A::FILTER_8CLK_PERIOD
    }
    #[doc = "Checks if the value of the field is `FILTER_16CLK_PERIOD`"]
    #[inline(always)]
    pub fn is_filter_16clk_period(&self) -> bool {
        *self == FILTERCGF_CLKDIV_A::FILTER_16CLK_PERIOD
    }
    #[doc = "Checks if the value of the field is `FILTER_32CLK_PERIOD`"]
    #[inline(always)]
    pub fn is_filter_32clk_period(&self) -> bool {
        *self == FILTERCGF_CLKDIV_A::FILTER_32CLK_PERIOD
    }
    #[doc = "Checks if the value of the field is `FILTER_64CLK_PERIOD`"]
    #[inline(always)]
    pub fn is_filter_64clk_period(&self) -> bool {
        *self == FILTERCGF_CLKDIV_A::FILTER_64CLK_PERIOD
    }
    #[doc = "Checks if the value of the field is `FILTER_128CLK_PERIOD`"]
    #[inline(always)]
    pub fn is_filter_128clk_period(&self) -> bool {
        *self == FILTERCGF_CLKDIV_A::FILTER_128CLK_PERIOD
    }
}
#[doc = "Field `FILTERCGF_CLKDIV` writer - Filter Clock divider."]
pub type FILTERCGF_CLKDIV_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, COMP_SPEC, u8, FILTERCGF_CLKDIV_A, 3, O>;
impl<'a, const O: u8> FILTERCGF_CLKDIV_W<'a, O> {
    #[doc = "Filter clock period duration equals 1 Analog Comparator clock period."]
    #[inline(always)]
    pub fn filter_1clk_period(self) -> &'a mut W {
        self.variant(FILTERCGF_CLKDIV_A::FILTER_1CLK_PERIOD)
    }
    #[doc = "Filter clock period duration equals 2 Analog Comparator clock period."]
    #[inline(always)]
    pub fn filter_2clk_period(self) -> &'a mut W {
        self.variant(FILTERCGF_CLKDIV_A::FILTER_2CLK_PERIOD)
    }
    #[doc = "Filter clock period duration equals 4 Analog Comparator clock period."]
    #[inline(always)]
    pub fn filter_4clk_period(self) -> &'a mut W {
        self.variant(FILTERCGF_CLKDIV_A::FILTER_4CLK_PERIOD)
    }
    #[doc = "Filter clock period duration equals 8 Analog Comparator clock period."]
    #[inline(always)]
    pub fn filter_8clk_period(self) -> &'a mut W {
        self.variant(FILTERCGF_CLKDIV_A::FILTER_8CLK_PERIOD)
    }
    #[doc = "Filter clock period duration equals 16 Analog Comparator clock period."]
    #[inline(always)]
    pub fn filter_16clk_period(self) -> &'a mut W {
        self.variant(FILTERCGF_CLKDIV_A::FILTER_16CLK_PERIOD)
    }
    #[doc = "Filter clock period duration equals 32 Analog Comparator clock period."]
    #[inline(always)]
    pub fn filter_32clk_period(self) -> &'a mut W {
        self.variant(FILTERCGF_CLKDIV_A::FILTER_32CLK_PERIOD)
    }
    #[doc = "Filter clock period duration equals 64 Analog Comparator clock period."]
    #[inline(always)]
    pub fn filter_64clk_period(self) -> &'a mut W {
        self.variant(FILTERCGF_CLKDIV_A::FILTER_64CLK_PERIOD)
    }
    #[doc = "Filter clock period duration equals 128 Analog Comparator clock period."]
    #[inline(always)]
    pub fn filter_128clk_period(self) -> &'a mut W {
        self.variant(FILTERCGF_CLKDIV_A::FILTER_128CLK_PERIOD)
    }
}
impl R {
    #[doc = "Bit 1 - Hysteris when hyst = '1'."]
    #[inline(always)]
    pub fn hyst(&self) -> HYST_R {
        HYST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Dedicated control bit to select between internal VREF and VDDA (for the resistive ladder)."]
    #[inline(always)]
    pub fn vrefinput(&self) -> VREFINPUT_R {
        VREFINPUT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Low power mode."]
    #[inline(always)]
    pub fn lowpower(&self) -> LOWPOWER_R {
        LOWPOWER_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Control word for P multiplexer:."]
    #[inline(always)]
    pub fn pmux(&self) -> PMUX_R {
        PMUX_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 7:9 - Control word for N multiplexer:."]
    #[inline(always)]
    pub fn nmux(&self) -> NMUX_R {
        NMUX_R::new(((self.bits >> 7) & 7) as u8)
    }
    #[doc = "Bits 10:14 - Control reference voltage step, per steps of (VREFINPUT/31)."]
    #[inline(always)]
    pub fn vref(&self) -> VREF_R {
        VREF_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bits 16:17 - Control the filtering of the Analog Comparator output."]
    #[inline(always)]
    pub fn filtercgf_samplemode(&self) -> FILTERCGF_SAMPLEMODE_R {
        FILTERCGF_SAMPLEMODE_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:20 - Filter Clock divider."]
    #[inline(always)]
    pub fn filtercgf_clkdiv(&self) -> FILTERCGF_CLKDIV_R {
        FILTERCGF_CLKDIV_R::new(((self.bits >> 18) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - Hysteris when hyst = '1'."]
    #[inline(always)]
    pub fn hyst(&mut self) -> HYST_W<1> {
        HYST_W::new(self)
    }
    #[doc = "Bit 2 - Dedicated control bit to select between internal VREF and VDDA (for the resistive ladder)."]
    #[inline(always)]
    pub fn vrefinput(&mut self) -> VREFINPUT_W<2> {
        VREFINPUT_W::new(self)
    }
    #[doc = "Bit 3 - Low power mode."]
    #[inline(always)]
    pub fn lowpower(&mut self) -> LOWPOWER_W<3> {
        LOWPOWER_W::new(self)
    }
    #[doc = "Bits 4:6 - Control word for P multiplexer:."]
    #[inline(always)]
    pub fn pmux(&mut self) -> PMUX_W<4> {
        PMUX_W::new(self)
    }
    #[doc = "Bits 7:9 - Control word for N multiplexer:."]
    #[inline(always)]
    pub fn nmux(&mut self) -> NMUX_W<7> {
        NMUX_W::new(self)
    }
    #[doc = "Bits 10:14 - Control reference voltage step, per steps of (VREFINPUT/31)."]
    #[inline(always)]
    pub fn vref(&mut self) -> VREF_W<10> {
        VREF_W::new(self)
    }
    #[doc = "Bits 16:17 - Control the filtering of the Analog Comparator output."]
    #[inline(always)]
    pub fn filtercgf_samplemode(&mut self) -> FILTERCGF_SAMPLEMODE_W<16> {
        FILTERCGF_SAMPLEMODE_W::new(self)
    }
    #[doc = "Bits 18:20 - Filter Clock divider."]
    #[inline(always)]
    pub fn filtercgf_clkdiv(&mut self) -> FILTERCGF_CLKDIV_W<18> {
        FILTERCGF_CLKDIV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Analog Comparator control register \\[Reset by: PoR, Pin Reset, Brown Out Detectors Reset, Deep Power Down Reset, Software Reset\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [comp](index.html) module"]
pub struct COMP_SPEC;
impl crate::RegisterSpec for COMP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [comp::R](R) reader structure"]
impl crate::Readable for COMP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [comp::W](W) writer structure"]
impl crate::Writable for COMP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets COMP to value 0x0a"]
impl crate::Resettable for COMP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0a
    }
}
