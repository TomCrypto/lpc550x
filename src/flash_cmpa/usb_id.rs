#[doc = "Register `USB_ID` reader"]
pub struct R(crate::R<USB_ID_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USB_ID_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USB_ID_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USB_ID_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USB_ID` writer"]
pub struct W(crate::W<USB_ID_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USB_ID_SPEC>;
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
impl From<crate::W<USB_ID_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USB_ID_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USB_VENDOR_ID` reader - no description available."]
pub type USB_VENDOR_ID_R = crate::FieldReader<u16, u16>;
#[doc = "Field `USB_VENDOR_ID` writer - no description available."]
pub type USB_VENDOR_ID_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, USB_ID_SPEC, u16, u16, 16, O>;
#[doc = "Field `USB_PRODUCT_ID` reader - no description available."]
pub type USB_PRODUCT_ID_R = crate::FieldReader<u16, u16>;
#[doc = "Field `USB_PRODUCT_ID` writer - no description available."]
pub type USB_PRODUCT_ID_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, USB_ID_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - no description available."]
    #[inline(always)]
    pub fn usb_vendor_id(&self) -> USB_VENDOR_ID_R {
        USB_VENDOR_ID_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - no description available."]
    #[inline(always)]
    pub fn usb_product_id(&self) -> USB_PRODUCT_ID_R {
        USB_PRODUCT_ID_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - no description available."]
    #[inline(always)]
    pub fn usb_vendor_id(&mut self) -> USB_VENDOR_ID_W<0> {
        USB_VENDOR_ID_W::new(self)
    }
    #[doc = "Bits 16:31 - no description available."]
    #[inline(always)]
    pub fn usb_product_id(&mut self) -> USB_PRODUCT_ID_W<16> {
        USB_PRODUCT_ID_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "no description available.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb_id](index.html) module"]
pub struct USB_ID_SPEC;
impl crate::RegisterSpec for USB_ID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usb_id::R](R) reader structure"]
impl crate::Readable for USB_ID_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usb_id::W](W) writer structure"]
impl crate::Writable for USB_ID_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets USB_ID to value 0"]
impl crate::Resettable for USB_ID_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
