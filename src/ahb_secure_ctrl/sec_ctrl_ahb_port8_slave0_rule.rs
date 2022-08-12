#[doc = "Register `SEC_CTRL_AHB_PORT8_SLAVE0_RULE` reader"]
pub struct R(crate::R<SEC_CTRL_AHB_PORT8_SLAVE0_RULE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SEC_CTRL_AHB_PORT8_SLAVE0_RULE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SEC_CTRL_AHB_PORT8_SLAVE0_RULE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SEC_CTRL_AHB_PORT8_SLAVE0_RULE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SEC_CTRL_AHB_PORT8_SLAVE0_RULE` writer"]
pub struct W(crate::W<SEC_CTRL_AHB_PORT8_SLAVE0_RULE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SEC_CTRL_AHB_PORT8_SLAVE0_RULE_SPEC>;
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
impl From<crate::W<SEC_CTRL_AHB_PORT8_SLAVE0_RULE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SEC_CTRL_AHB_PORT8_SLAVE0_RULE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CRC_RULE` reader - CRC engine."]
pub type CRC_RULE_R = crate::FieldReader<u8, CRC_RULE_A>;
#[doc = "CRC engine.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CRC_RULE_A {
    #[doc = "0: Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0,
    #[doc = "1: Non-secure and Privilege access allowed."]
    ENUM_NS_P = 1,
    #[doc = "2: Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 2,
    #[doc = "3: Secure and Priviledge user access allowed."]
    ENUM_S_P = 3,
}
impl From<CRC_RULE_A> for u8 {
    #[inline(always)]
    fn from(variant: CRC_RULE_A) -> Self {
        variant as _
    }
}
impl CRC_RULE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CRC_RULE_A {
        match self.bits {
            0 => CRC_RULE_A::ENUM_NS_NP,
            1 => CRC_RULE_A::ENUM_NS_P,
            2 => CRC_RULE_A::ENUM_S_NP,
            3 => CRC_RULE_A::ENUM_S_P,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_NP`"]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == CRC_RULE_A::ENUM_NS_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_P`"]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == CRC_RULE_A::ENUM_NS_P
    }
    #[doc = "Checks if the value of the field is `ENUM_S_NP`"]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == CRC_RULE_A::ENUM_S_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_S_P`"]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == CRC_RULE_A::ENUM_S_P
    }
}
#[doc = "Field `CRC_RULE` writer - CRC engine."]
pub type CRC_RULE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, SEC_CTRL_AHB_PORT8_SLAVE0_RULE_SPEC, u8, CRC_RULE_A, 2, O>;
impl<'a, const O: u8> CRC_RULE_W<'a, O> {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut W {
        self.variant(CRC_RULE_A::ENUM_NS_NP)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut W {
        self.variant(CRC_RULE_A::ENUM_NS_P)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut W {
        self.variant(CRC_RULE_A::ENUM_S_NP)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut W {
        self.variant(CRC_RULE_A::ENUM_S_P)
    }
}
#[doc = "Field `FLEXCOMM5_RULE` reader - Flexcomm interface 5."]
pub type FLEXCOMM5_RULE_R = crate::FieldReader<u8, FLEXCOMM5_RULE_A>;
#[doc = "Flexcomm interface 5.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FLEXCOMM5_RULE_A {
    #[doc = "0: Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0,
    #[doc = "1: Non-secure and Privilege access allowed."]
    ENUM_NS_P = 1,
    #[doc = "2: Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 2,
    #[doc = "3: Secure and Priviledge user access allowed."]
    ENUM_S_P = 3,
}
impl From<FLEXCOMM5_RULE_A> for u8 {
    #[inline(always)]
    fn from(variant: FLEXCOMM5_RULE_A) -> Self {
        variant as _
    }
}
impl FLEXCOMM5_RULE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLEXCOMM5_RULE_A {
        match self.bits {
            0 => FLEXCOMM5_RULE_A::ENUM_NS_NP,
            1 => FLEXCOMM5_RULE_A::ENUM_NS_P,
            2 => FLEXCOMM5_RULE_A::ENUM_S_NP,
            3 => FLEXCOMM5_RULE_A::ENUM_S_P,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_NP`"]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == FLEXCOMM5_RULE_A::ENUM_NS_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_P`"]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == FLEXCOMM5_RULE_A::ENUM_NS_P
    }
    #[doc = "Checks if the value of the field is `ENUM_S_NP`"]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == FLEXCOMM5_RULE_A::ENUM_S_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_S_P`"]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == FLEXCOMM5_RULE_A::ENUM_S_P
    }
}
#[doc = "Field `FLEXCOMM5_RULE` writer - Flexcomm interface 5."]
pub type FLEXCOMM5_RULE_W<'a, const O: u8> = crate::FieldWriterSafe<
    'a,
    u32,
    SEC_CTRL_AHB_PORT8_SLAVE0_RULE_SPEC,
    u8,
    FLEXCOMM5_RULE_A,
    2,
    O,
>;
impl<'a, const O: u8> FLEXCOMM5_RULE_W<'a, O> {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut W {
        self.variant(FLEXCOMM5_RULE_A::ENUM_NS_NP)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut W {
        self.variant(FLEXCOMM5_RULE_A::ENUM_NS_P)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut W {
        self.variant(FLEXCOMM5_RULE_A::ENUM_S_NP)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut W {
        self.variant(FLEXCOMM5_RULE_A::ENUM_S_P)
    }
}
#[doc = "Field `FLEXCOMM6_RULE` reader - Flexcomm interface 6."]
pub type FLEXCOMM6_RULE_R = crate::FieldReader<u8, FLEXCOMM6_RULE_A>;
#[doc = "Flexcomm interface 6.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FLEXCOMM6_RULE_A {
    #[doc = "0: Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0,
    #[doc = "1: Non-secure and Privilege access allowed."]
    ENUM_NS_P = 1,
    #[doc = "2: Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 2,
    #[doc = "3: Secure and Priviledge user access allowed."]
    ENUM_S_P = 3,
}
impl From<FLEXCOMM6_RULE_A> for u8 {
    #[inline(always)]
    fn from(variant: FLEXCOMM6_RULE_A) -> Self {
        variant as _
    }
}
impl FLEXCOMM6_RULE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLEXCOMM6_RULE_A {
        match self.bits {
            0 => FLEXCOMM6_RULE_A::ENUM_NS_NP,
            1 => FLEXCOMM6_RULE_A::ENUM_NS_P,
            2 => FLEXCOMM6_RULE_A::ENUM_S_NP,
            3 => FLEXCOMM6_RULE_A::ENUM_S_P,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_NP`"]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == FLEXCOMM6_RULE_A::ENUM_NS_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_P`"]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == FLEXCOMM6_RULE_A::ENUM_NS_P
    }
    #[doc = "Checks if the value of the field is `ENUM_S_NP`"]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == FLEXCOMM6_RULE_A::ENUM_S_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_S_P`"]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == FLEXCOMM6_RULE_A::ENUM_S_P
    }
}
#[doc = "Field `FLEXCOMM6_RULE` writer - Flexcomm interface 6."]
pub type FLEXCOMM6_RULE_W<'a, const O: u8> = crate::FieldWriterSafe<
    'a,
    u32,
    SEC_CTRL_AHB_PORT8_SLAVE0_RULE_SPEC,
    u8,
    FLEXCOMM6_RULE_A,
    2,
    O,
>;
impl<'a, const O: u8> FLEXCOMM6_RULE_W<'a, O> {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut W {
        self.variant(FLEXCOMM6_RULE_A::ENUM_NS_NP)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut W {
        self.variant(FLEXCOMM6_RULE_A::ENUM_NS_P)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut W {
        self.variant(FLEXCOMM6_RULE_A::ENUM_S_NP)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut W {
        self.variant(FLEXCOMM6_RULE_A::ENUM_S_P)
    }
}
impl R {
    #[doc = "Bits 20:21 - CRC engine."]
    #[inline(always)]
    pub fn crc_rule(&self) -> CRC_RULE_R {
        CRC_RULE_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 24:25 - Flexcomm interface 5."]
    #[inline(always)]
    pub fn flexcomm5_rule(&self) -> FLEXCOMM5_RULE_R {
        FLEXCOMM5_RULE_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 28:29 - Flexcomm interface 6."]
    #[inline(always)]
    pub fn flexcomm6_rule(&self) -> FLEXCOMM6_RULE_R {
        FLEXCOMM6_RULE_R::new(((self.bits >> 28) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 20:21 - CRC engine."]
    #[inline(always)]
    pub fn crc_rule(&mut self) -> CRC_RULE_W<20> {
        CRC_RULE_W::new(self)
    }
    #[doc = "Bits 24:25 - Flexcomm interface 5."]
    #[inline(always)]
    pub fn flexcomm5_rule(&mut self) -> FLEXCOMM5_RULE_W<24> {
        FLEXCOMM5_RULE_W::new(self)
    }
    #[doc = "Bits 28:29 - Flexcomm interface 6."]
    #[inline(always)]
    pub fn flexcomm6_rule(&mut self) -> FLEXCOMM6_RULE_W<28> {
        FLEXCOMM6_RULE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Security access rules for AHB peripherals.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sec_ctrl_ahb_port8_slave0_rule](index.html) module"]
pub struct SEC_CTRL_AHB_PORT8_SLAVE0_RULE_SPEC;
impl crate::RegisterSpec for SEC_CTRL_AHB_PORT8_SLAVE0_RULE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sec_ctrl_ahb_port8_slave0_rule::R](R) reader structure"]
impl crate::Readable for SEC_CTRL_AHB_PORT8_SLAVE0_RULE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sec_ctrl_ahb_port8_slave0_rule::W](W) writer structure"]
impl crate::Writable for SEC_CTRL_AHB_PORT8_SLAVE0_RULE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SEC_CTRL_AHB_PORT8_SLAVE0_RULE to value 0"]
impl crate::Resettable for SEC_CTRL_AHB_PORT8_SLAVE0_RULE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
