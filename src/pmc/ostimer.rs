#[doc = "Register `OSTIMER` reader"]
pub struct R(crate::R<OSTIMER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OSTIMER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OSTIMER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OSTIMER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OSTIMER` writer"]
pub struct W(crate::W<OSTIMER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OSTIMER_SPEC>;
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
impl From<crate::W<OSTIMER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OSTIMER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SOFTRESET` reader - Active high reset."]
pub type SOFTRESET_R = crate::BitReader<bool>;
#[doc = "Field `SOFTRESET` writer - Active high reset."]
pub type SOFTRESET_W<'a, const O: u8> = crate::BitWriter<'a, u32, OSTIMER_SPEC, bool, O>;
#[doc = "Field `CLOCKENABLE` reader - Enable OS event timer clock."]
pub type CLOCKENABLE_R = crate::BitReader<bool>;
#[doc = "Field `CLOCKENABLE` writer - Enable OS event timer clock."]
pub type CLOCKENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, OSTIMER_SPEC, bool, O>;
#[doc = "Field `DPDWAKEUPENABLE` reader - Wake up enable in Deep Power Down mode (To be used in Enable Deep Power Down mode)."]
pub type DPDWAKEUPENABLE_R = crate::BitReader<bool>;
#[doc = "Field `DPDWAKEUPENABLE` writer - Wake up enable in Deep Power Down mode (To be used in Enable Deep Power Down mode)."]
pub type DPDWAKEUPENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, OSTIMER_SPEC, bool, O>;
#[doc = "Field `OSC32KPD` reader - Oscilator 32KHz (either FRO32KHz or XTAL32KHz according to RTCOSC32K."]
pub type OSC32KPD_R = crate::BitReader<bool>;
#[doc = "Field `OSC32KPD` writer - Oscilator 32KHz (either FRO32KHz or XTAL32KHz according to RTCOSC32K."]
pub type OSC32KPD_W<'a, const O: u8> = crate::BitWriter<'a, u32, OSTIMER_SPEC, bool, O>;
#[doc = "Field `OSTIMERCLKSEL` reader - OS event timer clock select."]
pub type OSTIMERCLKSEL_R = crate::FieldReader<u8, OSTIMERCLKSEL_A>;
#[doc = "OS event timer clock select.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum OSTIMERCLKSEL_A {
    #[doc = "0: Oscillator 32 kHz clock."]
    OSC_32KHZ = 0,
    #[doc = "1: FRO 1MHz clock."]
    FRO_1MHZ = 1,
    #[doc = "2: Main clock for OS timer."]
    MAIN_CLOCK = 2,
    #[doc = "3: No clock."]
    NONE = 3,
}
impl From<OSTIMERCLKSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: OSTIMERCLKSEL_A) -> Self {
        variant as _
    }
}
impl OSTIMERCLKSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OSTIMERCLKSEL_A {
        match self.bits {
            0 => OSTIMERCLKSEL_A::OSC_32KHZ,
            1 => OSTIMERCLKSEL_A::FRO_1MHZ,
            2 => OSTIMERCLKSEL_A::MAIN_CLOCK,
            3 => OSTIMERCLKSEL_A::NONE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `OSC_32KHZ`"]
    #[inline(always)]
    pub fn is_osc_32khz(&self) -> bool {
        *self == OSTIMERCLKSEL_A::OSC_32KHZ
    }
    #[doc = "Checks if the value of the field is `FRO_1MHZ`"]
    #[inline(always)]
    pub fn is_fro_1mhz(&self) -> bool {
        *self == OSTIMERCLKSEL_A::FRO_1MHZ
    }
    #[doc = "Checks if the value of the field is `MAIN_CLOCK`"]
    #[inline(always)]
    pub fn is_main_clock(&self) -> bool {
        *self == OSTIMERCLKSEL_A::MAIN_CLOCK
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == OSTIMERCLKSEL_A::NONE
    }
}
#[doc = "Field `OSTIMERCLKSEL` writer - OS event timer clock select."]
pub type OSTIMERCLKSEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, OSTIMER_SPEC, u8, OSTIMERCLKSEL_A, 2, O>;
impl<'a, const O: u8> OSTIMERCLKSEL_W<'a, O> {
    #[doc = "Oscillator 32 kHz clock."]
    #[inline(always)]
    pub fn osc_32khz(self) -> &'a mut W {
        self.variant(OSTIMERCLKSEL_A::OSC_32KHZ)
    }
    #[doc = "FRO 1MHz clock."]
    #[inline(always)]
    pub fn fro_1mhz(self) -> &'a mut W {
        self.variant(OSTIMERCLKSEL_A::FRO_1MHZ)
    }
    #[doc = "Main clock for OS timer."]
    #[inline(always)]
    pub fn main_clock(self) -> &'a mut W {
        self.variant(OSTIMERCLKSEL_A::MAIN_CLOCK)
    }
    #[doc = "No clock."]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(OSTIMERCLKSEL_A::NONE)
    }
}
impl R {
    #[doc = "Bit 0 - Active high reset."]
    #[inline(always)]
    pub fn softreset(&self) -> SOFTRESET_R {
        SOFTRESET_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable OS event timer clock."]
    #[inline(always)]
    pub fn clockenable(&self) -> CLOCKENABLE_R {
        CLOCKENABLE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Wake up enable in Deep Power Down mode (To be used in Enable Deep Power Down mode)."]
    #[inline(always)]
    pub fn dpdwakeupenable(&self) -> DPDWAKEUPENABLE_R {
        DPDWAKEUPENABLE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Oscilator 32KHz (either FRO32KHz or XTAL32KHz according to RTCOSC32K."]
    #[inline(always)]
    pub fn osc32kpd(&self) -> OSC32KPD_R {
        OSC32KPD_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - OS event timer clock select."]
    #[inline(always)]
    pub fn ostimerclksel(&self) -> OSTIMERCLKSEL_R {
        OSTIMERCLKSEL_R::new(((self.bits >> 4) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Active high reset."]
    #[inline(always)]
    pub fn softreset(&mut self) -> SOFTRESET_W<0> {
        SOFTRESET_W::new(self)
    }
    #[doc = "Bit 1 - Enable OS event timer clock."]
    #[inline(always)]
    pub fn clockenable(&mut self) -> CLOCKENABLE_W<1> {
        CLOCKENABLE_W::new(self)
    }
    #[doc = "Bit 2 - Wake up enable in Deep Power Down mode (To be used in Enable Deep Power Down mode)."]
    #[inline(always)]
    pub fn dpdwakeupenable(&mut self) -> DPDWAKEUPENABLE_W<2> {
        DPDWAKEUPENABLE_W::new(self)
    }
    #[doc = "Bit 3 - Oscilator 32KHz (either FRO32KHz or XTAL32KHz according to RTCOSC32K."]
    #[inline(always)]
    pub fn osc32kpd(&mut self) -> OSC32KPD_W<3> {
        OSC32KPD_W::new(self)
    }
    #[doc = "Bits 4:5 - OS event timer clock select."]
    #[inline(always)]
    pub fn ostimerclksel(&mut self) -> OSTIMERCLKSEL_W<4> {
        OSTIMERCLKSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OS Timer control register \\[Reset by: PoR, Brown Out Detectors Reset\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ostimer](index.html) module"]
pub struct OSTIMER_SPEC;
impl crate::RegisterSpec for OSTIMER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ostimer::R](R) reader structure"]
impl crate::Readable for OSTIMER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ostimer::W](W) writer structure"]
impl crate::Writable for OSTIMER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OSTIMER to value 0x08"]
impl crate::Resettable for OSTIMER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x08
    }
}
