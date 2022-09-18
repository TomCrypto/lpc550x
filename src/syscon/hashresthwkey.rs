#[doc = "Register `HASHRESTHWKEY` reader"]
pub struct R(crate::R<HASHRESTHWKEY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HASHRESTHWKEY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HASHRESTHWKEY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HASHRESTHWKEY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HASHRESTHWKEY` writer"]
pub struct W(crate::W<HASHRESTHWKEY_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HASHRESTHWKEY_SPEC>;
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
impl From<crate::W<HASHRESTHWKEY_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HASHRESTHWKEY_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UNLOCKCODE` reader - Code value that controls whether HASH AES hardware secret key is unlocked."]
pub type UNLOCKCODE_R = crate::FieldReader<u32, UNLOCKCODE_A>;
#[doc = "Code value that controls whether HASH AES hardware secret key is unlocked.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum UNLOCKCODE_A {
    #[doc = "3275531610: HASH AES hardware secret key is unlocked for use by non-secure code. Any other value means that the hardware secret key is restricted to use by secure code only."]
    UNLOCK = 3275531610,
}
impl From<UNLOCKCODE_A> for u32 {
    #[inline(always)]
    fn from(variant: UNLOCKCODE_A) -> Self {
        variant as _
    }
}
impl UNLOCKCODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<UNLOCKCODE_A> {
        match self.bits {
            3275531610 => Some(UNLOCKCODE_A::UNLOCK),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `UNLOCK`"]
    #[inline(always)]
    pub fn is_unlock(&self) -> bool {
        *self == UNLOCKCODE_A::UNLOCK
    }
}
#[doc = "Field `UNLOCKCODE` writer - Code value that controls whether HASH AES hardware secret key is unlocked."]
pub type UNLOCKCODE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, HASHRESTHWKEY_SPEC, u32, UNLOCKCODE_A, 32, O>;
impl<'a, const O: u8> UNLOCKCODE_W<'a, O> {
    #[doc = "HASH AES hardware secret key is unlocked for use by non-secure code. Any other value means that the hardware secret key is restricted to use by secure code only."]
    #[inline(always)]
    pub fn unlock(self) -> &'a mut W {
        self.variant(UNLOCKCODE_A::UNLOCK)
    }
}
impl R {
    #[doc = "Bits 0:31 - Code value that controls whether HASH AES hardware secret key is unlocked."]
    #[inline(always)]
    pub fn unlockcode(&self) -> UNLOCKCODE_R {
        UNLOCKCODE_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Code value that controls whether HASH AES hardware secret key is unlocked."]
    #[inline(always)]
    pub fn unlockcode(&mut self) -> UNLOCKCODE_W<0> {
        UNLOCKCODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Controls whether the HASH AES hardware secret key is restricted to use by secure code.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hashresthwkey](index.html) module"]
pub struct HASHRESTHWKEY_SPEC;
impl crate::RegisterSpec for HASHRESTHWKEY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hashresthwkey::R](R) reader structure"]
impl crate::Readable for HASHRESTHWKEY_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hashresthwkey::W](W) writer structure"]
impl crate::Writable for HASHRESTHWKEY_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HASHRESTHWKEY to value 0"]
impl crate::Resettable for HASHRESTHWKEY_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
