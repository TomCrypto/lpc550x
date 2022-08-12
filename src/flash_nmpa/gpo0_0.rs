#[doc = "Register `GPO0_0` reader"]
pub struct R(crate::R<GPO0_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPO0_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPO0_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPO0_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPO0_0` writer"]
pub struct W(crate::W<GPO0_0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPO0_0_SPEC>;
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
impl From<crate::W<GPO0_0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPO0_0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FRO_TRIM_VALID` reader - no description available."]
pub type FRO_TRIM_VALID_R = crate::BitReader<bool>;
#[doc = "Field `FRO_TRIM_VALID` writer - no description available."]
pub type FRO_TRIM_VALID_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPO0_0_SPEC, bool, O>;
#[doc = "Field `FRO32K_NTAT` reader - no description available."]
pub type FRO32K_NTAT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FRO32K_NTAT` writer - no description available."]
pub type FRO32K_NTAT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GPO0_0_SPEC, u8, u8, 3, O>;
#[doc = "Field `FRO32K_PTAT` reader - no description available."]
pub type FRO32K_PTAT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FRO32K_PTAT` writer - no description available."]
pub type FRO32K_PTAT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GPO0_0_SPEC, u8, u8, 3, O>;
#[doc = "Field `FRO32K_CAPCAL` reader - no description available."]
pub type FRO32K_CAPCAL_R = crate::FieldReader<u16, u16>;
#[doc = "Field `FRO32K_CAPCAL` writer - no description available."]
pub type FRO32K_CAPCAL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GPO0_0_SPEC, u16, u16, 9, O>;
#[doc = "Field `FIELD` reader - no description available."]
pub type FIELD_R = crate::FieldReader<u16, u16>;
#[doc = "Field `FIELD` writer - no description available."]
pub type FIELD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GPO0_0_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bit 0 - no description available."]
    #[inline(always)]
    pub fn fro_trim_valid(&self) -> FRO_TRIM_VALID_R {
        FRO_TRIM_VALID_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - no description available."]
    #[inline(always)]
    pub fn fro32k_ntat(&self) -> FRO32K_NTAT_R {
        FRO32K_NTAT_R::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bits 4:6 - no description available."]
    #[inline(always)]
    pub fn fro32k_ptat(&self) -> FRO32K_PTAT_R {
        FRO32K_PTAT_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 7:15 - no description available."]
    #[inline(always)]
    pub fn fro32k_capcal(&self) -> FRO32K_CAPCAL_R {
        FRO32K_CAPCAL_R::new(((self.bits >> 7) & 0x01ff) as u16)
    }
    #[doc = "Bits 16:31 - no description available."]
    #[inline(always)]
    pub fn field(&self) -> FIELD_R {
        FIELD_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - no description available."]
    #[inline(always)]
    pub fn fro_trim_valid(&mut self) -> FRO_TRIM_VALID_W<0> {
        FRO_TRIM_VALID_W::new(self)
    }
    #[doc = "Bits 1:3 - no description available."]
    #[inline(always)]
    pub fn fro32k_ntat(&mut self) -> FRO32K_NTAT_W<1> {
        FRO32K_NTAT_W::new(self)
    }
    #[doc = "Bits 4:6 - no description available."]
    #[inline(always)]
    pub fn fro32k_ptat(&mut self) -> FRO32K_PTAT_W<4> {
        FRO32K_PTAT_W::new(self)
    }
    #[doc = "Bits 7:15 - no description available."]
    #[inline(always)]
    pub fn fro32k_capcal(&mut self) -> FRO32K_CAPCAL_W<7> {
        FRO32K_CAPCAL_W::new(self)
    }
    #[doc = "Bits 16:31 - no description available."]
    #[inline(always)]
    pub fn field(&mut self) -> FIELD_W<16> {
        FIELD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPO0 register 0 description.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpo0_0](index.html) module"]
pub struct GPO0_0_SPEC;
impl crate::RegisterSpec for GPO0_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpo0_0::R](R) reader structure"]
impl crate::Readable for GPO0_0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpo0_0::W](W) writer structure"]
impl crate::Writable for GPO0_0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GPO0_0 to value 0"]
impl crate::Resettable for GPO0_0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
