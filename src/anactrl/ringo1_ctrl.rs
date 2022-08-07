#[doc = "Register `RINGO1_CTRL` reader"]
pub struct R(crate::R<RINGO1_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RINGO1_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RINGO1_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RINGO1_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RINGO1_CTRL` writer"]
pub struct W(crate::W<RINGO1_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RINGO1_CTRL_SPEC>;
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
impl From<crate::W<RINGO1_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RINGO1_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CONFIG` reader - Second ring oscillator configuration."]
pub type CONFIG_R = crate::FieldReader<u32, u32>;
#[doc = "Field `CONFIG` writer - Second ring oscillator configuration."]
pub type CONFIG_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RINGO1_CTRL_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Second ring oscillator configuration."]
    #[inline(always)]
    pub fn config(&self) -> CONFIG_R {
        CONFIG_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Second ring oscillator configuration."]
    #[inline(always)]
    pub fn config(&mut self) -> CONFIG_W<0> {
        CONFIG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Second Ring Oscillator module control register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ringo1_ctrl](index.html) module"]
pub struct RINGO1_CTRL_SPEC;
impl crate::RegisterSpec for RINGO1_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ringo1_ctrl::R](R) reader structure"]
impl crate::Readable for RINGO1_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ringo1_ctrl::W](W) writer structure"]
impl crate::Writable for RINGO1_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RINGO1_CTRL to value 0x40"]
impl crate::Resettable for RINGO1_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x40
    }
}
