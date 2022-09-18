#[doc = "Register `SRAMCTRL` reader"]
pub struct R(crate::R<SRAMCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SRAMCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SRAMCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SRAMCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SRAMCTRL` writer"]
pub struct W(crate::W<SRAMCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SRAMCTRL_SPEC>;
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
impl From<crate::W<SRAMCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SRAMCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SMB` reader - Source Biasing voltage."]
pub type SMB_R = crate::FieldReader<u8, SMB_A>;
#[doc = "Source Biasing voltage.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SMB_A {
    #[doc = "0: Low leakage."]
    LOW = 0,
    #[doc = "1: Medium leakage."]
    MEDIUM = 1,
    #[doc = "2: Highest leakage."]
    HIGHEST = 2,
    #[doc = "3: Disable."]
    DISABLE = 3,
}
impl From<SMB_A> for u8 {
    #[inline(always)]
    fn from(variant: SMB_A) -> Self {
        variant as _
    }
}
impl SMB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMB_A {
        match self.bits {
            0 => SMB_A::LOW,
            1 => SMB_A::MEDIUM,
            2 => SMB_A::HIGHEST,
            3 => SMB_A::DISABLE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == SMB_A::LOW
    }
    #[doc = "Checks if the value of the field is `MEDIUM`"]
    #[inline(always)]
    pub fn is_medium(&self) -> bool {
        *self == SMB_A::MEDIUM
    }
    #[doc = "Checks if the value of the field is `HIGHEST`"]
    #[inline(always)]
    pub fn is_highest(&self) -> bool {
        *self == SMB_A::HIGHEST
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == SMB_A::DISABLE
    }
}
#[doc = "Field `SMB` writer - Source Biasing voltage."]
pub type SMB_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, SRAMCTRL_SPEC, u8, SMB_A, 2, O>;
impl<'a, const O: u8> SMB_W<'a, O> {
    #[doc = "Low leakage."]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(SMB_A::LOW)
    }
    #[doc = "Medium leakage."]
    #[inline(always)]
    pub fn medium(self) -> &'a mut W {
        self.variant(SMB_A::MEDIUM)
    }
    #[doc = "Highest leakage."]
    #[inline(always)]
    pub fn highest(self) -> &'a mut W {
        self.variant(SMB_A::HIGHEST)
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SMB_A::DISABLE)
    }
}
impl R {
    #[doc = "Bits 0:1 - Source Biasing voltage."]
    #[inline(always)]
    pub fn smb(&self) -> SMB_R {
        SMB_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Source Biasing voltage."]
    #[inline(always)]
    pub fn smb(&mut self) -> SMB_W<0> {
        SMB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "All SRAMs common control signals \\[Reset by: PoR, Pin Reset, Brown Out Detectors Reset, Software Reset\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sramctrl](index.html) module"]
pub struct SRAMCTRL_SPEC;
impl crate::RegisterSpec for SRAMCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sramctrl::R](R) reader structure"]
impl crate::Readable for SRAMCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sramctrl::W](W) writer structure"]
impl crate::Writable for SRAMCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SRAMCTRL to value 0x01"]
impl crate::Resettable for SRAMCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
