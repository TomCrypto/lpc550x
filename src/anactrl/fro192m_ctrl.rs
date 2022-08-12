#[doc = "Register `FRO192M_CTRL` reader"]
pub struct R(crate::R<FRO192M_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FRO192M_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FRO192M_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FRO192M_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FRO192M_CTRL` writer"]
pub struct W(crate::W<FRO192M_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FRO192M_CTRL_SPEC>;
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
impl From<crate::W<FRO192M_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FRO192M_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BIAS_TRIM` reader - Bias trimming bits (course frequency trimming)."]
pub type BIAS_TRIM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BIAS_TRIM` writer - Bias trimming bits (course frequency trimming)."]
pub type BIAS_TRIM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FRO192M_CTRL_SPEC, u8, u8, 6, O>;
#[doc = "Field `TEMP_TRIM` reader - Temperature coefficient trimming bits."]
pub type TEMP_TRIM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TEMP_TRIM` writer - Temperature coefficient trimming bits."]
pub type TEMP_TRIM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FRO192M_CTRL_SPEC, u8, u8, 7, O>;
#[doc = "Field `ENA_12MHZCLK` reader - 12 MHz clock control."]
pub type ENA_12MHZCLK_R = crate::BitReader<ENA_12MHZCLK_A>;
#[doc = "12 MHz clock control.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENA_12MHZCLK_A {
    #[doc = "0: 12 MHz clock is disabled."]
    DISABLE = 0,
    #[doc = "1: 12 MHz clock is enabled."]
    ENABLE = 1,
}
impl From<ENA_12MHZCLK_A> for bool {
    #[inline(always)]
    fn from(variant: ENA_12MHZCLK_A) -> Self {
        variant as u8 != 0
    }
}
impl ENA_12MHZCLK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENA_12MHZCLK_A {
        match self.bits {
            false => ENA_12MHZCLK_A::DISABLE,
            true => ENA_12MHZCLK_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ENA_12MHZCLK_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ENA_12MHZCLK_A::ENABLE
    }
}
#[doc = "Field `ENA_12MHZCLK` writer - 12 MHz clock control."]
pub type ENA_12MHZCLK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FRO192M_CTRL_SPEC, ENA_12MHZCLK_A, O>;
impl<'a, const O: u8> ENA_12MHZCLK_W<'a, O> {
    #[doc = "12 MHz clock is disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ENA_12MHZCLK_A::DISABLE)
    }
    #[doc = "12 MHz clock is enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ENA_12MHZCLK_A::ENABLE)
    }
}
#[doc = "Field `DAC_TRIM` reader - Frequency trim."]
pub type DAC_TRIM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DAC_TRIM` writer - Frequency trim."]
pub type DAC_TRIM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FRO192M_CTRL_SPEC, u8, u8, 8, O>;
#[doc = "Field `ENA_96MHZCLK` reader - 96 MHz clock control."]
pub type ENA_96MHZCLK_R = crate::BitReader<ENA_96MHZCLK_A>;
#[doc = "96 MHz clock control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENA_96MHZCLK_A {
    #[doc = "0: 96 MHz clock is disabled."]
    DISABLE = 0,
    #[doc = "1: 96 MHz clock is enabled."]
    ENABLE = 1,
}
impl From<ENA_96MHZCLK_A> for bool {
    #[inline(always)]
    fn from(variant: ENA_96MHZCLK_A) -> Self {
        variant as u8 != 0
    }
}
impl ENA_96MHZCLK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENA_96MHZCLK_A {
        match self.bits {
            false => ENA_96MHZCLK_A::DISABLE,
            true => ENA_96MHZCLK_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ENA_96MHZCLK_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ENA_96MHZCLK_A::ENABLE
    }
}
#[doc = "Field `ENA_96MHZCLK` writer - 96 MHz clock control."]
pub type ENA_96MHZCLK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FRO192M_CTRL_SPEC, ENA_96MHZCLK_A, O>;
impl<'a, const O: u8> ENA_96MHZCLK_W<'a, O> {
    #[doc = "96 MHz clock is disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ENA_96MHZCLK_A::DISABLE)
    }
    #[doc = "96 MHz clock is enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ENA_96MHZCLK_A::ENABLE)
    }
}
#[doc = "Field `WRTRIM` writer - This must be written to 1 to modify the BIAS_TRIM and TEMP_TRIM fields."]
pub type WRTRIM_W<'a, const O: u8> = crate::BitWriter<'a, u32, FRO192M_CTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:5 - Bias trimming bits (course frequency trimming)."]
    #[inline(always)]
    pub fn bias_trim(&self) -> BIAS_TRIM_R {
        BIAS_TRIM_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 7:13 - Temperature coefficient trimming bits."]
    #[inline(always)]
    pub fn temp_trim(&self) -> TEMP_TRIM_R {
        TEMP_TRIM_R::new(((self.bits >> 7) & 0x7f) as u8)
    }
    #[doc = "Bit 14 - 12 MHz clock control."]
    #[inline(always)]
    pub fn ena_12mhzclk(&self) -> ENA_12MHZCLK_R {
        ENA_12MHZCLK_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 16:23 - Frequency trim."]
    #[inline(always)]
    pub fn dac_trim(&self) -> DAC_TRIM_R {
        DAC_TRIM_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 30 - 96 MHz clock control."]
    #[inline(always)]
    pub fn ena_96mhzclk(&self) -> ENA_96MHZCLK_R {
        ENA_96MHZCLK_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - Bias trimming bits (course frequency trimming)."]
    #[inline(always)]
    pub fn bias_trim(&mut self) -> BIAS_TRIM_W<0> {
        BIAS_TRIM_W::new(self)
    }
    #[doc = "Bits 7:13 - Temperature coefficient trimming bits."]
    #[inline(always)]
    pub fn temp_trim(&mut self) -> TEMP_TRIM_W<7> {
        TEMP_TRIM_W::new(self)
    }
    #[doc = "Bit 14 - 12 MHz clock control."]
    #[inline(always)]
    pub fn ena_12mhzclk(&mut self) -> ENA_12MHZCLK_W<14> {
        ENA_12MHZCLK_W::new(self)
    }
    #[doc = "Bits 16:23 - Frequency trim."]
    #[inline(always)]
    pub fn dac_trim(&mut self) -> DAC_TRIM_W<16> {
        DAC_TRIM_W::new(self)
    }
    #[doc = "Bit 30 - 96 MHz clock control."]
    #[inline(always)]
    pub fn ena_96mhzclk(&mut self) -> ENA_96MHZCLK_W<30> {
        ENA_96MHZCLK_W::new(self)
    }
    #[doc = "Bit 31 - This must be written to 1 to modify the BIAS_TRIM and TEMP_TRIM fields."]
    #[inline(always)]
    pub fn wrtrim(&mut self) -> WRTRIM_W<31> {
        WRTRIM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "192MHz Free Running OScillator (FRO) Control register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fro192m_ctrl](index.html) module"]
pub struct FRO192M_CTRL_SPEC;
impl crate::RegisterSpec for FRO192M_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fro192m_ctrl::R](R) reader structure"]
impl crate::Readable for FRO192M_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fro192m_ctrl::W](W) writer structure"]
impl crate::Writable for FRO192M_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FRO192M_CTRL to value 0x0080_d01a"]
impl crate::Resettable for FRO192M_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0080_d01a
    }
}
