#[doc = "Register `CAP10` reader"]
pub struct R(crate::R<CAP10_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CAP10_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CAP10_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CAP10_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CAP10` writer"]
pub struct W(crate::W<CAP10_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CAP10_SPEC>;
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
impl From<crate::W<CAP10_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CAP10_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CAPn_L` reader - When UNIFY = 0, read the 16-bit counter value at which this register was last captured. When UNIFY = 1, read the lower 16 bits of the 32-bit value at which this register was last captured."]
pub type CAPN_L_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CAPn_L` writer - When UNIFY = 0, read the 16-bit counter value at which this register was last captured. When UNIFY = 1, read the lower 16 bits of the 32-bit value at which this register was last captured."]
pub type CAPN_L_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CAP10_SPEC, u16, u16, 16, O>;
#[doc = "Field `CAPn_H` reader - When UNIFY = 0, read the 16-bit counter value at which this register was last captured. When UNIFY = 1, read the upper 16 bits of the 32-bit value at which this register was last captured."]
pub type CAPN_H_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CAPn_H` writer - When UNIFY = 0, read the 16-bit counter value at which this register was last captured. When UNIFY = 1, read the upper 16 bits of the 32-bit value at which this register was last captured."]
pub type CAPN_H_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CAP10_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - When UNIFY = 0, read the 16-bit counter value at which this register was last captured. When UNIFY = 1, read the lower 16 bits of the 32-bit value at which this register was last captured."]
    #[inline(always)]
    pub fn capn_l(&self) -> CAPN_L_R {
        CAPN_L_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - When UNIFY = 0, read the 16-bit counter value at which this register was last captured. When UNIFY = 1, read the upper 16 bits of the 32-bit value at which this register was last captured."]
    #[inline(always)]
    pub fn capn_h(&self) -> CAPN_H_R {
        CAPN_H_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - When UNIFY = 0, read the 16-bit counter value at which this register was last captured. When UNIFY = 1, read the lower 16 bits of the 32-bit value at which this register was last captured."]
    #[inline(always)]
    pub fn capn_l(&mut self) -> CAPN_L_W<0> {
        CAPN_L_W::new(self)
    }
    #[doc = "Bits 16:31 - When UNIFY = 0, read the 16-bit counter value at which this register was last captured. When UNIFY = 1, read the upper 16 bits of the 32-bit value at which this register was last captured."]
    #[inline(always)]
    pub fn capn_h(&mut self) -> CAPN_H_W<16> {
        CAPN_H_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SCT capture register of capture channel.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cap10](index.html) module"]
pub struct CAP10_SPEC;
impl crate::RegisterSpec for CAP10_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cap10::R](R) reader structure"]
impl crate::Readable for CAP10_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cap10::W](W) writer structure"]
impl crate::Writable for CAP10_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CAP10 to value 0"]
impl crate::Resettable for CAP10_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}