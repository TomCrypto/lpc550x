#[doc = "Register `USBCFG` reader"]
pub struct R(crate::R<USBCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USBCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USBCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USBCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USBCFG` writer"]
pub struct W(crate::W<USBCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USBCFG_SPEC>;
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
impl From<crate::W<USBCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USBCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `XO32M_READY_TIME_OUT_MS` reader - no description available."]
pub type XO32M_READY_TIME_OUT_MS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `XO32M_READY_TIME_OUT_MS` writer - no description available."]
pub type XO32M_READY_TIME_OUT_MS_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, USBCFG_SPEC, u8, u8, 8, O>;
#[doc = "Field `USB_SPEED` reader - USB_SPEED\\[7:0\\]= 0x00 : USB High Speed Module used for ISP 0x01 : USB Full SPeed Module used for ISP 0x02 : Neither USB High Speed module nor USB Full Speed module used for ISP 0x03 - 0xFF : RESERVED."]
pub type USB_SPEED_R = crate::FieldReader<u8, u8>;
#[doc = "Field `USB_SPEED` writer - USB_SPEED\\[7:0\\]= 0x00 : USB High Speed Module used for ISP 0x01 : USB Full SPeed Module used for ISP 0x02 : Neither USB High Speed module nor USB Full Speed module used for ISP 0x03 - 0xFF : RESERVED."]
pub type USB_SPEED_W<'a, const O: u8> = crate::FieldWriter<'a, u32, USBCFG_SPEC, u8, u8, 8, O>;
#[doc = "Field `USB_USE_XO32M_CAPA_BANKS` reader - Enable the use of Crystal 32 MHz internal Capa Banks during the configuration of the High Speed USB for ISP: 0: Disable Crystal 32 MHz CapaBanks. 1: Enable Crystal 32 MHz CapaBanks."]
pub type USB_USE_XO32M_CAPA_BANKS_R = crate::BitReader<bool>;
#[doc = "Field `USB_USE_XO32M_CAPA_BANKS` writer - Enable the use of Crystal 32 MHz internal Capa Banks during the configuration of the High Speed USB for ISP: 0: Disable Crystal 32 MHz CapaBanks. 1: Enable Crystal 32 MHz CapaBanks."]
pub type USB_USE_XO32M_CAPA_BANKS_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, USBCFG_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:7 - no description available."]
    #[inline(always)]
    pub fn xo32m_ready_time_out_ms(&self) -> XO32M_READY_TIME_OUT_MS_R {
        XO32M_READY_TIME_OUT_MS_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - USB_SPEED\\[7:0\\]= 0x00 : USB High Speed Module used for ISP 0x01 : USB Full SPeed Module used for ISP 0x02 : Neither USB High Speed module nor USB Full Speed module used for ISP 0x03 - 0xFF : RESERVED."]
    #[inline(always)]
    pub fn usb_speed(&self) -> USB_SPEED_R {
        USB_SPEED_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 16 - Enable the use of Crystal 32 MHz internal Capa Banks during the configuration of the High Speed USB for ISP: 0: Disable Crystal 32 MHz CapaBanks. 1: Enable Crystal 32 MHz CapaBanks."]
    #[inline(always)]
    pub fn usb_use_xo32m_capa_banks(&self) -> USB_USE_XO32M_CAPA_BANKS_R {
        USB_USE_XO32M_CAPA_BANKS_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - no description available."]
    #[inline(always)]
    pub fn xo32m_ready_time_out_ms(&mut self) -> XO32M_READY_TIME_OUT_MS_W<0> {
        XO32M_READY_TIME_OUT_MS_W::new(self)
    }
    #[doc = "Bits 8:15 - USB_SPEED\\[7:0\\]= 0x00 : USB High Speed Module used for ISP 0x01 : USB Full SPeed Module used for ISP 0x02 : Neither USB High Speed module nor USB Full Speed module used for ISP 0x03 - 0xFF : RESERVED."]
    #[inline(always)]
    pub fn usb_speed(&mut self) -> USB_SPEED_W<8> {
        USB_SPEED_W::new(self)
    }
    #[doc = "Bit 16 - Enable the use of Crystal 32 MHz internal Capa Banks during the configuration of the High Speed USB for ISP: 0: Disable Crystal 32 MHz CapaBanks. 1: Enable Crystal 32 MHz CapaBanks."]
    #[inline(always)]
    pub fn usb_use_xo32m_capa_banks(&mut self) -> USB_USE_XO32M_CAPA_BANKS_W<16> {
        USB_USE_XO32M_CAPA_BANKS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "no description available.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbcfg](index.html) module"]
pub struct USBCFG_SPEC;
impl crate::RegisterSpec for USBCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usbcfg::R](R) reader structure"]
impl crate::Readable for USBCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usbcfg::W](W) writer structure"]
impl crate::Writable for USBCFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets USBCFG to value 0"]
impl crate::Resettable for USBCFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
