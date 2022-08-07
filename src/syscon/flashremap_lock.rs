#[doc = "Register `FLASHREMAP_LOCK` reader"]
pub struct R(crate::R<FLASHREMAP_LOCK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLASHREMAP_LOCK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLASHREMAP_LOCK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLASHREMAP_LOCK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FLASHREMAP_LOCK` writer"]
pub struct W(crate::W<FLASHREMAP_LOCK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLASHREMAP_LOCK_SPEC>;
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
impl From<crate::W<FLASHREMAP_LOCK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLASHREMAP_LOCK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LOCK` reader - Control write access to FLASHREMAP_SIZE and FLASHREMAP_OFFSET registers. Any value other than 0xC33CA55A and 0x3CC35AA5 does not modify the state."]
pub type LOCK_R = crate::FieldReader<u32, LOCK_A>;
#[doc = "Control write access to FLASHREMAP_SIZE and FLASHREMAP_OFFSET registers. Any value other than 0xC33CA55A and 0x3CC35AA5 does not modify the state.\n\nValue on reset: 3275531610"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum LOCK_A {
    #[doc = "1019435685: Write access to 4 registers FLASHREMAP_SIZE* and FLASHREMAP_OFFSET* is unlocked."]
    UNLOCK = 1019435685,
    #[doc = "3275531610: Write access to 4 registers FLASHREMAP_SIZE* and FLASHREMAP_OFFSET* is locked."]
    LOCK = 3275531610,
}
impl From<LOCK_A> for u32 {
    #[inline(always)]
    fn from(variant: LOCK_A) -> Self {
        variant as _
    }
}
impl LOCK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<LOCK_A> {
        match self.bits {
            1019435685 => Some(LOCK_A::UNLOCK),
            3275531610 => Some(LOCK_A::LOCK),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `UNLOCK`"]
    #[inline(always)]
    pub fn is_unlock(&self) -> bool {
        *self == LOCK_A::UNLOCK
    }
    #[doc = "Checks if the value of the field is `LOCK`"]
    #[inline(always)]
    pub fn is_lock(&self) -> bool {
        *self == LOCK_A::LOCK
    }
}
#[doc = "Field `LOCK` writer - Control write access to FLASHREMAP_SIZE and FLASHREMAP_OFFSET registers. Any value other than 0xC33CA55A and 0x3CC35AA5 does not modify the state."]
pub type LOCK_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FLASHREMAP_LOCK_SPEC, u32, LOCK_A, 32, O>;
impl<'a, const O: u8> LOCK_W<'a, O> {
    #[doc = "Write access to 4 registers FLASHREMAP_SIZE* and FLASHREMAP_OFFSET* is unlocked."]
    #[inline(always)]
    pub fn unlock(self) -> &'a mut W {
        self.variant(LOCK_A::UNLOCK)
    }
    #[doc = "Write access to 4 registers FLASHREMAP_SIZE* and FLASHREMAP_OFFSET* is locked."]
    #[inline(always)]
    pub fn lock(self) -> &'a mut W {
        self.variant(LOCK_A::LOCK)
    }
}
impl R {
    #[doc = "Bits 0:31 - Control write access to FLASHREMAP_SIZE and FLASHREMAP_OFFSET registers. Any value other than 0xC33CA55A and 0x3CC35AA5 does not modify the state."]
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Control write access to FLASHREMAP_SIZE and FLASHREMAP_OFFSET registers. Any value other than 0xC33CA55A and 0x3CC35AA5 does not modify the state."]
    #[inline(always)]
    pub fn lock(&mut self) -> LOCK_W<0> {
        LOCK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control write access to FLASHREMAP_SIZE and FLASHREMAP_OFFSET registers.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flashremap_lock](index.html) module"]
pub struct FLASHREMAP_LOCK_SPEC;
impl crate::RegisterSpec for FLASHREMAP_LOCK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [flashremap_lock::R](R) reader structure"]
impl crate::Readable for FLASHREMAP_LOCK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [flashremap_lock::W](W) writer structure"]
impl crate::Writable for FLASHREMAP_LOCK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FLASHREMAP_LOCK to value 0xc33c_a55a"]
impl crate::Resettable for FLASHREMAP_LOCK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xc33c_a55a
    }
}
