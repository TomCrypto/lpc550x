#[doc = "Register `POWERDOWN` reader"]
pub struct R(crate::R<POWERDOWN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<POWERDOWN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<POWERDOWN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<POWERDOWN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `POWERDOWN` writer"]
pub struct W(crate::W<POWERDOWN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<POWERDOWN_SPEC>;
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
impl From<crate::W<POWERDOWN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<POWERDOWN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SOFT_RESET` reader - Request softreset that will go low automaticaly after acknowledge from CORE."]
pub type SOFT_RESET_R = crate::BitReader<bool>;
#[doc = "Field `SOFT_RESET` writer - Request softreset that will go low automaticaly after acknowledge from CORE."]
pub type SOFT_RESET_W<'a, const O: u8> = crate::BitWriter<'a, u32, POWERDOWN_SPEC, bool, O>;
#[doc = "Field `FORCE_SOFT_RESET` reader - When used with softreset it forces CORE_RESETN to low on acknowledge from CORE."]
pub type FORCE_SOFT_RESET_R = crate::BitReader<bool>;
#[doc = "Field `FORCE_SOFT_RESET` writer - When used with softreset it forces CORE_RESETN to low on acknowledge from CORE."]
pub type FORCE_SOFT_RESET_W<'a, const O: u8> = crate::BitWriter<'a, u32, POWERDOWN_SPEC, bool, O>;
#[doc = "Field `POWERDOWN` reader - When set all accesses to standard registers are blocked."]
pub type POWERDOWN_R = crate::BitReader<bool>;
#[doc = "Field `POWERDOWN` writer - When set all accesses to standard registers are blocked."]
pub type POWERDOWN_W<'a, const O: u8> = crate::BitWriter<'a, u32, POWERDOWN_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Request softreset that will go low automaticaly after acknowledge from CORE."]
    #[inline(always)]
    pub fn soft_reset(&self) -> SOFT_RESET_R {
        SOFT_RESET_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - When used with softreset it forces CORE_RESETN to low on acknowledge from CORE."]
    #[inline(always)]
    pub fn force_soft_reset(&self) -> FORCE_SOFT_RESET_R {
        FORCE_SOFT_RESET_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 31 - When set all accesses to standard registers are blocked."]
    #[inline(always)]
    pub fn powerdown(&self) -> POWERDOWN_R {
        POWERDOWN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Request softreset that will go low automaticaly after acknowledge from CORE."]
    #[inline(always)]
    pub fn soft_reset(&mut self) -> SOFT_RESET_W<0> {
        SOFT_RESET_W::new(self)
    }
    #[doc = "Bit 1 - When used with softreset it forces CORE_RESETN to low on acknowledge from CORE."]
    #[inline(always)]
    pub fn force_soft_reset(&mut self) -> FORCE_SOFT_RESET_W<1> {
        FORCE_SOFT_RESET_W::new(self)
    }
    #[doc = "Bit 31 - When set all accesses to standard registers are blocked."]
    #[inline(always)]
    pub fn powerdown(&mut self) -> POWERDOWN_W<31> {
        POWERDOWN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Powerdown mode (standard but certainly useless here)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [powerdown](index.html) module"]
pub struct POWERDOWN_SPEC;
impl crate::RegisterSpec for POWERDOWN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [powerdown::R](R) reader structure"]
impl crate::Readable for POWERDOWN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [powerdown::W](W) writer structure"]
impl crate::Writable for POWERDOWN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets POWERDOWN to value 0"]
impl crate::Resettable for POWERDOWN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
