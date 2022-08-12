#[doc = "Register `CPSTAT` reader"]
pub struct R(crate::R<CPSTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CPSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CPSTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CPSTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CPSTAT` writer"]
pub struct W(crate::W<CPSTAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CPSTAT_SPEC>;
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
impl From<crate::W<CPSTAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CPSTAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CPU0SLEEPING` reader - The CPU0 sleeping state."]
pub type CPU0SLEEPING_R = crate::BitReader<CPU0SLEEPING_A>;
#[doc = "The CPU0 sleeping state.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPU0SLEEPING_A {
    #[doc = "0: the CPU is not sleeping."]
    AWAKE = 0,
    #[doc = "1: the CPU is sleeping."]
    SLEEPING = 1,
}
impl From<CPU0SLEEPING_A> for bool {
    #[inline(always)]
    fn from(variant: CPU0SLEEPING_A) -> Self {
        variant as u8 != 0
    }
}
impl CPU0SLEEPING_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPU0SLEEPING_A {
        match self.bits {
            false => CPU0SLEEPING_A::AWAKE,
            true => CPU0SLEEPING_A::SLEEPING,
        }
    }
    #[doc = "Checks if the value of the field is `AWAKE`"]
    #[inline(always)]
    pub fn is_awake(&self) -> bool {
        *self == CPU0SLEEPING_A::AWAKE
    }
    #[doc = "Checks if the value of the field is `SLEEPING`"]
    #[inline(always)]
    pub fn is_sleeping(&self) -> bool {
        *self == CPU0SLEEPING_A::SLEEPING
    }
}
#[doc = "Field `CPU0LOCKUP` reader - The CPU0 lockup state."]
pub type CPU0LOCKUP_R = crate::BitReader<CPU0LOCKUP_A>;
#[doc = "The CPU0 lockup state.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPU0LOCKUP_A {
    #[doc = "0: the CPU is not in lockup."]
    AWAKE = 0,
    #[doc = "1: the CPU is in lockup."]
    SLEEPING = 1,
}
impl From<CPU0LOCKUP_A> for bool {
    #[inline(always)]
    fn from(variant: CPU0LOCKUP_A) -> Self {
        variant as u8 != 0
    }
}
impl CPU0LOCKUP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPU0LOCKUP_A {
        match self.bits {
            false => CPU0LOCKUP_A::AWAKE,
            true => CPU0LOCKUP_A::SLEEPING,
        }
    }
    #[doc = "Checks if the value of the field is `AWAKE`"]
    #[inline(always)]
    pub fn is_awake(&self) -> bool {
        *self == CPU0LOCKUP_A::AWAKE
    }
    #[doc = "Checks if the value of the field is `SLEEPING`"]
    #[inline(always)]
    pub fn is_sleeping(&self) -> bool {
        *self == CPU0LOCKUP_A::SLEEPING
    }
}
impl R {
    #[doc = "Bit 0 - The CPU0 sleeping state."]
    #[inline(always)]
    pub fn cpu0sleeping(&self) -> CPU0SLEEPING_R {
        CPU0SLEEPING_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - The CPU0 lockup state."]
    #[inline(always)]
    pub fn cpu0lockup(&self) -> CPU0LOCKUP_R {
        CPU0LOCKUP_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CPU Status.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpstat](index.html) module"]
pub struct CPSTAT_SPEC;
impl crate::RegisterSpec for CPSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cpstat::R](R) reader structure"]
impl crate::Readable for CPSTAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cpstat::W](W) writer structure"]
impl crate::Writable for CPSTAT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CPSTAT to value 0"]
impl crate::Resettable for CPSTAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
