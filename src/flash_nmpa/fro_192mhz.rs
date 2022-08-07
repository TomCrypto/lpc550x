#[doc = "Register `FRO_192MHZ` reader"]
pub struct R(crate::R<FRO_192MHZ_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FRO_192MHZ_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FRO_192MHZ_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FRO_192MHZ_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FRO_192MHZ` writer"]
pub struct W(crate::W<FRO_192MHZ_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FRO_192MHZ_SPEC>;
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
impl From<crate::W<FRO_192MHZ_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FRO_192MHZ_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FRO192M_TRIM_VALID` reader - no description available"]
pub type FRO192M_TRIM_VALID_R = crate::BitReader<bool>;
#[doc = "Field `FRO192M_TRIM_VALID` writer - no description available"]
pub type FRO192M_TRIM_VALID_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FRO_192MHZ_SPEC, bool, O>;
#[doc = "Field `FRO192M_BIASTRIM` reader - FRO192M_BIASTRIM\\[5:0\\]."]
pub type FRO192M_BIASTRIM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FRO192M_BIASTRIM` writer - FRO192M_BIASTRIM\\[5:0\\]."]
pub type FRO192M_BIASTRIM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FRO_192MHZ_SPEC, u8, u8, 6, O>;
#[doc = "Field `FRO192M_TEMPTRIM` reader - FRO192M_TEMPTRIM\\[6:0\\]."]
pub type FRO192M_TEMPTRIM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FRO192M_TEMPTRIM` writer - FRO192M_TEMPTRIM\\[6:0\\]."]
pub type FRO192M_TEMPTRIM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FRO_192MHZ_SPEC, u8, u8, 7, O>;
#[doc = "Field `FRO192M_DACTRIM` reader - FRO192M_DACTRIM\\[7:0\\]."]
pub type FRO192M_DACTRIM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FRO192M_DACTRIM` writer - FRO192M_DACTRIM\\[7:0\\]."]
pub type FRO192M_DACTRIM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FRO_192MHZ_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    pub fn fro192m_trim_valid(&self) -> FRO192M_TRIM_VALID_R {
        FRO192M_TRIM_VALID_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:6 - FRO192M_BIASTRIM\\[5:0\\]."]
    #[inline(always)]
    pub fn fro192m_biastrim(&self) -> FRO192M_BIASTRIM_R {
        FRO192M_BIASTRIM_R::new(((self.bits >> 1) & 0x3f) as u8)
    }
    #[doc = "Bits 8:14 - FRO192M_TEMPTRIM\\[6:0\\]."]
    #[inline(always)]
    pub fn fro192m_temptrim(&self) -> FRO192M_TEMPTRIM_R {
        FRO192M_TEMPTRIM_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 17:24 - FRO192M_DACTRIM\\[7:0\\]."]
    #[inline(always)]
    pub fn fro192m_dactrim(&self) -> FRO192M_DACTRIM_R {
        FRO192M_DACTRIM_R::new(((self.bits >> 17) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    pub fn fro192m_trim_valid(&mut self) -> FRO192M_TRIM_VALID_W<0> {
        FRO192M_TRIM_VALID_W::new(self)
    }
    #[doc = "Bits 1:6 - FRO192M_BIASTRIM\\[5:0\\]."]
    #[inline(always)]
    pub fn fro192m_biastrim(&mut self) -> FRO192M_BIASTRIM_W<1> {
        FRO192M_BIASTRIM_W::new(self)
    }
    #[doc = "Bits 8:14 - FRO192M_TEMPTRIM\\[6:0\\]."]
    #[inline(always)]
    pub fn fro192m_temptrim(&mut self) -> FRO192M_TEMPTRIM_W<8> {
        FRO192M_TEMPTRIM_W::new(self)
    }
    #[doc = "Bits 17:24 - FRO192M_DACTRIM\\[7:0\\]."]
    #[inline(always)]
    pub fn fro192m_dactrim(&mut self) -> FRO192M_DACTRIM_W<17> {
        FRO192M_DACTRIM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "no description available\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fro_192mhz](index.html) module"]
pub struct FRO_192MHZ_SPEC;
impl crate::RegisterSpec for FRO_192MHZ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fro_192mhz::R](R) reader structure"]
impl crate::Readable for FRO_192MHZ_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fro_192mhz::W](W) writer structure"]
impl crate::Writable for FRO_192MHZ_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FRO_192MHZ to value 0"]
impl crate::Resettable for FRO_192MHZ_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
