#[doc = "Register `DIS_ROM_HIDING` reader"]
pub struct R(crate::R<DIS_ROM_HIDING_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DIS_ROM_HIDING_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DIS_ROM_HIDING_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DIS_ROM_HIDING_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DIS_ROM_HIDING` writer"]
pub struct W(crate::W<DIS_ROM_HIDING_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DIS_ROM_HIDING_SPEC>;
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
impl From<crate::W<DIS_ROM_HIDING_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DIS_ROM_HIDING_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DIS_ROM_HIDING` reader - When 0x3CC35AA5 ROM hiding feture is disabled. All other values critical ROM is hidden."]
pub type DIS_ROM_HIDING_R = crate::FieldReader<u32, u32>;
#[doc = "Field `DIS_ROM_HIDING` writer - When 0x3CC35AA5 ROM hiding feture is disabled. All other values critical ROM is hidden."]
pub type DIS_ROM_HIDING_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DIS_ROM_HIDING_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - When 0x3CC35AA5 ROM hiding feture is disabled. All other values critical ROM is hidden."]
    #[inline(always)]
    pub fn dis_rom_hiding(&self) -> DIS_ROM_HIDING_R {
        DIS_ROM_HIDING_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - When 0x3CC35AA5 ROM hiding feture is disabled. All other values critical ROM is hidden."]
    #[inline(always)]
    pub fn dis_rom_hiding(&mut self) -> DIS_ROM_HIDING_W<0> {
        DIS_ROM_HIDING_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "no description available\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dis_rom_hiding](index.html) module"]
pub struct DIS_ROM_HIDING_SPEC;
impl crate::RegisterSpec for DIS_ROM_HIDING_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dis_rom_hiding::R](R) reader structure"]
impl crate::Readable for DIS_ROM_HIDING_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dis_rom_hiding::W](W) writer structure"]
impl crate::Writable for DIS_ROM_HIDING_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DIS_ROM_HIDING to value 0"]
impl crate::Resettable for DIS_ROM_HIDING_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
