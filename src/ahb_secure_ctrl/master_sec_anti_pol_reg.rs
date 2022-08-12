#[doc = "Register `MASTER_SEC_ANTI_POL_REG` reader"]
pub struct R(crate::R<MASTER_SEC_ANTI_POL_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MASTER_SEC_ANTI_POL_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MASTER_SEC_ANTI_POL_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MASTER_SEC_ANTI_POL_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MASTER_SEC_ANTI_POL_REG` writer"]
pub struct W(crate::W<MASTER_SEC_ANTI_POL_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MASTER_SEC_ANTI_POL_REG_SPEC>;
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
impl From<crate::W<MASTER_SEC_ANTI_POL_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MASTER_SEC_ANTI_POL_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SDMA0` reader - System DMA 0. Must be equal to NOT(MASTER_SEC_LEVEL.SDMA0)"]
pub type SDMA0_R = crate::FieldReader<u8, SDMA0_A>;
#[doc = "System DMA 0. Must be equal to NOT(MASTER_SEC_LEVEL.SDMA0)\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SDMA0_A {
    #[doc = "0: Secure and Priviledge user access allowed."]
    ENUM_S_P = 0,
    #[doc = "1: Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 1,
    #[doc = "2: Non-secure and Privilege access allowed."]
    ENUM_NS_P = 2,
    #[doc = "3: Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 3,
}
impl From<SDMA0_A> for u8 {
    #[inline(always)]
    fn from(variant: SDMA0_A) -> Self {
        variant as _
    }
}
impl SDMA0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SDMA0_A {
        match self.bits {
            0 => SDMA0_A::ENUM_S_P,
            1 => SDMA0_A::ENUM_S_NP,
            2 => SDMA0_A::ENUM_NS_P,
            3 => SDMA0_A::ENUM_NS_NP,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ENUM_S_P`"]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == SDMA0_A::ENUM_S_P
    }
    #[doc = "Checks if the value of the field is `ENUM_S_NP`"]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == SDMA0_A::ENUM_S_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_P`"]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == SDMA0_A::ENUM_NS_P
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_NP`"]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == SDMA0_A::ENUM_NS_NP
    }
}
#[doc = "Field `SDMA0` writer - System DMA 0. Must be equal to NOT(MASTER_SEC_LEVEL.SDMA0)"]
pub type SDMA0_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, MASTER_SEC_ANTI_POL_REG_SPEC, u8, SDMA0_A, 2, O>;
impl<'a, const O: u8> SDMA0_W<'a, O> {
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut W {
        self.variant(SDMA0_A::ENUM_S_P)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut W {
        self.variant(SDMA0_A::ENUM_S_NP)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut W {
        self.variant(SDMA0_A::ENUM_NS_P)
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut W {
        self.variant(SDMA0_A::ENUM_NS_NP)
    }
}
#[doc = "Field `HASH` reader - Hash. Must be equal to NOT(MASTER_SEC_LEVEL.HASH)"]
pub type HASH_R = crate::FieldReader<u8, HASH_A>;
#[doc = "Hash. Must be equal to NOT(MASTER_SEC_LEVEL.HASH)\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum HASH_A {
    #[doc = "0: Secure and Priviledge user access allowed."]
    ENUM_S_P = 0,
    #[doc = "1: Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 1,
    #[doc = "2: Non-secure and Privilege access allowed."]
    ENUM_NS_P = 2,
    #[doc = "3: Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 3,
}
impl From<HASH_A> for u8 {
    #[inline(always)]
    fn from(variant: HASH_A) -> Self {
        variant as _
    }
}
impl HASH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HASH_A {
        match self.bits {
            0 => HASH_A::ENUM_S_P,
            1 => HASH_A::ENUM_S_NP,
            2 => HASH_A::ENUM_NS_P,
            3 => HASH_A::ENUM_NS_NP,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ENUM_S_P`"]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == HASH_A::ENUM_S_P
    }
    #[doc = "Checks if the value of the field is `ENUM_S_NP`"]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == HASH_A::ENUM_S_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_P`"]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == HASH_A::ENUM_NS_P
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_NP`"]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == HASH_A::ENUM_NS_NP
    }
}
#[doc = "Field `HASH` writer - Hash. Must be equal to NOT(MASTER_SEC_LEVEL.HASH)"]
pub type HASH_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, MASTER_SEC_ANTI_POL_REG_SPEC, u8, HASH_A, 2, O>;
impl<'a, const O: u8> HASH_W<'a, O> {
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut W {
        self.variant(HASH_A::ENUM_S_P)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut W {
        self.variant(HASH_A::ENUM_S_NP)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut W {
        self.variant(HASH_A::ENUM_NS_P)
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut W {
        self.variant(HASH_A::ENUM_NS_NP)
    }
}
#[doc = "Field `SDMA1` reader - System DMA 1 security level. Must be equal to NOT(MASTER_SEC_LEVEL.SDMA1)"]
pub type SDMA1_R = crate::FieldReader<u8, SDMA1_A>;
#[doc = "System DMA 1 security level. Must be equal to NOT(MASTER_SEC_LEVEL.SDMA1)\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SDMA1_A {
    #[doc = "0: Secure and Priviledge user access allowed."]
    ENUM_S_P = 0,
    #[doc = "1: Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 1,
    #[doc = "2: Non-secure and Privilege access allowed."]
    ENUM_NS_P = 2,
    #[doc = "3: Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 3,
}
impl From<SDMA1_A> for u8 {
    #[inline(always)]
    fn from(variant: SDMA1_A) -> Self {
        variant as _
    }
}
impl SDMA1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SDMA1_A {
        match self.bits {
            0 => SDMA1_A::ENUM_S_P,
            1 => SDMA1_A::ENUM_S_NP,
            2 => SDMA1_A::ENUM_NS_P,
            3 => SDMA1_A::ENUM_NS_NP,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ENUM_S_P`"]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == SDMA1_A::ENUM_S_P
    }
    #[doc = "Checks if the value of the field is `ENUM_S_NP`"]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == SDMA1_A::ENUM_S_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_P`"]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == SDMA1_A::ENUM_NS_P
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_NP`"]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == SDMA1_A::ENUM_NS_NP
    }
}
#[doc = "Field `SDMA1` writer - System DMA 1 security level. Must be equal to NOT(MASTER_SEC_LEVEL.SDMA1)"]
pub type SDMA1_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, MASTER_SEC_ANTI_POL_REG_SPEC, u8, SDMA1_A, 2, O>;
impl<'a, const O: u8> SDMA1_W<'a, O> {
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut W {
        self.variant(SDMA1_A::ENUM_S_P)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut W {
        self.variant(SDMA1_A::ENUM_S_NP)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut W {
        self.variant(SDMA1_A::ENUM_NS_P)
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut W {
        self.variant(SDMA1_A::ENUM_NS_NP)
    }
}
#[doc = "Field `CANFD` reader - CAN FD. Must be equal to NOT(MASTER_SEC_LEVEL.CANFD)"]
pub type CANFD_R = crate::FieldReader<u8, CANFD_A>;
#[doc = "CAN FD. Must be equal to NOT(MASTER_SEC_LEVEL.CANFD)\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CANFD_A {
    #[doc = "0: Secure and Priviledge user access allowed."]
    ENUM_S_P = 0,
    #[doc = "1: Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 1,
    #[doc = "2: Non-secure and Privilege access allowed."]
    ENUM_NS_P = 2,
    #[doc = "3: Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 3,
}
impl From<CANFD_A> for u8 {
    #[inline(always)]
    fn from(variant: CANFD_A) -> Self {
        variant as _
    }
}
impl CANFD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CANFD_A {
        match self.bits {
            0 => CANFD_A::ENUM_S_P,
            1 => CANFD_A::ENUM_S_NP,
            2 => CANFD_A::ENUM_NS_P,
            3 => CANFD_A::ENUM_NS_NP,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ENUM_S_P`"]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == CANFD_A::ENUM_S_P
    }
    #[doc = "Checks if the value of the field is `ENUM_S_NP`"]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == CANFD_A::ENUM_S_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_P`"]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == CANFD_A::ENUM_NS_P
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_NP`"]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == CANFD_A::ENUM_NS_NP
    }
}
#[doc = "Field `CANFD` writer - CAN FD. Must be equal to NOT(MASTER_SEC_LEVEL.CANFD)"]
pub type CANFD_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, MASTER_SEC_ANTI_POL_REG_SPEC, u8, CANFD_A, 2, O>;
impl<'a, const O: u8> CANFD_W<'a, O> {
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut W {
        self.variant(CANFD_A::ENUM_S_P)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut W {
        self.variant(CANFD_A::ENUM_S_NP)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut W {
        self.variant(CANFD_A::ENUM_NS_P)
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut W {
        self.variant(CANFD_A::ENUM_NS_NP)
    }
}
#[doc = "Field `MASTER_SEC_LEVEL_ANTIPOL_LOCK` reader - MASTER_SEC_ANTI_POL_REG register write-lock."]
pub type MASTER_SEC_LEVEL_ANTIPOL_LOCK_R = crate::FieldReader<u8, MASTER_SEC_LEVEL_ANTIPOL_LOCK_A>;
#[doc = "MASTER_SEC_ANTI_POL_REG register write-lock.\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MASTER_SEC_LEVEL_ANTIPOL_LOCK_A {
    #[doc = "1: Restricted mode."]
    BLOCKED = 1,
    #[doc = "2: Writable."]
    WRITABLE = 2,
}
impl From<MASTER_SEC_LEVEL_ANTIPOL_LOCK_A> for u8 {
    #[inline(always)]
    fn from(variant: MASTER_SEC_LEVEL_ANTIPOL_LOCK_A) -> Self {
        variant as _
    }
}
impl MASTER_SEC_LEVEL_ANTIPOL_LOCK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MASTER_SEC_LEVEL_ANTIPOL_LOCK_A> {
        match self.bits {
            1 => Some(MASTER_SEC_LEVEL_ANTIPOL_LOCK_A::BLOCKED),
            2 => Some(MASTER_SEC_LEVEL_ANTIPOL_LOCK_A::WRITABLE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == MASTER_SEC_LEVEL_ANTIPOL_LOCK_A::BLOCKED
    }
    #[doc = "Checks if the value of the field is `WRITABLE`"]
    #[inline(always)]
    pub fn is_writable(&self) -> bool {
        *self == MASTER_SEC_LEVEL_ANTIPOL_LOCK_A::WRITABLE
    }
}
#[doc = "Field `MASTER_SEC_LEVEL_ANTIPOL_LOCK` writer - MASTER_SEC_ANTI_POL_REG register write-lock."]
pub type MASTER_SEC_LEVEL_ANTIPOL_LOCK_W<'a, const O: u8> = crate::FieldWriter<
    'a,
    u32,
    MASTER_SEC_ANTI_POL_REG_SPEC,
    u8,
    MASTER_SEC_LEVEL_ANTIPOL_LOCK_A,
    2,
    O,
>;
impl<'a, const O: u8> MASTER_SEC_LEVEL_ANTIPOL_LOCK_W<'a, O> {
    #[doc = "Restricted mode."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut W {
        self.variant(MASTER_SEC_LEVEL_ANTIPOL_LOCK_A::BLOCKED)
    }
    #[doc = "Writable."]
    #[inline(always)]
    pub fn writable(self) -> &'a mut W {
        self.variant(MASTER_SEC_LEVEL_ANTIPOL_LOCK_A::WRITABLE)
    }
}
impl R {
    #[doc = "Bits 10:11 - System DMA 0. Must be equal to NOT(MASTER_SEC_LEVEL.SDMA0)"]
    #[inline(always)]
    pub fn sdma0(&self) -> SDMA0_R {
        SDMA0_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Hash. Must be equal to NOT(MASTER_SEC_LEVEL.HASH)"]
    #[inline(always)]
    pub fn hash(&self) -> HASH_R {
        HASH_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 24:25 - System DMA 1 security level. Must be equal to NOT(MASTER_SEC_LEVEL.SDMA1)"]
    #[inline(always)]
    pub fn sdma1(&self) -> SDMA1_R {
        SDMA1_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - CAN FD. Must be equal to NOT(MASTER_SEC_LEVEL.CANFD)"]
    #[inline(always)]
    pub fn canfd(&self) -> CANFD_R {
        CANFD_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 30:31 - MASTER_SEC_ANTI_POL_REG register write-lock."]
    #[inline(always)]
    pub fn master_sec_level_antipol_lock(&self) -> MASTER_SEC_LEVEL_ANTIPOL_LOCK_R {
        MASTER_SEC_LEVEL_ANTIPOL_LOCK_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 10:11 - System DMA 0. Must be equal to NOT(MASTER_SEC_LEVEL.SDMA0)"]
    #[inline(always)]
    pub fn sdma0(&mut self) -> SDMA0_W<10> {
        SDMA0_W::new(self)
    }
    #[doc = "Bits 20:21 - Hash. Must be equal to NOT(MASTER_SEC_LEVEL.HASH)"]
    #[inline(always)]
    pub fn hash(&mut self) -> HASH_W<20> {
        HASH_W::new(self)
    }
    #[doc = "Bits 24:25 - System DMA 1 security level. Must be equal to NOT(MASTER_SEC_LEVEL.SDMA1)"]
    #[inline(always)]
    pub fn sdma1(&mut self) -> SDMA1_W<24> {
        SDMA1_W::new(self)
    }
    #[doc = "Bits 26:27 - CAN FD. Must be equal to NOT(MASTER_SEC_LEVEL.CANFD)"]
    #[inline(always)]
    pub fn canfd(&mut self) -> CANFD_W<26> {
        CANFD_W::new(self)
    }
    #[doc = "Bits 30:31 - MASTER_SEC_ANTI_POL_REG register write-lock."]
    #[inline(always)]
    pub fn master_sec_level_antipol_lock(&mut self) -> MASTER_SEC_LEVEL_ANTIPOL_LOCK_W<30> {
        MASTER_SEC_LEVEL_ANTIPOL_LOCK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "master secure level anti-pole register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [master_sec_anti_pol_reg](index.html) module"]
pub struct MASTER_SEC_ANTI_POL_REG_SPEC;
impl crate::RegisterSpec for MASTER_SEC_ANTI_POL_REG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [master_sec_anti_pol_reg::R](R) reader structure"]
impl crate::Readable for MASTER_SEC_ANTI_POL_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [master_sec_anti_pol_reg::W](W) writer structure"]
impl crate::Writable for MASTER_SEC_ANTI_POL_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MASTER_SEC_ANTI_POL_REG to value 0xbfff_ffff"]
impl crate::Resettable for MASTER_SEC_ANTI_POL_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xbfff_ffff
    }
}
