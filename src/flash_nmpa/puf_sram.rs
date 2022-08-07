#[doc = "Register `PUF_SRAM` reader"]
pub struct R(crate::R<PUF_SRAM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PUF_SRAM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PUF_SRAM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PUF_SRAM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PUF_SRAM` writer"]
pub struct W(crate::W<PUF_SRAM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PUF_SRAM_SPEC>;
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
impl From<crate::W<PUF_SRAM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PUF_SRAM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PUF_SRAM_VALID` reader - 1: PUF_SRAM is valid."]
pub type PUF_SRAM_VALID_R = crate::BitReader<bool>;
#[doc = "Field `PUF_SRAM_VALID` writer - 1: PUF_SRAM is valid."]
pub type PUF_SRAM_VALID_W<'a, const O: u8> = crate::BitWriter<'a, u32, PUF_SRAM_SPEC, bool, O>;
#[doc = "Field `mode` reader - PUF SRAM Controller operating mode"]
pub type MODE_R = crate::BitReader<bool>;
#[doc = "Field `mode` writer - PUF SRAM Controller operating mode"]
pub type MODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, PUF_SRAM_SPEC, bool, O>;
#[doc = "Field `ckgating` reader - PUF SRAM Clock Gating control"]
pub type CKGATING_R = crate::BitReader<bool>;
#[doc = "Field `ckgating` writer - PUF SRAM Clock Gating control"]
pub type CKGATING_W<'a, const O: u8> = crate::BitWriter<'a, u32, PUF_SRAM_SPEC, bool, O>;
#[doc = "Field `SMB` reader - Source Biasing voltage."]
pub type SMB_R = crate::FieldReader<u8, SMB_A>;
#[doc = "Source Biasing voltage.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SMB_A {
    #[doc = "0: Low leakage."]
    LOW = 0,
    #[doc = "1: Medium leakage."]
    MEDIUM = 1,
    #[doc = "2: Highest leakage."]
    HIGHEST = 2,
    #[doc = "3: Disable."]
    DISABLE = 3,
}
impl From<SMB_A> for u8 {
    #[inline(always)]
    fn from(variant: SMB_A) -> Self {
        variant as _
    }
}
impl SMB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMB_A {
        match self.bits {
            0 => SMB_A::LOW,
            1 => SMB_A::MEDIUM,
            2 => SMB_A::HIGHEST,
            3 => SMB_A::DISABLE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == SMB_A::LOW
    }
    #[doc = "Checks if the value of the field is `MEDIUM`"]
    #[inline(always)]
    pub fn is_medium(&self) -> bool {
        *self == SMB_A::MEDIUM
    }
    #[doc = "Checks if the value of the field is `HIGHEST`"]
    #[inline(always)]
    pub fn is_highest(&self) -> bool {
        *self == SMB_A::HIGHEST
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == SMB_A::DISABLE
    }
}
#[doc = "Field `SMB` writer - Source Biasing voltage."]
pub type SMB_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, PUF_SRAM_SPEC, u8, SMB_A, 2, O>;
impl<'a, const O: u8> SMB_W<'a, O> {
    #[doc = "Low leakage."]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(SMB_A::LOW)
    }
    #[doc = "Medium leakage."]
    #[inline(always)]
    pub fn medium(self) -> &'a mut W {
        self.variant(SMB_A::MEDIUM)
    }
    #[doc = "Highest leakage."]
    #[inline(always)]
    pub fn highest(self) -> &'a mut W {
        self.variant(SMB_A::HIGHEST)
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SMB_A::DISABLE)
    }
}
#[doc = "Field `RM` reader - Read Margin control settings."]
pub type RM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RM` writer - Read Margin control settings."]
pub type RM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PUF_SRAM_SPEC, u8, u8, 3, O>;
#[doc = "Field `WM` reader - Write Margin control settings."]
pub type WM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WM` writer - Write Margin control settings."]
pub type WM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PUF_SRAM_SPEC, u8, u8, 3, O>;
#[doc = "Field `WRME` reader - Write read margin enable."]
pub type WRME_R = crate::BitReader<bool>;
#[doc = "Field `WRME` writer - Write read margin enable."]
pub type WRME_W<'a, const O: u8> = crate::BitWriter<'a, u32, PUF_SRAM_SPEC, bool, O>;
#[doc = "Field `RAEN` reader - SRAM Read Assist Enable"]
pub type RAEN_R = crate::BitReader<bool>;
#[doc = "Field `RAEN` writer - SRAM Read Assist Enable"]
pub type RAEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PUF_SRAM_SPEC, bool, O>;
#[doc = "Field `RAM` reader - SRAM Read Assist settings"]
pub type RAM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RAM` writer - SRAM Read Assist settings"]
pub type RAM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PUF_SRAM_SPEC, u8, u8, 4, O>;
#[doc = "Field `WAEN` reader - SRAM Write Assist Enable"]
pub type WAEN_R = crate::BitReader<bool>;
#[doc = "Field `WAEN` writer - SRAM Write Assist Enable"]
pub type WAEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PUF_SRAM_SPEC, bool, O>;
#[doc = "Field `WAM` reader - SRAM Write Assist settings"]
pub type WAM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WAM` writer - SRAM Write Assist settings"]
pub type WAM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PUF_SRAM_SPEC, u8, u8, 2, O>;
#[doc = "Field `STBP` reader - STBP"]
pub type STBP_R = crate::BitReader<bool>;
#[doc = "Field `STBP` writer - STBP"]
pub type STBP_W<'a, const O: u8> = crate::BitWriter<'a, u32, PUF_SRAM_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - 1: PUF_SRAM is valid."]
    #[inline(always)]
    pub fn puf_sram_valid(&self) -> PUF_SRAM_VALID_R {
        PUF_SRAM_VALID_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PUF SRAM Controller operating mode"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PUF SRAM Clock Gating control"]
    #[inline(always)]
    pub fn ckgating(&self) -> CKGATING_R {
        CKGATING_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Source Biasing voltage."]
    #[inline(always)]
    pub fn smb(&self) -> SMB_R {
        SMB_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:12 - Read Margin control settings."]
    #[inline(always)]
    pub fn rm(&self) -> RM_R {
        RM_R::new(((self.bits >> 10) & 7) as u8)
    }
    #[doc = "Bits 13:15 - Write Margin control settings."]
    #[inline(always)]
    pub fn wm(&self) -> WM_R {
        WM_R::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bit 16 - Write read margin enable."]
    #[inline(always)]
    pub fn wrme(&self) -> WRME_R {
        WRME_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - SRAM Read Assist Enable"]
    #[inline(always)]
    pub fn raen(&self) -> RAEN_R {
        RAEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:21 - SRAM Read Assist settings"]
    #[inline(always)]
    pub fn ram(&self) -> RAM_R {
        RAM_R::new(((self.bits >> 18) & 0x0f) as u8)
    }
    #[doc = "Bit 22 - SRAM Write Assist Enable"]
    #[inline(always)]
    pub fn waen(&self) -> WAEN_R {
        WAEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bits 23:24 - SRAM Write Assist settings"]
    #[inline(always)]
    pub fn wam(&self) -> WAM_R {
        WAM_R::new(((self.bits >> 23) & 3) as u8)
    }
    #[doc = "Bit 25 - STBP"]
    #[inline(always)]
    pub fn stbp(&self) -> STBP_R {
        STBP_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 1: PUF_SRAM is valid."]
    #[inline(always)]
    pub fn puf_sram_valid(&mut self) -> PUF_SRAM_VALID_W<0> {
        PUF_SRAM_VALID_W::new(self)
    }
    #[doc = "Bit 1 - PUF SRAM Controller operating mode"]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W<1> {
        MODE_W::new(self)
    }
    #[doc = "Bit 2 - PUF SRAM Clock Gating control"]
    #[inline(always)]
    pub fn ckgating(&mut self) -> CKGATING_W<2> {
        CKGATING_W::new(self)
    }
    #[doc = "Bits 8:9 - Source Biasing voltage."]
    #[inline(always)]
    pub fn smb(&mut self) -> SMB_W<8> {
        SMB_W::new(self)
    }
    #[doc = "Bits 10:12 - Read Margin control settings."]
    #[inline(always)]
    pub fn rm(&mut self) -> RM_W<10> {
        RM_W::new(self)
    }
    #[doc = "Bits 13:15 - Write Margin control settings."]
    #[inline(always)]
    pub fn wm(&mut self) -> WM_W<13> {
        WM_W::new(self)
    }
    #[doc = "Bit 16 - Write read margin enable."]
    #[inline(always)]
    pub fn wrme(&mut self) -> WRME_W<16> {
        WRME_W::new(self)
    }
    #[doc = "Bit 17 - SRAM Read Assist Enable"]
    #[inline(always)]
    pub fn raen(&mut self) -> RAEN_W<17> {
        RAEN_W::new(self)
    }
    #[doc = "Bits 18:21 - SRAM Read Assist settings"]
    #[inline(always)]
    pub fn ram(&mut self) -> RAM_W<18> {
        RAM_W::new(self)
    }
    #[doc = "Bit 22 - SRAM Write Assist Enable"]
    #[inline(always)]
    pub fn waen(&mut self) -> WAEN_W<22> {
        WAEN_W::new(self)
    }
    #[doc = "Bits 23:24 - SRAM Write Assist settings"]
    #[inline(always)]
    pub fn wam(&mut self) -> WAM_W<23> {
        WAM_W::new(self)
    }
    #[doc = "Bit 25 - STBP"]
    #[inline(always)]
    pub fn stbp(&mut self) -> STBP_W<25> {
        STBP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "no description available\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [puf_sram](index.html) module"]
pub struct PUF_SRAM_SPEC;
impl crate::RegisterSpec for PUF_SRAM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [puf_sram::R](R) reader structure"]
impl crate::Readable for PUF_SRAM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [puf_sram::W](W) writer structure"]
impl crate::Writable for PUF_SRAM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PUF_SRAM to value 0"]
impl crate::Resettable for PUF_SRAM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
