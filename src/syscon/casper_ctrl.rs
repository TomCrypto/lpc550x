#[doc = "Register `CASPER_CTRL` reader"]
pub struct R(crate::R<CASPER_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CASPER_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CASPER_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CASPER_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CASPER_CTRL` writer"]
pub struct W(crate::W<CASPER_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CASPER_CTRL_SPEC>;
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
impl From<crate::W<CASPER_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CASPER_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INTERLEAVE` reader - Control RAM access for RAMX0 and RAMX1."]
pub type INTERLEAVE_R = crate::BitReader<INTERLEAVE_A>;
#[doc = "Control RAM access for RAMX0 and RAMX1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INTERLEAVE_A {
    #[doc = "0: RAM access to RAMX0 and RAMX1 is consecutive."]
    NORMAL = 0,
    #[doc = "1: RAM access to RAMX0 and RAMX1 is interleaved."]
    INTERLEAVE = 1,
}
impl From<INTERLEAVE_A> for bool {
    #[inline(always)]
    fn from(variant: INTERLEAVE_A) -> Self {
        variant as u8 != 0
    }
}
impl INTERLEAVE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTERLEAVE_A {
        match self.bits {
            false => INTERLEAVE_A::NORMAL,
            true => INTERLEAVE_A::INTERLEAVE,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == INTERLEAVE_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `INTERLEAVE`"]
    #[inline(always)]
    pub fn is_interleave(&self) -> bool {
        *self == INTERLEAVE_A::INTERLEAVE
    }
}
#[doc = "Field `INTERLEAVE` writer - Control RAM access for RAMX0 and RAMX1."]
pub type INTERLEAVE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CASPER_CTRL_SPEC, INTERLEAVE_A, O>;
impl<'a, const O: u8> INTERLEAVE_W<'a, O> {
    #[doc = "RAM access to RAMX0 and RAMX1 is consecutive."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(INTERLEAVE_A::NORMAL)
    }
    #[doc = "RAM access to RAMX0 and RAMX1 is interleaved."]
    #[inline(always)]
    pub fn interleave(self) -> &'a mut W {
        self.variant(INTERLEAVE_A::INTERLEAVE)
    }
}
impl R {
    #[doc = "Bit 0 - Control RAM access for RAMX0 and RAMX1."]
    #[inline(always)]
    pub fn interleave(&self) -> INTERLEAVE_R {
        INTERLEAVE_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Control RAM access for RAMX0 and RAMX1."]
    #[inline(always)]
    pub fn interleave(&mut self) -> INTERLEAVE_W<0> {
        INTERLEAVE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control CASPER integration.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [casper_ctrl](index.html) module"]
pub struct CASPER_CTRL_SPEC;
impl crate::RegisterSpec for CASPER_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [casper_ctrl::R](R) reader structure"]
impl crate::Readable for CASPER_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [casper_ctrl::W](W) writer structure"]
impl crate::Writable for CASPER_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CASPER_CTRL to value 0"]
impl crate::Resettable for CASPER_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
