#[doc = "Register `ENCRYPTED_NUMBER` reader"]
pub struct R(crate::R<ENCRYPTED_NUMBER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ENCRYPTED_NUMBER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ENCRYPTED_NUMBER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ENCRYPTED_NUMBER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ENCRYPTED_NUMBER` writer"]
pub struct W(crate::W<ENCRYPTED_NUMBER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ENCRYPTED_NUMBER_SPEC>;
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
impl From<crate::W<ENCRYPTED_NUMBER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ENCRYPTED_NUMBER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENCRYPTED_NUMBER` reader - This register contains a random 32 bit number which is pre-computed."]
pub type ENCRYPTED_NUMBER_R = crate::FieldReader<u32, u32>;
#[doc = "Field `ENCRYPTED_NUMBER` writer - This register contains a random 32 bit number which is pre-computed."]
pub type ENCRYPTED_NUMBER_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ENCRYPTED_NUMBER_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - This register contains a random 32 bit number which is pre-computed."]
    #[inline(always)]
    pub fn encrypted_number(&self) -> ENCRYPTED_NUMBER_R {
        ENCRYPTED_NUMBER_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - This register contains a random 32 bit number which is pre-computed."]
    #[inline(always)]
    pub fn encrypted_number(&mut self) -> ENCRYPTED_NUMBER_W<0> {
        ENCRYPTED_NUMBER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register contains a random 32 bit number which is pre-computed\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [encrypted_number](index.html) module"]
pub struct ENCRYPTED_NUMBER_SPEC;
impl crate::RegisterSpec for ENCRYPTED_NUMBER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [encrypted_number::R](R) reader structure"]
impl crate::Readable for ENCRYPTED_NUMBER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [encrypted_number::W](W) writer structure"]
impl crate::Writable for ENCRYPTED_NUMBER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ENCRYPTED_NUMBER to value 0"]
impl crate::Resettable for ENCRYPTED_NUMBER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
