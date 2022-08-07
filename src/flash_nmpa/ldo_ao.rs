#[doc = "Register `LDO_AO` reader"]
pub struct R(crate::R<LDO_AO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LDO_AO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LDO_AO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LDO_AO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LDO_AO` writer"]
pub struct W(crate::W<LDO_AO_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LDO_AO_SPEC>;
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
impl From<crate::W<LDO_AO_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LDO_AO_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ACTIVE_TRIM_VALID` reader - no description available"]
pub type ACTIVE_TRIM_VALID_R = crate::BitReader<bool>;
#[doc = "Field `ACTIVE_TRIM_VALID` writer - no description available"]
pub type ACTIVE_TRIM_VALID_W<'a, const O: u8> = crate::BitWriter<'a, u32, LDO_AO_SPEC, bool, O>;
#[doc = "Field `ACTIVE_TRIM` reader - no description available"]
pub type ACTIVE_TRIM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ACTIVE_TRIM` writer - no description available"]
pub type ACTIVE_TRIM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LDO_AO_SPEC, u8, u8, 5, O>;
#[doc = "Field `DSLP_TRIM_VALID` reader - no description available"]
pub type DSLP_TRIM_VALID_R = crate::BitReader<bool>;
#[doc = "Field `DSLP_TRIM_VALID` writer - no description available"]
pub type DSLP_TRIM_VALID_W<'a, const O: u8> = crate::BitWriter<'a, u32, LDO_AO_SPEC, bool, O>;
#[doc = "Field `DSLP_TRIM` reader - no description available"]
pub type DSLP_TRIM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DSLP_TRIM` writer - no description available"]
pub type DSLP_TRIM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LDO_AO_SPEC, u8, u8, 5, O>;
#[doc = "Field `PDWN_TRIM_VALID` reader - no description available"]
pub type PDWN_TRIM_VALID_R = crate::BitReader<bool>;
#[doc = "Field `PDWN_TRIM_VALID` writer - no description available"]
pub type PDWN_TRIM_VALID_W<'a, const O: u8> = crate::BitWriter<'a, u32, LDO_AO_SPEC, bool, O>;
#[doc = "Field `PDWN_TRIM` reader - no description available"]
pub type PDWN_TRIM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PDWN_TRIM` writer - no description available"]
pub type PDWN_TRIM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LDO_AO_SPEC, u8, u8, 5, O>;
#[doc = "Field `DPDW_TRIM_VALID` reader - no description available"]
pub type DPDW_TRIM_VALID_R = crate::BitReader<bool>;
#[doc = "Field `DPDW_TRIM_VALID` writer - no description available"]
pub type DPDW_TRIM_VALID_W<'a, const O: u8> = crate::BitWriter<'a, u32, LDO_AO_SPEC, bool, O>;
#[doc = "Field `DPDW_TRIM` reader - no description available"]
pub type DPDW_TRIM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DPDW_TRIM` writer - no description available"]
pub type DPDW_TRIM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LDO_AO_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    pub fn active_trim_valid(&self) -> ACTIVE_TRIM_VALID_R {
        ACTIVE_TRIM_VALID_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:5 - no description available"]
    #[inline(always)]
    pub fn active_trim(&self) -> ACTIVE_TRIM_R {
        ACTIVE_TRIM_R::new(((self.bits >> 1) & 0x1f) as u8)
    }
    #[doc = "Bit 8 - no description available"]
    #[inline(always)]
    pub fn dslp_trim_valid(&self) -> DSLP_TRIM_VALID_R {
        DSLP_TRIM_VALID_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:13 - no description available"]
    #[inline(always)]
    pub fn dslp_trim(&self) -> DSLP_TRIM_R {
        DSLP_TRIM_R::new(((self.bits >> 9) & 0x1f) as u8)
    }
    #[doc = "Bit 16 - no description available"]
    #[inline(always)]
    pub fn pdwn_trim_valid(&self) -> PDWN_TRIM_VALID_R {
        PDWN_TRIM_VALID_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:21 - no description available"]
    #[inline(always)]
    pub fn pdwn_trim(&self) -> PDWN_TRIM_R {
        PDWN_TRIM_R::new(((self.bits >> 17) & 0x1f) as u8)
    }
    #[doc = "Bit 24 - no description available"]
    #[inline(always)]
    pub fn dpdw_trim_valid(&self) -> DPDW_TRIM_VALID_R {
        DPDW_TRIM_VALID_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:29 - no description available"]
    #[inline(always)]
    pub fn dpdw_trim(&self) -> DPDW_TRIM_R {
        DPDW_TRIM_R::new(((self.bits >> 25) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    pub fn active_trim_valid(&mut self) -> ACTIVE_TRIM_VALID_W<0> {
        ACTIVE_TRIM_VALID_W::new(self)
    }
    #[doc = "Bits 1:5 - no description available"]
    #[inline(always)]
    pub fn active_trim(&mut self) -> ACTIVE_TRIM_W<1> {
        ACTIVE_TRIM_W::new(self)
    }
    #[doc = "Bit 8 - no description available"]
    #[inline(always)]
    pub fn dslp_trim_valid(&mut self) -> DSLP_TRIM_VALID_W<8> {
        DSLP_TRIM_VALID_W::new(self)
    }
    #[doc = "Bits 9:13 - no description available"]
    #[inline(always)]
    pub fn dslp_trim(&mut self) -> DSLP_TRIM_W<9> {
        DSLP_TRIM_W::new(self)
    }
    #[doc = "Bit 16 - no description available"]
    #[inline(always)]
    pub fn pdwn_trim_valid(&mut self) -> PDWN_TRIM_VALID_W<16> {
        PDWN_TRIM_VALID_W::new(self)
    }
    #[doc = "Bits 17:21 - no description available"]
    #[inline(always)]
    pub fn pdwn_trim(&mut self) -> PDWN_TRIM_W<17> {
        PDWN_TRIM_W::new(self)
    }
    #[doc = "Bit 24 - no description available"]
    #[inline(always)]
    pub fn dpdw_trim_valid(&mut self) -> DPDW_TRIM_VALID_W<24> {
        DPDW_TRIM_VALID_W::new(self)
    }
    #[doc = "Bits 25:29 - no description available"]
    #[inline(always)]
    pub fn dpdw_trim(&mut self) -> DPDW_TRIM_W<25> {
        DPDW_TRIM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "no description available\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ldo_ao](index.html) module"]
pub struct LDO_AO_SPEC;
impl crate::RegisterSpec for LDO_AO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ldo_ao::R](R) reader structure"]
impl crate::Readable for LDO_AO_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ldo_ao::W](W) writer structure"]
impl crate::Writable for LDO_AO_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LDO_AO to value 0"]
impl crate::Resettable for LDO_AO_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
