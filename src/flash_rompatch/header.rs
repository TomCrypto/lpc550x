#[doc = "Register `HEADER` reader"]
pub struct R(crate::R<HEADER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HEADER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HEADER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HEADER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HEADER` writer"]
pub struct W(crate::W<HEADER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HEADER_SPEC>;
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
impl From<crate::W<HEADER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HEADER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENTRIES` reader - ."]
pub type ENTRIES_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ENTRIES` writer - ."]
pub type ENTRIES_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HEADER_SPEC, u8, u8, 8, O>;
#[doc = "Field `SUB_TYPE` reader - ."]
pub type SUB_TYPE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SUB_TYPE` writer - ."]
pub type SUB_TYPE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HEADER_SPEC, u8, u8, 8, O>;
#[doc = "Field `TYPE` reader - ."]
pub type TYPE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TYPE` writer - ."]
pub type TYPE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HEADER_SPEC, u8, u8, 8, O>;
#[doc = "Field `IDENTIFIER` reader - ."]
pub type IDENTIFIER_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IDENTIFIER` writer - ."]
pub type IDENTIFIER_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HEADER_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - ."]
    #[inline(always)]
    pub fn entries(&self) -> ENTRIES_R {
        ENTRIES_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - ."]
    #[inline(always)]
    pub fn sub_type(&self) -> SUB_TYPE_R {
        SUB_TYPE_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - ."]
    #[inline(always)]
    pub fn type_(&self) -> TYPE_R {
        TYPE_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - ."]
    #[inline(always)]
    pub fn identifier(&self) -> IDENTIFIER_R {
        IDENTIFIER_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - ."]
    #[inline(always)]
    pub fn entries(&mut self) -> ENTRIES_W<0> {
        ENTRIES_W::new(self)
    }
    #[doc = "Bits 8:15 - ."]
    #[inline(always)]
    pub fn sub_type(&mut self) -> SUB_TYPE_W<8> {
        SUB_TYPE_W::new(self)
    }
    #[doc = "Bits 16:23 - ."]
    #[inline(always)]
    pub fn type_(&mut self) -> TYPE_W<16> {
        TYPE_W::new(self)
    }
    #[doc = "Bits 24:31 - ."]
    #[inline(always)]
    pub fn identifier(&mut self) -> IDENTIFIER_W<24> {
        IDENTIFIER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [header](index.html) module"]
pub struct HEADER_SPEC;
impl crate::RegisterSpec for HEADER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [header::R](R) reader structure"]
impl crate::Readable for HEADER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [header::W](W) writer structure"]
impl crate::Writable for HEADER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HEADER to value 0"]
impl crate::Resettable for HEADER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
