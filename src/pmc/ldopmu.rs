#[doc = "Register `LDOPMU` reader"]
pub struct R(crate::R<LDOPMU_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LDOPMU_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LDOPMU_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LDOPMU_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LDOPMU` writer"]
pub struct W(crate::W<LDOPMU_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LDOPMU_SPEC>;
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
impl From<crate::W<LDOPMU_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LDOPMU_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VADJ` reader - Sets the Always-On domain LDO output level."]
pub type VADJ_R = crate::FieldReader<u8, VADJ_A>;
#[doc = "Sets the Always-On domain LDO output level.\n\nValue on reset: 24"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum VADJ_A {
    #[doc = "1: 0.7 V."]
    V_0P700 = 1,
    #[doc = "2: 0.725 V."]
    V_0P725 = 2,
    #[doc = "3: 0.75 V."]
    V_0P750 = 3,
    #[doc = "4: 0.775 V."]
    V_0P775 = 4,
    #[doc = "5: 0.8 V."]
    V_0P800 = 5,
    #[doc = "6: 0.825 V."]
    V_0P825 = 6,
    #[doc = "7: 0.85 V."]
    V_0P850 = 7,
    #[doc = "8: 0.875 V."]
    V_0P875 = 8,
    #[doc = "9: 0.9 V."]
    V_0P900 = 9,
    #[doc = "10: 0.96 V."]
    V_0P960 = 10,
    #[doc = "11: 0.97 V."]
    V_0P970 = 11,
    #[doc = "12: 0.98 V."]
    V_0P980 = 12,
    #[doc = "13: 0.99 V."]
    V_0P990 = 13,
    #[doc = "14: 1 V."]
    V_1P000 = 14,
    #[doc = "15: 1.01 V."]
    V_1P010 = 15,
    #[doc = "16: 1.02 V."]
    V_1P020 = 16,
    #[doc = "17: 1.03 V."]
    V_1P030 = 17,
    #[doc = "18: 1.04 V."]
    V_1P040 = 18,
    #[doc = "19: 1.05 V."]
    V_1P050 = 19,
    #[doc = "20: 1.06 V."]
    V_1P060 = 20,
    #[doc = "21: 1.07 V."]
    V_1P070 = 21,
    #[doc = "22: 1.08 V."]
    V_1P080 = 22,
    #[doc = "23: 1.09 V."]
    V_1P090 = 23,
    #[doc = "24: 1.1 V."]
    V_1P100 = 24,
    #[doc = "25: 1.11 V."]
    V_1P110 = 25,
    #[doc = "26: 1.12 V."]
    V_1P120 = 26,
    #[doc = "27: 1.13 V."]
    V_1P130 = 27,
    #[doc = "28: 1.14 V."]
    V_1P140 = 28,
    #[doc = "29: 1.15 V."]
    V_1P150 = 29,
    #[doc = "30: 1.16 V."]
    V_1P160 = 30,
    #[doc = "31: 1.22 V."]
    V_1P220 = 31,
}
impl From<VADJ_A> for u8 {
    #[inline(always)]
    fn from(variant: VADJ_A) -> Self {
        variant as _
    }
}
impl VADJ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<VADJ_A> {
        match self.bits {
            1 => Some(VADJ_A::V_0P700),
            2 => Some(VADJ_A::V_0P725),
            3 => Some(VADJ_A::V_0P750),
            4 => Some(VADJ_A::V_0P775),
            5 => Some(VADJ_A::V_0P800),
            6 => Some(VADJ_A::V_0P825),
            7 => Some(VADJ_A::V_0P850),
            8 => Some(VADJ_A::V_0P875),
            9 => Some(VADJ_A::V_0P900),
            10 => Some(VADJ_A::V_0P960),
            11 => Some(VADJ_A::V_0P970),
            12 => Some(VADJ_A::V_0P980),
            13 => Some(VADJ_A::V_0P990),
            14 => Some(VADJ_A::V_1P000),
            15 => Some(VADJ_A::V_1P010),
            16 => Some(VADJ_A::V_1P020),
            17 => Some(VADJ_A::V_1P030),
            18 => Some(VADJ_A::V_1P040),
            19 => Some(VADJ_A::V_1P050),
            20 => Some(VADJ_A::V_1P060),
            21 => Some(VADJ_A::V_1P070),
            22 => Some(VADJ_A::V_1P080),
            23 => Some(VADJ_A::V_1P090),
            24 => Some(VADJ_A::V_1P100),
            25 => Some(VADJ_A::V_1P110),
            26 => Some(VADJ_A::V_1P120),
            27 => Some(VADJ_A::V_1P130),
            28 => Some(VADJ_A::V_1P140),
            29 => Some(VADJ_A::V_1P150),
            30 => Some(VADJ_A::V_1P160),
            31 => Some(VADJ_A::V_1P220),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `V_0P700`"]
    #[inline(always)]
    pub fn is_v_0p700(&self) -> bool {
        *self == VADJ_A::V_0P700
    }
    #[doc = "Checks if the value of the field is `V_0P725`"]
    #[inline(always)]
    pub fn is_v_0p725(&self) -> bool {
        *self == VADJ_A::V_0P725
    }
    #[doc = "Checks if the value of the field is `V_0P750`"]
    #[inline(always)]
    pub fn is_v_0p750(&self) -> bool {
        *self == VADJ_A::V_0P750
    }
    #[doc = "Checks if the value of the field is `V_0P775`"]
    #[inline(always)]
    pub fn is_v_0p775(&self) -> bool {
        *self == VADJ_A::V_0P775
    }
    #[doc = "Checks if the value of the field is `V_0P800`"]
    #[inline(always)]
    pub fn is_v_0p800(&self) -> bool {
        *self == VADJ_A::V_0P800
    }
    #[doc = "Checks if the value of the field is `V_0P825`"]
    #[inline(always)]
    pub fn is_v_0p825(&self) -> bool {
        *self == VADJ_A::V_0P825
    }
    #[doc = "Checks if the value of the field is `V_0P850`"]
    #[inline(always)]
    pub fn is_v_0p850(&self) -> bool {
        *self == VADJ_A::V_0P850
    }
    #[doc = "Checks if the value of the field is `V_0P875`"]
    #[inline(always)]
    pub fn is_v_0p875(&self) -> bool {
        *self == VADJ_A::V_0P875
    }
    #[doc = "Checks if the value of the field is `V_0P900`"]
    #[inline(always)]
    pub fn is_v_0p900(&self) -> bool {
        *self == VADJ_A::V_0P900
    }
    #[doc = "Checks if the value of the field is `V_0P960`"]
    #[inline(always)]
    pub fn is_v_0p960(&self) -> bool {
        *self == VADJ_A::V_0P960
    }
    #[doc = "Checks if the value of the field is `V_0P970`"]
    #[inline(always)]
    pub fn is_v_0p970(&self) -> bool {
        *self == VADJ_A::V_0P970
    }
    #[doc = "Checks if the value of the field is `V_0P980`"]
    #[inline(always)]
    pub fn is_v_0p980(&self) -> bool {
        *self == VADJ_A::V_0P980
    }
    #[doc = "Checks if the value of the field is `V_0P990`"]
    #[inline(always)]
    pub fn is_v_0p990(&self) -> bool {
        *self == VADJ_A::V_0P990
    }
    #[doc = "Checks if the value of the field is `V_1P000`"]
    #[inline(always)]
    pub fn is_v_1p000(&self) -> bool {
        *self == VADJ_A::V_1P000
    }
    #[doc = "Checks if the value of the field is `V_1P010`"]
    #[inline(always)]
    pub fn is_v_1p010(&self) -> bool {
        *self == VADJ_A::V_1P010
    }
    #[doc = "Checks if the value of the field is `V_1P020`"]
    #[inline(always)]
    pub fn is_v_1p020(&self) -> bool {
        *self == VADJ_A::V_1P020
    }
    #[doc = "Checks if the value of the field is `V_1P030`"]
    #[inline(always)]
    pub fn is_v_1p030(&self) -> bool {
        *self == VADJ_A::V_1P030
    }
    #[doc = "Checks if the value of the field is `V_1P040`"]
    #[inline(always)]
    pub fn is_v_1p040(&self) -> bool {
        *self == VADJ_A::V_1P040
    }
    #[doc = "Checks if the value of the field is `V_1P050`"]
    #[inline(always)]
    pub fn is_v_1p050(&self) -> bool {
        *self == VADJ_A::V_1P050
    }
    #[doc = "Checks if the value of the field is `V_1P060`"]
    #[inline(always)]
    pub fn is_v_1p060(&self) -> bool {
        *self == VADJ_A::V_1P060
    }
    #[doc = "Checks if the value of the field is `V_1P070`"]
    #[inline(always)]
    pub fn is_v_1p070(&self) -> bool {
        *self == VADJ_A::V_1P070
    }
    #[doc = "Checks if the value of the field is `V_1P080`"]
    #[inline(always)]
    pub fn is_v_1p080(&self) -> bool {
        *self == VADJ_A::V_1P080
    }
    #[doc = "Checks if the value of the field is `V_1P090`"]
    #[inline(always)]
    pub fn is_v_1p090(&self) -> bool {
        *self == VADJ_A::V_1P090
    }
    #[doc = "Checks if the value of the field is `V_1P100`"]
    #[inline(always)]
    pub fn is_v_1p100(&self) -> bool {
        *self == VADJ_A::V_1P100
    }
    #[doc = "Checks if the value of the field is `V_1P110`"]
    #[inline(always)]
    pub fn is_v_1p110(&self) -> bool {
        *self == VADJ_A::V_1P110
    }
    #[doc = "Checks if the value of the field is `V_1P120`"]
    #[inline(always)]
    pub fn is_v_1p120(&self) -> bool {
        *self == VADJ_A::V_1P120
    }
    #[doc = "Checks if the value of the field is `V_1P130`"]
    #[inline(always)]
    pub fn is_v_1p130(&self) -> bool {
        *self == VADJ_A::V_1P130
    }
    #[doc = "Checks if the value of the field is `V_1P140`"]
    #[inline(always)]
    pub fn is_v_1p140(&self) -> bool {
        *self == VADJ_A::V_1P140
    }
    #[doc = "Checks if the value of the field is `V_1P150`"]
    #[inline(always)]
    pub fn is_v_1p150(&self) -> bool {
        *self == VADJ_A::V_1P150
    }
    #[doc = "Checks if the value of the field is `V_1P160`"]
    #[inline(always)]
    pub fn is_v_1p160(&self) -> bool {
        *self == VADJ_A::V_1P160
    }
    #[doc = "Checks if the value of the field is `V_1P220`"]
    #[inline(always)]
    pub fn is_v_1p220(&self) -> bool {
        *self == VADJ_A::V_1P220
    }
}
#[doc = "Field `VADJ` writer - Sets the Always-On domain LDO output level."]
pub type VADJ_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LDOPMU_SPEC, u8, VADJ_A, 5, O>;
impl<'a, const O: u8> VADJ_W<'a, O> {
    #[doc = "0.7 V."]
    #[inline(always)]
    pub fn v_0p700(self) -> &'a mut W {
        self.variant(VADJ_A::V_0P700)
    }
    #[doc = "0.725 V."]
    #[inline(always)]
    pub fn v_0p725(self) -> &'a mut W {
        self.variant(VADJ_A::V_0P725)
    }
    #[doc = "0.75 V."]
    #[inline(always)]
    pub fn v_0p750(self) -> &'a mut W {
        self.variant(VADJ_A::V_0P750)
    }
    #[doc = "0.775 V."]
    #[inline(always)]
    pub fn v_0p775(self) -> &'a mut W {
        self.variant(VADJ_A::V_0P775)
    }
    #[doc = "0.8 V."]
    #[inline(always)]
    pub fn v_0p800(self) -> &'a mut W {
        self.variant(VADJ_A::V_0P800)
    }
    #[doc = "0.825 V."]
    #[inline(always)]
    pub fn v_0p825(self) -> &'a mut W {
        self.variant(VADJ_A::V_0P825)
    }
    #[doc = "0.85 V."]
    #[inline(always)]
    pub fn v_0p850(self) -> &'a mut W {
        self.variant(VADJ_A::V_0P850)
    }
    #[doc = "0.875 V."]
    #[inline(always)]
    pub fn v_0p875(self) -> &'a mut W {
        self.variant(VADJ_A::V_0P875)
    }
    #[doc = "0.9 V."]
    #[inline(always)]
    pub fn v_0p900(self) -> &'a mut W {
        self.variant(VADJ_A::V_0P900)
    }
    #[doc = "0.96 V."]
    #[inline(always)]
    pub fn v_0p960(self) -> &'a mut W {
        self.variant(VADJ_A::V_0P960)
    }
    #[doc = "0.97 V."]
    #[inline(always)]
    pub fn v_0p970(self) -> &'a mut W {
        self.variant(VADJ_A::V_0P970)
    }
    #[doc = "0.98 V."]
    #[inline(always)]
    pub fn v_0p980(self) -> &'a mut W {
        self.variant(VADJ_A::V_0P980)
    }
    #[doc = "0.99 V."]
    #[inline(always)]
    pub fn v_0p990(self) -> &'a mut W {
        self.variant(VADJ_A::V_0P990)
    }
    #[doc = "1 V."]
    #[inline(always)]
    pub fn v_1p000(self) -> &'a mut W {
        self.variant(VADJ_A::V_1P000)
    }
    #[doc = "1.01 V."]
    #[inline(always)]
    pub fn v_1p010(self) -> &'a mut W {
        self.variant(VADJ_A::V_1P010)
    }
    #[doc = "1.02 V."]
    #[inline(always)]
    pub fn v_1p020(self) -> &'a mut W {
        self.variant(VADJ_A::V_1P020)
    }
    #[doc = "1.03 V."]
    #[inline(always)]
    pub fn v_1p030(self) -> &'a mut W {
        self.variant(VADJ_A::V_1P030)
    }
    #[doc = "1.04 V."]
    #[inline(always)]
    pub fn v_1p040(self) -> &'a mut W {
        self.variant(VADJ_A::V_1P040)
    }
    #[doc = "1.05 V."]
    #[inline(always)]
    pub fn v_1p050(self) -> &'a mut W {
        self.variant(VADJ_A::V_1P050)
    }
    #[doc = "1.06 V."]
    #[inline(always)]
    pub fn v_1p060(self) -> &'a mut W {
        self.variant(VADJ_A::V_1P060)
    }
    #[doc = "1.07 V."]
    #[inline(always)]
    pub fn v_1p070(self) -> &'a mut W {
        self.variant(VADJ_A::V_1P070)
    }
    #[doc = "1.08 V."]
    #[inline(always)]
    pub fn v_1p080(self) -> &'a mut W {
        self.variant(VADJ_A::V_1P080)
    }
    #[doc = "1.09 V."]
    #[inline(always)]
    pub fn v_1p090(self) -> &'a mut W {
        self.variant(VADJ_A::V_1P090)
    }
    #[doc = "1.1 V."]
    #[inline(always)]
    pub fn v_1p100(self) -> &'a mut W {
        self.variant(VADJ_A::V_1P100)
    }
    #[doc = "1.11 V."]
    #[inline(always)]
    pub fn v_1p110(self) -> &'a mut W {
        self.variant(VADJ_A::V_1P110)
    }
    #[doc = "1.12 V."]
    #[inline(always)]
    pub fn v_1p120(self) -> &'a mut W {
        self.variant(VADJ_A::V_1P120)
    }
    #[doc = "1.13 V."]
    #[inline(always)]
    pub fn v_1p130(self) -> &'a mut W {
        self.variant(VADJ_A::V_1P130)
    }
    #[doc = "1.14 V."]
    #[inline(always)]
    pub fn v_1p140(self) -> &'a mut W {
        self.variant(VADJ_A::V_1P140)
    }
    #[doc = "1.15 V."]
    #[inline(always)]
    pub fn v_1p150(self) -> &'a mut W {
        self.variant(VADJ_A::V_1P150)
    }
    #[doc = "1.16 V."]
    #[inline(always)]
    pub fn v_1p160(self) -> &'a mut W {
        self.variant(VADJ_A::V_1P160)
    }
    #[doc = "1.22 V."]
    #[inline(always)]
    pub fn v_1p220(self) -> &'a mut W {
        self.variant(VADJ_A::V_1P220)
    }
}
#[doc = "Field `VADJ_BOOST` reader - Sets the Always-On domain LDO Boost output level."]
pub type VADJ_BOOST_R = crate::FieldReader<u8, VADJ_BOOST_A>;
#[doc = "Sets the Always-On domain LDO Boost output level.\n\nValue on reset: 29"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum VADJ_BOOST_A {
    #[doc = "1: 0.7 V."]
    V_0P700 = 1,
    #[doc = "2: 0.725 V."]
    V_0P725 = 2,
    #[doc = "3: 0.75 V."]
    V_0P750 = 3,
    #[doc = "4: 0.775 V."]
    V_0P775 = 4,
    #[doc = "5: 0.8 V."]
    V_0P800 = 5,
    #[doc = "6: 0.825 V."]
    V_0P825 = 6,
    #[doc = "7: 0.85 V."]
    V_0P850 = 7,
    #[doc = "8: 0.875 V."]
    V_0P875 = 8,
    #[doc = "9: 0.9 V."]
    V_0P900 = 9,
    #[doc = "10: 0.96 V."]
    V_0P960 = 10,
    #[doc = "11: 0.97 V."]
    V_0P970 = 11,
    #[doc = "12: 0.98 V."]
    V_0P980 = 12,
    #[doc = "13: 0.99 V."]
    V_0P990 = 13,
    #[doc = "14: 1 V."]
    V_1P000 = 14,
    #[doc = "15: 1.01 V."]
    V_1P010 = 15,
    #[doc = "16: 1.02 V."]
    V_1P020 = 16,
    #[doc = "17: 1.03 V."]
    V_1P030 = 17,
    #[doc = "18: 1.04 V."]
    V_1P040 = 18,
    #[doc = "19: 1.05 V."]
    V_1P050 = 19,
    #[doc = "20: 1.06 V."]
    V_1P060 = 20,
    #[doc = "21: 1.07 V."]
    V_1P070 = 21,
    #[doc = "22: 1.08 V."]
    V_1P080 = 22,
    #[doc = "23: 1.09 V."]
    V_1P090 = 23,
    #[doc = "24: 1.1 V."]
    V_1P100 = 24,
    #[doc = "25: 1.11 V."]
    V_1P110 = 25,
    #[doc = "26: 1.12 V."]
    V_1P120 = 26,
    #[doc = "27: 1.13 V."]
    V_1P130 = 27,
    #[doc = "28: 1.14 V."]
    V_1P140 = 28,
    #[doc = "29: 1.15 V."]
    V_1P150 = 29,
    #[doc = "30: 1.16 V."]
    V_1P160 = 30,
    #[doc = "31: 1.22 V."]
    V_1P220 = 31,
}
impl From<VADJ_BOOST_A> for u8 {
    #[inline(always)]
    fn from(variant: VADJ_BOOST_A) -> Self {
        variant as _
    }
}
impl VADJ_BOOST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<VADJ_BOOST_A> {
        match self.bits {
            1 => Some(VADJ_BOOST_A::V_0P700),
            2 => Some(VADJ_BOOST_A::V_0P725),
            3 => Some(VADJ_BOOST_A::V_0P750),
            4 => Some(VADJ_BOOST_A::V_0P775),
            5 => Some(VADJ_BOOST_A::V_0P800),
            6 => Some(VADJ_BOOST_A::V_0P825),
            7 => Some(VADJ_BOOST_A::V_0P850),
            8 => Some(VADJ_BOOST_A::V_0P875),
            9 => Some(VADJ_BOOST_A::V_0P900),
            10 => Some(VADJ_BOOST_A::V_0P960),
            11 => Some(VADJ_BOOST_A::V_0P970),
            12 => Some(VADJ_BOOST_A::V_0P980),
            13 => Some(VADJ_BOOST_A::V_0P990),
            14 => Some(VADJ_BOOST_A::V_1P000),
            15 => Some(VADJ_BOOST_A::V_1P010),
            16 => Some(VADJ_BOOST_A::V_1P020),
            17 => Some(VADJ_BOOST_A::V_1P030),
            18 => Some(VADJ_BOOST_A::V_1P040),
            19 => Some(VADJ_BOOST_A::V_1P050),
            20 => Some(VADJ_BOOST_A::V_1P060),
            21 => Some(VADJ_BOOST_A::V_1P070),
            22 => Some(VADJ_BOOST_A::V_1P080),
            23 => Some(VADJ_BOOST_A::V_1P090),
            24 => Some(VADJ_BOOST_A::V_1P100),
            25 => Some(VADJ_BOOST_A::V_1P110),
            26 => Some(VADJ_BOOST_A::V_1P120),
            27 => Some(VADJ_BOOST_A::V_1P130),
            28 => Some(VADJ_BOOST_A::V_1P140),
            29 => Some(VADJ_BOOST_A::V_1P150),
            30 => Some(VADJ_BOOST_A::V_1P160),
            31 => Some(VADJ_BOOST_A::V_1P220),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `V_0P700`"]
    #[inline(always)]
    pub fn is_v_0p700(&self) -> bool {
        *self == VADJ_BOOST_A::V_0P700
    }
    #[doc = "Checks if the value of the field is `V_0P725`"]
    #[inline(always)]
    pub fn is_v_0p725(&self) -> bool {
        *self == VADJ_BOOST_A::V_0P725
    }
    #[doc = "Checks if the value of the field is `V_0P750`"]
    #[inline(always)]
    pub fn is_v_0p750(&self) -> bool {
        *self == VADJ_BOOST_A::V_0P750
    }
    #[doc = "Checks if the value of the field is `V_0P775`"]
    #[inline(always)]
    pub fn is_v_0p775(&self) -> bool {
        *self == VADJ_BOOST_A::V_0P775
    }
    #[doc = "Checks if the value of the field is `V_0P800`"]
    #[inline(always)]
    pub fn is_v_0p800(&self) -> bool {
        *self == VADJ_BOOST_A::V_0P800
    }
    #[doc = "Checks if the value of the field is `V_0P825`"]
    #[inline(always)]
    pub fn is_v_0p825(&self) -> bool {
        *self == VADJ_BOOST_A::V_0P825
    }
    #[doc = "Checks if the value of the field is `V_0P850`"]
    #[inline(always)]
    pub fn is_v_0p850(&self) -> bool {
        *self == VADJ_BOOST_A::V_0P850
    }
    #[doc = "Checks if the value of the field is `V_0P875`"]
    #[inline(always)]
    pub fn is_v_0p875(&self) -> bool {
        *self == VADJ_BOOST_A::V_0P875
    }
    #[doc = "Checks if the value of the field is `V_0P900`"]
    #[inline(always)]
    pub fn is_v_0p900(&self) -> bool {
        *self == VADJ_BOOST_A::V_0P900
    }
    #[doc = "Checks if the value of the field is `V_0P960`"]
    #[inline(always)]
    pub fn is_v_0p960(&self) -> bool {
        *self == VADJ_BOOST_A::V_0P960
    }
    #[doc = "Checks if the value of the field is `V_0P970`"]
    #[inline(always)]
    pub fn is_v_0p970(&self) -> bool {
        *self == VADJ_BOOST_A::V_0P970
    }
    #[doc = "Checks if the value of the field is `V_0P980`"]
    #[inline(always)]
    pub fn is_v_0p980(&self) -> bool {
        *self == VADJ_BOOST_A::V_0P980
    }
    #[doc = "Checks if the value of the field is `V_0P990`"]
    #[inline(always)]
    pub fn is_v_0p990(&self) -> bool {
        *self == VADJ_BOOST_A::V_0P990
    }
    #[doc = "Checks if the value of the field is `V_1P000`"]
    #[inline(always)]
    pub fn is_v_1p000(&self) -> bool {
        *self == VADJ_BOOST_A::V_1P000
    }
    #[doc = "Checks if the value of the field is `V_1P010`"]
    #[inline(always)]
    pub fn is_v_1p010(&self) -> bool {
        *self == VADJ_BOOST_A::V_1P010
    }
    #[doc = "Checks if the value of the field is `V_1P020`"]
    #[inline(always)]
    pub fn is_v_1p020(&self) -> bool {
        *self == VADJ_BOOST_A::V_1P020
    }
    #[doc = "Checks if the value of the field is `V_1P030`"]
    #[inline(always)]
    pub fn is_v_1p030(&self) -> bool {
        *self == VADJ_BOOST_A::V_1P030
    }
    #[doc = "Checks if the value of the field is `V_1P040`"]
    #[inline(always)]
    pub fn is_v_1p040(&self) -> bool {
        *self == VADJ_BOOST_A::V_1P040
    }
    #[doc = "Checks if the value of the field is `V_1P050`"]
    #[inline(always)]
    pub fn is_v_1p050(&self) -> bool {
        *self == VADJ_BOOST_A::V_1P050
    }
    #[doc = "Checks if the value of the field is `V_1P060`"]
    #[inline(always)]
    pub fn is_v_1p060(&self) -> bool {
        *self == VADJ_BOOST_A::V_1P060
    }
    #[doc = "Checks if the value of the field is `V_1P070`"]
    #[inline(always)]
    pub fn is_v_1p070(&self) -> bool {
        *self == VADJ_BOOST_A::V_1P070
    }
    #[doc = "Checks if the value of the field is `V_1P080`"]
    #[inline(always)]
    pub fn is_v_1p080(&self) -> bool {
        *self == VADJ_BOOST_A::V_1P080
    }
    #[doc = "Checks if the value of the field is `V_1P090`"]
    #[inline(always)]
    pub fn is_v_1p090(&self) -> bool {
        *self == VADJ_BOOST_A::V_1P090
    }
    #[doc = "Checks if the value of the field is `V_1P100`"]
    #[inline(always)]
    pub fn is_v_1p100(&self) -> bool {
        *self == VADJ_BOOST_A::V_1P100
    }
    #[doc = "Checks if the value of the field is `V_1P110`"]
    #[inline(always)]
    pub fn is_v_1p110(&self) -> bool {
        *self == VADJ_BOOST_A::V_1P110
    }
    #[doc = "Checks if the value of the field is `V_1P120`"]
    #[inline(always)]
    pub fn is_v_1p120(&self) -> bool {
        *self == VADJ_BOOST_A::V_1P120
    }
    #[doc = "Checks if the value of the field is `V_1P130`"]
    #[inline(always)]
    pub fn is_v_1p130(&self) -> bool {
        *self == VADJ_BOOST_A::V_1P130
    }
    #[doc = "Checks if the value of the field is `V_1P140`"]
    #[inline(always)]
    pub fn is_v_1p140(&self) -> bool {
        *self == VADJ_BOOST_A::V_1P140
    }
    #[doc = "Checks if the value of the field is `V_1P150`"]
    #[inline(always)]
    pub fn is_v_1p150(&self) -> bool {
        *self == VADJ_BOOST_A::V_1P150
    }
    #[doc = "Checks if the value of the field is `V_1P160`"]
    #[inline(always)]
    pub fn is_v_1p160(&self) -> bool {
        *self == VADJ_BOOST_A::V_1P160
    }
    #[doc = "Checks if the value of the field is `V_1P220`"]
    #[inline(always)]
    pub fn is_v_1p220(&self) -> bool {
        *self == VADJ_BOOST_A::V_1P220
    }
}
#[doc = "Field `VADJ_BOOST` writer - Sets the Always-On domain LDO Boost output level."]
pub type VADJ_BOOST_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LDOPMU_SPEC, u8, VADJ_BOOST_A, 5, O>;
impl<'a, const O: u8> VADJ_BOOST_W<'a, O> {
    #[doc = "0.7 V."]
    #[inline(always)]
    pub fn v_0p700(self) -> &'a mut W {
        self.variant(VADJ_BOOST_A::V_0P700)
    }
    #[doc = "0.725 V."]
    #[inline(always)]
    pub fn v_0p725(self) -> &'a mut W {
        self.variant(VADJ_BOOST_A::V_0P725)
    }
    #[doc = "0.75 V."]
    #[inline(always)]
    pub fn v_0p750(self) -> &'a mut W {
        self.variant(VADJ_BOOST_A::V_0P750)
    }
    #[doc = "0.775 V."]
    #[inline(always)]
    pub fn v_0p775(self) -> &'a mut W {
        self.variant(VADJ_BOOST_A::V_0P775)
    }
    #[doc = "0.8 V."]
    #[inline(always)]
    pub fn v_0p800(self) -> &'a mut W {
        self.variant(VADJ_BOOST_A::V_0P800)
    }
    #[doc = "0.825 V."]
    #[inline(always)]
    pub fn v_0p825(self) -> &'a mut W {
        self.variant(VADJ_BOOST_A::V_0P825)
    }
    #[doc = "0.85 V."]
    #[inline(always)]
    pub fn v_0p850(self) -> &'a mut W {
        self.variant(VADJ_BOOST_A::V_0P850)
    }
    #[doc = "0.875 V."]
    #[inline(always)]
    pub fn v_0p875(self) -> &'a mut W {
        self.variant(VADJ_BOOST_A::V_0P875)
    }
    #[doc = "0.9 V."]
    #[inline(always)]
    pub fn v_0p900(self) -> &'a mut W {
        self.variant(VADJ_BOOST_A::V_0P900)
    }
    #[doc = "0.96 V."]
    #[inline(always)]
    pub fn v_0p960(self) -> &'a mut W {
        self.variant(VADJ_BOOST_A::V_0P960)
    }
    #[doc = "0.97 V."]
    #[inline(always)]
    pub fn v_0p970(self) -> &'a mut W {
        self.variant(VADJ_BOOST_A::V_0P970)
    }
    #[doc = "0.98 V."]
    #[inline(always)]
    pub fn v_0p980(self) -> &'a mut W {
        self.variant(VADJ_BOOST_A::V_0P980)
    }
    #[doc = "0.99 V."]
    #[inline(always)]
    pub fn v_0p990(self) -> &'a mut W {
        self.variant(VADJ_BOOST_A::V_0P990)
    }
    #[doc = "1 V."]
    #[inline(always)]
    pub fn v_1p000(self) -> &'a mut W {
        self.variant(VADJ_BOOST_A::V_1P000)
    }
    #[doc = "1.01 V."]
    #[inline(always)]
    pub fn v_1p010(self) -> &'a mut W {
        self.variant(VADJ_BOOST_A::V_1P010)
    }
    #[doc = "1.02 V."]
    #[inline(always)]
    pub fn v_1p020(self) -> &'a mut W {
        self.variant(VADJ_BOOST_A::V_1P020)
    }
    #[doc = "1.03 V."]
    #[inline(always)]
    pub fn v_1p030(self) -> &'a mut W {
        self.variant(VADJ_BOOST_A::V_1P030)
    }
    #[doc = "1.04 V."]
    #[inline(always)]
    pub fn v_1p040(self) -> &'a mut W {
        self.variant(VADJ_BOOST_A::V_1P040)
    }
    #[doc = "1.05 V."]
    #[inline(always)]
    pub fn v_1p050(self) -> &'a mut W {
        self.variant(VADJ_BOOST_A::V_1P050)
    }
    #[doc = "1.06 V."]
    #[inline(always)]
    pub fn v_1p060(self) -> &'a mut W {
        self.variant(VADJ_BOOST_A::V_1P060)
    }
    #[doc = "1.07 V."]
    #[inline(always)]
    pub fn v_1p070(self) -> &'a mut W {
        self.variant(VADJ_BOOST_A::V_1P070)
    }
    #[doc = "1.08 V."]
    #[inline(always)]
    pub fn v_1p080(self) -> &'a mut W {
        self.variant(VADJ_BOOST_A::V_1P080)
    }
    #[doc = "1.09 V."]
    #[inline(always)]
    pub fn v_1p090(self) -> &'a mut W {
        self.variant(VADJ_BOOST_A::V_1P090)
    }
    #[doc = "1.1 V."]
    #[inline(always)]
    pub fn v_1p100(self) -> &'a mut W {
        self.variant(VADJ_BOOST_A::V_1P100)
    }
    #[doc = "1.11 V."]
    #[inline(always)]
    pub fn v_1p110(self) -> &'a mut W {
        self.variant(VADJ_BOOST_A::V_1P110)
    }
    #[doc = "1.12 V."]
    #[inline(always)]
    pub fn v_1p120(self) -> &'a mut W {
        self.variant(VADJ_BOOST_A::V_1P120)
    }
    #[doc = "1.13 V."]
    #[inline(always)]
    pub fn v_1p130(self) -> &'a mut W {
        self.variant(VADJ_BOOST_A::V_1P130)
    }
    #[doc = "1.14 V."]
    #[inline(always)]
    pub fn v_1p140(self) -> &'a mut W {
        self.variant(VADJ_BOOST_A::V_1P140)
    }
    #[doc = "1.15 V."]
    #[inline(always)]
    pub fn v_1p150(self) -> &'a mut W {
        self.variant(VADJ_BOOST_A::V_1P150)
    }
    #[doc = "1.16 V."]
    #[inline(always)]
    pub fn v_1p160(self) -> &'a mut W {
        self.variant(VADJ_BOOST_A::V_1P160)
    }
    #[doc = "1.22 V."]
    #[inline(always)]
    pub fn v_1p220(self) -> &'a mut W {
        self.variant(VADJ_BOOST_A::V_1P220)
    }
}
impl R {
    #[doc = "Bits 0:4 - Sets the Always-On domain LDO output level."]
    #[inline(always)]
    pub fn vadj(&self) -> VADJ_R {
        VADJ_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 10:14 - Sets the Always-On domain LDO Boost output level."]
    #[inline(always)]
    pub fn vadj_boost(&self) -> VADJ_BOOST_R {
        VADJ_BOOST_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Sets the Always-On domain LDO output level."]
    #[inline(always)]
    pub fn vadj(&mut self) -> VADJ_W<0> {
        VADJ_W::new(self)
    }
    #[doc = "Bits 10:14 - Sets the Always-On domain LDO Boost output level."]
    #[inline(always)]
    pub fn vadj_boost(&mut self) -> VADJ_BOOST_W<10> {
        VADJ_BOOST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Power Management Unit (PMU) and Always-On domains LDO control \\[Reset by: PoR, Pin Reset, Brown Out Detectors Reset, Deep Power Down Reset, Software Reset\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ldopmu](index.html) module"]
pub struct LDOPMU_SPEC;
impl crate::RegisterSpec for LDOPMU_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ldopmu::R](R) reader structure"]
impl crate::Readable for LDOPMU_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ldopmu::W](W) writer structure"]
impl crate::Writable for LDOPMU_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LDOPMU to value 0x010e_f718"]
impl crate::Resettable for LDOPMU_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x010e_f718
    }
}
