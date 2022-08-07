#[doc = "Register `GPO1_2` reader"]
pub struct R(crate::R<GPO1_GPO1_2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPO1_GPO1_2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPO1_GPO1_2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPO1_GPO1_2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPO1_2` writer"]
pub struct W(crate::W<GPO1_GPO1_2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPO1_GPO1_2_SPEC>;
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
impl From<crate::W<GPO1_GPO1_2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPO1_GPO1_2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HVST` reader - High Voltage Stress: 0=not done; 1=done."]
pub type HVST_R = crate::BitReader<bool>;
#[doc = "Field `HVST` writer - High Voltage Stress: 0=not done; 1=done."]
pub type HVST_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPO1_GPO1_2_SPEC, bool, O>;
#[doc = "Field `FIELD` reader - no description available"]
pub type FIELD_R = crate::FieldReader<u32, u32>;
#[doc = "Field `FIELD` writer - no description available"]
pub type FIELD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GPO1_GPO1_2_SPEC, u32, u32, 31, O>;
impl R {
    #[doc = "Bit 0 - High Voltage Stress: 0=not done; 1=done."]
    #[inline(always)]
    pub fn hvst(&self) -> HVST_R {
        HVST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:31 - no description available"]
    #[inline(always)]
    pub fn field(&self) -> FIELD_R {
        FIELD_R::new(((self.bits >> 1) & 0x7fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bit 0 - High Voltage Stress: 0=not done; 1=done."]
    #[inline(always)]
    pub fn hvst(&mut self) -> HVST_W<0> {
        HVST_W::new(self)
    }
    #[doc = "Bits 1:31 - no description available"]
    #[inline(always)]
    pub fn field(&mut self) -> FIELD_W<1> {
        FIELD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPO1 register 2 description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpo1_gpo1_2](index.html) module"]
pub struct GPO1_GPO1_2_SPEC;
impl crate::RegisterSpec for GPO1_GPO1_2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpo1_gpo1_2::R](R) reader structure"]
impl crate::Readable for GPO1_GPO1_2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpo1_gpo1_2::W](W) writer structure"]
impl crate::Writable for GPO1_GPO1_2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GPO1_2 to value 0"]
impl crate::Resettable for GPO1_GPO1_2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
