#[doc = "Register `FUNCRETENTIONCTRL` reader"]
pub struct R(crate::R<FUNCRETENTIONCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FUNCRETENTIONCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FUNCRETENTIONCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FUNCRETENTIONCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FUNCRETENTIONCTRL` writer"]
pub struct W(crate::W<FUNCRETENTIONCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FUNCRETENTIONCTRL_SPEC>;
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
impl From<crate::W<FUNCRETENTIONCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FUNCRETENTIONCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FUNCRETENA` reader - functional retention in power down only."]
pub type FUNCRETENA_R = crate::BitReader<FUNCRETENA_A>;
#[doc = "functional retention in power down only.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FUNCRETENA_A {
    #[doc = "0: disable functional retention."]
    DISABLE = 0,
    #[doc = "1: enable functional retention."]
    ENABLE = 1,
}
impl From<FUNCRETENA_A> for bool {
    #[inline(always)]
    fn from(variant: FUNCRETENA_A) -> Self {
        variant as u8 != 0
    }
}
impl FUNCRETENA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FUNCRETENA_A {
        match self.bits {
            false => FUNCRETENA_A::DISABLE,
            true => FUNCRETENA_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == FUNCRETENA_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == FUNCRETENA_A::ENABLE
    }
}
#[doc = "Field `FUNCRETENA` writer - functional retention in power down only."]
pub type FUNCRETENA_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FUNCRETENTIONCTRL_SPEC, FUNCRETENA_A, O>;
impl<'a, const O: u8> FUNCRETENA_W<'a, O> {
    #[doc = "disable functional retention."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(FUNCRETENA_A::DISABLE)
    }
    #[doc = "enable functional retention."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(FUNCRETENA_A::ENABLE)
    }
}
#[doc = "Field `RET_START` reader - Start address divided by 4 inside SRAMX bank."]
pub type RET_START_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RET_START` writer - Start address divided by 4 inside SRAMX bank."]
pub type RET_START_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FUNCRETENTIONCTRL_SPEC, u16, u16, 13, O>;
#[doc = "Field `RET_LENTH` reader - lenth of Scan chains to save."]
pub type RET_LENTH_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RET_LENTH` writer - lenth of Scan chains to save."]
pub type RET_LENTH_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FUNCRETENTIONCTRL_SPEC, u16, u16, 10, O>;
impl R {
    #[doc = "Bit 0 - functional retention in power down only."]
    #[inline(always)]
    pub fn funcretena(&self) -> FUNCRETENA_R {
        FUNCRETENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:13 - Start address divided by 4 inside SRAMX bank."]
    #[inline(always)]
    pub fn ret_start(&self) -> RET_START_R {
        RET_START_R::new(((self.bits >> 1) & 0x1fff) as u16)
    }
    #[doc = "Bits 14:23 - lenth of Scan chains to save."]
    #[inline(always)]
    pub fn ret_lenth(&self) -> RET_LENTH_R {
        RET_LENTH_R::new(((self.bits >> 14) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - functional retention in power down only."]
    #[inline(always)]
    pub fn funcretena(&mut self) -> FUNCRETENA_W<0> {
        FUNCRETENA_W::new(self)
    }
    #[doc = "Bits 1:13 - Start address divided by 4 inside SRAMX bank."]
    #[inline(always)]
    pub fn ret_start(&mut self) -> RET_START_W<1> {
        RET_START_W::new(self)
    }
    #[doc = "Bits 14:23 - lenth of Scan chains to save."]
    #[inline(always)]
    pub fn ret_lenth(&mut self) -> RET_LENTH_W<14> {
        RET_LENTH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Functional retention control register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [funcretentionctrl](index.html) module"]
pub struct FUNCRETENTIONCTRL_SPEC;
impl crate::RegisterSpec for FUNCRETENTIONCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [funcretentionctrl::R](R) reader structure"]
impl crate::Readable for FUNCRETENTIONCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [funcretentionctrl::W](W) writer structure"]
impl crate::Writable for FUNCRETENTIONCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FUNCRETENTIONCTRL to value 0x0050_c000"]
impl crate::Resettable for FUNCRETENTIONCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0050_c000
    }
}
