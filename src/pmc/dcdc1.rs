#[doc = "Register `DCDC1` reader"]
pub struct R(crate::R<DCDC1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DCDC1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DCDC1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DCDC1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DCDC1` writer"]
pub struct W(crate::W<DCDC1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DCDC1_SPEC>;
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
impl From<crate::W<DCDC1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DCDC1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CONFIG2` reader - DCDC configuration."]
pub type CONFIG2_R = crate::FieldReader<u32, u32>;
#[doc = "Field `CONFIG2` writer - DCDC configuration."]
pub type CONFIG2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DCDC1_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - DCDC configuration."]
    #[inline(always)]
    pub fn config2(&self) -> CONFIG2_R {
        CONFIG2_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - DCDC configuration."]
    #[inline(always)]
    pub fn config2(&mut self) -> CONFIG2_W<0> {
        CONFIG2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DCDC (second) control register \\[Reset by: PoR, Pin Reset, Brown Out Detectors Reset, Deep Power Down Reset, Software Reset\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcdc1](index.html) module"]
pub struct DCDC1_SPEC;
impl crate::RegisterSpec for DCDC1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dcdc1::R](R) reader structure"]
impl crate::Readable for DCDC1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dcdc1::W](W) writer structure"]
impl crate::Writable for DCDC1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DCDC1 to value 0x0180_3a98"]
impl crate::Resettable for DCDC1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0180_3a98
    }
}
