#[doc = "Register `FIFOSIZE` reader"]
pub struct R(crate::R<FIFOSIZE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FIFOSIZE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FIFOSIZE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FIFOSIZE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FIFOSIZE` writer"]
pub struct W(crate::W<FIFOSIZE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FIFOSIZE_SPEC>;
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
impl From<crate::W<FIFOSIZE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FIFOSIZE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FIFOSIZE` reader - Provides the size of the FIFO for software. The size of the SPI FIFO is 8 entries."]
pub type FIFOSIZE_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:4 - Provides the size of the FIFO for software. The size of the SPI FIFO is 8 entries."]
    #[inline(always)]
    pub fn fifosize(&self) -> FIFOSIZE_R {
        FIFOSIZE_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FIFO size register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifosize](index.html) module"]
pub struct FIFOSIZE_SPEC;
impl crate::RegisterSpec for FIFOSIZE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fifosize::R](R) reader structure"]
impl crate::Readable for FIFOSIZE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fifosize::W](W) writer structure"]
impl crate::Writable for FIFOSIZE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FIFOSIZE to value 0x08"]
impl crate::Resettable for FIFOSIZE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x08
    }
}
