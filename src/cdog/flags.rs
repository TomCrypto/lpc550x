#[doc = "Register `FLAGS` reader"]
pub struct R(crate::R<FLAGS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLAGS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLAGS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLAGS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FLAGS` writer"]
pub struct W(crate::W<FLAGS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLAGS_SPEC>;
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
impl From<crate::W<FLAGS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLAGS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TO_FLAG` reader - Timeout flag"]
pub type TO_FLAG_R = crate::BitReader<bool>;
#[doc = "Field `TO_FLAG` writer - Timeout flag"]
pub type TO_FLAG_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLAGS_SPEC, bool, O>;
#[doc = "Field `MISCOM_FLAG` reader - Miscompare flag"]
pub type MISCOM_FLAG_R = crate::BitReader<bool>;
#[doc = "Field `MISCOM_FLAG` writer - Miscompare flag"]
pub type MISCOM_FLAG_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLAGS_SPEC, bool, O>;
#[doc = "Field `SEQ_FLAG` reader - Sequence flag"]
pub type SEQ_FLAG_R = crate::BitReader<bool>;
#[doc = "Field `SEQ_FLAG` writer - Sequence flag"]
pub type SEQ_FLAG_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLAGS_SPEC, bool, O>;
#[doc = "Field `CNT_FLAG` reader - Control (fault) flag"]
pub type CNT_FLAG_R = crate::BitReader<bool>;
#[doc = "Field `CNT_FLAG` writer - Control (fault) flag"]
pub type CNT_FLAG_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLAGS_SPEC, bool, O>;
#[doc = "Field `STATE_FLAG` reader - State flag"]
pub type STATE_FLAG_R = crate::BitReader<bool>;
#[doc = "Field `STATE_FLAG` writer - State flag"]
pub type STATE_FLAG_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLAGS_SPEC, bool, O>;
#[doc = "Field `ADDR_FLAG` reader - Address flag"]
pub type ADDR_FLAG_R = crate::BitReader<bool>;
#[doc = "Field `ADDR_FLAG` writer - Address flag"]
pub type ADDR_FLAG_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLAGS_SPEC, bool, O>;
#[doc = "Field `POR_FLAG` reader - Power-on reset flag"]
pub type POR_FLAG_R = crate::BitReader<bool>;
#[doc = "Field `POR_FLAG` writer - Power-on reset flag"]
pub type POR_FLAG_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLAGS_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Timeout flag"]
    #[inline(always)]
    pub fn to_flag(&self) -> TO_FLAG_R {
        TO_FLAG_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Miscompare flag"]
    #[inline(always)]
    pub fn miscom_flag(&self) -> MISCOM_FLAG_R {
        MISCOM_FLAG_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Sequence flag"]
    #[inline(always)]
    pub fn seq_flag(&self) -> SEQ_FLAG_R {
        SEQ_FLAG_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Control (fault) flag"]
    #[inline(always)]
    pub fn cnt_flag(&self) -> CNT_FLAG_R {
        CNT_FLAG_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - State flag"]
    #[inline(always)]
    pub fn state_flag(&self) -> STATE_FLAG_R {
        STATE_FLAG_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Address flag"]
    #[inline(always)]
    pub fn addr_flag(&self) -> ADDR_FLAG_R {
        ADDR_FLAG_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 16 - Power-on reset flag"]
    #[inline(always)]
    pub fn por_flag(&self) -> POR_FLAG_R {
        POR_FLAG_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Timeout flag"]
    #[inline(always)]
    pub fn to_flag(&mut self) -> TO_FLAG_W<0> {
        TO_FLAG_W::new(self)
    }
    #[doc = "Bit 1 - Miscompare flag"]
    #[inline(always)]
    pub fn miscom_flag(&mut self) -> MISCOM_FLAG_W<1> {
        MISCOM_FLAG_W::new(self)
    }
    #[doc = "Bit 2 - Sequence flag"]
    #[inline(always)]
    pub fn seq_flag(&mut self) -> SEQ_FLAG_W<2> {
        SEQ_FLAG_W::new(self)
    }
    #[doc = "Bit 3 - Control (fault) flag"]
    #[inline(always)]
    pub fn cnt_flag(&mut self) -> CNT_FLAG_W<3> {
        CNT_FLAG_W::new(self)
    }
    #[doc = "Bit 4 - State flag"]
    #[inline(always)]
    pub fn state_flag(&mut self) -> STATE_FLAG_W<4> {
        STATE_FLAG_W::new(self)
    }
    #[doc = "Bit 5 - Address flag"]
    #[inline(always)]
    pub fn addr_flag(&mut self) -> ADDR_FLAG_W<5> {
        ADDR_FLAG_W::new(self)
    }
    #[doc = "Bit 16 - Power-on reset flag"]
    #[inline(always)]
    pub fn por_flag(&mut self) -> POR_FLAG_W<16> {
        POR_FLAG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Hardware flags\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flags](index.html) module"]
pub struct FLAGS_SPEC;
impl crate::RegisterSpec for FLAGS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [flags::R](R) reader structure"]
impl crate::Readable for FLAGS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [flags::W](W) writer structure"]
impl crate::Writable for FLAGS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FLAGS to value 0"]
impl crate::Resettable for FLAGS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
