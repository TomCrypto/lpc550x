#[doc = "Register `XO_32MHZ` reader"]
pub struct R(crate::R<XO_32MHZ_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<XO_32MHZ_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<XO_32MHZ_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<XO_32MHZ_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `XO_32MHZ` writer"]
pub struct W(crate::W<XO_32MHZ_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<XO_32MHZ_SPEC>;
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
impl From<crate::W<XO_32MHZ_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<XO_32MHZ_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `XO32M_XIN_TRIM_VALID` reader - no description available."]
pub type XO32M_XIN_TRIM_VALID_R = crate::BitReader<bool>;
#[doc = "Field `XO32M_XIN_TRIM_VALID` writer - no description available."]
pub type XO32M_XIN_TRIM_VALID_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, XO_32MHZ_SPEC, bool, O>;
#[doc = "Field `XO32M_XIN_CAPCAL_6PF` reader - no description available."]
pub type XO32M_XIN_CAPCAL_6PF_R = crate::FieldReader<u8, u8>;
#[doc = "Field `XO32M_XIN_CAPCAL_6PF` writer - no description available."]
pub type XO32M_XIN_CAPCAL_6PF_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, XO_32MHZ_SPEC, u8, u8, 7, O>;
#[doc = "Field `XO32M_XIN_CAPCAL_8PF` reader - no description available."]
pub type XO32M_XIN_CAPCAL_8PF_R = crate::FieldReader<u8, u8>;
#[doc = "Field `XO32M_XIN_CAPCAL_8PF` writer - no description available."]
pub type XO32M_XIN_CAPCAL_8PF_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, XO_32MHZ_SPEC, u8, u8, 7, O>;
#[doc = "Field `XO32M_XOUT_TRIM_VALID` reader - no description available."]
pub type XO32M_XOUT_TRIM_VALID_R = crate::BitReader<bool>;
#[doc = "Field `XO32M_XOUT_TRIM_VALID` writer - no description available."]
pub type XO32M_XOUT_TRIM_VALID_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, XO_32MHZ_SPEC, bool, O>;
#[doc = "Field `XO32M_XOUT_CAPCAL_6PF` reader - no description available."]
pub type XO32M_XOUT_CAPCAL_6PF_R = crate::FieldReader<u8, u8>;
#[doc = "Field `XO32M_XOUT_CAPCAL_6PF` writer - no description available."]
pub type XO32M_XOUT_CAPCAL_6PF_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, XO_32MHZ_SPEC, u8, u8, 7, O>;
#[doc = "Field `XO32M_XOUT_CAPCAL_8PF` reader - no description available."]
pub type XO32M_XOUT_CAPCAL_8PF_R = crate::FieldReader<u8, u8>;
#[doc = "Field `XO32M_XOUT_CAPCAL_8PF` writer - no description available."]
pub type XO32M_XOUT_CAPCAL_8PF_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, XO_32MHZ_SPEC, u8, u8, 7, O>;
#[doc = "Field `XO32M_XO_SLAVE_STATUS` reader - no description available."]
pub type XO32M_XO_SLAVE_STATUS_R = crate::BitReader<bool>;
#[doc = "Field `XO32M_XO_SLAVE_STATUS` writer - no description available."]
pub type XO32M_XO_SLAVE_STATUS_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, XO_32MHZ_SPEC, bool, O>;
#[doc = "Field `XO32M_XO_AC_BUF_STATUS` reader - no description available."]
pub type XO32M_XO_AC_BUF_STATUS_R = crate::BitReader<bool>;
#[doc = "Field `XO32M_XO_AC_BUF_STATUS` writer - no description available."]
pub type XO32M_XO_AC_BUF_STATUS_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, XO_32MHZ_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - no description available."]
    #[inline(always)]
    pub fn xo32m_xin_trim_valid(&self) -> XO32M_XIN_TRIM_VALID_R {
        XO32M_XIN_TRIM_VALID_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:7 - no description available."]
    #[inline(always)]
    pub fn xo32m_xin_capcal_6pf(&self) -> XO32M_XIN_CAPCAL_6PF_R {
        XO32M_XIN_CAPCAL_6PF_R::new(((self.bits >> 1) & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - no description available."]
    #[inline(always)]
    pub fn xo32m_xin_capcal_8pf(&self) -> XO32M_XIN_CAPCAL_8PF_R {
        XO32M_XIN_CAPCAL_8PF_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bit 15 - no description available."]
    #[inline(always)]
    pub fn xo32m_xout_trim_valid(&self) -> XO32M_XOUT_TRIM_VALID_R {
        XO32M_XOUT_TRIM_VALID_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:22 - no description available."]
    #[inline(always)]
    pub fn xo32m_xout_capcal_6pf(&self) -> XO32M_XOUT_CAPCAL_6PF_R {
        XO32M_XOUT_CAPCAL_6PF_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 23:29 - no description available."]
    #[inline(always)]
    pub fn xo32m_xout_capcal_8pf(&self) -> XO32M_XOUT_CAPCAL_8PF_R {
        XO32M_XOUT_CAPCAL_8PF_R::new(((self.bits >> 23) & 0x7f) as u8)
    }
    #[doc = "Bit 30 - no description available."]
    #[inline(always)]
    pub fn xo32m_xo_slave_status(&self) -> XO32M_XO_SLAVE_STATUS_R {
        XO32M_XO_SLAVE_STATUS_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - no description available."]
    #[inline(always)]
    pub fn xo32m_xo_ac_buf_status(&self) -> XO32M_XO_AC_BUF_STATUS_R {
        XO32M_XO_AC_BUF_STATUS_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - no description available."]
    #[inline(always)]
    pub fn xo32m_xin_trim_valid(&mut self) -> XO32M_XIN_TRIM_VALID_W<0> {
        XO32M_XIN_TRIM_VALID_W::new(self)
    }
    #[doc = "Bits 1:7 - no description available."]
    #[inline(always)]
    pub fn xo32m_xin_capcal_6pf(&mut self) -> XO32M_XIN_CAPCAL_6PF_W<1> {
        XO32M_XIN_CAPCAL_6PF_W::new(self)
    }
    #[doc = "Bits 8:14 - no description available."]
    #[inline(always)]
    pub fn xo32m_xin_capcal_8pf(&mut self) -> XO32M_XIN_CAPCAL_8PF_W<8> {
        XO32M_XIN_CAPCAL_8PF_W::new(self)
    }
    #[doc = "Bit 15 - no description available."]
    #[inline(always)]
    pub fn xo32m_xout_trim_valid(&mut self) -> XO32M_XOUT_TRIM_VALID_W<15> {
        XO32M_XOUT_TRIM_VALID_W::new(self)
    }
    #[doc = "Bits 16:22 - no description available."]
    #[inline(always)]
    pub fn xo32m_xout_capcal_6pf(&mut self) -> XO32M_XOUT_CAPCAL_6PF_W<16> {
        XO32M_XOUT_CAPCAL_6PF_W::new(self)
    }
    #[doc = "Bits 23:29 - no description available."]
    #[inline(always)]
    pub fn xo32m_xout_capcal_8pf(&mut self) -> XO32M_XOUT_CAPCAL_8PF_W<23> {
        XO32M_XOUT_CAPCAL_8PF_W::new(self)
    }
    #[doc = "Bit 30 - no description available."]
    #[inline(always)]
    pub fn xo32m_xo_slave_status(&mut self) -> XO32M_XO_SLAVE_STATUS_W<30> {
        XO32M_XO_SLAVE_STATUS_W::new(self)
    }
    #[doc = "Bit 31 - no description available."]
    #[inline(always)]
    pub fn xo32m_xo_ac_buf_status(&mut self) -> XO32M_XO_AC_BUF_STATUS_W<31> {
        XO32M_XO_AC_BUF_STATUS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "no description available.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [xo_32mhz](index.html) module"]
pub struct XO_32MHZ_SPEC;
impl crate::RegisterSpec for XO_32MHZ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [xo_32mhz::R](R) reader structure"]
impl crate::Readable for XO_32MHZ_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [xo_32mhz::W](W) writer structure"]
impl crate::Writable for XO_32MHZ_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets XO_32MHZ to value 0"]
impl crate::Resettable for XO_32MHZ_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
