#[doc = "Register `SWD_ACCESS_CPU0` reader"]
pub struct R(crate::R<SWD_ACCESS_CPU0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SWD_ACCESS_CPU0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SWD_ACCESS_CPU0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SWD_ACCESS_CPU0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SWD_ACCESS_CPU0` writer"]
pub struct W(crate::W<SWD_ACCESS_CPU0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SWD_ACCESS_CPU0_SPEC>;
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
impl From<crate::W<SWD_ACCESS_CPU0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SWD_ACCESS_CPU0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEC_CODE` reader - CPU0 SWD-AP: 0x12345678."]
pub type SEC_CODE_R = crate::FieldReader<u32, SEC_CODE_A>;
#[doc = "CPU0 SWD-AP: 0x12345678.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum SEC_CODE_A {
    #[doc = "0: CPU0 DAP is not allowed. Reading back register will be read as 0x5."]
    DISABLE = 0,
    #[doc = "305419896: Value to write to enable CPU0 SWD access. Reading back register will be read as 0xA."]
    ENABLE = 305419896,
}
impl From<SEC_CODE_A> for u32 {
    #[inline(always)]
    fn from(variant: SEC_CODE_A) -> Self {
        variant as _
    }
}
impl SEC_CODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SEC_CODE_A> {
        match self.bits {
            0 => Some(SEC_CODE_A::DISABLE),
            305419896 => Some(SEC_CODE_A::ENABLE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == SEC_CODE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == SEC_CODE_A::ENABLE
    }
}
#[doc = "Field `SEC_CODE` writer - CPU0 SWD-AP: 0x12345678."]
pub type SEC_CODE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SWD_ACCESS_CPU0_SPEC, u32, SEC_CODE_A, 32, O>;
impl<'a, const O: u8> SEC_CODE_W<'a, O> {
    #[doc = "CPU0 DAP is not allowed. Reading back register will be read as 0x5."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SEC_CODE_A::DISABLE)
    }
    #[doc = "Value to write to enable CPU0 SWD access. Reading back register will be read as 0xA."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SEC_CODE_A::ENABLE)
    }
}
impl R {
    #[doc = "Bits 0:31 - CPU0 SWD-AP: 0x12345678."]
    #[inline(always)]
    pub fn sec_code(&self) -> SEC_CODE_R {
        SEC_CODE_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CPU0 SWD-AP: 0x12345678."]
    #[inline(always)]
    pub fn sec_code(&mut self) -> SEC_CODE_W<0> {
        SEC_CODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register is used by ROM during DEBUG authentication mechanism to enable debug access port for CPU0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [swd_access_cpu0](index.html) module"]
pub struct SWD_ACCESS_CPU0_SPEC;
impl crate::RegisterSpec for SWD_ACCESS_CPU0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [swd_access_cpu0::R](R) reader structure"]
impl crate::Readable for SWD_ACCESS_CPU0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [swd_access_cpu0::W](W) writer structure"]
impl crate::Writable for SWD_ACCESS_CPU0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SWD_ACCESS_CPU0 to value 0"]
impl crate::Resettable for SWD_ACCESS_CPU0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
