#[doc = "Register `AUX_BIAS` reader"]
pub struct R(crate::R<AUX_BIAS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AUX_BIAS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AUX_BIAS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AUX_BIAS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AUX_BIAS` writer"]
pub struct W(crate::W<AUX_BIAS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AUX_BIAS_SPEC>;
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
impl From<crate::W<AUX_BIAS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AUX_BIAS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VREF1VENABLE` reader - Control output of 1V reference voltage."]
pub type VREF1VENABLE_R = crate::BitReader<VREF1VENABLE_A>;
#[doc = "Control output of 1V reference voltage.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VREF1VENABLE_A {
    #[doc = "0: Output of 1V reference voltage buffer is bypassed."]
    DISABLE = 0,
    #[doc = "1: Output of 1V reference voltage is enabled."]
    ENABLE = 1,
}
impl From<VREF1VENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: VREF1VENABLE_A) -> Self {
        variant as u8 != 0
    }
}
impl VREF1VENABLE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VREF1VENABLE_A {
        match self.bits {
            false => VREF1VENABLE_A::DISABLE,
            true => VREF1VENABLE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == VREF1VENABLE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == VREF1VENABLE_A::ENABLE
    }
}
#[doc = "Field `VREF1VENABLE` writer - Control output of 1V reference voltage."]
pub type VREF1VENABLE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AUX_BIAS_SPEC, VREF1VENABLE_A, O>;
impl<'a, const O: u8> VREF1VENABLE_W<'a, O> {
    #[doc = "Output of 1V reference voltage buffer is bypassed."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(VREF1VENABLE_A::DISABLE)
    }
    #[doc = "Output of 1V reference voltage is enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(VREF1VENABLE_A::ENABLE)
    }
}
#[doc = "Field `ITRIM` reader - current trimming control word."]
pub type ITRIM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ITRIM` writer - current trimming control word."]
pub type ITRIM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, AUX_BIAS_SPEC, u8, u8, 5, O>;
#[doc = "Field `PTATITRIM` reader - current trimming control word for ptat current."]
pub type PTATITRIM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PTATITRIM` writer - current trimming control word for ptat current."]
pub type PTATITRIM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, AUX_BIAS_SPEC, u8, u8, 5, O>;
#[doc = "Field `VREF1VTRIM` reader - voltage trimming control word."]
pub type VREF1VTRIM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VREF1VTRIM` writer - voltage trimming control word."]
pub type VREF1VTRIM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, AUX_BIAS_SPEC, u8, u8, 5, O>;
#[doc = "Field `VREF1VCURVETRIM` reader - Control bit to configure trimming state of mirror."]
pub type VREF1VCURVETRIM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VREF1VCURVETRIM` writer - Control bit to configure trimming state of mirror."]
pub type VREF1VCURVETRIM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, AUX_BIAS_SPEC, u8, u8, 3, O>;
#[doc = "Field `ITRIMCTRL0` reader - Control bit to configure trimming state of mirror."]
pub type ITRIMCTRL0_R = crate::BitReader<bool>;
#[doc = "Field `ITRIMCTRL0` writer - Control bit to configure trimming state of mirror."]
pub type ITRIMCTRL0_W<'a, const O: u8> = crate::BitWriter<'a, u32, AUX_BIAS_SPEC, bool, O>;
#[doc = "Field `ITRIMCTRL1` reader - Control bit to configure trimming state of mirror."]
pub type ITRIMCTRL1_R = crate::BitReader<bool>;
#[doc = "Field `ITRIMCTRL1` writer - Control bit to configure trimming state of mirror."]
pub type ITRIMCTRL1_W<'a, const O: u8> = crate::BitWriter<'a, u32, AUX_BIAS_SPEC, bool, O>;
impl R {
    #[doc = "Bit 1 - Control output of 1V reference voltage."]
    #[inline(always)]
    pub fn vref1venable(&self) -> VREF1VENABLE_R {
        VREF1VENABLE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:6 - current trimming control word."]
    #[inline(always)]
    pub fn itrim(&self) -> ITRIM_R {
        ITRIM_R::new(((self.bits >> 2) & 0x1f) as u8)
    }
    #[doc = "Bits 7:11 - current trimming control word for ptat current."]
    #[inline(always)]
    pub fn ptatitrim(&self) -> PTATITRIM_R {
        PTATITRIM_R::new(((self.bits >> 7) & 0x1f) as u8)
    }
    #[doc = "Bits 12:16 - voltage trimming control word."]
    #[inline(always)]
    pub fn vref1vtrim(&self) -> VREF1VTRIM_R {
        VREF1VTRIM_R::new(((self.bits >> 12) & 0x1f) as u8)
    }
    #[doc = "Bits 17:19 - Control bit to configure trimming state of mirror."]
    #[inline(always)]
    pub fn vref1vcurvetrim(&self) -> VREF1VCURVETRIM_R {
        VREF1VCURVETRIM_R::new(((self.bits >> 17) & 7) as u8)
    }
    #[doc = "Bit 20 - Control bit to configure trimming state of mirror."]
    #[inline(always)]
    pub fn itrimctrl0(&self) -> ITRIMCTRL0_R {
        ITRIMCTRL0_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Control bit to configure trimming state of mirror."]
    #[inline(always)]
    pub fn itrimctrl1(&self) -> ITRIMCTRL1_R {
        ITRIMCTRL1_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Control output of 1V reference voltage."]
    #[inline(always)]
    pub fn vref1venable(&mut self) -> VREF1VENABLE_W<1> {
        VREF1VENABLE_W::new(self)
    }
    #[doc = "Bits 2:6 - current trimming control word."]
    #[inline(always)]
    pub fn itrim(&mut self) -> ITRIM_W<2> {
        ITRIM_W::new(self)
    }
    #[doc = "Bits 7:11 - current trimming control word for ptat current."]
    #[inline(always)]
    pub fn ptatitrim(&mut self) -> PTATITRIM_W<7> {
        PTATITRIM_W::new(self)
    }
    #[doc = "Bits 12:16 - voltage trimming control word."]
    #[inline(always)]
    pub fn vref1vtrim(&mut self) -> VREF1VTRIM_W<12> {
        VREF1VTRIM_W::new(self)
    }
    #[doc = "Bits 17:19 - Control bit to configure trimming state of mirror."]
    #[inline(always)]
    pub fn vref1vcurvetrim(&mut self) -> VREF1VCURVETRIM_W<17> {
        VREF1VCURVETRIM_W::new(self)
    }
    #[doc = "Bit 20 - Control bit to configure trimming state of mirror."]
    #[inline(always)]
    pub fn itrimctrl0(&mut self) -> ITRIMCTRL0_W<20> {
        ITRIMCTRL0_W::new(self)
    }
    #[doc = "Bit 21 - Control bit to configure trimming state of mirror."]
    #[inline(always)]
    pub fn itrimctrl1(&mut self) -> ITRIMCTRL1_W<21> {
        ITRIMCTRL1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AUX_BIAS.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aux_bias](index.html) module"]
pub struct AUX_BIAS_SPEC;
impl crate::RegisterSpec for AUX_BIAS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [aux_bias::R](R) reader structure"]
impl crate::Readable for AUX_BIAS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [aux_bias::W](W) writer structure"]
impl crate::Writable for AUX_BIAS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AUX_BIAS to value 0x0007_03a0"]
impl crate::Resettable for AUX_BIAS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0007_03a0
    }
}
