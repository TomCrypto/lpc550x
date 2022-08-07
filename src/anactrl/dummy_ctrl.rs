#[doc = "Register `DUMMY_CTRL` reader"]
pub struct R(crate::R<DUMMY_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DUMMY_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DUMMY_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DUMMY_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DUMMY_CTRL` writer"]
pub struct W(crate::W<DUMMY_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DUMMY_CTRL_SPEC>;
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
impl From<crate::W<DUMMY_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DUMMY_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `XO32M_ADC_CLK_MODE` reader - Control High speed Crystal oscillator mode of the ADC clock."]
pub type XO32M_ADC_CLK_MODE_R = crate::FieldReader<u8, XO32M_ADC_CLK_MODE_A>;
#[doc = "Control High speed Crystal oscillator mode of the ADC clock.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum XO32M_ADC_CLK_MODE_A {
    #[doc = "0: High speed Crystal oscillator output to ADC is disabled."]
    DISABLE = 0,
    #[doc = "1: High speed Crystal oscillator output to ADC is enable."]
    XO_ADC_ENABLE = 1,
}
impl From<XO32M_ADC_CLK_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: XO32M_ADC_CLK_MODE_A) -> Self {
        variant as _
    }
}
impl XO32M_ADC_CLK_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<XO32M_ADC_CLK_MODE_A> {
        match self.bits {
            0 => Some(XO32M_ADC_CLK_MODE_A::DISABLE),
            1 => Some(XO32M_ADC_CLK_MODE_A::XO_ADC_ENABLE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == XO32M_ADC_CLK_MODE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `XO_ADC_ENABLE`"]
    #[inline(always)]
    pub fn is_xo_adc_enable(&self) -> bool {
        *self == XO32M_ADC_CLK_MODE_A::XO_ADC_ENABLE
    }
}
#[doc = "Field `XO32M_ADC_CLK_MODE` writer - Control High speed Crystal oscillator mode of the ADC clock."]
pub type XO32M_ADC_CLK_MODE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DUMMY_CTRL_SPEC, u8, XO32M_ADC_CLK_MODE_A, 2, O>;
impl<'a, const O: u8> XO32M_ADC_CLK_MODE_W<'a, O> {
    #[doc = "High speed Crystal oscillator output to ADC is disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(XO32M_ADC_CLK_MODE_A::DISABLE)
    }
    #[doc = "High speed Crystal oscillator output to ADC is enable."]
    #[inline(always)]
    pub fn xo_adc_enable(self) -> &'a mut W {
        self.variant(XO32M_ADC_CLK_MODE_A::XO_ADC_ENABLE)
    }
}
impl R {
    #[doc = "Bits 10:11 - Control High speed Crystal oscillator mode of the ADC clock."]
    #[inline(always)]
    pub fn xo32m_adc_clk_mode(&self) -> XO32M_ADC_CLK_MODE_R {
        XO32M_ADC_CLK_MODE_R::new(((self.bits >> 10) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 10:11 - Control High speed Crystal oscillator mode of the ADC clock."]
    #[inline(always)]
    pub fn xo32m_adc_clk_mode(&mut self) -> XO32M_ADC_CLK_MODE_W<10> {
        XO32M_ADC_CLK_MODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Dummy Control bus to analog modules\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dummy_ctrl](index.html) module"]
pub struct DUMMY_CTRL_SPEC;
impl crate::RegisterSpec for DUMMY_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dummy_ctrl::R](R) reader structure"]
impl crate::Readable for DUMMY_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dummy_ctrl::W](W) writer structure"]
impl crate::Writable for DUMMY_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DUMMY_CTRL to value 0"]
impl crate::Resettable for DUMMY_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
