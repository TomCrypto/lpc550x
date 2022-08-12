#[doc = "Register `PERSISTENT` reader"]
pub struct R(crate::R<PERSISTENT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PERSISTENT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PERSISTENT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PERSISTENT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PERSISTENT` writer"]
pub struct W(crate::W<PERSISTENT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PERSISTENT_SPEC>;
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
impl From<crate::W<PERSISTENT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PERSISTENT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PERSIS` reader - 32 regs free for user SW to enjoy."]
pub type PERSIS_R = crate::FieldReader<u32, u32>;
#[doc = "Field `PERSIS` writer - 32 regs free for user SW to enjoy."]
pub type PERSIS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PERSISTENT_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - 32 regs free for user SW to enjoy."]
    #[inline(always)]
    pub fn persis(&self) -> PERSIS_R {
        PERSIS_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 32 regs free for user SW to enjoy."]
    #[inline(always)]
    pub fn persis(&mut self) -> PERSIS_W<0> {
        PERSIS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Persistent (Ad. Hoc., quasi-NV) data storage.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [persistent](index.html) module"]
pub struct PERSISTENT_SPEC;
impl crate::RegisterSpec for PERSISTENT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [persistent::R](R) reader structure"]
impl crate::Readable for PERSISTENT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [persistent::W](W) writer structure"]
impl crate::Writable for PERSISTENT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PERSISTENT to value 0"]
impl crate::Resettable for PERSISTENT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
