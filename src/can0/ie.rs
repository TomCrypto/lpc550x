#[doc = "Register `IE` reader"]
pub struct R(crate::R<IE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IE` writer"]
pub struct W(crate::W<IE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IE_SPEC>;
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
impl From<crate::W<IE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RF0NE` reader - Rx FIFO 0 new message interrupt enable."]
pub type RF0NE_R = crate::BitReader<bool>;
#[doc = "Field `RF0NE` writer - Rx FIFO 0 new message interrupt enable."]
pub type RF0NE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IE_SPEC, bool, O>;
#[doc = "Field `RF0WE` reader - Rx FIFO 0 watermark reached interrupt enable."]
pub type RF0WE_R = crate::BitReader<bool>;
#[doc = "Field `RF0WE` writer - Rx FIFO 0 watermark reached interrupt enable."]
pub type RF0WE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IE_SPEC, bool, O>;
#[doc = "Field `RF0FE` reader - Rx FIFO 0 full interrupt enable."]
pub type RF0FE_R = crate::BitReader<bool>;
#[doc = "Field `RF0FE` writer - Rx FIFO 0 full interrupt enable."]
pub type RF0FE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IE_SPEC, bool, O>;
#[doc = "Field `RF0LE` reader - Rx FIFO 0 message lost interrupt enable."]
pub type RF0LE_R = crate::BitReader<bool>;
#[doc = "Field `RF0LE` writer - Rx FIFO 0 message lost interrupt enable."]
pub type RF0LE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IE_SPEC, bool, O>;
#[doc = "Field `RF1NE` reader - Rx FIFO 1 new message interrupt enable."]
pub type RF1NE_R = crate::BitReader<bool>;
#[doc = "Field `RF1NE` writer - Rx FIFO 1 new message interrupt enable."]
pub type RF1NE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IE_SPEC, bool, O>;
#[doc = "Field `RF1WE` reader - Rx FIFO 1 watermark reached interrupt enable."]
pub type RF1WE_R = crate::BitReader<bool>;
#[doc = "Field `RF1WE` writer - Rx FIFO 1 watermark reached interrupt enable."]
pub type RF1WE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IE_SPEC, bool, O>;
#[doc = "Field `RF1FE` reader - Rx FIFO 1 full interrupt enable."]
pub type RF1FE_R = crate::BitReader<bool>;
#[doc = "Field `RF1FE` writer - Rx FIFO 1 full interrupt enable."]
pub type RF1FE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IE_SPEC, bool, O>;
#[doc = "Field `RF1LE` reader - Rx FIFO 1 message lost interrupt enable."]
pub type RF1LE_R = crate::BitReader<bool>;
#[doc = "Field `RF1LE` writer - Rx FIFO 1 message lost interrupt enable."]
pub type RF1LE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IE_SPEC, bool, O>;
#[doc = "Field `HPME` reader - High priority message interrupt enable."]
pub type HPME_R = crate::BitReader<bool>;
#[doc = "Field `HPME` writer - High priority message interrupt enable."]
pub type HPME_W<'a, const O: u8> = crate::BitWriter<'a, u32, IE_SPEC, bool, O>;
#[doc = "Field `TCE` reader - Transmission completed interrupt enable."]
pub type TCE_R = crate::BitReader<bool>;
#[doc = "Field `TCE` writer - Transmission completed interrupt enable."]
pub type TCE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IE_SPEC, bool, O>;
#[doc = "Field `TCFE` reader - Transmission cancellation finished interrupt enable."]
pub type TCFE_R = crate::BitReader<bool>;
#[doc = "Field `TCFE` writer - Transmission cancellation finished interrupt enable."]
pub type TCFE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IE_SPEC, bool, O>;
#[doc = "Field `TFEE` reader - Tx FIFO empty interrupt enable."]
pub type TFEE_R = crate::BitReader<bool>;
#[doc = "Field `TFEE` writer - Tx FIFO empty interrupt enable."]
pub type TFEE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IE_SPEC, bool, O>;
#[doc = "Field `TEFNE` reader - Tx event FIFO new entry interrupt enable."]
pub type TEFNE_R = crate::BitReader<bool>;
#[doc = "Field `TEFNE` writer - Tx event FIFO new entry interrupt enable."]
pub type TEFNE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IE_SPEC, bool, O>;
#[doc = "Field `TEFWE` reader - Tx event FIFO watermark reached interrupt enable."]
pub type TEFWE_R = crate::BitReader<bool>;
#[doc = "Field `TEFWE` writer - Tx event FIFO watermark reached interrupt enable."]
pub type TEFWE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IE_SPEC, bool, O>;
#[doc = "Field `TEFFE` reader - Tx event FIFO full interrupt enable."]
pub type TEFFE_R = crate::BitReader<bool>;
#[doc = "Field `TEFFE` writer - Tx event FIFO full interrupt enable."]
pub type TEFFE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IE_SPEC, bool, O>;
#[doc = "Field `TEFLE` reader - Tx event FIFO element lost interrupt enable."]
pub type TEFLE_R = crate::BitReader<bool>;
#[doc = "Field `TEFLE` writer - Tx event FIFO element lost interrupt enable."]
pub type TEFLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IE_SPEC, bool, O>;
#[doc = "Field `TSWE` reader - Timestamp wraparound interrupt enable."]
pub type TSWE_R = crate::BitReader<bool>;
#[doc = "Field `TSWE` writer - Timestamp wraparound interrupt enable."]
pub type TSWE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IE_SPEC, bool, O>;
#[doc = "Field `MRAFE` reader - Message RAM access failure interrupt enable."]
pub type MRAFE_R = crate::BitReader<bool>;
#[doc = "Field `MRAFE` writer - Message RAM access failure interrupt enable."]
pub type MRAFE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IE_SPEC, bool, O>;
#[doc = "Field `TOOE` reader - Timeout occurred interrupt enable."]
pub type TOOE_R = crate::BitReader<bool>;
#[doc = "Field `TOOE` writer - Timeout occurred interrupt enable."]
pub type TOOE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IE_SPEC, bool, O>;
#[doc = "Field `DRXE` reader - Message stored in dedicated Rx buffer interrupt enable."]
pub type DRXE_R = crate::BitReader<bool>;
#[doc = "Field `DRXE` writer - Message stored in dedicated Rx buffer interrupt enable."]
pub type DRXE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IE_SPEC, bool, O>;
#[doc = "Field `BECE` reader - Bit error corrected interrupt enable."]
pub type BECE_R = crate::BitReader<bool>;
#[doc = "Field `BECE` writer - Bit error corrected interrupt enable."]
pub type BECE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IE_SPEC, bool, O>;
#[doc = "Field `BEUE` reader - Bit error uncorrected interrupt enable."]
pub type BEUE_R = crate::BitReader<bool>;
#[doc = "Field `BEUE` writer - Bit error uncorrected interrupt enable."]
pub type BEUE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IE_SPEC, bool, O>;
#[doc = "Field `ELOE` reader - Error logging overflow interrupt enable."]
pub type ELOE_R = crate::BitReader<bool>;
#[doc = "Field `ELOE` writer - Error logging overflow interrupt enable."]
pub type ELOE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IE_SPEC, bool, O>;
#[doc = "Field `EPE` reader - Error passive interrupt enable."]
pub type EPE_R = crate::BitReader<bool>;
#[doc = "Field `EPE` writer - Error passive interrupt enable."]
pub type EPE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IE_SPEC, bool, O>;
#[doc = "Field `EWE` reader - Warning status interrupt enable."]
pub type EWE_R = crate::BitReader<bool>;
#[doc = "Field `EWE` writer - Warning status interrupt enable."]
pub type EWE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IE_SPEC, bool, O>;
#[doc = "Field `BOE` reader - Bus_Off Status interrupt enable."]
pub type BOE_R = crate::BitReader<bool>;
#[doc = "Field `BOE` writer - Bus_Off Status interrupt enable."]
pub type BOE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IE_SPEC, bool, O>;
#[doc = "Field `WDIE` reader - Watchdog interrupt enable."]
pub type WDIE_R = crate::BitReader<bool>;
#[doc = "Field `WDIE` writer - Watchdog interrupt enable."]
pub type WDIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IE_SPEC, bool, O>;
#[doc = "Field `PEAE` reader - Protocol error in arbitration phase interrupt enable."]
pub type PEAE_R = crate::BitReader<bool>;
#[doc = "Field `PEAE` writer - Protocol error in arbitration phase interrupt enable."]
pub type PEAE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IE_SPEC, bool, O>;
#[doc = "Field `PEDE` reader - Protocol error in data phase interrupt enable."]
pub type PEDE_R = crate::BitReader<bool>;
#[doc = "Field `PEDE` writer - Protocol error in data phase interrupt enable."]
pub type PEDE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IE_SPEC, bool, O>;
#[doc = "Field `ARAE` reader - Access to reserved address interrupt enable."]
pub type ARAE_R = crate::BitReader<bool>;
#[doc = "Field `ARAE` writer - Access to reserved address interrupt enable."]
pub type ARAE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IE_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Rx FIFO 0 new message interrupt enable."]
    #[inline(always)]
    pub fn rf0ne(&self) -> RF0NE_R {
        RF0NE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Rx FIFO 0 watermark reached interrupt enable."]
    #[inline(always)]
    pub fn rf0we(&self) -> RF0WE_R {
        RF0WE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Rx FIFO 0 full interrupt enable."]
    #[inline(always)]
    pub fn rf0fe(&self) -> RF0FE_R {
        RF0FE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Rx FIFO 0 message lost interrupt enable."]
    #[inline(always)]
    pub fn rf0le(&self) -> RF0LE_R {
        RF0LE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Rx FIFO 1 new message interrupt enable."]
    #[inline(always)]
    pub fn rf1ne(&self) -> RF1NE_R {
        RF1NE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Rx FIFO 1 watermark reached interrupt enable."]
    #[inline(always)]
    pub fn rf1we(&self) -> RF1WE_R {
        RF1WE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Rx FIFO 1 full interrupt enable."]
    #[inline(always)]
    pub fn rf1fe(&self) -> RF1FE_R {
        RF1FE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Rx FIFO 1 message lost interrupt enable."]
    #[inline(always)]
    pub fn rf1le(&self) -> RF1LE_R {
        RF1LE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - High priority message interrupt enable."]
    #[inline(always)]
    pub fn hpme(&self) -> HPME_R {
        HPME_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Transmission completed interrupt enable."]
    #[inline(always)]
    pub fn tce(&self) -> TCE_R {
        TCE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Transmission cancellation finished interrupt enable."]
    #[inline(always)]
    pub fn tcfe(&self) -> TCFE_R {
        TCFE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Tx FIFO empty interrupt enable."]
    #[inline(always)]
    pub fn tfee(&self) -> TFEE_R {
        TFEE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Tx event FIFO new entry interrupt enable."]
    #[inline(always)]
    pub fn tefne(&self) -> TEFNE_R {
        TEFNE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Tx event FIFO watermark reached interrupt enable."]
    #[inline(always)]
    pub fn tefwe(&self) -> TEFWE_R {
        TEFWE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Tx event FIFO full interrupt enable."]
    #[inline(always)]
    pub fn teffe(&self) -> TEFFE_R {
        TEFFE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Tx event FIFO element lost interrupt enable."]
    #[inline(always)]
    pub fn tefle(&self) -> TEFLE_R {
        TEFLE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Timestamp wraparound interrupt enable."]
    #[inline(always)]
    pub fn tswe(&self) -> TSWE_R {
        TSWE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Message RAM access failure interrupt enable."]
    #[inline(always)]
    pub fn mrafe(&self) -> MRAFE_R {
        MRAFE_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Timeout occurred interrupt enable."]
    #[inline(always)]
    pub fn tooe(&self) -> TOOE_R {
        TOOE_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Message stored in dedicated Rx buffer interrupt enable."]
    #[inline(always)]
    pub fn drxe(&self) -> DRXE_R {
        DRXE_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Bit error corrected interrupt enable."]
    #[inline(always)]
    pub fn bece(&self) -> BECE_R {
        BECE_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Bit error uncorrected interrupt enable."]
    #[inline(always)]
    pub fn beue(&self) -> BEUE_R {
        BEUE_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Error logging overflow interrupt enable."]
    #[inline(always)]
    pub fn eloe(&self) -> ELOE_R {
        ELOE_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Error passive interrupt enable."]
    #[inline(always)]
    pub fn epe(&self) -> EPE_R {
        EPE_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Warning status interrupt enable."]
    #[inline(always)]
    pub fn ewe(&self) -> EWE_R {
        EWE_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Bus_Off Status interrupt enable."]
    #[inline(always)]
    pub fn boe(&self) -> BOE_R {
        BOE_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Watchdog interrupt enable."]
    #[inline(always)]
    pub fn wdie(&self) -> WDIE_R {
        WDIE_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Protocol error in arbitration phase interrupt enable."]
    #[inline(always)]
    pub fn peae(&self) -> PEAE_R {
        PEAE_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Protocol error in data phase interrupt enable."]
    #[inline(always)]
    pub fn pede(&self) -> PEDE_R {
        PEDE_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Access to reserved address interrupt enable."]
    #[inline(always)]
    pub fn arae(&self) -> ARAE_R {
        ARAE_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Rx FIFO 0 new message interrupt enable."]
    #[inline(always)]
    pub fn rf0ne(&mut self) -> RF0NE_W<0> {
        RF0NE_W::new(self)
    }
    #[doc = "Bit 1 - Rx FIFO 0 watermark reached interrupt enable."]
    #[inline(always)]
    pub fn rf0we(&mut self) -> RF0WE_W<1> {
        RF0WE_W::new(self)
    }
    #[doc = "Bit 2 - Rx FIFO 0 full interrupt enable."]
    #[inline(always)]
    pub fn rf0fe(&mut self) -> RF0FE_W<2> {
        RF0FE_W::new(self)
    }
    #[doc = "Bit 3 - Rx FIFO 0 message lost interrupt enable."]
    #[inline(always)]
    pub fn rf0le(&mut self) -> RF0LE_W<3> {
        RF0LE_W::new(self)
    }
    #[doc = "Bit 4 - Rx FIFO 1 new message interrupt enable."]
    #[inline(always)]
    pub fn rf1ne(&mut self) -> RF1NE_W<4> {
        RF1NE_W::new(self)
    }
    #[doc = "Bit 5 - Rx FIFO 1 watermark reached interrupt enable."]
    #[inline(always)]
    pub fn rf1we(&mut self) -> RF1WE_W<5> {
        RF1WE_W::new(self)
    }
    #[doc = "Bit 6 - Rx FIFO 1 full interrupt enable."]
    #[inline(always)]
    pub fn rf1fe(&mut self) -> RF1FE_W<6> {
        RF1FE_W::new(self)
    }
    #[doc = "Bit 7 - Rx FIFO 1 message lost interrupt enable."]
    #[inline(always)]
    pub fn rf1le(&mut self) -> RF1LE_W<7> {
        RF1LE_W::new(self)
    }
    #[doc = "Bit 8 - High priority message interrupt enable."]
    #[inline(always)]
    pub fn hpme(&mut self) -> HPME_W<8> {
        HPME_W::new(self)
    }
    #[doc = "Bit 9 - Transmission completed interrupt enable."]
    #[inline(always)]
    pub fn tce(&mut self) -> TCE_W<9> {
        TCE_W::new(self)
    }
    #[doc = "Bit 10 - Transmission cancellation finished interrupt enable."]
    #[inline(always)]
    pub fn tcfe(&mut self) -> TCFE_W<10> {
        TCFE_W::new(self)
    }
    #[doc = "Bit 11 - Tx FIFO empty interrupt enable."]
    #[inline(always)]
    pub fn tfee(&mut self) -> TFEE_W<11> {
        TFEE_W::new(self)
    }
    #[doc = "Bit 12 - Tx event FIFO new entry interrupt enable."]
    #[inline(always)]
    pub fn tefne(&mut self) -> TEFNE_W<12> {
        TEFNE_W::new(self)
    }
    #[doc = "Bit 13 - Tx event FIFO watermark reached interrupt enable."]
    #[inline(always)]
    pub fn tefwe(&mut self) -> TEFWE_W<13> {
        TEFWE_W::new(self)
    }
    #[doc = "Bit 14 - Tx event FIFO full interrupt enable."]
    #[inline(always)]
    pub fn teffe(&mut self) -> TEFFE_W<14> {
        TEFFE_W::new(self)
    }
    #[doc = "Bit 15 - Tx event FIFO element lost interrupt enable."]
    #[inline(always)]
    pub fn tefle(&mut self) -> TEFLE_W<15> {
        TEFLE_W::new(self)
    }
    #[doc = "Bit 16 - Timestamp wraparound interrupt enable."]
    #[inline(always)]
    pub fn tswe(&mut self) -> TSWE_W<16> {
        TSWE_W::new(self)
    }
    #[doc = "Bit 17 - Message RAM access failure interrupt enable."]
    #[inline(always)]
    pub fn mrafe(&mut self) -> MRAFE_W<17> {
        MRAFE_W::new(self)
    }
    #[doc = "Bit 18 - Timeout occurred interrupt enable."]
    #[inline(always)]
    pub fn tooe(&mut self) -> TOOE_W<18> {
        TOOE_W::new(self)
    }
    #[doc = "Bit 19 - Message stored in dedicated Rx buffer interrupt enable."]
    #[inline(always)]
    pub fn drxe(&mut self) -> DRXE_W<19> {
        DRXE_W::new(self)
    }
    #[doc = "Bit 20 - Bit error corrected interrupt enable."]
    #[inline(always)]
    pub fn bece(&mut self) -> BECE_W<20> {
        BECE_W::new(self)
    }
    #[doc = "Bit 21 - Bit error uncorrected interrupt enable."]
    #[inline(always)]
    pub fn beue(&mut self) -> BEUE_W<21> {
        BEUE_W::new(self)
    }
    #[doc = "Bit 22 - Error logging overflow interrupt enable."]
    #[inline(always)]
    pub fn eloe(&mut self) -> ELOE_W<22> {
        ELOE_W::new(self)
    }
    #[doc = "Bit 23 - Error passive interrupt enable."]
    #[inline(always)]
    pub fn epe(&mut self) -> EPE_W<23> {
        EPE_W::new(self)
    }
    #[doc = "Bit 24 - Warning status interrupt enable."]
    #[inline(always)]
    pub fn ewe(&mut self) -> EWE_W<24> {
        EWE_W::new(self)
    }
    #[doc = "Bit 25 - Bus_Off Status interrupt enable."]
    #[inline(always)]
    pub fn boe(&mut self) -> BOE_W<25> {
        BOE_W::new(self)
    }
    #[doc = "Bit 26 - Watchdog interrupt enable."]
    #[inline(always)]
    pub fn wdie(&mut self) -> WDIE_W<26> {
        WDIE_W::new(self)
    }
    #[doc = "Bit 27 - Protocol error in arbitration phase interrupt enable."]
    #[inline(always)]
    pub fn peae(&mut self) -> PEAE_W<27> {
        PEAE_W::new(self)
    }
    #[doc = "Bit 28 - Protocol error in data phase interrupt enable."]
    #[inline(always)]
    pub fn pede(&mut self) -> PEDE_W<28> {
        PEDE_W::new(self)
    }
    #[doc = "Bit 29 - Access to reserved address interrupt enable."]
    #[inline(always)]
    pub fn arae(&mut self) -> ARAE_W<29> {
        ARAE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Enable.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ie](index.html) module"]
pub struct IE_SPEC;
impl crate::RegisterSpec for IE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ie::R](R) reader structure"]
impl crate::Readable for IE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ie::W](W) writer structure"]
impl crate::Writable for IE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IE to value 0"]
impl crate::Resettable for IE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
