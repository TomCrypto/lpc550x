#[doc = "Register `DCDC0` reader"]
pub struct R(crate::R<DCDC0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DCDC0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DCDC0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DCDC0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DCDC0` writer"]
pub struct W(crate::W<DCDC0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DCDC0_SPEC>;
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
impl From<crate::W<DCDC0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DCDC0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CONFIG0` reader - DCDC configuration."]
pub type CONFIG0_R = crate::FieldReader<u32, u32>;
#[doc = "Field `CONFIG0` writer - DCDC configuration."]
pub type CONFIG0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DCDC0_SPEC, u32, u32, 17, O>;
#[doc = "Field `VOUT` reader - Set output regulation voltage."]
pub type VOUT_R = crate::FieldReader<u8, VOUT_A>;
#[doc = "Set output regulation voltage.\n\nValue on reset: 6"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum VOUT_A {
    #[doc = "0: 0.95 V."]
    V_DCDC_0P950 = 0,
    #[doc = "1: 0.975 V."]
    V_DCDC_0P975 = 1,
    #[doc = "2: 1 V."]
    V_DCDC_1P000 = 2,
    #[doc = "3: 1.025 V."]
    V_DCDC_1P025 = 3,
    #[doc = "4: 1.05 V."]
    V_DCDC_1P050 = 4,
    #[doc = "5: 1.075 V."]
    V_DCDC_1P075 = 5,
    #[doc = "6: 1.1 V."]
    V_DCDC_1P100 = 6,
    #[doc = "7: 1.125 V."]
    V_DCDC_1P125 = 7,
    #[doc = "8: 1.15 V."]
    V_DCDC_1P150 = 8,
    #[doc = "9: 1.175 V."]
    V_DCDC_1P175 = 9,
    #[doc = "10: 1.2 V."]
    V_DCDC_1P200 = 10,
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
    pub fn variant(&self) -> Option<VOUT_A> {
        match self.bits {
            0 => Some(VOUT_A::V_DCDC_0P950),
            1 => Some(VOUT_A::V_DCDC_0P975),
            2 => Some(VOUT_A::V_DCDC_1P000),
            3 => Some(VOUT_A::V_DCDC_1P025),
            4 => Some(VOUT_A::V_DCDC_1P050),
            5 => Some(VOUT_A::V_DCDC_1P075),
            6 => Some(VOUT_A::V_DCDC_1P100),
            7 => Some(VOUT_A::V_DCDC_1P125),
            8 => Some(VOUT_A::V_DCDC_1P150),
            9 => Some(VOUT_A::V_DCDC_1P175),
            10 => Some(VOUT_A::V_DCDC_1P200),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `V_DCDC_0P950`"]
    #[inline(always)]
    pub fn is_v_dcdc_0p950(&self) -> bool {
        *self == VOUT_A::V_DCDC_0P950
    }
    #[doc = "Checks if the value of the field is `V_DCDC_0P975`"]
    #[inline(always)]
    pub fn is_v_dcdc_0p975(&self) -> bool {
        *self == VOUT_A::V_DCDC_0P975
    }
    #[doc = "Checks if the value of the field is `V_DCDC_1P000`"]
    #[inline(always)]
    pub fn is_v_dcdc_1p000(&self) -> bool {
        *self == VOUT_A::V_DCDC_1P000
    }
    #[doc = "Checks if the value of the field is `V_DCDC_1P025`"]
    #[inline(always)]
    pub fn is_v_dcdc_1p025(&self) -> bool {
        *self == VOUT_A::V_DCDC_1P025
    }
    #[doc = "Checks if the value of the field is `V_DCDC_1P050`"]
    #[inline(always)]
    pub fn is_v_dcdc_1p050(&self) -> bool {
        *self == VOUT_A::V_DCDC_1P050
    }
    #[doc = "Checks if the value of the field is `V_DCDC_1P075`"]
    #[inline(always)]
    pub fn is_v_dcdc_1p075(&self) -> bool {
        *self == VOUT_A::V_DCDC_1P075
    }
    #[doc = "Checks if the value of the field is `V_DCDC_1P100`"]
    #[inline(always)]
    pub fn is_v_dcdc_1p100(&self) -> bool {
        *self == VOUT_A::V_DCDC_1P100
    }
    #[doc = "Checks if the value of the field is `V_DCDC_1P125`"]
    #[inline(always)]
    pub fn is_v_dcdc_1p125(&self) -> bool {
        *self == VOUT_A::V_DCDC_1P125
    }
    #[doc = "Checks if the value of the field is `V_DCDC_1P150`"]
    #[inline(always)]
    pub fn is_v_dcdc_1p150(&self) -> bool {
        *self == VOUT_A::V_DCDC_1P150
    }
    #[doc = "Checks if the value of the field is `V_DCDC_1P175`"]
    #[inline(always)]
    pub fn is_v_dcdc_1p175(&self) -> bool {
        *self == VOUT_A::V_DCDC_1P175
    }
    #[doc = "Checks if the value of the field is `V_DCDC_1P200`"]
    #[inline(always)]
    pub fn is_v_dcdc_1p200(&self) -> bool {
        *self == VOUT_A::V_DCDC_1P200
    }
}
#[doc = "Field `VOUT` writer - Set output regulation voltage."]
pub type VOUT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DCDC0_SPEC, u8, VOUT_A, 4, O>;
impl<'a, const O: u8> VOUT_W<'a, O> {
    #[doc = "0.95 V."]
    #[inline(always)]
    pub fn v_dcdc_0p950(self) -> &'a mut W {
        self.variant(VOUT_A::V_DCDC_0P950)
    }
    #[doc = "0.975 V."]
    #[inline(always)]
    pub fn v_dcdc_0p975(self) -> &'a mut W {
        self.variant(VOUT_A::V_DCDC_0P975)
    }
    #[doc = "1 V."]
    #[inline(always)]
    pub fn v_dcdc_1p000(self) -> &'a mut W {
        self.variant(VOUT_A::V_DCDC_1P000)
    }
    #[doc = "1.025 V."]
    #[inline(always)]
    pub fn v_dcdc_1p025(self) -> &'a mut W {
        self.variant(VOUT_A::V_DCDC_1P025)
    }
    #[doc = "1.05 V."]
    #[inline(always)]
    pub fn v_dcdc_1p050(self) -> &'a mut W {
        self.variant(VOUT_A::V_DCDC_1P050)
    }
    #[doc = "1.075 V."]
    #[inline(always)]
    pub fn v_dcdc_1p075(self) -> &'a mut W {
        self.variant(VOUT_A::V_DCDC_1P075)
    }
    #[doc = "1.1 V."]
    #[inline(always)]
    pub fn v_dcdc_1p100(self) -> &'a mut W {
        self.variant(VOUT_A::V_DCDC_1P100)
    }
    #[doc = "1.125 V."]
    #[inline(always)]
    pub fn v_dcdc_1p125(self) -> &'a mut W {
        self.variant(VOUT_A::V_DCDC_1P125)
    }
    #[doc = "1.15 V."]
    #[inline(always)]
    pub fn v_dcdc_1p150(self) -> &'a mut W {
        self.variant(VOUT_A::V_DCDC_1P150)
    }
    #[doc = "1.175 V."]
    #[inline(always)]
    pub fn v_dcdc_1p175(self) -> &'a mut W {
        self.variant(VOUT_A::V_DCDC_1P175)
    }
    #[doc = "1.2 V."]
    #[inline(always)]
    pub fn v_dcdc_1p200(self) -> &'a mut W {
        self.variant(VOUT_A::V_DCDC_1P200)
    }
}
#[doc = "Field `CONFIG1` reader - DCDC configuration."]
pub type CONFIG1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CONFIG1` writer - DCDC configuration."]
pub type CONFIG1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DCDC0_SPEC, u8, u8, 6, O>;
impl R {
    #[doc = "Bits 0:16 - DCDC configuration."]
    #[inline(always)]
    pub fn config0(&self) -> CONFIG0_R {
        CONFIG0_R::new((self.bits & 0x0001_ffff) as u32)
    }
    #[doc = "Bits 17:20 - Set output regulation voltage."]
    #[inline(always)]
    pub fn vout(&self) -> VOUT_R {
        VOUT_R::new(((self.bits >> 17) & 0x0f) as u8)
    }
    #[doc = "Bits 21:26 - DCDC configuration."]
    #[inline(always)]
    pub fn config1(&self) -> CONFIG1_R {
        CONFIG1_R::new(((self.bits >> 21) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:16 - DCDC configuration."]
    #[inline(always)]
    pub fn config0(&mut self) -> CONFIG0_W<0> {
        CONFIG0_W::new(self)
    }
    #[doc = "Bits 17:20 - Set output regulation voltage."]
    #[inline(always)]
    pub fn vout(&mut self) -> VOUT_W<17> {
        VOUT_W::new(self)
    }
    #[doc = "Bits 21:26 - DCDC configuration."]
    #[inline(always)]
    pub fn config1(&mut self) -> CONFIG1_W<21> {
        CONFIG1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DCDC (first) control register \\[Reset by: PoR, Pin Reset, Brown Out Detectors Reset, Deep Power Down Reset, Software Reset\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcdc0](index.html) module"]
pub struct DCDC0_SPEC;
impl crate::RegisterSpec for DCDC0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dcdc0::R](R) reader structure"]
impl crate::Readable for DCDC0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dcdc0::W](W) writer structure"]
impl crate::Writable for DCDC0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DCDC0 to value 0x010c_4e68"]
impl crate::Resettable for DCDC0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x010c_4e68
    }
}
