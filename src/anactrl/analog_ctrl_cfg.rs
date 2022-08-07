#[doc = "Register `ANALOG_CTRL_CFG` reader"]
pub struct R(crate::R<ANALOG_CTRL_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ANALOG_CTRL_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ANALOG_CTRL_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ANALOG_CTRL_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ANALOG_CTRL_CFG` writer"]
pub struct W(crate::W<ANALOG_CTRL_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ANALOG_CTRL_CFG_SPEC>;
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
impl From<crate::W<ANALOG_CTRL_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ANALOG_CTRL_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CONFIG` reader - Analog blocks configuration."]
pub type CONFIG_R = crate::FieldReader<u32, u32>;
#[doc = "Field `CONFIG` writer - Analog blocks configuration."]
pub type CONFIG_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ANALOG_CTRL_CFG_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Analog blocks configuration."]
    #[inline(always)]
    pub fn config(&self) -> CONFIG_R {
        CONFIG_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Analog blocks configuration."]
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
#[doc = "Various Analog blocks configuration (like FRO 192MHz trimmings source ...)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [analog_ctrl_cfg](index.html) module"]
pub struct ANALOG_CTRL_CFG_SPEC;
impl crate::RegisterSpec for ANALOG_CTRL_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [analog_ctrl_cfg::R](R) reader structure"]
impl crate::Readable for ANALOG_CTRL_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [analog_ctrl_cfg::W](W) writer structure"]
impl crate::Writable for ANALOG_CTRL_CFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ANALOG_CTRL_CFG to value 0"]
impl crate::Resettable for ANALOG_CTRL_CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
