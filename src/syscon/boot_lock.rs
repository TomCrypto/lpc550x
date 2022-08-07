#[doc = "Register `BOOT_LOCK` reader"]
pub struct R(crate::R<BOOT_LOCK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BOOT_LOCK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BOOT_LOCK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BOOT_LOCK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BOOT_LOCK` writer"]
pub struct W(crate::W<BOOT_LOCK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BOOT_LOCK_SPEC>;
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
impl From<crate::W<BOOT_LOCK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BOOT_LOCK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LOCK_BOOT_SEED` reader - Control write access to BOOT_SEED_REG registers."]
pub type LOCK_BOOT_SEED_R = crate::BitReader<LOCK_BOOT_SEED_A>;
#[doc = "Control write access to BOOT_SEED_REG registers.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOCK_BOOT_SEED_A {
    #[doc = "1: write access to all 8 registers BOOT_SEED_REG is locked. This register is write once."]
    LOCK = 1,
}
impl From<LOCK_BOOT_SEED_A> for bool {
    #[inline(always)]
    fn from(variant: LOCK_BOOT_SEED_A) -> Self {
        variant as u8 != 0
    }
}
impl LOCK_BOOT_SEED_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<LOCK_BOOT_SEED_A> {
        match self.bits {
            true => Some(LOCK_BOOT_SEED_A::LOCK),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `LOCK`"]
    #[inline(always)]
    pub fn is_lock(&self) -> bool {
        *self == LOCK_BOOT_SEED_A::LOCK
    }
}
#[doc = "Field `LOCK_BOOT_SEED` writer - Control write access to BOOT_SEED_REG registers."]
pub type LOCK_BOOT_SEED_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, BOOT_LOCK_SPEC, LOCK_BOOT_SEED_A, O>;
impl<'a, const O: u8> LOCK_BOOT_SEED_W<'a, O> {
    #[doc = "write access to all 8 registers BOOT_SEED_REG is locked. This register is write once."]
    #[inline(always)]
    pub fn lock(self) -> &'a mut W {
        self.variant(LOCK_BOOT_SEED_A::LOCK)
    }
}
#[doc = "Field `LOCK_HMAC` reader - Control write access to HMAC_REG registers."]
pub type LOCK_HMAC_R = crate::BitReader<LOCK_HMAC_A>;
#[doc = "Control write access to HMAC_REG registers.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOCK_HMAC_A {
    #[doc = "1: write access to all 8 registers HMAC_REG is locked. This register is write once."]
    LOCK = 1,
}
impl From<LOCK_HMAC_A> for bool {
    #[inline(always)]
    fn from(variant: LOCK_HMAC_A) -> Self {
        variant as u8 != 0
    }
}
impl LOCK_HMAC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<LOCK_HMAC_A> {
        match self.bits {
            true => Some(LOCK_HMAC_A::LOCK),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `LOCK`"]
    #[inline(always)]
    pub fn is_lock(&self) -> bool {
        *self == LOCK_HMAC_A::LOCK
    }
}
#[doc = "Field `LOCK_HMAC` writer - Control write access to HMAC_REG registers."]
pub type LOCK_HMAC_W<'a, const O: u8> = crate::BitWriter<'a, u32, BOOT_LOCK_SPEC, LOCK_HMAC_A, O>;
impl<'a, const O: u8> LOCK_HMAC_W<'a, O> {
    #[doc = "write access to all 8 registers HMAC_REG is locked. This register is write once."]
    #[inline(always)]
    pub fn lock(self) -> &'a mut W {
        self.variant(LOCK_HMAC_A::LOCK)
    }
}
impl R {
    #[doc = "Bit 0 - Control write access to BOOT_SEED_REG registers."]
    #[inline(always)]
    pub fn lock_boot_seed(&self) -> LOCK_BOOT_SEED_R {
        LOCK_BOOT_SEED_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Control write access to HMAC_REG registers."]
    #[inline(always)]
    pub fn lock_hmac(&self) -> LOCK_HMAC_R {
        LOCK_HMAC_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Control write access to BOOT_SEED_REG registers."]
    #[inline(always)]
    pub fn lock_boot_seed(&mut self) -> LOCK_BOOT_SEED_W<0> {
        LOCK_BOOT_SEED_W::new(self)
    }
    #[doc = "Bit 1 - Control write access to HMAC_REG registers."]
    #[inline(always)]
    pub fn lock_hmac(&mut self) -> LOCK_HMAC_W<1> {
        LOCK_HMAC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control write access to boot seed security registers.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [boot_lock](index.html) module"]
pub struct BOOT_LOCK_SPEC;
impl crate::RegisterSpec for BOOT_LOCK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [boot_lock::R](R) reader structure"]
impl crate::Readable for BOOT_LOCK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [boot_lock::W](W) writer structure"]
impl crate::Writable for BOOT_LOCK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BOOT_LOCK to value 0"]
impl crate::Resettable for BOOT_LOCK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
