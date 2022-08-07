#[doc = "Register `GPO1_1` reader"]
pub struct R(crate::R<GPO1_GPO1_1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPO1_GPO1_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPO1_GPO1_1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPO1_GPO1_1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPO1_1` writer"]
pub struct W(crate::W<GPO1_GPO1_1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPO1_GPO1_1_SPEC>;
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
impl From<crate::W<GPO1_GPO1_1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPO1_GPO1_1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ROM_PATCH_VERSION` reader - ROM Patch Version \\[3:0\\]"]
pub type ROM_PATCH_VERSION_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ROM_PATCH_VERSION` writer - ROM Patch Version \\[3:0\\]"]
pub type ROM_PATCH_VERSION_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GPO1_GPO1_1_SPEC, u8, u8, 4, O>;
#[doc = "Field `CUSTOMER_REVISION_ID` reader - CUSTOMER REVISION ID\\[3:0\\]"]
pub type CUSTOMER_REVISION_ID_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CUSTOMER_REVISION_ID` writer - CUSTOMER REVISION ID\\[3:0\\]"]
pub type CUSTOMER_REVISION_ID_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GPO1_GPO1_1_SPEC, u8, u8, 4, O>;
#[doc = "Field `FIELD` reader - no description available"]
pub type FIELD_R = crate::FieldReader<u32, u32>;
#[doc = "Field `FIELD` writer - no description available"]
pub type FIELD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GPO1_GPO1_1_SPEC, u32, u32, 24, O>;
impl R {
    #[doc = "Bits 0:3 - ROM Patch Version \\[3:0\\]"]
    #[inline(always)]
    pub fn rom_patch_version(&self) -> ROM_PATCH_VERSION_R {
        ROM_PATCH_VERSION_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - CUSTOMER REVISION ID\\[3:0\\]"]
    #[inline(always)]
    pub fn customer_revision_id(&self) -> CUSTOMER_REVISION_ID_R {
        CUSTOMER_REVISION_ID_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:31 - no description available"]
    #[inline(always)]
    pub fn field(&self) -> FIELD_R {
        FIELD_R::new(((self.bits >> 8) & 0x00ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:3 - ROM Patch Version \\[3:0\\]"]
    #[inline(always)]
    pub fn rom_patch_version(&mut self) -> ROM_PATCH_VERSION_W<0> {
        ROM_PATCH_VERSION_W::new(self)
    }
    #[doc = "Bits 4:7 - CUSTOMER REVISION ID\\[3:0\\]"]
    #[inline(always)]
    pub fn customer_revision_id(&mut self) -> CUSTOMER_REVISION_ID_W<4> {
        CUSTOMER_REVISION_ID_W::new(self)
    }
    #[doc = "Bits 8:31 - no description available"]
    #[inline(always)]
    pub fn field(&mut self) -> FIELD_W<8> {
        FIELD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPO1 register 1 description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpo1_gpo1_1](index.html) module"]
pub struct GPO1_GPO1_1_SPEC;
impl crate::RegisterSpec for GPO1_GPO1_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpo1_gpo1_1::R](R) reader structure"]
impl crate::Readable for GPO1_GPO1_1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpo1_gpo1_1::W](W) writer structure"]
impl crate::Writable for GPO1_GPO1_1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GPO1_1 to value 0"]
impl crate::Resettable for GPO1_GPO1_1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
