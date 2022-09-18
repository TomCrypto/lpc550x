#[doc = "Register `CTIMERCLKSEL1` reader"]
pub struct R(crate::R<CTIMERCLKSEL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTIMERCLKSEL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTIMERCLKSEL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTIMERCLKSEL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTIMERCLKSEL1` writer"]
pub struct W(crate::W<CTIMERCLKSEL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTIMERCLKSEL1_SPEC>;
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
impl From<crate::W<CTIMERCLKSEL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTIMERCLKSEL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEL` reader - CTimer 1 clock source select."]
pub type SEL_R = crate::FieldReader<u8, SEL_A>;
#[doc = "CTimer 1 clock source select.\n\nValue on reset: 7"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SEL_A {
    #[doc = "0: Main clock."]
    MAIN_CLK = 0,
    #[doc = "1: PLL0 clock."]
    PLL0 = 1,
    #[doc = "3: FRO 96 MHz clock."]
    FRO_96MHZ = 3,
    #[doc = "4: FRO 1MHz clock."]
    FRO_1MHZ = 4,
    #[doc = "5: MCLK clock."]
    MCLK = 5,
    #[doc = "6: Oscillator 32kHz clock."]
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
    pub fn variant(&self) -> Option<SEL_A> {
        match self.bits {
            0 => Some(SEL_A::MAIN_CLK),
            1 => Some(SEL_A::PLL0),
            3 => Some(SEL_A::FRO_96MHZ),
            4 => Some(SEL_A::FRO_1MHZ),
            5 => Some(SEL_A::MCLK),
            6 => Some(SEL_A::OSC_32KHZ),
            7 => Some(SEL_A::NONE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `MAIN_CLK`"]
    #[inline(always)]
    pub fn is_main_clk(&self) -> bool {
        *self == SEL_A::MAIN_CLK
    }
    #[doc = "Checks if the value of the field is `PLL0`"]
    #[inline(always)]
    pub fn is_pll0(&self) -> bool {
        *self == SEL_A::PLL0
    }
    #[doc = "Checks if the value of the field is `FRO_96MHZ`"]
    #[inline(always)]
    pub fn is_fro_96mhz(&self) -> bool {
        *self == SEL_A::FRO_96MHZ
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
#[doc = "Field `SEL` writer - CTimer 1 clock source select."]
pub type SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTIMERCLKSEL1_SPEC, u8, SEL_A, 3, O>;
impl<'a, const O: u8> SEL_W<'a, O> {
    #[doc = "Main clock."]
    #[inline(always)]
    pub fn main_clk(self) -> &'a mut W {
        self.variant(SEL_A::MAIN_CLK)
    }
    #[doc = "PLL0 clock."]
    #[inline(always)]
    pub fn pll0(self) -> &'a mut W {
        self.variant(SEL_A::PLL0)
    }
    #[doc = "FRO 96 MHz clock."]
    #[inline(always)]
    pub fn fro_96mhz(self) -> &'a mut W {
        self.variant(SEL_A::FRO_96MHZ)
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
    #[doc = "Oscillator 32kHz clock."]
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
    #[doc = "Bits 0:2 - CTimer 1 clock source select."]
    #[inline(always)]
    pub fn sel(&self) -> SEL_R {
        SEL_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - CTimer 1 clock source select."]
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
#[doc = "CTimer 1 clock source select.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctimerclksel1](index.html) module"]
pub struct CTIMERCLKSEL1_SPEC;
impl crate::RegisterSpec for CTIMERCLKSEL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctimerclksel1::R](R) reader structure"]
impl crate::Readable for CTIMERCLKSEL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctimerclksel1::W](W) writer structure"]
impl crate::Writable for CTIMERCLKSEL1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTIMERCLKSEL1 to value 0x07"]
impl crate::Resettable for CTIMERCLKSEL1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x07
    }
}
