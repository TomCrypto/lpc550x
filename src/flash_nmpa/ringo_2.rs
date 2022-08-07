#[doc = "Register `RINGO_2` reader"]
pub struct R(crate::R<RINGO_2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RINGO_2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RINGO_2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RINGO_2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RINGO_2` writer"]
pub struct W(crate::W<RINGO_2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RINGO_2_SPEC>;
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
impl From<crate::W<RINGO_2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RINGO_2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RINGO_2_CTRL_VALID` reader - 1: RINGO_2_CTRL is valid."]
pub type RINGO_2_CTRL_VALID_R = crate::BitReader<bool>;
#[doc = "Field `RINGO_2_CTRL_VALID` writer - 1: RINGO_2_CTRL is valid."]
pub type RINGO_2_CTRL_VALID_W<'a, const O: u8> = crate::BitWriter<'a, u32, RINGO_2_SPEC, bool, O>;
#[doc = "Field `RINGO_2_CTRL` reader - To copy RINGO_2_CTRL = ANACTRL->RINGO2_CTRL\\[30:0\\]"]
pub type RINGO_2_CTRL_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RINGO_2_CTRL` writer - To copy RINGO_2_CTRL = ANACTRL->RINGO2_CTRL\\[30:0\\]"]
pub type RINGO_2_CTRL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RINGO_2_SPEC, u32, u32, 31, O>;
impl R {
    #[doc = "Bit 0 - 1: RINGO_2_CTRL is valid."]
    #[inline(always)]
    pub fn ringo_2_ctrl_valid(&self) -> RINGO_2_CTRL_VALID_R {
        RINGO_2_CTRL_VALID_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:31 - To copy RINGO_2_CTRL = ANACTRL->RINGO2_CTRL\\[30:0\\]"]
    #[inline(always)]
    pub fn ringo_2_ctrl(&self) -> RINGO_2_CTRL_R {
        RINGO_2_CTRL_R::new(((self.bits >> 1) & 0x7fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bit 0 - 1: RINGO_2_CTRL is valid."]
    #[inline(always)]
    pub fn ringo_2_ctrl_valid(&mut self) -> RINGO_2_CTRL_VALID_W<0> {
        RINGO_2_CTRL_VALID_W::new(self)
    }
    #[doc = "Bits 1:31 - To copy RINGO_2_CTRL = ANACTRL->RINGO2_CTRL\\[30:0\\]"]
    #[inline(always)]
    pub fn ringo_2_ctrl(&mut self) -> RINGO_2_CTRL_W<1> {
        RINGO_2_CTRL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "no description available\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ringo_2](index.html) module"]
pub struct RINGO_2_SPEC;
impl crate::RegisterSpec for RINGO_2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ringo_2::R](R) reader structure"]
impl crate::Readable for RINGO_2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ringo_2::W](W) writer structure"]
impl crate::Writable for RINGO_2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RINGO_2 to value 0"]
impl crate::Resettable for RINGO_2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
