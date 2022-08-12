#[doc = "Register `FRO_1MHZ` reader"]
pub struct R(crate::R<FRO_1MHZ_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FRO_1MHZ_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FRO_1MHZ_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FRO_1MHZ_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FRO_1MHZ` writer"]
pub struct W(crate::W<FRO_1MHZ_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FRO_1MHZ_SPEC>;
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
impl From<crate::W<FRO_1MHZ_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FRO_1MHZ_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FRO1M_TRIM_VALID` reader - no description available."]
pub type FRO1M_TRIM_VALID_R = crate::BitReader<bool>;
#[doc = "Field `FRO1M_TRIM_VALID` writer - no description available."]
pub type FRO1M_TRIM_VALID_W<'a, const O: u8> = crate::BitWriter<'a, u32, FRO_1MHZ_SPEC, bool, O>;
#[doc = "Field `FRO1M_FREQSEL` reader - Frequency trimming bits."]
pub type FRO1M_FREQSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FRO1M_FREQSEL` writer - Frequency trimming bits."]
pub type FRO1M_FREQSEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FRO_1MHZ_SPEC, u8, u8, 7, O>;
impl R {
    #[doc = "Bit 0 - no description available."]
    #[inline(always)]
    pub fn fro1m_trim_valid(&self) -> FRO1M_TRIM_VALID_R {
        FRO1M_TRIM_VALID_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:7 - Frequency trimming bits."]
    #[inline(always)]
    pub fn fro1m_freqsel(&self) -> FRO1M_FREQSEL_R {
        FRO1M_FREQSEL_R::new(((self.bits >> 1) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - no description available."]
    #[inline(always)]
    pub fn fro1m_trim_valid(&mut self) -> FRO1M_TRIM_VALID_W<0> {
        FRO1M_TRIM_VALID_W::new(self)
    }
    #[doc = "Bits 1:7 - Frequency trimming bits."]
    #[inline(always)]
    pub fn fro1m_freqsel(&mut self) -> FRO1M_FREQSEL_W<1> {
        FRO1M_FREQSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "no description available.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fro_1mhz](index.html) module"]
pub struct FRO_1MHZ_SPEC;
impl crate::RegisterSpec for FRO_1MHZ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fro_1mhz::R](R) reader structure"]
impl crate::Readable for FRO_1MHZ_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fro_1mhz::W](W) writer structure"]
impl crate::Writable for FRO_1MHZ_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FRO_1MHZ to value 0"]
impl crate::Resettable for FRO_1MHZ_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
