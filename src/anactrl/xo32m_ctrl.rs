#[doc = "Register `XO32M_CTRL` reader"]
pub struct R(crate::R<XO32M_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<XO32M_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<XO32M_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<XO32M_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `XO32M_CTRL` writer"]
pub struct W(crate::W<XO32M_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<XO32M_CTRL_SPEC>;
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
impl From<crate::W<XO32M_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<XO32M_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GM` reader - Gm value for Xo."]
pub type GM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `GM` writer - Gm value for Xo."]
pub type GM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, XO32M_CTRL_SPEC, u8, u8, 3, O>;
#[doc = "Field `SLAVE` reader - Xo in slave mode."]
pub type SLAVE_R = crate::BitReader<bool>;
#[doc = "Field `SLAVE` writer - Xo in slave mode."]
pub type SLAVE_W<'a, const O: u8> = crate::BitWriter<'a, u32, XO32M_CTRL_SPEC, bool, O>;
#[doc = "Field `AMP` reader - Amplitude selection , Min amp : 001, Max amp : 110."]
pub type AMP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AMP` writer - Amplitude selection , Min amp : 001, Max amp : 110."]
pub type AMP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, XO32M_CTRL_SPEC, u8, u8, 3, O>;
#[doc = "Field `OSC_CAP_IN` reader - Tune capa banks of High speed Crystal Oscillator input pin."]
pub type OSC_CAP_IN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `OSC_CAP_IN` writer - Tune capa banks of High speed Crystal Oscillator input pin."]
pub type OSC_CAP_IN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, XO32M_CTRL_SPEC, u8, u8, 7, O>;
#[doc = "Field `OSC_CAP_OUT` reader - Tune capa banks of High speed Crystal Oscillator output pin."]
pub type OSC_CAP_OUT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `OSC_CAP_OUT` writer - Tune capa banks of High speed Crystal Oscillator output pin."]
pub type OSC_CAP_OUT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, XO32M_CTRL_SPEC, u8, u8, 7, O>;
#[doc = "Field `ACBUF_PASS_ENABLE` reader - Bypass enable of XO AC buffer enable in pll and top level."]
pub type ACBUF_PASS_ENABLE_R = crate::BitReader<ACBUF_PASS_ENABLE_A>;
#[doc = "Bypass enable of XO AC buffer enable in pll and top level.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ACBUF_PASS_ENABLE_A {
    #[doc = "0: XO AC buffer bypass is disabled."]
    DISABLE = 0,
    #[doc = "1: XO AC buffer bypass is enabled."]
    ENABLE = 1,
}
impl From<ACBUF_PASS_ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: ACBUF_PASS_ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
impl ACBUF_PASS_ENABLE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACBUF_PASS_ENABLE_A {
        match self.bits {
            false => ACBUF_PASS_ENABLE_A::DISABLE,
            true => ACBUF_PASS_ENABLE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ACBUF_PASS_ENABLE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ACBUF_PASS_ENABLE_A::ENABLE
    }
}
#[doc = "Field `ACBUF_PASS_ENABLE` writer - Bypass enable of XO AC buffer enable in pll and top level."]
pub type ACBUF_PASS_ENABLE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, XO32M_CTRL_SPEC, ACBUF_PASS_ENABLE_A, O>;
impl<'a, const O: u8> ACBUF_PASS_ENABLE_W<'a, O> {
    #[doc = "XO AC buffer bypass is disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ACBUF_PASS_ENABLE_A::DISABLE)
    }
    #[doc = "XO AC buffer bypass is enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ACBUF_PASS_ENABLE_A::ENABLE)
    }
}
#[doc = "Field `ENABLE_SYSTEM_CLK_OUT` reader - Enable High speed Crystal oscillator output to CPU system."]
pub type ENABLE_SYSTEM_CLK_OUT_R = crate::BitReader<ENABLE_SYSTEM_CLK_OUT_A>;
#[doc = "Enable High speed Crystal oscillator output to CPU system.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENABLE_SYSTEM_CLK_OUT_A {
    #[doc = "0: High speed Crystal oscillator output to CPU system is disabled."]
    DISABLE = 0,
    #[doc = "1: High speed Crystal oscillator output to CPU system is enabled."]
    ENABLE = 1,
}
impl From<ENABLE_SYSTEM_CLK_OUT_A> for bool {
    #[inline(always)]
    fn from(variant: ENABLE_SYSTEM_CLK_OUT_A) -> Self {
        variant as u8 != 0
    }
}
impl ENABLE_SYSTEM_CLK_OUT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENABLE_SYSTEM_CLK_OUT_A {
        match self.bits {
            false => ENABLE_SYSTEM_CLK_OUT_A::DISABLE,
            true => ENABLE_SYSTEM_CLK_OUT_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ENABLE_SYSTEM_CLK_OUT_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ENABLE_SYSTEM_CLK_OUT_A::ENABLE
    }
}
#[doc = "Field `ENABLE_SYSTEM_CLK_OUT` writer - Enable High speed Crystal oscillator output to CPU system."]
pub type ENABLE_SYSTEM_CLK_OUT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, XO32M_CTRL_SPEC, ENABLE_SYSTEM_CLK_OUT_A, O>;
impl<'a, const O: u8> ENABLE_SYSTEM_CLK_OUT_W<'a, O> {
    #[doc = "High speed Crystal oscillator output to CPU system is disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ENABLE_SYSTEM_CLK_OUT_A::DISABLE)
    }
    #[doc = "High speed Crystal oscillator output to CPU system is enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ENABLE_SYSTEM_CLK_OUT_A::ENABLE)
    }
}
impl R {
    #[doc = "Bits 1:3 - Gm value for Xo."]
    #[inline(always)]
    pub fn gm(&self) -> GM_R {
        GM_R::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bit 4 - Xo in slave mode."]
    #[inline(always)]
    pub fn slave(&self) -> SLAVE_R {
        SLAVE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:7 - Amplitude selection , Min amp : 001, Max amp : 110."]
    #[inline(always)]
    pub fn amp(&self) -> AMP_R {
        AMP_R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bits 8:14 - Tune capa banks of High speed Crystal Oscillator input pin."]
    #[inline(always)]
    pub fn osc_cap_in(&self) -> OSC_CAP_IN_R {
        OSC_CAP_IN_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 15:21 - Tune capa banks of High speed Crystal Oscillator output pin."]
    #[inline(always)]
    pub fn osc_cap_out(&self) -> OSC_CAP_OUT_R {
        OSC_CAP_OUT_R::new(((self.bits >> 15) & 0x7f) as u8)
    }
    #[doc = "Bit 22 - Bypass enable of XO AC buffer enable in pll and top level."]
    #[inline(always)]
    pub fn acbuf_pass_enable(&self) -> ACBUF_PASS_ENABLE_R {
        ACBUF_PASS_ENABLE_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 24 - Enable High speed Crystal oscillator output to CPU system."]
    #[inline(always)]
    pub fn enable_system_clk_out(&self) -> ENABLE_SYSTEM_CLK_OUT_R {
        ENABLE_SYSTEM_CLK_OUT_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 1:3 - Gm value for Xo."]
    #[inline(always)]
    pub fn gm(&mut self) -> GM_W<1> {
        GM_W::new(self)
    }
    #[doc = "Bit 4 - Xo in slave mode."]
    #[inline(always)]
    pub fn slave(&mut self) -> SLAVE_W<4> {
        SLAVE_W::new(self)
    }
    #[doc = "Bits 5:7 - Amplitude selection , Min amp : 001, Max amp : 110."]
    #[inline(always)]
    pub fn amp(&mut self) -> AMP_W<5> {
        AMP_W::new(self)
    }
    #[doc = "Bits 8:14 - Tune capa banks of High speed Crystal Oscillator input pin."]
    #[inline(always)]
    pub fn osc_cap_in(&mut self) -> OSC_CAP_IN_W<8> {
        OSC_CAP_IN_W::new(self)
    }
    #[doc = "Bits 15:21 - Tune capa banks of High speed Crystal Oscillator output pin."]
    #[inline(always)]
    pub fn osc_cap_out(&mut self) -> OSC_CAP_OUT_W<15> {
        OSC_CAP_OUT_W::new(self)
    }
    #[doc = "Bit 22 - Bypass enable of XO AC buffer enable in pll and top level."]
    #[inline(always)]
    pub fn acbuf_pass_enable(&mut self) -> ACBUF_PASS_ENABLE_W<22> {
        ACBUF_PASS_ENABLE_W::new(self)
    }
    #[doc = "Bit 24 - Enable High speed Crystal oscillator output to CPU system."]
    #[inline(always)]
    pub fn enable_system_clk_out(&mut self) -> ENABLE_SYSTEM_CLK_OUT_W<24> {
        ENABLE_SYSTEM_CLK_OUT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "High speed Crystal Oscillator Control register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [xo32m_ctrl](index.html) module"]
pub struct XO32M_CTRL_SPEC;
impl crate::RegisterSpec for XO32M_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [xo32m_ctrl::R](R) reader structure"]
impl crate::Readable for XO32M_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [xo32m_ctrl::W](W) writer structure"]
impl crate::Writable for XO32M_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets XO32M_CTRL to value 0x0021_428a"]
impl crate::Resettable for XO32M_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0021_428a
    }
}
