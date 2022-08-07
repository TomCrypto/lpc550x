#[doc = "Register `BOD` reader"]
pub struct R(crate::R<BOD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BOD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BOD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BOD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BOD` writer"]
pub struct W(crate::W<BOD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BOD_SPEC>;
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
impl From<crate::W<BOD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BOD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BOD_VBAT_TRIM_VALID` reader - no description available"]
pub type BOD_VBAT_TRIM_VALID_R = crate::BitReader<bool>;
#[doc = "Field `BOD_VBAT_TRIM_VALID` writer - no description available"]
pub type BOD_VBAT_TRIM_VALID_W<'a, const O: u8> = crate::BitWriter<'a, u32, BOD_SPEC, bool, O>;
#[doc = "Field `BOD_VBAT_TRIGLVL` reader - no description available"]
pub type BOD_VBAT_TRIGLVL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BOD_VBAT_TRIGLVL` writer - no description available"]
pub type BOD_VBAT_TRIGLVL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BOD_SPEC, u8, u8, 5, O>;
#[doc = "Field `BOD_VBAT_HYST` reader - no description available"]
pub type BOD_VBAT_HYST_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BOD_VBAT_HYST` writer - no description available"]
pub type BOD_VBAT_HYST_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BOD_SPEC, u8, u8, 2, O>;
#[doc = "Field `BOD_CORE_TRIM_VALID` reader - no description available"]
pub type BOD_CORE_TRIM_VALID_R = crate::BitReader<bool>;
#[doc = "Field `BOD_CORE_TRIM_VALID` writer - no description available"]
pub type BOD_CORE_TRIM_VALID_W<'a, const O: u8> = crate::BitWriter<'a, u32, BOD_SPEC, bool, O>;
#[doc = "Field `BOD_CORE_TRIGLVL` reader - no description available"]
pub type BOD_CORE_TRIGLVL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BOD_CORE_TRIGLVL` writer - no description available"]
pub type BOD_CORE_TRIGLVL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BOD_SPEC, u8, u8, 3, O>;
#[doc = "Field `BOD_CORE_HYST` reader - no description available"]
pub type BOD_CORE_HYST_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BOD_CORE_HYST` writer - no description available"]
pub type BOD_CORE_HYST_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BOD_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    pub fn bod_vbat_trim_valid(&self) -> BOD_VBAT_TRIM_VALID_R {
        BOD_VBAT_TRIM_VALID_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:5 - no description available"]
    #[inline(always)]
    pub fn bod_vbat_triglvl(&self) -> BOD_VBAT_TRIGLVL_R {
        BOD_VBAT_TRIGLVL_R::new(((self.bits >> 1) & 0x1f) as u8)
    }
    #[doc = "Bits 6:7 - no description available"]
    #[inline(always)]
    pub fn bod_vbat_hyst(&self) -> BOD_VBAT_HYST_R {
        BOD_VBAT_HYST_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 16 - no description available"]
    #[inline(always)]
    pub fn bod_core_trim_valid(&self) -> BOD_CORE_TRIM_VALID_R {
        BOD_CORE_TRIM_VALID_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:19 - no description available"]
    #[inline(always)]
    pub fn bod_core_triglvl(&self) -> BOD_CORE_TRIGLVL_R {
        BOD_CORE_TRIGLVL_R::new(((self.bits >> 17) & 7) as u8)
    }
    #[doc = "Bits 21:22 - no description available"]
    #[inline(always)]
    pub fn bod_core_hyst(&self) -> BOD_CORE_HYST_R {
        BOD_CORE_HYST_R::new(((self.bits >> 21) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    pub fn bod_vbat_trim_valid(&mut self) -> BOD_VBAT_TRIM_VALID_W<0> {
        BOD_VBAT_TRIM_VALID_W::new(self)
    }
    #[doc = "Bits 1:5 - no description available"]
    #[inline(always)]
    pub fn bod_vbat_triglvl(&mut self) -> BOD_VBAT_TRIGLVL_W<1> {
        BOD_VBAT_TRIGLVL_W::new(self)
    }
    #[doc = "Bits 6:7 - no description available"]
    #[inline(always)]
    pub fn bod_vbat_hyst(&mut self) -> BOD_VBAT_HYST_W<6> {
        BOD_VBAT_HYST_W::new(self)
    }
    #[doc = "Bit 16 - no description available"]
    #[inline(always)]
    pub fn bod_core_trim_valid(&mut self) -> BOD_CORE_TRIM_VALID_W<16> {
        BOD_CORE_TRIM_VALID_W::new(self)
    }
    #[doc = "Bits 17:19 - no description available"]
    #[inline(always)]
    pub fn bod_core_triglvl(&mut self) -> BOD_CORE_TRIGLVL_W<17> {
        BOD_CORE_TRIGLVL_W::new(self)
    }
    #[doc = "Bits 21:22 - no description available"]
    #[inline(always)]
    pub fn bod_core_hyst(&mut self) -> BOD_CORE_HYST_W<21> {
        BOD_CORE_HYST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "no description available\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bod](index.html) module"]
pub struct BOD_SPEC;
impl crate::RegisterSpec for BOD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bod::R](R) reader structure"]
impl crate::Readable for BOD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bod::W](W) writer structure"]
impl crate::Writable for BOD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BOD to value 0"]
impl crate::Resettable for BOD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
