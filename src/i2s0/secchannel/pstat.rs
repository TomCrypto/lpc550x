#[doc = "Register `PSTAT` reader"]
pub struct R(crate::R<PSTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PSTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PSTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PSTAT` writer"]
pub struct W(crate::W<PSTAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PSTAT_SPEC>;
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
impl From<crate::W<PSTAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PSTAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BUSY` reader - Busy status for this channel pair."]
pub type BUSY_R = crate::BitReader<BUSY_A>;
#[doc = "Busy status for this channel pair.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUSY_A {
    #[doc = "0: The transmitter/receiver for this channel pair is currently idle."]
    IDLE = 0,
    #[doc = "1: The transmitter/receiver for this channel pair is currently processing data."]
    PROCESSING = 1,
}
impl From<BUSY_A> for bool {
    #[inline(always)]
    fn from(variant: BUSY_A) -> Self {
        variant as u8 != 0
    }
}
impl BUSY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUSY_A {
        match self.bits {
            false => BUSY_A::IDLE,
            true => BUSY_A::PROCESSING,
        }
    }
    #[doc = "Checks if the value of the field is `IDLE`"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == BUSY_A::IDLE
    }
    #[doc = "Checks if the value of the field is `PROCESSING`"]
    #[inline(always)]
    pub fn is_processing(&self) -> bool {
        *self == BUSY_A::PROCESSING
    }
}
#[doc = "Field `BUSY` writer - Busy status for this channel pair."]
pub type BUSY_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSTAT_SPEC, BUSY_A, O>;
impl<'a, const O: u8> BUSY_W<'a, O> {
    #[doc = "The transmitter/receiver for this channel pair is currently idle."]
    #[inline(always)]
    pub fn idle(self) -> &'a mut W {
        self.variant(BUSY_A::IDLE)
    }
    #[doc = "The transmitter/receiver for this channel pair is currently processing data."]
    #[inline(always)]
    pub fn processing(self) -> &'a mut W {
        self.variant(BUSY_A::PROCESSING)
    }
}
#[doc = "Field `SLVFRMERR` reader - Save Frame Error flag."]
pub type SLVFRMERR_R = crate::BitReader<bool>;
#[doc = "Field `SLVFRMERR` writer - Save Frame Error flag."]
pub type SLVFRMERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSTAT_SPEC, bool, O>;
#[doc = "Field `LR` reader - Left/Right indication."]
pub type LR_R = crate::BitReader<bool>;
#[doc = "Field `LR` writer - Left/Right indication."]
pub type LR_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSTAT_SPEC, bool, O>;
#[doc = "Field `DATAPAUSED` reader - Data Paused status flag."]
pub type DATAPAUSED_R = crate::BitReader<DATAPAUSED_A>;
#[doc = "Data Paused status flag.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DATAPAUSED_A {
    #[doc = "0: Data is not currently paused."]
    PAUSE = 0,
    #[doc = "1: A data pause has been requested and is now in force."]
    FORCE = 1,
}
impl From<DATAPAUSED_A> for bool {
    #[inline(always)]
    fn from(variant: DATAPAUSED_A) -> Self {
        variant as u8 != 0
    }
}
impl DATAPAUSED_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DATAPAUSED_A {
        match self.bits {
            false => DATAPAUSED_A::PAUSE,
            true => DATAPAUSED_A::FORCE,
        }
    }
    #[doc = "Checks if the value of the field is `PAUSE`"]
    #[inline(always)]
    pub fn is_pause(&self) -> bool {
        *self == DATAPAUSED_A::PAUSE
    }
    #[doc = "Checks if the value of the field is `FORCE`"]
    #[inline(always)]
    pub fn is_force(&self) -> bool {
        *self == DATAPAUSED_A::FORCE
    }
}
impl R {
    #[doc = "Bit 0 - Busy status for this channel pair."]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Save Frame Error flag."]
    #[inline(always)]
    pub fn slvfrmerr(&self) -> SLVFRMERR_R {
        SLVFRMERR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Left/Right indication."]
    #[inline(always)]
    pub fn lr(&self) -> LR_R {
        LR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Data Paused status flag."]
    #[inline(always)]
    pub fn datapaused(&self) -> DATAPAUSED_R {
        DATAPAUSED_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Busy status for this channel pair."]
    #[inline(always)]
    pub fn busy(&mut self) -> BUSY_W<0> {
        BUSY_W::new(self)
    }
    #[doc = "Bit 1 - Save Frame Error flag."]
    #[inline(always)]
    pub fn slvfrmerr(&mut self) -> SLVFRMERR_W<1> {
        SLVFRMERR_W::new(self)
    }
    #[doc = "Bit 2 - Left/Right indication."]
    #[inline(always)]
    pub fn lr(&mut self) -> LR_W<2> {
        LR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Status register for channel pair.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pstat](index.html) module"]
pub struct PSTAT_SPEC;
impl crate::RegisterSpec for PSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pstat::R](R) reader structure"]
impl crate::Readable for PSTAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pstat::W](W) writer structure"]
impl crate::Writable for PSTAT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PSTAT to value 0"]
impl crate::Resettable for PSTAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
