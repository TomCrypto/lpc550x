#[doc = "Register `SFAR` reader"]
pub struct R(crate::R<SFAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SFAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SFAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SFAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SFAR` writer"]
pub struct W(crate::W<SFAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SFAR_SPEC>;
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
impl From<crate::W<SFAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SFAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADDRESS` reader - When the SFARVALID bit of the SFSR is set to 1, this field holds the address of an access that caused an SAU violation."]
pub type ADDRESS_R = crate::FieldReader<u32, u32>;
#[doc = "Field `ADDRESS` writer - When the SFARVALID bit of the SFSR is set to 1, this field holds the address of an access that caused an SAU violation."]
pub type ADDRESS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SFAR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - When the SFARVALID bit of the SFSR is set to 1, this field holds the address of an access that caused an SAU violation."]
    #[inline(always)]
    pub fn address(&self) -> ADDRESS_R {
        ADDRESS_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - When the SFARVALID bit of the SFSR is set to 1, this field holds the address of an access that caused an SAU violation."]
    #[inline(always)]
    pub fn address(&mut self) -> ADDRESS_W<0> {
        ADDRESS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Secure Fault Address Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sfar](index.html) module"]
pub struct SFAR_SPEC;
impl crate::RegisterSpec for SFAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sfar::R](R) reader structure"]
impl crate::Readable for SFAR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sfar::W](W) writer structure"]
impl crate::Writable for SFAR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SFAR to value 0"]
impl crate::Resettable for SFAR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
