#[doc = "Register `FREQMEAS_TARGET` reader"]
pub struct R(crate::R<FREQMEAS_TARGET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FREQMEAS_TARGET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FREQMEAS_TARGET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FREQMEAS_TARGET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FREQMEAS_TARGET` writer"]
pub struct W(crate::W<FREQMEAS_TARGET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FREQMEAS_TARGET_SPEC>;
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
impl From<crate::W<FREQMEAS_TARGET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FREQMEAS_TARGET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLKIN` reader - Clock source number (decimal value) for frequency measure function target clock:"]
pub type CLKIN_R = crate::FieldReader<u8, CLKIN_A>;
#[doc = "Clock source number (decimal value) for frequency measure function target clock:\n\nValue on reset: 31"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CLKIN_A {
    #[doc = "0: External main crystal oscilator (Clock_in)."]
    VALUE0 = 0,
    #[doc = "1: FRO 12MHz clock."]
    VALUE1 = 1,
    #[doc = "2: FRO 96MHz clock."]
    VALUE2 = 2,
    #[doc = "3: Watchdog oscillator / FRO1MHz clock."]
    VALUE3 = 3,
    #[doc = "4: 32 kHz oscillator (32k_clk) clock."]
    VALUE4 = 4,
    #[doc = "5: main clock (main_clock)."]
    VALUE5 = 5,
    #[doc = "6: FREQME_GPIO_CLK_A."]
    VALUE6 = 6,
    #[doc = "7: FREQME_GPIO_CLK_B."]
    VALUE7 = 7,
}
impl From<CLKIN_A> for u8 {
    #[inline(always)]
    fn from(variant: CLKIN_A) -> Self {
        variant as _
    }
}
impl CLKIN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CLKIN_A> {
        match self.bits {
            0 => Some(CLKIN_A::VALUE0),
            1 => Some(CLKIN_A::VALUE1),
            2 => Some(CLKIN_A::VALUE2),
            3 => Some(CLKIN_A::VALUE3),
            4 => Some(CLKIN_A::VALUE4),
            5 => Some(CLKIN_A::VALUE5),
            6 => Some(CLKIN_A::VALUE6),
            7 => Some(CLKIN_A::VALUE7),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE0`"]
    #[inline(always)]
    pub fn is_value0(&self) -> bool {
        *self == CLKIN_A::VALUE0
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CLKIN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CLKIN_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == CLKIN_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == CLKIN_A::VALUE4
    }
    #[doc = "Checks if the value of the field is `VALUE5`"]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        *self == CLKIN_A::VALUE5
    }
    #[doc = "Checks if the value of the field is `VALUE6`"]
    #[inline(always)]
    pub fn is_value6(&self) -> bool {
        *self == CLKIN_A::VALUE6
    }
    #[doc = "Checks if the value of the field is `VALUE7`"]
    #[inline(always)]
    pub fn is_value7(&self) -> bool {
        *self == CLKIN_A::VALUE7
    }
}
#[doc = "Field `CLKIN` writer - Clock source number (decimal value) for frequency measure function target clock:"]
pub type CLKIN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FREQMEAS_TARGET_SPEC, u8, CLKIN_A, 5, O>;
impl<'a, const O: u8> CLKIN_W<'a, O> {
    #[doc = "External main crystal oscilator (Clock_in)."]
    #[inline(always)]
    pub fn value0(self) -> &'a mut W {
        self.variant(CLKIN_A::VALUE0)
    }
    #[doc = "FRO 12MHz clock."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CLKIN_A::VALUE1)
    }
    #[doc = "FRO 96MHz clock."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CLKIN_A::VALUE2)
    }
    #[doc = "Watchdog oscillator / FRO1MHz clock."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(CLKIN_A::VALUE3)
    }
    #[doc = "32 kHz oscillator (32k_clk) clock."]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(CLKIN_A::VALUE4)
    }
    #[doc = "main clock (main_clock)."]
    #[inline(always)]
    pub fn value5(self) -> &'a mut W {
        self.variant(CLKIN_A::VALUE5)
    }
    #[doc = "FREQME_GPIO_CLK_A."]
    #[inline(always)]
    pub fn value6(self) -> &'a mut W {
        self.variant(CLKIN_A::VALUE6)
    }
    #[doc = "FREQME_GPIO_CLK_B."]
    #[inline(always)]
    pub fn value7(self) -> &'a mut W {
        self.variant(CLKIN_A::VALUE7)
    }
}
impl R {
    #[doc = "Bits 0:4 - Clock source number (decimal value) for frequency measure function target clock:"]
    #[inline(always)]
    pub fn clkin(&self) -> CLKIN_R {
        CLKIN_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Clock source number (decimal value) for frequency measure function target clock:"]
    #[inline(always)]
    pub fn clkin(&mut self) -> CLKIN_W<0> {
        CLKIN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Selection for frequency measurement target clock\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [freqmeas_target](index.html) module"]
pub struct FREQMEAS_TARGET_SPEC;
impl crate::RegisterSpec for FREQMEAS_TARGET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [freqmeas_target::R](R) reader structure"]
impl crate::Readable for FREQMEAS_TARGET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [freqmeas_target::W](W) writer structure"]
impl crate::Writable for FREQMEAS_TARGET_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FREQMEAS_TARGET to value 0x1f"]
impl crate::Resettable for FREQMEAS_TARGET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x1f
    }
}
