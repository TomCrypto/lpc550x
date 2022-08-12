#[doc = "Register `LDO_XO32M` reader"]
pub struct R(crate::R<LDO_XO32M_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LDO_XO32M_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LDO_XO32M_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LDO_XO32M_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LDO_XO32M` writer"]
pub struct W(crate::W<LDO_XO32M_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LDO_XO32M_SPEC>;
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
impl From<crate::W<LDO_XO32M_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LDO_XO32M_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BYPASS` reader - Activate LDO bypass."]
pub type BYPASS_R = crate::BitReader<BYPASS_A>;
#[doc = "Activate LDO bypass.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BYPASS_A {
    #[doc = "0: Disable bypass mode (for normal operations)."]
    DISABLE = 0,
    #[doc = "1: Activate LDO bypass."]
    ENABLE = 1,
}
impl From<BYPASS_A> for bool {
    #[inline(always)]
    fn from(variant: BYPASS_A) -> Self {
        variant as u8 != 0
    }
}
impl BYPASS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BYPASS_A {
        match self.bits {
            false => BYPASS_A::DISABLE,
            true => BYPASS_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == BYPASS_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == BYPASS_A::ENABLE
    }
}
#[doc = "Field `BYPASS` writer - Activate LDO bypass."]
pub type BYPASS_W<'a, const O: u8> = crate::BitWriter<'a, u32, LDO_XO32M_SPEC, BYPASS_A, O>;
impl<'a, const O: u8> BYPASS_W<'a, O> {
    #[doc = "Disable bypass mode (for normal operations)."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(BYPASS_A::DISABLE)
    }
    #[doc = "Activate LDO bypass."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(BYPASS_A::ENABLE)
    }
}
#[doc = "Field `HIGHZ` reader - ."]
pub type HIGHZ_R = crate::BitReader<HIGHZ_A>;
#[doc = ".\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HIGHZ_A {
    #[doc = "0: Output in High normal state."]
    NORMALMPEDANCE = 0,
    #[doc = "1: Output in High Impedance state."]
    HIGHIMPEDANCE = 1,
}
impl From<HIGHZ_A> for bool {
    #[inline(always)]
    fn from(variant: HIGHZ_A) -> Self {
        variant as u8 != 0
    }
}
impl HIGHZ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HIGHZ_A {
        match self.bits {
            false => HIGHZ_A::NORMALMPEDANCE,
            true => HIGHZ_A::HIGHIMPEDANCE,
        }
    }
    #[doc = "Checks if the value of the field is `NORMALMPEDANCE`"]
    #[inline(always)]
    pub fn is_normalmpedance(&self) -> bool {
        *self == HIGHZ_A::NORMALMPEDANCE
    }
    #[doc = "Checks if the value of the field is `HIGHIMPEDANCE`"]
    #[inline(always)]
    pub fn is_highimpedance(&self) -> bool {
        *self == HIGHZ_A::HIGHIMPEDANCE
    }
}
#[doc = "Field `HIGHZ` writer - ."]
pub type HIGHZ_W<'a, const O: u8> = crate::BitWriter<'a, u32, LDO_XO32M_SPEC, HIGHZ_A, O>;
impl<'a, const O: u8> HIGHZ_W<'a, O> {
    #[doc = "Output in High normal state."]
    #[inline(always)]
    pub fn normalmpedance(self) -> &'a mut W {
        self.variant(HIGHZ_A::NORMALMPEDANCE)
    }
    #[doc = "Output in High Impedance state."]
    #[inline(always)]
    pub fn highimpedance(self) -> &'a mut W {
        self.variant(HIGHZ_A::HIGHIMPEDANCE)
    }
}
#[doc = "Field `VOUT` reader - Sets the LDO output level."]
pub type VOUT_R = crate::FieldReader<u8, VOUT_A>;
#[doc = "Sets the LDO output level.\n\nValue on reset: 4"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum VOUT_A {
    #[doc = "0: 0.750 V."]
    V_0P750 = 0,
    #[doc = "1: 0.775 V."]
    V_0P775 = 1,
    #[doc = "2: 0.800 V."]
    V_0P800 = 2,
    #[doc = "3: 0.825 V."]
    V_0P825 = 3,
    #[doc = "4: 0.850 V."]
    V_0P850 = 4,
    #[doc = "5: 0.875 V."]
    V_0P875 = 5,
    #[doc = "6: 0.900 V."]
    V_0P900 = 6,
    #[doc = "7: 0.925 V."]
    V_0P925 = 7,
}
impl From<VOUT_A> for u8 {
    #[inline(always)]
    fn from(variant: VOUT_A) -> Self {
        variant as _
    }
}
impl VOUT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VOUT_A {
        match self.bits {
            0 => VOUT_A::V_0P750,
            1 => VOUT_A::V_0P775,
            2 => VOUT_A::V_0P800,
            3 => VOUT_A::V_0P825,
            4 => VOUT_A::V_0P850,
            5 => VOUT_A::V_0P875,
            6 => VOUT_A::V_0P900,
            7 => VOUT_A::V_0P925,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `V_0P750`"]
    #[inline(always)]
    pub fn is_v_0p750(&self) -> bool {
        *self == VOUT_A::V_0P750
    }
    #[doc = "Checks if the value of the field is `V_0P775`"]
    #[inline(always)]
    pub fn is_v_0p775(&self) -> bool {
        *self == VOUT_A::V_0P775
    }
    #[doc = "Checks if the value of the field is `V_0P800`"]
    #[inline(always)]
    pub fn is_v_0p800(&self) -> bool {
        *self == VOUT_A::V_0P800
    }
    #[doc = "Checks if the value of the field is `V_0P825`"]
    #[inline(always)]
    pub fn is_v_0p825(&self) -> bool {
        *self == VOUT_A::V_0P825
    }
    #[doc = "Checks if the value of the field is `V_0P850`"]
    #[inline(always)]
    pub fn is_v_0p850(&self) -> bool {
        *self == VOUT_A::V_0P850
    }
    #[doc = "Checks if the value of the field is `V_0P875`"]
    #[inline(always)]
    pub fn is_v_0p875(&self) -> bool {
        *self == VOUT_A::V_0P875
    }
    #[doc = "Checks if the value of the field is `V_0P900`"]
    #[inline(always)]
    pub fn is_v_0p900(&self) -> bool {
        *self == VOUT_A::V_0P900
    }
    #[doc = "Checks if the value of the field is `V_0P925`"]
    #[inline(always)]
    pub fn is_v_0p925(&self) -> bool {
        *self == VOUT_A::V_0P925
    }
}
#[doc = "Field `VOUT` writer - Sets the LDO output level."]
pub type VOUT_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, LDO_XO32M_SPEC, u8, VOUT_A, 3, O>;
impl<'a, const O: u8> VOUT_W<'a, O> {
    #[doc = "0.750 V."]
    #[inline(always)]
    pub fn v_0p750(self) -> &'a mut W {
        self.variant(VOUT_A::V_0P750)
    }
    #[doc = "0.775 V."]
    #[inline(always)]
    pub fn v_0p775(self) -> &'a mut W {
        self.variant(VOUT_A::V_0P775)
    }
    #[doc = "0.800 V."]
    #[inline(always)]
    pub fn v_0p800(self) -> &'a mut W {
        self.variant(VOUT_A::V_0P800)
    }
    #[doc = "0.825 V."]
    #[inline(always)]
    pub fn v_0p825(self) -> &'a mut W {
        self.variant(VOUT_A::V_0P825)
    }
    #[doc = "0.850 V."]
    #[inline(always)]
    pub fn v_0p850(self) -> &'a mut W {
        self.variant(VOUT_A::V_0P850)
    }
    #[doc = "0.875 V."]
    #[inline(always)]
    pub fn v_0p875(self) -> &'a mut W {
        self.variant(VOUT_A::V_0P875)
    }
    #[doc = "0.900 V."]
    #[inline(always)]
    pub fn v_0p900(self) -> &'a mut W {
        self.variant(VOUT_A::V_0P900)
    }
    #[doc = "0.925 V."]
    #[inline(always)]
    pub fn v_0p925(self) -> &'a mut W {
        self.variant(VOUT_A::V_0P925)
    }
}
#[doc = "Field `IBIAS` reader - Adjust the biasing current."]
pub type IBIAS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IBIAS` writer - Adjust the biasing current."]
pub type IBIAS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LDO_XO32M_SPEC, u8, u8, 2, O>;
#[doc = "Field `STABMODE` reader - Stability configuration."]
pub type STABMODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `STABMODE` writer - Stability configuration."]
pub type STABMODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LDO_XO32M_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bit 1 - Activate LDO bypass."]
    #[inline(always)]
    pub fn bypass(&self) -> BYPASS_R {
        BYPASS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ."]
    #[inline(always)]
    pub fn highz(&self) -> HIGHZ_R {
        HIGHZ_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:5 - Sets the LDO output level."]
    #[inline(always)]
    pub fn vout(&self) -> VOUT_R {
        VOUT_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:7 - Adjust the biasing current."]
    #[inline(always)]
    pub fn ibias(&self) -> IBIAS_R {
        IBIAS_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Stability configuration."]
    #[inline(always)]
    pub fn stabmode(&self) -> STABMODE_R {
        STABMODE_R::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - Activate LDO bypass."]
    #[inline(always)]
    pub fn bypass(&mut self) -> BYPASS_W<1> {
        BYPASS_W::new(self)
    }
    #[doc = "Bit 2 - ."]
    #[inline(always)]
    pub fn highz(&mut self) -> HIGHZ_W<2> {
        HIGHZ_W::new(self)
    }
    #[doc = "Bits 3:5 - Sets the LDO output level."]
    #[inline(always)]
    pub fn vout(&mut self) -> VOUT_W<3> {
        VOUT_W::new(self)
    }
    #[doc = "Bits 6:7 - Adjust the biasing current."]
    #[inline(always)]
    pub fn ibias(&mut self) -> IBIAS_W<6> {
        IBIAS_W::new(self)
    }
    #[doc = "Bits 8:9 - Stability configuration."]
    #[inline(always)]
    pub fn stabmode(&mut self) -> STABMODE_W<8> {
        STABMODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "High Speed Crystal Oscillator (12 MHz - 32 MHz) Voltage Source Supply Control register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ldo_xo32m](index.html) module"]
pub struct LDO_XO32M_SPEC;
impl crate::RegisterSpec for LDO_XO32M_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ldo_xo32m::R](R) reader structure"]
impl crate::Readable for LDO_XO32M_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ldo_xo32m::W](W) writer structure"]
impl crate::Writable for LDO_XO32M_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LDO_XO32M to value 0x03a0"]
impl crate::Resettable for LDO_XO32M_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x03a0
    }
}
