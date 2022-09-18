#[doc = "Register `MAINCLKSELB` reader"]
pub struct R(crate::R<MAINCLKSELB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MAINCLKSELB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MAINCLKSELB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MAINCLKSELB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MAINCLKSELB` writer"]
pub struct W(crate::W<MAINCLKSELB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MAINCLKSELB_SPEC>;
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
impl From<crate::W<MAINCLKSELB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MAINCLKSELB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEL` reader - Main clock source select."]
pub type SEL_R = crate::FieldReader<u8, SEL_A>;
#[doc = "Main clock source select.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SEL_A {
    #[doc = "0: Main Clock A."]
    MAIN_CLOCK_A = 0,
    #[doc = "1: PLL0 clock."]
    PLL0 = 1,
    #[doc = "2: PLL1 clock."]
    PLL1 = 2,
    #[doc = "3: Oscillator 32 kHz clock."]
    OSC_32KHZ = 3,
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
            0 => Some(SEL_A::MAIN_CLOCK_A),
            1 => Some(SEL_A::PLL0),
            2 => Some(SEL_A::PLL1),
            3 => Some(SEL_A::OSC_32KHZ),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `MAIN_CLOCK_A`"]
    #[inline(always)]
    pub fn is_main_clock_a(&self) -> bool {
        *self == SEL_A::MAIN_CLOCK_A
    }
    #[doc = "Checks if the value of the field is `PLL0`"]
    #[inline(always)]
    pub fn is_pll0(&self) -> bool {
        *self == SEL_A::PLL0
    }
    #[doc = "Checks if the value of the field is `PLL1`"]
    #[inline(always)]
    pub fn is_pll1(&self) -> bool {
        *self == SEL_A::PLL1
    }
    #[doc = "Checks if the value of the field is `OSC_32KHZ`"]
    #[inline(always)]
    pub fn is_osc_32khz(&self) -> bool {
        *self == SEL_A::OSC_32KHZ
    }
}
#[doc = "Field `SEL` writer - Main clock source select."]
pub type SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MAINCLKSELB_SPEC, u8, SEL_A, 3, O>;
impl<'a, const O: u8> SEL_W<'a, O> {
    #[doc = "Main Clock A."]
    #[inline(always)]
    pub fn main_clock_a(self) -> &'a mut W {
        self.variant(SEL_A::MAIN_CLOCK_A)
    }
    #[doc = "PLL0 clock."]
    #[inline(always)]
    pub fn pll0(self) -> &'a mut W {
        self.variant(SEL_A::PLL0)
    }
    #[doc = "PLL1 clock."]
    #[inline(always)]
    pub fn pll1(self) -> &'a mut W {
        self.variant(SEL_A::PLL1)
    }
    #[doc = "Oscillator 32 kHz clock."]
    #[inline(always)]
    pub fn osc_32khz(self) -> &'a mut W {
        self.variant(SEL_A::OSC_32KHZ)
    }
}
impl R {
    #[doc = "Bits 0:2 - Main clock source select."]
    #[inline(always)]
    pub fn sel(&self) -> SEL_R {
        SEL_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Main clock source select."]
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
#[doc = "Main clock source select.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mainclkselb](index.html) module"]
pub struct MAINCLKSELB_SPEC;
impl crate::RegisterSpec for MAINCLKSELB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mainclkselb::R](R) reader structure"]
impl crate::Readable for MAINCLKSELB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mainclkselb::W](W) writer structure"]
impl crate::Writable for MAINCLKSELB_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MAINCLKSELB to value 0"]
impl crate::Resettable for MAINCLKSELB_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
