#[doc = "Register `FCCLKSEL7` reader"]
pub struct R(crate::R<FCCLKSEL7_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FCCLKSEL7_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FCCLKSEL7_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FCCLKSEL7_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FCCLKSEL7` writer"]
pub struct W(crate::W<FCCLKSEL7_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FCCLKSEL7_SPEC>;
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
impl From<crate::W<FCCLKSEL7_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FCCLKSEL7_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEL` reader - Flexcomm Interface 7 clock source select for Fractional Rate Divider."]
pub type SEL_R = crate::FieldReader<u8, SEL_A>;
#[doc = "Flexcomm Interface 7 clock source select for Fractional Rate Divider.\n\nValue on reset: 7"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SEL_A {
    #[doc = "0: Main clock."]
    MAIN_CLK = 0,
    #[doc = "1: PLL0 divided clock."]
    PLL0_DIV = 1,
    #[doc = "2: FRO 12 MHz clock."]
    FRO_12MHZ = 2,
    #[doc = "3: FRO HF divided clock."]
    FRO_HF_DIV = 3,
    #[doc = "4: FRO 1MHz clock."]
    FRO_1MHZ = 4,
    #[doc = "5: MCLK clock."]
    MCLK = 5,
    #[doc = "6: Oscillator 32 kHz clock."]
    OSC_32KHZ = 6,
    #[doc = "7: No clock."]
    NONE = 7,
}
impl From<SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: SEL_A) -> Self {
        variant as _
    }
}
impl SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SEL_A {
        match self.bits {
            0 => SEL_A::MAIN_CLK,
            1 => SEL_A::PLL0_DIV,
            2 => SEL_A::FRO_12MHZ,
            3 => SEL_A::FRO_HF_DIV,
            4 => SEL_A::FRO_1MHZ,
            5 => SEL_A::MCLK,
            6 => SEL_A::OSC_32KHZ,
            7 => SEL_A::NONE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `MAIN_CLK`"]
    #[inline(always)]
    pub fn is_main_clk(&self) -> bool {
        *self == SEL_A::MAIN_CLK
    }
    #[doc = "Checks if the value of the field is `PLL0_DIV`"]
    #[inline(always)]
    pub fn is_pll0_div(&self) -> bool {
        *self == SEL_A::PLL0_DIV
    }
    #[doc = "Checks if the value of the field is `FRO_12MHZ`"]
    #[inline(always)]
    pub fn is_fro_12mhz(&self) -> bool {
        *self == SEL_A::FRO_12MHZ
    }
    #[doc = "Checks if the value of the field is `FRO_HF_DIV`"]
    #[inline(always)]
    pub fn is_fro_hf_div(&self) -> bool {
        *self == SEL_A::FRO_HF_DIV
    }
    #[doc = "Checks if the value of the field is `FRO_1MHZ`"]
    #[inline(always)]
    pub fn is_fro_1mhz(&self) -> bool {
        *self == SEL_A::FRO_1MHZ
    }
    #[doc = "Checks if the value of the field is `MCLK`"]
    #[inline(always)]
    pub fn is_mclk(&self) -> bool {
        *self == SEL_A::MCLK
    }
    #[doc = "Checks if the value of the field is `OSC_32KHZ`"]
    #[inline(always)]
    pub fn is_osc_32khz(&self) -> bool {
        *self == SEL_A::OSC_32KHZ
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == SEL_A::NONE
    }
}
#[doc = "Field `SEL` writer - Flexcomm Interface 7 clock source select for Fractional Rate Divider."]
pub type SEL_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, FCCLKSEL7_SPEC, u8, SEL_A, 3, O>;
impl<'a, const O: u8> SEL_W<'a, O> {
    #[doc = "Main clock."]
    #[inline(always)]
    pub fn main_clk(self) -> &'a mut W {
        self.variant(SEL_A::MAIN_CLK)
    }
    #[doc = "PLL0 divided clock."]
    #[inline(always)]
    pub fn pll0_div(self) -> &'a mut W {
        self.variant(SEL_A::PLL0_DIV)
    }
    #[doc = "FRO 12 MHz clock."]
    #[inline(always)]
    pub fn fro_12mhz(self) -> &'a mut W {
        self.variant(SEL_A::FRO_12MHZ)
    }
    #[doc = "FRO HF divided clock."]
    #[inline(always)]
    pub fn fro_hf_div(self) -> &'a mut W {
        self.variant(SEL_A::FRO_HF_DIV)
    }
    #[doc = "FRO 1MHz clock."]
    #[inline(always)]
    pub fn fro_1mhz(self) -> &'a mut W {
        self.variant(SEL_A::FRO_1MHZ)
    }
    #[doc = "MCLK clock."]
    #[inline(always)]
    pub fn mclk(self) -> &'a mut W {
        self.variant(SEL_A::MCLK)
    }
    #[doc = "Oscillator 32 kHz clock."]
    #[inline(always)]
    pub fn osc_32khz(self) -> &'a mut W {
        self.variant(SEL_A::OSC_32KHZ)
    }
    #[doc = "No clock."]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(SEL_A::NONE)
    }
}
impl R {
    #[doc = "Bits 0:2 - Flexcomm Interface 7 clock source select for Fractional Rate Divider."]
    #[inline(always)]
    pub fn sel(&self) -> SEL_R {
        SEL_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Flexcomm Interface 7 clock source select for Fractional Rate Divider."]
    #[inline(always)]
    pub fn sel(&mut self) -> SEL_W<0> {
        SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flexcomm Interface 7 clock source select for Fractional Rate Divider.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fcclksel7](index.html) module"]
pub struct FCCLKSEL7_SPEC;
impl crate::RegisterSpec for FCCLKSEL7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fcclksel7::R](R) reader structure"]
impl crate::Readable for FCCLKSEL7_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fcclksel7::W](W) writer structure"]
impl crate::Writable for FCCLKSEL7_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FCCLKSEL7 to value 0x07"]
impl crate::Resettable for FCCLKSEL7_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x07
    }
}
