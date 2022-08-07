#[doc = "Register `CPU0STCKCAL` reader"]
pub struct R(crate::R<CPU0STCKCAL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CPU0STCKCAL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CPU0STCKCAL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CPU0STCKCAL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CPU0STCKCAL` writer"]
pub struct W(crate::W<CPU0STCKCAL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CPU0STCKCAL_SPEC>;
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
impl From<crate::W<CPU0STCKCAL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CPU0STCKCAL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TENMS` reader - Reload value for 10ms (100Hz) timing, subject to system clock skew errors. If the value reads as zero, the calibration value is not known."]
pub type TENMS_R = crate::FieldReader<u32, u32>;
#[doc = "Field `TENMS` writer - Reload value for 10ms (100Hz) timing, subject to system clock skew errors. If the value reads as zero, the calibration value is not known."]
pub type TENMS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CPU0STCKCAL_SPEC, u32, u32, 24, O>;
#[doc = "Field `SKEW` reader - Initial value for the Systick timer."]
pub type SKEW_R = crate::BitReader<bool>;
#[doc = "Field `SKEW` writer - Initial value for the Systick timer."]
pub type SKEW_W<'a, const O: u8> = crate::BitWriter<'a, u32, CPU0STCKCAL_SPEC, bool, O>;
#[doc = "Field `NOREF` reader - Indicates whether the device provides a reference clock to the processor: 0 = reference clock provided; 1 = no reference clock provided."]
pub type NOREF_R = crate::BitReader<bool>;
#[doc = "Field `NOREF` writer - Indicates whether the device provides a reference clock to the processor: 0 = reference clock provided; 1 = no reference clock provided."]
pub type NOREF_W<'a, const O: u8> = crate::BitWriter<'a, u32, CPU0STCKCAL_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:23 - Reload value for 10ms (100Hz) timing, subject to system clock skew errors. If the value reads as zero, the calibration value is not known."]
    #[inline(always)]
    pub fn tenms(&self) -> TENMS_R {
        TENMS_R::new((self.bits & 0x00ff_ffff) as u32)
    }
    #[doc = "Bit 24 - Initial value for the Systick timer."]
    #[inline(always)]
    pub fn skew(&self) -> SKEW_R {
        SKEW_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Indicates whether the device provides a reference clock to the processor: 0 = reference clock provided; 1 = no reference clock provided."]
    #[inline(always)]
    pub fn noref(&self) -> NOREF_R {
        NOREF_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:23 - Reload value for 10ms (100Hz) timing, subject to system clock skew errors. If the value reads as zero, the calibration value is not known."]
    #[inline(always)]
    pub fn tenms(&mut self) -> TENMS_W<0> {
        TENMS_W::new(self)
    }
    #[doc = "Bit 24 - Initial value for the Systick timer."]
    #[inline(always)]
    pub fn skew(&mut self) -> SKEW_W<24> {
        SKEW_W::new(self)
    }
    #[doc = "Bit 25 - Indicates whether the device provides a reference clock to the processor: 0 = reference clock provided; 1 = no reference clock provided."]
    #[inline(always)]
    pub fn noref(&mut self) -> NOREF_W<25> {
        NOREF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "System tick calibration for secure part of CPU0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpu0stckcal](index.html) module"]
pub struct CPU0STCKCAL_SPEC;
impl crate::RegisterSpec for CPU0STCKCAL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cpu0stckcal::R](R) reader structure"]
impl crate::Readable for CPU0STCKCAL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cpu0stckcal::W](W) writer structure"]
impl crate::Writable for CPU0STCKCAL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CPU0STCKCAL to value 0"]
impl crate::Resettable for CPU0STCKCAL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
