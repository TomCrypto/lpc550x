#[doc = "Register `PVT_MONITOR_1_DELAYS_LSB` reader"]
pub struct R(crate::R<PVT_MONITOR_1_PVT_MONITOR_1_DELAYS_LSB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PVT_MONITOR_1_PVT_MONITOR_1_DELAYS_LSB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PVT_MONITOR_1_PVT_MONITOR_1_DELAYS_LSB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PVT_MONITOR_1_PVT_MONITOR_1_DELAYS_LSB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PVT_MONITOR_1_DELAYS_LSB` writer"]
pub struct W(crate::W<PVT_MONITOR_1_PVT_MONITOR_1_DELAYS_LSB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PVT_MONITOR_1_PVT_MONITOR_1_DELAYS_LSB_SPEC>;
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
impl From<crate::W<PVT_MONITOR_1_PVT_MONITOR_1_DELAYS_LSB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PVT_MONITOR_1_PVT_MONITOR_1_DELAYS_LSB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DELAY_VALID` reader - no description available"]
pub type DELAY_VALID_R = crate::BitReader<bool>;
#[doc = "Field `DELAY_VALID` writer - no description available"]
pub type DELAY_VALID_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PVT_MONITOR_1_PVT_MONITOR_1_DELAYS_LSB_SPEC, bool, O>;
#[doc = "Field `DELAY_0` reader - Delay in us."]
pub type DELAY_0_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DELAY_0` writer - Delay in us."]
pub type DELAY_0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PVT_MONITOR_1_PVT_MONITOR_1_DELAYS_LSB_SPEC, u16, u16, 10, O>;
#[doc = "Field `DELAY_1` reader - Delay in us."]
pub type DELAY_1_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DELAY_1` writer - Delay in us."]
pub type DELAY_1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PVT_MONITOR_1_PVT_MONITOR_1_DELAYS_LSB_SPEC, u16, u16, 10, O>;
#[doc = "Field `DELAY_2` reader - Delay in us."]
pub type DELAY_2_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DELAY_2` writer - Delay in us."]
pub type DELAY_2_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PVT_MONITOR_1_PVT_MONITOR_1_DELAYS_LSB_SPEC, u16, u16, 10, O>;
impl R {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    pub fn delay_valid(&self) -> DELAY_VALID_R {
        DELAY_VALID_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:10 - Delay in us."]
    #[inline(always)]
    pub fn delay_0(&self) -> DELAY_0_R {
        DELAY_0_R::new(((self.bits >> 1) & 0x03ff) as u16)
    }
    #[doc = "Bits 11:20 - Delay in us."]
    #[inline(always)]
    pub fn delay_1(&self) -> DELAY_1_R {
        DELAY_1_R::new(((self.bits >> 11) & 0x03ff) as u16)
    }
    #[doc = "Bits 21:30 - Delay in us."]
    #[inline(always)]
    pub fn delay_2(&self) -> DELAY_2_R {
        DELAY_2_R::new(((self.bits >> 21) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    pub fn delay_valid(&mut self) -> DELAY_VALID_W<0> {
        DELAY_VALID_W::new(self)
    }
    #[doc = "Bits 1:10 - Delay in us."]
    #[inline(always)]
    pub fn delay_0(&mut self) -> DELAY_0_W<1> {
        DELAY_0_W::new(self)
    }
    #[doc = "Bits 11:20 - Delay in us."]
    #[inline(always)]
    pub fn delay_1(&mut self) -> DELAY_1_W<11> {
        DELAY_1_W::new(self)
    }
    #[doc = "Bits 21:30 - Delay in us."]
    #[inline(always)]
    pub fn delay_2(&mut self) -> DELAY_2_W<21> {
        DELAY_2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "no description available\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pvt_monitor_1_pvt_monitor_1_delays_lsb](index.html) module"]
pub struct PVT_MONITOR_1_PVT_MONITOR_1_DELAYS_LSB_SPEC;
impl crate::RegisterSpec for PVT_MONITOR_1_PVT_MONITOR_1_DELAYS_LSB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pvt_monitor_1_pvt_monitor_1_delays_lsb::R](R) reader structure"]
impl crate::Readable for PVT_MONITOR_1_PVT_MONITOR_1_DELAYS_LSB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pvt_monitor_1_pvt_monitor_1_delays_lsb::W](W) writer structure"]
impl crate::Writable for PVT_MONITOR_1_PVT_MONITOR_1_DELAYS_LSB_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PVT_MONITOR_1_DELAYS_LSB to value 0"]
impl crate::Resettable for PVT_MONITOR_1_PVT_MONITOR_1_DELAYS_LSB_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
