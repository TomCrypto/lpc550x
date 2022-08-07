#[doc = "Register `PVT_MONITOR_1_RINGO` reader"]
pub struct R(crate::R<PVT_MONITOR_1_PVT_MONITOR_1_RINGO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PVT_MONITOR_1_PVT_MONITOR_1_RINGO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PVT_MONITOR_1_PVT_MONITOR_1_RINGO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PVT_MONITOR_1_PVT_MONITOR_1_RINGO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PVT_MONITOR_1_RINGO` writer"]
pub struct W(crate::W<PVT_MONITOR_1_PVT_MONITOR_1_RINGO_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PVT_MONITOR_1_PVT_MONITOR_1_RINGO_SPEC>;
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
impl From<crate::W<PVT_MONITOR_1_PVT_MONITOR_1_RINGO_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PVT_MONITOR_1_PVT_MONITOR_1_RINGO_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RINGO_VALID` reader - no description available"]
pub type RINGO_VALID_R = crate::BitReader<bool>;
#[doc = "Field `RINGO_VALID` writer - no description available"]
pub type RINGO_VALID_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PVT_MONITOR_1_PVT_MONITOR_1_RINGO_SPEC, bool, O>;
#[doc = "Field `RINGO_FREQ_HZ` reader - no description available"]
pub type RINGO_FREQ_HZ_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RINGO_FREQ_HZ` writer - no description available"]
pub type RINGO_FREQ_HZ_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PVT_MONITOR_1_PVT_MONITOR_1_RINGO_SPEC, u32, u32, 31, O>;
impl R {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    pub fn ringo_valid(&self) -> RINGO_VALID_R {
        RINGO_VALID_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:31 - no description available"]
    #[inline(always)]
    pub fn ringo_freq_hz(&self) -> RINGO_FREQ_HZ_R {
        RINGO_FREQ_HZ_R::new(((self.bits >> 1) & 0x7fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    pub fn ringo_valid(&mut self) -> RINGO_VALID_W<0> {
        RINGO_VALID_W::new(self)
    }
    #[doc = "Bits 1:31 - no description available"]
    #[inline(always)]
    pub fn ringo_freq_hz(&mut self) -> RINGO_FREQ_HZ_W<1> {
        RINGO_FREQ_HZ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "no description available\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pvt_monitor_1_pvt_monitor_1_ringo](index.html) module"]
pub struct PVT_MONITOR_1_PVT_MONITOR_1_RINGO_SPEC;
impl crate::RegisterSpec for PVT_MONITOR_1_PVT_MONITOR_1_RINGO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pvt_monitor_1_pvt_monitor_1_ringo::R](R) reader structure"]
impl crate::Readable for PVT_MONITOR_1_PVT_MONITOR_1_RINGO_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pvt_monitor_1_pvt_monitor_1_ringo::W](W) writer structure"]
impl crate::Writable for PVT_MONITOR_1_PVT_MONITOR_1_RINGO_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PVT_MONITOR_1_RINGO to value 0"]
impl crate::Resettable for PVT_MONITOR_1_PVT_MONITOR_1_RINGO_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
