#[doc = "Register `TRACECLKSEL` reader"]
pub struct R(crate::R<TRACECLKSEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TRACECLKSEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TRACECLKSEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TRACECLKSEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TRACECLKSEL` writer"]
pub struct W(crate::W<TRACECLKSEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TRACECLKSEL_SPEC>;
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
impl From<crate::W<TRACECLKSEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TRACECLKSEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEL` reader - Trace clock source select."]
pub type SEL_R = crate::FieldReader<u8, SEL_A>;
#[doc = "Trace clock source select.\n\nValue on reset: 7"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SEL_A {
    #[doc = "0: Trace divided clock."]
    TRACE_CLOCK = 0,
    #[doc = "1: FRO 1MHz clock."]
    FRO_1MHZ = 1,
    #[doc = "2: Oscillator 32 kHz clock."]
    OSC_32KHZ = 2,
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
            0 => Some(SEL_A::TRACE_CLOCK),
            1 => Some(SEL_A::FRO_1MHZ),
            2 => Some(SEL_A::OSC_32KHZ),
            7 => Some(SEL_A::NONE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `TRACE_CLOCK`"]
    #[inline(always)]
    pub fn is_trace_clock(&self) -> bool {
        *self == SEL_A::TRACE_CLOCK
    }
    #[doc = "Checks if the value of the field is `FRO_1MHZ`"]
    #[inline(always)]
    pub fn is_fro_1mhz(&self) -> bool {
        *self == SEL_A::FRO_1MHZ
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
#[doc = "Field `SEL` writer - Trace clock source select."]
pub type SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TRACECLKSEL_SPEC, u8, SEL_A, 3, O>;
impl<'a, const O: u8> SEL_W<'a, O> {
    #[doc = "Trace divided clock."]
    #[inline(always)]
    pub fn trace_clock(self) -> &'a mut W {
        self.variant(SEL_A::TRACE_CLOCK)
    }
    #[doc = "FRO 1MHz clock."]
    #[inline(always)]
    pub fn fro_1mhz(self) -> &'a mut W {
        self.variant(SEL_A::FRO_1MHZ)
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
    #[doc = "Bits 0:2 - Trace clock source select."]
    #[inline(always)]
    pub fn sel(&self) -> SEL_R {
        SEL_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Trace clock source select."]
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
#[doc = "Trace clock source select.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [traceclksel](index.html) module"]
pub struct TRACECLKSEL_SPEC;
impl crate::RegisterSpec for TRACECLKSEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [traceclksel::R](R) reader structure"]
impl crate::Readable for TRACECLKSEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [traceclksel::W](W) writer structure"]
impl crate::Writable for TRACECLKSEL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TRACECLKSEL to value 0x07"]
impl crate::Resettable for TRACECLKSEL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x07
    }
}
