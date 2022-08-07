#[doc = "Register `XTAL32K` reader"]
pub struct R(crate::R<XTAL32K_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<XTAL32K_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<XTAL32K_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<XTAL32K_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `XTAL32K` writer"]
pub struct W(crate::W<XTAL32K_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<XTAL32K_SPEC>;
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
impl From<crate::W<XTAL32K_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<XTAL32K_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CAPBANKIN` reader - Capa bank setting input."]
pub type CAPBANKIN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CAPBANKIN` writer - Capa bank setting input."]
pub type CAPBANKIN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, XTAL32K_SPEC, u8, u8, 7, O>;
#[doc = "Field `CAPBANKOUT` reader - Capa bank setting output."]
pub type CAPBANKOUT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CAPBANKOUT` writer - Capa bank setting output."]
pub type CAPBANKOUT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, XTAL32K_SPEC, u8, u8, 7, O>;
impl R {
    #[doc = "Bits 8:14 - Capa bank setting input."]
    #[inline(always)]
    pub fn capbankin(&self) -> CAPBANKIN_R {
        CAPBANKIN_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 15:21 - Capa bank setting output."]
    #[inline(always)]
    pub fn capbankout(&self) -> CAPBANKOUT_R {
        CAPBANKOUT_R::new(((self.bits >> 15) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 8:14 - Capa bank setting input."]
    #[inline(always)]
    pub fn capbankin(&mut self) -> CAPBANKIN_W<8> {
        CAPBANKIN_W::new(self)
    }
    #[doc = "Bits 15:21 - Capa bank setting output."]
    #[inline(always)]
    pub fn capbankout(&mut self) -> CAPBANKOUT_W<15> {
        CAPBANKOUT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "32 KHz Crystal oscillator (XTAL) control register \\[Reset by: PoR, Brown Out Detectors Reset\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [xtal32k](index.html) module"]
pub struct XTAL32K_SPEC;
impl crate::RegisterSpec for XTAL32K_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [xtal32k::R](R) reader structure"]
impl crate::Readable for XTAL32K_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [xtal32k::W](W) writer structure"]
impl crate::Writable for XTAL32K_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets XTAL32K to value 0x0020_4052"]
impl crate::Resettable for XTAL32K_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0020_4052
    }
}
