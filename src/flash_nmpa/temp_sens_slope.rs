#[doc = "Register `TEMP_SENS_SLOPE` reader"]
pub struct R(crate::R<TEMP_SENS_SLOPE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TEMP_SENS_SLOPE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TEMP_SENS_SLOPE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TEMP_SENS_SLOPE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TEMP_SENS_SLOPE` writer"]
pub struct W(crate::W<TEMP_SENS_SLOPE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TEMP_SENS_SLOPE_SPEC>;
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
impl From<crate::W<TEMP_SENS_SLOPE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TEMP_SENS_SLOPE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VALID` reader - no description available."]
pub type VALID_R = crate::BitReader<bool>;
#[doc = "Field `VALID` writer - no description available."]
pub type VALID_W<'a, const O: u8> = crate::BitWriter<'a, u32, TEMP_SENS_SLOPE_SPEC, bool, O>;
#[doc = "Field `SLOPE_x1024` reader - SLOPE_x1024\\[30:0\\]"]
pub type SLOPE_X1024_R = crate::FieldReader<u32, u32>;
#[doc = "Field `SLOPE_x1024` writer - SLOPE_x1024\\[30:0\\]"]
pub type SLOPE_X1024_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TEMP_SENS_SLOPE_SPEC, u32, u32, 31, O>;
impl R {
    #[doc = "Bit 0 - no description available."]
    #[inline(always)]
    pub fn valid(&self) -> VALID_R {
        VALID_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:31 - SLOPE_x1024\\[30:0\\]"]
    #[inline(always)]
    pub fn slope_x1024(&self) -> SLOPE_X1024_R {
        SLOPE_X1024_R::new(((self.bits >> 1) & 0x7fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bit 0 - no description available."]
    #[inline(always)]
    pub fn valid(&mut self) -> VALID_W<0> {
        VALID_W::new(self)
    }
    #[doc = "Bits 1:31 - SLOPE_x1024\\[30:0\\]"]
    #[inline(always)]
    pub fn slope_x1024(&mut self) -> SLOPE_X1024_W<1> {
        SLOPE_X1024_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "no description available.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [temp_sens_slope](index.html) module"]
pub struct TEMP_SENS_SLOPE_SPEC;
impl crate::RegisterSpec for TEMP_SENS_SLOPE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [temp_sens_slope::R](R) reader structure"]
impl crate::Readable for TEMP_SENS_SLOPE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [temp_sens_slope::W](W) writer structure"]
impl crate::Writable for TEMP_SENS_SLOPE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TEMP_SENS_SLOPE to value 0"]
impl crate::Resettable for TEMP_SENS_SLOPE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
