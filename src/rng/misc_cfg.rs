#[doc = "Register `MISC_CFG` reader"]
pub struct R(crate::R<MISC_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MISC_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MISC_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MISC_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MISC_CFG` writer"]
pub struct W(crate::W<MISC_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MISC_CFG_SPEC>;
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
impl From<crate::W<MISC_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MISC_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AES_RESEED` reader - If set, ENCRYPTED_NUMBER generation becomes predictable, provided all secrets and current internal state are known: independant from entropy source."]
pub type AES_RESEED_R = crate::BitReader<bool>;
#[doc = "Field `AES_RESEED` writer - If set, ENCRYPTED_NUMBER generation becomes predictable, provided all secrets and current internal state are known: independant from entropy source."]
pub type AES_RESEED_W<'a, const O: u8> = crate::BitWriter<'a, u32, MISC_CFG_SPEC, bool, O>;
#[doc = "Field `AES_DT_CFG` reader - Set this bit to re-seed AES."]
pub type AES_DT_CFG_R = crate::BitReader<bool>;
#[doc = "Field `AES_DT_CFG` writer - Set this bit to re-seed AES."]
pub type AES_DT_CFG_W<'a, const O: u8> = crate::BitWriter<'a, u32, MISC_CFG_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - If set, ENCRYPTED_NUMBER generation becomes predictable, provided all secrets and current internal state are known: independant from entropy source."]
    #[inline(always)]
    pub fn aes_reseed(&self) -> AES_RESEED_R {
        AES_RESEED_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set this bit to re-seed AES."]
    #[inline(always)]
    pub fn aes_dt_cfg(&self) -> AES_DT_CFG_R {
        AES_DT_CFG_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - If set, ENCRYPTED_NUMBER generation becomes predictable, provided all secrets and current internal state are known: independant from entropy source."]
    #[inline(always)]
    pub fn aes_reseed(&mut self) -> AES_RESEED_W<0> {
        AES_RESEED_W::new(self)
    }
    #[doc = "Bit 1 - Set this bit to re-seed AES."]
    #[inline(always)]
    pub fn aes_dt_cfg(&mut self) -> AES_DT_CFG_W<1> {
        AES_DT_CFG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "no description available.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [misc_cfg](index.html) module"]
pub struct MISC_CFG_SPEC;
impl crate::RegisterSpec for MISC_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [misc_cfg::R](R) reader structure"]
impl crate::Readable for MISC_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [misc_cfg::W](W) writer structure"]
impl crate::Writable for MISC_CFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MISC_CFG to value 0"]
impl crate::Resettable for MISC_CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
