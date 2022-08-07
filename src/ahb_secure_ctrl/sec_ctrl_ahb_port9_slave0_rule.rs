#[doc = "Register `SEC_CTRL_AHB_PORT9_SLAVE0_RULE` reader"]
pub struct R(crate::R<SEC_CTRL_AHB_PORT9_SLAVE0_RULE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SEC_CTRL_AHB_PORT9_SLAVE0_RULE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SEC_CTRL_AHB_PORT9_SLAVE0_RULE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SEC_CTRL_AHB_PORT9_SLAVE0_RULE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SEC_CTRL_AHB_PORT9_SLAVE0_RULE` writer"]
pub struct W(crate::W<SEC_CTRL_AHB_PORT9_SLAVE0_RULE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SEC_CTRL_AHB_PORT9_SLAVE0_RULE_SPEC>;
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
impl From<crate::W<SEC_CTRL_AHB_PORT9_SLAVE0_RULE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SEC_CTRL_AHB_PORT9_SLAVE0_RULE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADC_RULE` reader - ADC"]
pub type ADC_RULE_R = crate::FieldReader<u8, ADC_RULE_A>;
#[doc = "ADC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ADC_RULE_A {
    #[doc = "0: Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0,
    #[doc = "1: Non-secure and Privilege access allowed."]
    ENUM_NS_P = 1,
    #[doc = "2: Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 2,
    #[doc = "3: Secure and Priviledge user access allowed."]
    ENUM_S_P = 3,
}
impl From<ADC_RULE_A> for u8 {
    #[inline(always)]
    fn from(variant: ADC_RULE_A) -> Self {
        variant as _
    }
}
impl ADC_RULE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC_RULE_A {
        match self.bits {
            0 => ADC_RULE_A::ENUM_NS_NP,
            1 => ADC_RULE_A::ENUM_NS_P,
            2 => ADC_RULE_A::ENUM_S_NP,
            3 => ADC_RULE_A::ENUM_S_P,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_NP`"]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == ADC_RULE_A::ENUM_NS_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_P`"]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == ADC_RULE_A::ENUM_NS_P
    }
    #[doc = "Checks if the value of the field is `ENUM_S_NP`"]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == ADC_RULE_A::ENUM_S_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_S_P`"]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == ADC_RULE_A::ENUM_S_P
    }
}
#[doc = "Field `ADC_RULE` writer - ADC"]
pub type ADC_RULE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, SEC_CTRL_AHB_PORT9_SLAVE0_RULE_SPEC, u8, ADC_RULE_A, 2, O>;
impl<'a, const O: u8> ADC_RULE_W<'a, O> {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut W {
        self.variant(ADC_RULE_A::ENUM_NS_NP)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut W {
        self.variant(ADC_RULE_A::ENUM_NS_P)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut W {
        self.variant(ADC_RULE_A::ENUM_S_NP)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut W {
        self.variant(ADC_RULE_A::ENUM_S_P)
    }
}
#[doc = "Field `HASH_RULE` reader - SHA-2 crypto registers"]
pub type HASH_RULE_R = crate::FieldReader<u8, HASH_RULE_A>;
#[doc = "SHA-2 crypto registers\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum HASH_RULE_A {
    #[doc = "0: Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0,
    #[doc = "1: Non-secure and Privilege access allowed."]
    ENUM_NS_P = 1,
    #[doc = "2: Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 2,
    #[doc = "3: Secure and Priviledge user access allowed."]
    ENUM_S_P = 3,
}
impl From<HASH_RULE_A> for u8 {
    #[inline(always)]
    fn from(variant: HASH_RULE_A) -> Self {
        variant as _
    }
}
impl HASH_RULE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HASH_RULE_A {
        match self.bits {
            0 => HASH_RULE_A::ENUM_NS_NP,
            1 => HASH_RULE_A::ENUM_NS_P,
            2 => HASH_RULE_A::ENUM_S_NP,
            3 => HASH_RULE_A::ENUM_S_P,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_NP`"]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == HASH_RULE_A::ENUM_NS_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_P`"]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == HASH_RULE_A::ENUM_NS_P
    }
    #[doc = "Checks if the value of the field is `ENUM_S_NP`"]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == HASH_RULE_A::ENUM_S_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_S_P`"]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == HASH_RULE_A::ENUM_S_P
    }
}
#[doc = "Field `HASH_RULE` writer - SHA-2 crypto registers"]
pub type HASH_RULE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, SEC_CTRL_AHB_PORT9_SLAVE0_RULE_SPEC, u8, HASH_RULE_A, 2, O>;
impl<'a, const O: u8> HASH_RULE_W<'a, O> {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut W {
        self.variant(HASH_RULE_A::ENUM_NS_NP)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut W {
        self.variant(HASH_RULE_A::ENUM_NS_P)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut W {
        self.variant(HASH_RULE_A::ENUM_S_NP)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut W {
        self.variant(HASH_RULE_A::ENUM_S_P)
    }
}
#[doc = "Field `CASPER_RULE` reader - RSA/ECC crypto accelerator"]
pub type CASPER_RULE_R = crate::FieldReader<u8, CASPER_RULE_A>;
#[doc = "RSA/ECC crypto accelerator\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CASPER_RULE_A {
    #[doc = "0: Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0,
    #[doc = "1: Non-secure and Privilege access allowed."]
    ENUM_NS_P = 1,
    #[doc = "2: Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 2,
    #[doc = "3: Secure and Priviledge user access allowed."]
    ENUM_S_P = 3,
}
impl From<CASPER_RULE_A> for u8 {
    #[inline(always)]
    fn from(variant: CASPER_RULE_A) -> Self {
        variant as _
    }
}
impl CASPER_RULE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CASPER_RULE_A {
        match self.bits {
            0 => CASPER_RULE_A::ENUM_NS_NP,
            1 => CASPER_RULE_A::ENUM_NS_P,
            2 => CASPER_RULE_A::ENUM_S_NP,
            3 => CASPER_RULE_A::ENUM_S_P,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_NP`"]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == CASPER_RULE_A::ENUM_NS_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_P`"]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == CASPER_RULE_A::ENUM_NS_P
    }
    #[doc = "Checks if the value of the field is `ENUM_S_NP`"]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == CASPER_RULE_A::ENUM_S_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_S_P`"]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == CASPER_RULE_A::ENUM_S_P
    }
}
#[doc = "Field `CASPER_RULE` writer - RSA/ECC crypto accelerator"]
pub type CASPER_RULE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, SEC_CTRL_AHB_PORT9_SLAVE0_RULE_SPEC, u8, CASPER_RULE_A, 2, O>;
impl<'a, const O: u8> CASPER_RULE_W<'a, O> {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut W {
        self.variant(CASPER_RULE_A::ENUM_NS_NP)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut W {
        self.variant(CASPER_RULE_A::ENUM_NS_P)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut W {
        self.variant(CASPER_RULE_A::ENUM_S_NP)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut W {
        self.variant(CASPER_RULE_A::ENUM_S_P)
    }
}
#[doc = "Field `DMA1_RULE` reader - DMA Controller (Secure)"]
pub type DMA1_RULE_R = crate::FieldReader<u8, DMA1_RULE_A>;
#[doc = "DMA Controller (Secure)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DMA1_RULE_A {
    #[doc = "0: Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0,
    #[doc = "1: Non-secure and Privilege access allowed."]
    ENUM_NS_P = 1,
    #[doc = "2: Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 2,
    #[doc = "3: Secure and Priviledge user access allowed."]
    ENUM_S_P = 3,
}
impl From<DMA1_RULE_A> for u8 {
    #[inline(always)]
    fn from(variant: DMA1_RULE_A) -> Self {
        variant as _
    }
}
impl DMA1_RULE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMA1_RULE_A {
        match self.bits {
            0 => DMA1_RULE_A::ENUM_NS_NP,
            1 => DMA1_RULE_A::ENUM_NS_P,
            2 => DMA1_RULE_A::ENUM_S_NP,
            3 => DMA1_RULE_A::ENUM_S_P,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_NP`"]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == DMA1_RULE_A::ENUM_NS_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_P`"]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == DMA1_RULE_A::ENUM_NS_P
    }
    #[doc = "Checks if the value of the field is `ENUM_S_NP`"]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == DMA1_RULE_A::ENUM_S_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_S_P`"]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == DMA1_RULE_A::ENUM_S_P
    }
}
#[doc = "Field `DMA1_RULE` writer - DMA Controller (Secure)"]
pub type DMA1_RULE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, SEC_CTRL_AHB_PORT9_SLAVE0_RULE_SPEC, u8, DMA1_RULE_A, 2, O>;
impl<'a, const O: u8> DMA1_RULE_W<'a, O> {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut W {
        self.variant(DMA1_RULE_A::ENUM_NS_NP)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut W {
        self.variant(DMA1_RULE_A::ENUM_NS_P)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut W {
        self.variant(DMA1_RULE_A::ENUM_S_NP)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut W {
        self.variant(DMA1_RULE_A::ENUM_S_P)
    }
}
impl R {
    #[doc = "Bits 0:1 - ADC"]
    #[inline(always)]
    pub fn adc_rule(&self) -> ADC_RULE_R {
        ADC_RULE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 16:17 - SHA-2 crypto registers"]
    #[inline(always)]
    pub fn hash_rule(&self) -> HASH_RULE_R {
        HASH_RULE_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 20:21 - RSA/ECC crypto accelerator"]
    #[inline(always)]
    pub fn casper_rule(&self) -> CASPER_RULE_R {
        CASPER_RULE_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 28:29 - DMA Controller (Secure)"]
    #[inline(always)]
    pub fn dma1_rule(&self) -> DMA1_RULE_R {
        DMA1_RULE_R::new(((self.bits >> 28) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - ADC"]
    #[inline(always)]
    pub fn adc_rule(&mut self) -> ADC_RULE_W<0> {
        ADC_RULE_W::new(self)
    }
    #[doc = "Bits 16:17 - SHA-2 crypto registers"]
    #[inline(always)]
    pub fn hash_rule(&mut self) -> HASH_RULE_W<16> {
        HASH_RULE_W::new(self)
    }
    #[doc = "Bits 20:21 - RSA/ECC crypto accelerator"]
    #[inline(always)]
    pub fn casper_rule(&mut self) -> CASPER_RULE_W<20> {
        CASPER_RULE_W::new(self)
    }
    #[doc = "Bits 28:29 - DMA Controller (Secure)"]
    #[inline(always)]
    pub fn dma1_rule(&mut self) -> DMA1_RULE_W<28> {
        DMA1_RULE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Security access rules for AHB peripherals.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sec_ctrl_ahb_port9_slave0_rule](index.html) module"]
pub struct SEC_CTRL_AHB_PORT9_SLAVE0_RULE_SPEC;
impl crate::RegisterSpec for SEC_CTRL_AHB_PORT9_SLAVE0_RULE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sec_ctrl_ahb_port9_slave0_rule::R](R) reader structure"]
impl crate::Readable for SEC_CTRL_AHB_PORT9_SLAVE0_RULE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sec_ctrl_ahb_port9_slave0_rule::W](W) writer structure"]
impl crate::Writable for SEC_CTRL_AHB_PORT9_SLAVE0_RULE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SEC_CTRL_AHB_PORT9_SLAVE0_RULE to value 0"]
impl crate::Resettable for SEC_CTRL_AHB_PORT9_SLAVE0_RULE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
