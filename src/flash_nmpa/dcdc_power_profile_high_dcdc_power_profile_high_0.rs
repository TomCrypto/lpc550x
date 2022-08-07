#[doc = "Register `DCDC_POWER_PROFILE_HIGH_0` reader"]
pub struct R(crate::R<DCDC_POWER_PROFILE_HIGH_DCDC_POWER_PROFILE_HIGH_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DCDC_POWER_PROFILE_HIGH_DCDC_POWER_PROFILE_HIGH_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DCDC_POWER_PROFILE_HIGH_DCDC_POWER_PROFILE_HIGH_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DCDC_POWER_PROFILE_HIGH_DCDC_POWER_PROFILE_HIGH_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DCDC_POWER_PROFILE_HIGH_0` writer"]
pub struct W(crate::W<DCDC_POWER_PROFILE_HIGH_DCDC_POWER_PROFILE_HIGH_0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DCDC_POWER_PROFILE_HIGH_DCDC_POWER_PROFILE_HIGH_0_SPEC>;
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
impl From<crate::W<DCDC_POWER_PROFILE_HIGH_DCDC_POWER_PROFILE_HIGH_0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DCDC_POWER_PROFILE_HIGH_DCDC_POWER_PROFILE_HIGH_0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DCDC_TRIM_VALID` reader - DCDC is trimed."]
pub type DCDC_TRIM_VALID_R = crate::BitReader<bool>;
#[doc = "Field `DCDC_TRIM_VALID` writer - DCDC is trimed."]
pub type DCDC_TRIM_VALID_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DCDC_POWER_PROFILE_HIGH_DCDC_POWER_PROFILE_HIGH_0_SPEC, bool, O>;
#[doc = "Field `RC` reader - Constant On-Time calibration."]
pub type RC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RC` writer - Constant On-Time calibration."]
pub type RC_W<'a, const O: u8> = crate::FieldWriter<
    'a,
    u32,
    DCDC_POWER_PROFILE_HIGH_DCDC_POWER_PROFILE_HIGH_0_SPEC,
    u8,
    u8,
    6,
    O,
>;
#[doc = "Field `ICOMP` reader - Select the type of ZCD comparator."]
pub type ICOMP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ICOMP` writer - Select the type of ZCD comparator."]
pub type ICOMP_W<'a, const O: u8> = crate::FieldWriter<
    'a,
    u32,
    DCDC_POWER_PROFILE_HIGH_DCDC_POWER_PROFILE_HIGH_0_SPEC,
    u8,
    u8,
    2,
    O,
>;
#[doc = "Field `ISEL` reader - Alter Internal biasing currents."]
pub type ISEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ISEL` writer - Alter Internal biasing currents."]
pub type ISEL_W<'a, const O: u8> = crate::FieldWriter<
    'a,
    u32,
    DCDC_POWER_PROFILE_HIGH_DCDC_POWER_PROFILE_HIGH_0_SPEC,
    u8,
    u8,
    2,
    O,
>;
#[doc = "Field `ICENABLE` reader - Selection of auto scaling of COT period with variations in VDD."]
pub type ICENABLE_R = crate::BitReader<bool>;
#[doc = "Field `ICENABLE` writer - Selection of auto scaling of COT period with variations in VDD."]
pub type ICENABLE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DCDC_POWER_PROFILE_HIGH_DCDC_POWER_PROFILE_HIGH_0_SPEC, bool, O>;
#[doc = "Field `TMOS` reader - One-shot generator reference current trimming signal."]
pub type TMOS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TMOS` writer - One-shot generator reference current trimming signal."]
pub type TMOS_W<'a, const O: u8> = crate::FieldWriter<
    'a,
    u32,
    DCDC_POWER_PROFILE_HIGH_DCDC_POWER_PROFILE_HIGH_0_SPEC,
    u8,
    u8,
    5,
    O,
>;
#[doc = "Field `DISABLEISENSE` reader - Disable Current sensing."]
pub type DISABLEISENSE_R = crate::BitReader<bool>;
#[doc = "Field `DISABLEISENSE` writer - Disable Current sensing."]
pub type DISABLEISENSE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DCDC_POWER_PROFILE_HIGH_DCDC_POWER_PROFILE_HIGH_0_SPEC, bool, O>;
#[doc = "Field `VOUT` reader - Set output regulation voltage."]
pub type VOUT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VOUT` writer - Set output regulation voltage."]
pub type VOUT_W<'a, const O: u8> = crate::FieldWriter<
    'a,
    u32,
    DCDC_POWER_PROFILE_HIGH_DCDC_POWER_PROFILE_HIGH_0_SPEC,
    u8,
    u8,
    4,
    O,
>;
#[doc = "Field `SLICINGENABLE` reader - Enable staggered switching of power switches."]
pub type SLICINGENABLE_R = crate::BitReader<bool>;
#[doc = "Field `SLICINGENABLE` writer - Enable staggered switching of power switches."]
pub type SLICINGENABLE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DCDC_POWER_PROFILE_HIGH_DCDC_POWER_PROFILE_HIGH_0_SPEC, bool, O>;
#[doc = "Field `INDUCTORCLAMPENABLE` reader - Enable shorting of Inductor during PFM idle time."]
pub type INDUCTORCLAMPENABLE_R = crate::BitReader<bool>;
#[doc = "Field `INDUCTORCLAMPENABLE` writer - Enable shorting of Inductor during PFM idle time."]
pub type INDUCTORCLAMPENABLE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DCDC_POWER_PROFILE_HIGH_DCDC_POWER_PROFILE_HIGH_0_SPEC, bool, O>;
#[doc = "Field `VOUT_PWD` reader - Set output regulation voltage during Deep Sleep."]
pub type VOUT_PWD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VOUT_PWD` writer - Set output regulation voltage during Deep Sleep."]
pub type VOUT_PWD_W<'a, const O: u8> = crate::FieldWriter<
    'a,
    u32,
    DCDC_POWER_PROFILE_HIGH_DCDC_POWER_PROFILE_HIGH_0_SPEC,
    u8,
    u8,
    4,
    O,
>;
impl R {
    #[doc = "Bit 0 - DCDC is trimed."]
    #[inline(always)]
    pub fn dcdc_trim_valid(&self) -> DCDC_TRIM_VALID_R {
        DCDC_TRIM_VALID_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:6 - Constant On-Time calibration."]
    #[inline(always)]
    pub fn rc(&self) -> RC_R {
        RC_R::new(((self.bits >> 1) & 0x3f) as u8)
    }
    #[doc = "Bits 7:8 - Select the type of ZCD comparator."]
    #[inline(always)]
    pub fn icomp(&self) -> ICOMP_R {
        ICOMP_R::new(((self.bits >> 7) & 3) as u8)
    }
    #[doc = "Bits 9:10 - Alter Internal biasing currents."]
    #[inline(always)]
    pub fn isel(&self) -> ISEL_R {
        ISEL_R::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bit 11 - Selection of auto scaling of COT period with variations in VDD."]
    #[inline(always)]
    pub fn icenable(&self) -> ICENABLE_R {
        ICENABLE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:16 - One-shot generator reference current trimming signal."]
    #[inline(always)]
    pub fn tmos(&self) -> TMOS_R {
        TMOS_R::new(((self.bits >> 12) & 0x1f) as u8)
    }
    #[doc = "Bit 17 - Disable Current sensing."]
    #[inline(always)]
    pub fn disableisense(&self) -> DISABLEISENSE_R {
        DISABLEISENSE_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:21 - Set output regulation voltage."]
    #[inline(always)]
    pub fn vout(&self) -> VOUT_R {
        VOUT_R::new(((self.bits >> 18) & 0x0f) as u8)
    }
    #[doc = "Bit 22 - Enable staggered switching of power switches."]
    #[inline(always)]
    pub fn slicingenable(&self) -> SLICINGENABLE_R {
        SLICINGENABLE_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Enable shorting of Inductor during PFM idle time."]
    #[inline(always)]
    pub fn inductorclampenable(&self) -> INDUCTORCLAMPENABLE_R {
        INDUCTORCLAMPENABLE_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:27 - Set output regulation voltage during Deep Sleep."]
    #[inline(always)]
    pub fn vout_pwd(&self) -> VOUT_PWD_R {
        VOUT_PWD_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - DCDC is trimed."]
    #[inline(always)]
    pub fn dcdc_trim_valid(&mut self) -> DCDC_TRIM_VALID_W<0> {
        DCDC_TRIM_VALID_W::new(self)
    }
    #[doc = "Bits 1:6 - Constant On-Time calibration."]
    #[inline(always)]
    pub fn rc(&mut self) -> RC_W<1> {
        RC_W::new(self)
    }
    #[doc = "Bits 7:8 - Select the type of ZCD comparator."]
    #[inline(always)]
    pub fn icomp(&mut self) -> ICOMP_W<7> {
        ICOMP_W::new(self)
    }
    #[doc = "Bits 9:10 - Alter Internal biasing currents."]
    #[inline(always)]
    pub fn isel(&mut self) -> ISEL_W<9> {
        ISEL_W::new(self)
    }
    #[doc = "Bit 11 - Selection of auto scaling of COT period with variations in VDD."]
    #[inline(always)]
    pub fn icenable(&mut self) -> ICENABLE_W<11> {
        ICENABLE_W::new(self)
    }
    #[doc = "Bits 12:16 - One-shot generator reference current trimming signal."]
    #[inline(always)]
    pub fn tmos(&mut self) -> TMOS_W<12> {
        TMOS_W::new(self)
    }
    #[doc = "Bit 17 - Disable Current sensing."]
    #[inline(always)]
    pub fn disableisense(&mut self) -> DISABLEISENSE_W<17> {
        DISABLEISENSE_W::new(self)
    }
    #[doc = "Bits 18:21 - Set output regulation voltage."]
    #[inline(always)]
    pub fn vout(&mut self) -> VOUT_W<18> {
        VOUT_W::new(self)
    }
    #[doc = "Bit 22 - Enable staggered switching of power switches."]
    #[inline(always)]
    pub fn slicingenable(&mut self) -> SLICINGENABLE_W<22> {
        SLICINGENABLE_W::new(self)
    }
    #[doc = "Bit 23 - Enable shorting of Inductor during PFM idle time."]
    #[inline(always)]
    pub fn inductorclampenable(&mut self) -> INDUCTORCLAMPENABLE_W<23> {
        INDUCTORCLAMPENABLE_W::new(self)
    }
    #[doc = "Bits 24:27 - Set output regulation voltage during Deep Sleep."]
    #[inline(always)]
    pub fn vout_pwd(&mut self) -> VOUT_PWD_W<24> {
        VOUT_PWD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "no description available\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcdc_power_profile_high_dcdc_power_profile_high_0](index.html) module"]
pub struct DCDC_POWER_PROFILE_HIGH_DCDC_POWER_PROFILE_HIGH_0_SPEC;
impl crate::RegisterSpec for DCDC_POWER_PROFILE_HIGH_DCDC_POWER_PROFILE_HIGH_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dcdc_power_profile_high_dcdc_power_profile_high_0::R](R) reader structure"]
impl crate::Readable for DCDC_POWER_PROFILE_HIGH_DCDC_POWER_PROFILE_HIGH_0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dcdc_power_profile_high_dcdc_power_profile_high_0::W](W) writer structure"]
impl crate::Writable for DCDC_POWER_PROFILE_HIGH_DCDC_POWER_PROFILE_HIGH_0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DCDC_POWER_PROFILE_HIGH_0 to value 0"]
impl crate::Resettable for DCDC_POWER_PROFILE_HIGH_DCDC_POWER_PROFILE_HIGH_0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
