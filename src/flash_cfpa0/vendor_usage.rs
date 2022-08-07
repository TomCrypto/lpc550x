#[doc = "Register `VENDOR_USAGE` reader"]
pub struct R(crate::R<VENDOR_USAGE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VENDOR_USAGE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VENDOR_USAGE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VENDOR_USAGE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `VENDOR_USAGE` writer"]
pub struct W(crate::W<VENDOR_USAGE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<VENDOR_USAGE_SPEC>;
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
impl From<crate::W<VENDOR_USAGE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<VENDOR_USAGE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DBG_VENDOR_USAGE` reader - DBG_VENDOR_USAGE."]
pub type DBG_VENDOR_USAGE_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DBG_VENDOR_USAGE` writer - DBG_VENDOR_USAGE."]
pub type DBG_VENDOR_USAGE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, VENDOR_USAGE_SPEC, u16, u16, 16, O>;
#[doc = "Field `INVERSE_VALUE` reader - inverse value of bits \\[15:0\\]"]
pub type INVERSE_VALUE_R = crate::FieldReader<u16, u16>;
#[doc = "Field `INVERSE_VALUE` writer - inverse value of bits \\[15:0\\]"]
pub type INVERSE_VALUE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, VENDOR_USAGE_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - DBG_VENDOR_USAGE."]
    #[inline(always)]
    pub fn dbg_vendor_usage(&self) -> DBG_VENDOR_USAGE_R {
        DBG_VENDOR_USAGE_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - inverse value of bits \\[15:0\\]"]
    #[inline(always)]
    pub fn inverse_value(&self) -> INVERSE_VALUE_R {
        INVERSE_VALUE_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - DBG_VENDOR_USAGE."]
    #[inline(always)]
    pub fn dbg_vendor_usage(&mut self) -> DBG_VENDOR_USAGE_W<0> {
        DBG_VENDOR_USAGE_W::new(self)
    }
    #[doc = "Bits 16:31 - inverse value of bits \\[15:0\\]"]
    #[inline(always)]
    pub fn inverse_value(&mut self) -> INVERSE_VALUE_W<16> {
        INVERSE_VALUE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "no description available\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vendor_usage](index.html) module"]
pub struct VENDOR_USAGE_SPEC;
impl crate::RegisterSpec for VENDOR_USAGE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [vendor_usage::R](R) reader structure"]
impl crate::Readable for VENDOR_USAGE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [vendor_usage::W](W) writer structure"]
impl crate::Writable for VENDOR_USAGE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets VENDOR_USAGE to value 0"]
impl crate::Resettable for VENDOR_USAGE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
