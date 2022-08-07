#[doc = "Register `XO_32KHZ` reader"]
pub struct R(crate::R<XO_32KHZ_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<XO_32KHZ_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<XO_32KHZ_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<XO_32KHZ_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `XO_32KHZ` writer"]
pub struct W(crate::W<XO_32KHZ_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<XO_32KHZ_SPEC>;
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
impl From<crate::W<XO_32KHZ_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<XO_32KHZ_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `XO32K_XIN_TRIM_VALID` reader - no description available"]
pub type XO32K_XIN_TRIM_VALID_R = crate::BitReader<bool>;
#[doc = "Field `XO32K_XIN_TRIM_VALID` writer - no description available"]
pub type XO32K_XIN_TRIM_VALID_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, XO_32KHZ_SPEC, bool, O>;
#[doc = "Field `XO32K_XIN_CAPCAL_6PF` reader - no description available"]
pub type XO32K_XIN_CAPCAL_6PF_R = crate::FieldReader<u8, u8>;
#[doc = "Field `XO32K_XIN_CAPCAL_6PF` writer - no description available"]
pub type XO32K_XIN_CAPCAL_6PF_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, XO_32KHZ_SPEC, u8, u8, 7, O>;
#[doc = "Field `XO32K_XIN_CAPCAL_8PF` reader - no description available"]
pub type XO32K_XIN_CAPCAL_8PF_R = crate::FieldReader<u8, u8>;
#[doc = "Field `XO32K_XIN_CAPCAL_8PF` writer - no description available"]
pub type XO32K_XIN_CAPCAL_8PF_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, XO_32KHZ_SPEC, u8, u8, 7, O>;
#[doc = "Field `XO32K_XOUT_TRIM_VALID` reader - no description available"]
pub type XO32K_XOUT_TRIM_VALID_R = crate::BitReader<bool>;
#[doc = "Field `XO32K_XOUT_TRIM_VALID` writer - no description available"]
pub type XO32K_XOUT_TRIM_VALID_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, XO_32KHZ_SPEC, bool, O>;
#[doc = "Field `XO32K_XOUT_CAPCAL_6PF` reader - no description available"]
pub type XO32K_XOUT_CAPCAL_6PF_R = crate::FieldReader<u8, u8>;
#[doc = "Field `XO32K_XOUT_CAPCAL_6PF` writer - no description available"]
pub type XO32K_XOUT_CAPCAL_6PF_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, XO_32KHZ_SPEC, u8, u8, 7, O>;
#[doc = "Field `XO32K_XOUT_CAPCAL_8PF` reader - no description available"]
pub type XO32K_XOUT_CAPCAL_8PF_R = crate::FieldReader<u8, u8>;
#[doc = "Field `XO32K_XOUT_CAPCAL_8PF` writer - no description available"]
pub type XO32K_XOUT_CAPCAL_8PF_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, XO_32KHZ_SPEC, u8, u8, 7, O>;
impl R {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    pub fn xo32k_xin_trim_valid(&self) -> XO32K_XIN_TRIM_VALID_R {
        XO32K_XIN_TRIM_VALID_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:7 - no description available"]
    #[inline(always)]
    pub fn xo32k_xin_capcal_6pf(&self) -> XO32K_XIN_CAPCAL_6PF_R {
        XO32K_XIN_CAPCAL_6PF_R::new(((self.bits >> 1) & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - no description available"]
    #[inline(always)]
    pub fn xo32k_xin_capcal_8pf(&self) -> XO32K_XIN_CAPCAL_8PF_R {
        XO32K_XIN_CAPCAL_8PF_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bit 15 - no description available"]
    #[inline(always)]
    pub fn xo32k_xout_trim_valid(&self) -> XO32K_XOUT_TRIM_VALID_R {
        XO32K_XOUT_TRIM_VALID_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:22 - no description available"]
    #[inline(always)]
    pub fn xo32k_xout_capcal_6pf(&self) -> XO32K_XOUT_CAPCAL_6PF_R {
        XO32K_XOUT_CAPCAL_6PF_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 23:29 - no description available"]
    #[inline(always)]
    pub fn xo32k_xout_capcal_8pf(&self) -> XO32K_XOUT_CAPCAL_8PF_R {
        XO32K_XOUT_CAPCAL_8PF_R::new(((self.bits >> 23) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    pub fn xo32k_xin_trim_valid(&mut self) -> XO32K_XIN_TRIM_VALID_W<0> {
        XO32K_XIN_TRIM_VALID_W::new(self)
    }
    #[doc = "Bits 1:7 - no description available"]
    #[inline(always)]
    pub fn xo32k_xin_capcal_6pf(&mut self) -> XO32K_XIN_CAPCAL_6PF_W<1> {
        XO32K_XIN_CAPCAL_6PF_W::new(self)
    }
    #[doc = "Bits 8:14 - no description available"]
    #[inline(always)]
    pub fn xo32k_xin_capcal_8pf(&mut self) -> XO32K_XIN_CAPCAL_8PF_W<8> {
        XO32K_XIN_CAPCAL_8PF_W::new(self)
    }
    #[doc = "Bit 15 - no description available"]
    #[inline(always)]
    pub fn xo32k_xout_trim_valid(&mut self) -> XO32K_XOUT_TRIM_VALID_W<15> {
        XO32K_XOUT_TRIM_VALID_W::new(self)
    }
    #[doc = "Bits 16:22 - no description available"]
    #[inline(always)]
    pub fn xo32k_xout_capcal_6pf(&mut self) -> XO32K_XOUT_CAPCAL_6PF_W<16> {
        XO32K_XOUT_CAPCAL_6PF_W::new(self)
    }
    #[doc = "Bits 23:29 - no description available"]
    #[inline(always)]
    pub fn xo32k_xout_capcal_8pf(&mut self) -> XO32K_XOUT_CAPCAL_8PF_W<23> {
        XO32K_XOUT_CAPCAL_8PF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "no description available\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [xo_32khz](index.html) module"]
pub struct XO_32KHZ_SPEC;
impl crate::RegisterSpec for XO_32KHZ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [xo_32khz::R](R) reader structure"]
impl crate::Readable for XO_32KHZ_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [xo_32khz::W](W) writer structure"]
impl crate::Writable for XO_32KHZ_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets XO_32KHZ to value 0"]
impl crate::Resettable for XO_32KHZ_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
