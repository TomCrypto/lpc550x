#[doc = "Register `PCFG1` reader"]
pub struct R(crate::R<PCFG1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCFG1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCFG1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCFG1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PCFG1` writer"]
pub struct W(crate::W<PCFG1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PCFG1_SPEC>;
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
impl From<crate::W<PCFG1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PCFG1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PAIRENABLE` reader - Enable for this channel pair."]
pub type PAIRENABLE_R = crate::BitReader<PAIRENABLE_A>;
#[doc = "Enable for this channel pair.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAIRENABLE_A {
    #[doc = "0: This I2S channel pair is disabled."]
    DISABLE = 0,
    #[doc = "1: This I2S channel pair is enabled."]
    ENABLE = 1,
}
impl From<PAIRENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: PAIRENABLE_A) -> Self {
        variant as u8 != 0
    }
}
impl PAIRENABLE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAIRENABLE_A {
        match self.bits {
            false => PAIRENABLE_A::DISABLE,
            true => PAIRENABLE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == PAIRENABLE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == PAIRENABLE_A::ENABLE
    }
}
#[doc = "Field `PAIRENABLE` writer - Enable for this channel pair."]
pub type PAIRENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCFG1_SPEC, PAIRENABLE_A, O>;
impl<'a, const O: u8> PAIRENABLE_W<'a, O> {
    #[doc = "This I2S channel pair is disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(PAIRENABLE_A::DISABLE)
    }
    #[doc = "This I2S channel pair is enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(PAIRENABLE_A::ENABLE)
    }
}
#[doc = "Field `ONECHANNEL` reader - Single channel mode."]
pub type ONECHANNEL_R = crate::BitReader<ONECHANNEL_A>;
#[doc = "Single channel mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ONECHANNEL_A {
    #[doc = "0: I2S data for this channel pair is treated as left and right channels."]
    STEREO = 0,
    #[doc = "1: I2S data for this channel pair is treated as a single channel, functionally the left channel for this pair."]
    SINGLE = 1,
}
impl From<ONECHANNEL_A> for bool {
    #[inline(always)]
    fn from(variant: ONECHANNEL_A) -> Self {
        variant as u8 != 0
    }
}
impl ONECHANNEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ONECHANNEL_A {
        match self.bits {
            false => ONECHANNEL_A::STEREO,
            true => ONECHANNEL_A::SINGLE,
        }
    }
    #[doc = "Checks if the value of the field is `STEREO`"]
    #[inline(always)]
    pub fn is_stereo(&self) -> bool {
        *self == ONECHANNEL_A::STEREO
    }
    #[doc = "Checks if the value of the field is `SINGLE`"]
    #[inline(always)]
    pub fn is_single(&self) -> bool {
        *self == ONECHANNEL_A::SINGLE
    }
}
#[doc = "Field `ONECHANNEL` writer - Single channel mode."]
pub type ONECHANNEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCFG1_SPEC, ONECHANNEL_A, O>;
impl<'a, const O: u8> ONECHANNEL_W<'a, O> {
    #[doc = "I2S data for this channel pair is treated as left and right channels."]
    #[inline(always)]
    pub fn stereo(self) -> &'a mut W {
        self.variant(ONECHANNEL_A::STEREO)
    }
    #[doc = "I2S data for this channel pair is treated as a single channel, functionally the left channel for this pair."]
    #[inline(always)]
    pub fn single(self) -> &'a mut W {
        self.variant(ONECHANNEL_A::SINGLE)
    }
}
impl R {
    #[doc = "Bit 0 - Enable for this channel pair."]
    #[inline(always)]
    pub fn pairenable(&self) -> PAIRENABLE_R {
        PAIRENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 10 - Single channel mode."]
    #[inline(always)]
    pub fn onechannel(&self) -> ONECHANNEL_R {
        ONECHANNEL_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable for this channel pair."]
    #[inline(always)]
    pub fn pairenable(&mut self) -> PAIRENABLE_W<0> {
        PAIRENABLE_W::new(self)
    }
    #[doc = "Bit 10 - Single channel mode."]
    #[inline(always)]
    pub fn onechannel(&mut self) -> ONECHANNEL_W<10> {
        ONECHANNEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configuration register 1 for channel pair\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcfg1](index.html) module"]
pub struct PCFG1_SPEC;
impl crate::RegisterSpec for PCFG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pcfg1::R](R) reader structure"]
impl crate::Readable for PCFG1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pcfg1::W](W) writer structure"]
impl crate::Writable for PCFG1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PCFG1 to value 0"]
impl crate::Resettable for PCFG1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
