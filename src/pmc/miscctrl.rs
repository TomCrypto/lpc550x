#[doc = "Register `MISCCTRL` reader"]
pub struct R(crate::R<MISCCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MISCCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MISCCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MISCCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MISCCTRL` writer"]
pub struct W(crate::W<MISCCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MISCCTRL_SPEC>;
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
impl From<crate::W<MISCCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MISCCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LDOMEMHIGHZMODE` reader - Control the activation of LDO MEM High Z mode."]
pub type LDOMEMHIGHZMODE_R = crate::BitReader<LDOMEMHIGHZMODE_A>;
#[doc = "Control the activation of LDO MEM High Z mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LDOMEMHIGHZMODE_A {
    #[doc = "0: LDO MEM High Z mode is disabled."]
    DISABLE = 0,
    #[doc = "1: LDO MEM High Z mode is enabled."]
    ENABLE = 1,
}
impl From<LDOMEMHIGHZMODE_A> for bool {
    #[inline(always)]
    fn from(variant: LDOMEMHIGHZMODE_A) -> Self {
        variant as u8 != 0
    }
}
impl LDOMEMHIGHZMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LDOMEMHIGHZMODE_A {
        match self.bits {
            false => LDOMEMHIGHZMODE_A::DISABLE,
            true => LDOMEMHIGHZMODE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == LDOMEMHIGHZMODE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == LDOMEMHIGHZMODE_A::ENABLE
    }
}
#[doc = "Field `LDOMEMHIGHZMODE` writer - Control the activation of LDO MEM High Z mode."]
pub type LDOMEMHIGHZMODE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MISCCTRL_SPEC, LDOMEMHIGHZMODE_A, O>;
impl<'a, const O: u8> LDOMEMHIGHZMODE_W<'a, O> {
    #[doc = "LDO MEM High Z mode is disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(LDOMEMHIGHZMODE_A::DISABLE)
    }
    #[doc = "LDO MEM High Z mode is enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(LDOMEMHIGHZMODE_A::ENABLE)
    }
}
#[doc = "Field `DISABLE_BLEED` reader - Controls LDO MEM bleed current. This field is expected to be controlled by the Low Power Software only in DEEP SLEEP low power mode."]
pub type DISABLE_BLEED_R = crate::BitReader<DISABLE_BLEED_A>;
#[doc = "Controls LDO MEM bleed current. This field is expected to be controlled by the Low Power Software only in DEEP SLEEP low power mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DISABLE_BLEED_A {
    #[doc = "0: LDO_MEM bleed current is enabled."]
    BLEED_ENABLE = 0,
    #[doc = "1: LDO_MEM bleed current is disabled. Should be set before entering in Deep Sleep low power mode and cleared after wake up from Deep SLeep low power mode."]
    BLEED_DISABLE = 1,
}
impl From<DISABLE_BLEED_A> for bool {
    #[inline(always)]
    fn from(variant: DISABLE_BLEED_A) -> Self {
        variant as u8 != 0
    }
}
impl DISABLE_BLEED_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DISABLE_BLEED_A {
        match self.bits {
            false => DISABLE_BLEED_A::BLEED_ENABLE,
            true => DISABLE_BLEED_A::BLEED_DISABLE,
        }
    }
    #[doc = "Checks if the value of the field is `BLEED_ENABLE`"]
    #[inline(always)]
    pub fn is_bleed_enable(&self) -> bool {
        *self == DISABLE_BLEED_A::BLEED_ENABLE
    }
    #[doc = "Checks if the value of the field is `BLEED_DISABLE`"]
    #[inline(always)]
    pub fn is_bleed_disable(&self) -> bool {
        *self == DISABLE_BLEED_A::BLEED_DISABLE
    }
}
#[doc = "Field `DISABLE_BLEED` writer - Controls LDO MEM bleed current. This field is expected to be controlled by the Low Power Software only in DEEP SLEEP low power mode."]
pub type DISABLE_BLEED_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MISCCTRL_SPEC, DISABLE_BLEED_A, O>;
impl<'a, const O: u8> DISABLE_BLEED_W<'a, O> {
    #[doc = "LDO_MEM bleed current is enabled."]
    #[inline(always)]
    pub fn bleed_enable(self) -> &'a mut W {
        self.variant(DISABLE_BLEED_A::BLEED_ENABLE)
    }
    #[doc = "LDO_MEM bleed current is disabled. Should be set before entering in Deep Sleep low power mode and cleared after wake up from Deep SLeep low power mode."]
    #[inline(always)]
    pub fn bleed_disable(self) -> &'a mut W {
        self.variant(DISABLE_BLEED_A::BLEED_DISABLE)
    }
}
impl R {
    #[doc = "Bit 1 - Control the activation of LDO MEM High Z mode."]
    #[inline(always)]
    pub fn ldomemhighzmode(&self) -> LDOMEMHIGHZMODE_R {
        LDOMEMHIGHZMODE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 12 - Controls LDO MEM bleed current. This field is expected to be controlled by the Low Power Software only in DEEP SLEEP low power mode."]
    #[inline(always)]
    pub fn disable_bleed(&self) -> DISABLE_BLEED_R {
        DISABLE_BLEED_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Control the activation of LDO MEM High Z mode."]
    #[inline(always)]
    pub fn ldomemhighzmode(&mut self) -> LDOMEMHIGHZMODE_W<1> {
        LDOMEMHIGHZMODE_W::new(self)
    }
    #[doc = "Bit 12 - Controls LDO MEM bleed current. This field is expected to be controlled by the Low Power Software only in DEEP SLEEP low power mode."]
    #[inline(always)]
    pub fn disable_bleed(&mut self) -> DISABLE_BLEED_W<12> {
        DISABLE_BLEED_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Dummy Control bus to PMU \\[Reset by: PoR, Pin Reset, Brown Out Detectors Reset, Deep Power Down Reset, Software Reset\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [miscctrl](index.html) module"]
pub struct MISCCTRL_SPEC;
impl crate::RegisterSpec for MISCCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [miscctrl::R](R) reader structure"]
impl crate::Readable for MISCCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [miscctrl::W](W) writer structure"]
impl crate::Writable for MISCCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MISCCTRL to value 0"]
impl crate::Resettable for MISCCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
