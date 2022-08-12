#[doc = "Register `PVT_MONITOR_0_DELAYS_MSB` reader"]
pub struct R(crate::R<PVT_MONITOR_0_DELAYS_MSB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PVT_MONITOR_0_DELAYS_MSB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PVT_MONITOR_0_DELAYS_MSB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PVT_MONITOR_0_DELAYS_MSB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PVT_MONITOR_0_DELAYS_MSB` writer"]
pub struct W(crate::W<PVT_MONITOR_0_DELAYS_MSB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PVT_MONITOR_0_DELAYS_MSB_SPEC>;
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
impl From<crate::W<PVT_MONITOR_0_DELAYS_MSB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PVT_MONITOR_0_DELAYS_MSB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DELAY_3` reader - Delay in us."]
pub type DELAY_3_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DELAY_3` writer - Delay in us."]
pub type DELAY_3_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PVT_MONITOR_0_DELAYS_MSB_SPEC, u16, u16, 10, O>;
#[doc = "Field `DELAY_4` reader - Delay in us."]
pub type DELAY_4_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DELAY_4` writer - Delay in us."]
pub type DELAY_4_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PVT_MONITOR_0_DELAYS_MSB_SPEC, u16, u16, 10, O>;
#[doc = "Field `DELAY_5` reader - Delay in us."]
pub type DELAY_5_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DELAY_5` writer - Delay in us."]
pub type DELAY_5_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PVT_MONITOR_0_DELAYS_MSB_SPEC, u16, u16, 10, O>;
impl R {
    #[doc = "Bits 0:9 - Delay in us."]
    #[inline(always)]
    pub fn delay_3(&self) -> DELAY_3_R {
        DELAY_3_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 10:19 - Delay in us."]
    #[inline(always)]
    pub fn delay_4(&self) -> DELAY_4_R {
        DELAY_4_R::new(((self.bits >> 10) & 0x03ff) as u16)
    }
    #[doc = "Bits 20:29 - Delay in us."]
    #[inline(always)]
    pub fn delay_5(&self) -> DELAY_5_R {
        DELAY_5_R::new(((self.bits >> 20) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Delay in us."]
    #[inline(always)]
    pub fn delay_3(&mut self) -> DELAY_3_W<0> {
        DELAY_3_W::new(self)
    }
    #[doc = "Bits 10:19 - Delay in us."]
    #[inline(always)]
    pub fn delay_4(&mut self) -> DELAY_4_W<10> {
        DELAY_4_W::new(self)
    }
    #[doc = "Bits 20:29 - Delay in us."]
    #[inline(always)]
    pub fn delay_5(&mut self) -> DELAY_5_W<20> {
        DELAY_5_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "no description available.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pvt_monitor_0_delays_msb](index.html) module"]
pub struct PVT_MONITOR_0_DELAYS_MSB_SPEC;
impl crate::RegisterSpec for PVT_MONITOR_0_DELAYS_MSB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pvt_monitor_0_delays_msb::R](R) reader structure"]
impl crate::Readable for PVT_MONITOR_0_DELAYS_MSB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pvt_monitor_0_delays_msb::W](W) writer structure"]
impl crate::Writable for PVT_MONITOR_0_DELAYS_MSB_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PVT_MONITOR_0_DELAYS_MSB to value 0"]
impl crate::Resettable for PVT_MONITOR_0_DELAYS_MSB_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
