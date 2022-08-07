#[doc = "Register `DEVICE_TYPE` reader"]
pub struct R(crate::R<DEVICE_TYPE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DEVICE_TYPE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DEVICE_TYPE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DEVICE_TYPE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DEVICE_TYPE` writer"]
pub struct W(crate::W<DEVICE_TYPE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DEVICE_TYPE_SPEC>;
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
impl From<crate::W<DEVICE_TYPE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DEVICE_TYPE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DEVICE_TYPE_NUM` reader - Device type number. (E.g : LPC5569 stored as 5569 decimal)"]
pub type DEVICE_TYPE_NUM_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DEVICE_TYPE_NUM` writer - Device type number. (E.g : LPC5569 stored as 5569 decimal)"]
pub type DEVICE_TYPE_NUM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DEVICE_TYPE_SPEC, u16, u16, 16, O>;
#[doc = "Field `DEVICE_TYPE_SEC` reader - Security device type: 0: LPC55xxx (Non Secure Familly) 1: LPC55Sxxx (Secure Familly)"]
pub type DEVICE_TYPE_SEC_R = crate::BitReader<bool>;
#[doc = "Field `DEVICE_TYPE_SEC` writer - Security device type: 0: LPC55xxx (Non Secure Familly) 1: LPC55Sxxx (Secure Familly)"]
pub type DEVICE_TYPE_SEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, DEVICE_TYPE_SPEC, bool, O>;
#[doc = "Field `DEVICE_TYPE_PKG` reader - Device package type: 0000 : HLQFP 0001 : HTQFP 0010 : HVQFN 0100 : VFBGA 1000 : WLCSP"]
pub type DEVICE_TYPE_PKG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DEVICE_TYPE_PKG` writer - Device package type: 0000 : HLQFP 0001 : HTQFP 0010 : HVQFN 0100 : VFBGA 1000 : WLCSP"]
pub type DEVICE_TYPE_PKG_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DEVICE_TYPE_SPEC, u8, u8, 4, O>;
#[doc = "Field `DEVICE_TYPE_PIN` reader - Number of pins on the package."]
pub type DEVICE_TYPE_PIN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DEVICE_TYPE_PIN` writer - Number of pins on the package."]
pub type DEVICE_TYPE_PIN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DEVICE_TYPE_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:15 - Device type number. (E.g : LPC5569 stored as 5569 decimal)"]
    #[inline(always)]
    pub fn device_type_num(&self) -> DEVICE_TYPE_NUM_R {
        DEVICE_TYPE_NUM_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - Security device type: 0: LPC55xxx (Non Secure Familly) 1: LPC55Sxxx (Secure Familly)"]
    #[inline(always)]
    pub fn device_type_sec(&self) -> DEVICE_TYPE_SEC_R {
        DEVICE_TYPE_SEC_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 20:23 - Device package type: 0000 : HLQFP 0001 : HTQFP 0010 : HVQFN 0100 : VFBGA 1000 : WLCSP"]
    #[inline(always)]
    pub fn device_type_pkg(&self) -> DEVICE_TYPE_PKG_R {
        DEVICE_TYPE_PKG_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:31 - Number of pins on the package."]
    #[inline(always)]
    pub fn device_type_pin(&self) -> DEVICE_TYPE_PIN_R {
        DEVICE_TYPE_PIN_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - Device type number. (E.g : LPC5569 stored as 5569 decimal)"]
    #[inline(always)]
    pub fn device_type_num(&mut self) -> DEVICE_TYPE_NUM_W<0> {
        DEVICE_TYPE_NUM_W::new(self)
    }
    #[doc = "Bit 16 - Security device type: 0: LPC55xxx (Non Secure Familly) 1: LPC55Sxxx (Secure Familly)"]
    #[inline(always)]
    pub fn device_type_sec(&mut self) -> DEVICE_TYPE_SEC_W<16> {
        DEVICE_TYPE_SEC_W::new(self)
    }
    #[doc = "Bits 20:23 - Device package type: 0000 : HLQFP 0001 : HTQFP 0010 : HVQFN 0100 : VFBGA 1000 : WLCSP"]
    #[inline(always)]
    pub fn device_type_pkg(&mut self) -> DEVICE_TYPE_PKG_W<20> {
        DEVICE_TYPE_PKG_W::new(self)
    }
    #[doc = "Bits 24:31 - Number of pins on the package."]
    #[inline(always)]
    pub fn device_type_pin(&mut self) -> DEVICE_TYPE_PIN_W<24> {
        DEVICE_TYPE_PIN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "no description available\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [device_type](index.html) module"]
pub struct DEVICE_TYPE_SPEC;
impl crate::RegisterSpec for DEVICE_TYPE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [device_type::R](R) reader structure"]
impl crate::Readable for DEVICE_TYPE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [device_type::W](W) writer structure"]
impl crate::Writable for DEVICE_TYPE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DEVICE_TYPE to value 0"]
impl crate::Resettable for DEVICE_TYPE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
