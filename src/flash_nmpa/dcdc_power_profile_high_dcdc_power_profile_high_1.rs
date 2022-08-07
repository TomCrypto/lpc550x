#[doc = "Register `DCDC_POWER_PROFILE_HIGH_1` reader"]
pub struct R(crate::R<DCDC_POWER_PROFILE_HIGH_DCDC_POWER_PROFILE_HIGH_1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DCDC_POWER_PROFILE_HIGH_DCDC_POWER_PROFILE_HIGH_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DCDC_POWER_PROFILE_HIGH_DCDC_POWER_PROFILE_HIGH_1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DCDC_POWER_PROFILE_HIGH_DCDC_POWER_PROFILE_HIGH_1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DCDC_POWER_PROFILE_HIGH_1` writer"]
pub struct W(crate::W<DCDC_POWER_PROFILE_HIGH_DCDC_POWER_PROFILE_HIGH_1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DCDC_POWER_PROFILE_HIGH_DCDC_POWER_PROFILE_HIGH_1_SPEC>;
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
impl From<crate::W<DCDC_POWER_PROFILE_HIGH_DCDC_POWER_PROFILE_HIGH_1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DCDC_POWER_PROFILE_HIGH_DCDC_POWER_PROFILE_HIGH_1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RTRIMOFFET` reader - Adjust the offset voltage of BJT based comparator."]
pub type RTRIMOFFET_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RTRIMOFFET` writer - Adjust the offset voltage of BJT based comparator."]
pub type RTRIMOFFET_W<'a, const O: u8> = crate::FieldWriter<
    'a,
    u32,
    DCDC_POWER_PROFILE_HIGH_DCDC_POWER_PROFILE_HIGH_1_SPEC,
    u8,
    u8,
    4,
    O,
>;
#[doc = "Field `RSENSETRIM` reader - Adjust Max inductor peak current limiting."]
pub type RSENSETRIM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RSENSETRIM` writer - Adjust Max inductor peak current limiting."]
pub type RSENSETRIM_W<'a, const O: u8> = crate::FieldWriter<
    'a,
    u32,
    DCDC_POWER_PROFILE_HIGH_DCDC_POWER_PROFILE_HIGH_1_SPEC,
    u8,
    u8,
    4,
    O,
>;
#[doc = "Field `DTESTENABLE` reader - Enable Digital test signals."]
pub type DTESTENABLE_R = crate::BitReader<bool>;
#[doc = "Field `DTESTENABLE` writer - Enable Digital test signals."]
pub type DTESTENABLE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DCDC_POWER_PROFILE_HIGH_DCDC_POWER_PROFILE_HIGH_1_SPEC, bool, O>;
#[doc = "Field `SETCURVE` reader - Bandgap calibration parameter."]
pub type SETCURVE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SETCURVE` writer - Bandgap calibration parameter."]
pub type SETCURVE_W<'a, const O: u8> = crate::FieldWriter<
    'a,
    u32,
    DCDC_POWER_PROFILE_HIGH_DCDC_POWER_PROFILE_HIGH_1_SPEC,
    u8,
    u8,
    2,
    O,
>;
#[doc = "Field `SETDC` reader - Bandgap calibration parameter."]
pub type SETDC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SETDC` writer - Bandgap calibration parameter."]
pub type SETDC_W<'a, const O: u8> = crate::FieldWriter<
    'a,
    u32,
    DCDC_POWER_PROFILE_HIGH_DCDC_POWER_PROFILE_HIGH_1_SPEC,
    u8,
    u8,
    4,
    O,
>;
#[doc = "Field `DTESTSEL` reader - Select the output signal for test."]
pub type DTESTSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DTESTSEL` writer - Select the output signal for test."]
pub type DTESTSEL_W<'a, const O: u8> = crate::FieldWriter<
    'a,
    u32,
    DCDC_POWER_PROFILE_HIGH_DCDC_POWER_PROFILE_HIGH_1_SPEC,
    u8,
    u8,
    3,
    O,
>;
#[doc = "Field `ISCALEENABLE` reader - Modify COT behavior."]
pub type ISCALEENABLE_R = crate::BitReader<bool>;
#[doc = "Field `ISCALEENABLE` writer - Modify COT behavior."]
pub type ISCALEENABLE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DCDC_POWER_PROFILE_HIGH_DCDC_POWER_PROFILE_HIGH_1_SPEC, bool, O>;
#[doc = "Field `FORCEBYPASS` reader - Force bypass mode."]
pub type FORCEBYPASS_R = crate::BitReader<bool>;
#[doc = "Field `FORCEBYPASS` writer - Force bypass mode."]
pub type FORCEBYPASS_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DCDC_POWER_PROFILE_HIGH_DCDC_POWER_PROFILE_HIGH_1_SPEC, bool, O>;
#[doc = "Field `TRIMAUTOCOT` reader - Change the scaling ratio of the feedforward compensation."]
pub type TRIMAUTOCOT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TRIMAUTOCOT` writer - Change the scaling ratio of the feedforward compensation."]
pub type TRIMAUTOCOT_W<'a, const O: u8> = crate::FieldWriter<
    'a,
    u32,
    DCDC_POWER_PROFILE_HIGH_DCDC_POWER_PROFILE_HIGH_1_SPEC,
    u8,
    u8,
    4,
    O,
>;
#[doc = "Field `FORCEFULLCYCLE` reader - Force full PFM PMOS and NMOS cycle."]
pub type FORCEFULLCYCLE_R = crate::BitReader<bool>;
#[doc = "Field `FORCEFULLCYCLE` writer - Force full PFM PMOS and NMOS cycle."]
pub type FORCEFULLCYCLE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DCDC_POWER_PROFILE_HIGH_DCDC_POWER_PROFILE_HIGH_1_SPEC, bool, O>;
#[doc = "Field `LCENABLE` reader - Change the range of the peak detector of current inside the inductor."]
pub type LCENABLE_R = crate::BitReader<bool>;
#[doc = "Field `LCENABLE` writer - Change the range of the peak detector of current inside the inductor."]
pub type LCENABLE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DCDC_POWER_PROFILE_HIGH_DCDC_POWER_PROFILE_HIGH_1_SPEC, bool, O>;
#[doc = "Field `TOFF` reader - Constant Off-Time calibration input."]
pub type TOFF_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TOFF` writer - Constant Off-Time calibration input."]
pub type TOFF_W<'a, const O: u8> = crate::FieldWriter<
    'a,
    u32,
    DCDC_POWER_PROFILE_HIGH_DCDC_POWER_PROFILE_HIGH_1_SPEC,
    u8,
    u8,
    5,
    O,
>;
#[doc = "Field `TOFFENABLE` reader - Enable Constant Off-Time feature."]
pub type TOFFENABLE_R = crate::BitReader<bool>;
#[doc = "Field `TOFFENABLE` writer - Enable Constant Off-Time feature."]
pub type TOFFENABLE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DCDC_POWER_PROFILE_HIGH_DCDC_POWER_PROFILE_HIGH_1_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:3 - Adjust the offset voltage of BJT based comparator."]
    #[inline(always)]
    pub fn rtrimoffet(&self) -> RTRIMOFFET_R {
        RTRIMOFFET_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Adjust Max inductor peak current limiting."]
    #[inline(always)]
    pub fn rsensetrim(&self) -> RSENSETRIM_R {
        RSENSETRIM_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Enable Digital test signals."]
    #[inline(always)]
    pub fn dtestenable(&self) -> DTESTENABLE_R {
        DTESTENABLE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:10 - Bandgap calibration parameter."]
    #[inline(always)]
    pub fn setcurve(&self) -> SETCURVE_R {
        SETCURVE_R::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bits 11:14 - Bandgap calibration parameter."]
    #[inline(always)]
    pub fn setdc(&self) -> SETDC_R {
        SETDC_R::new(((self.bits >> 11) & 0x0f) as u8)
    }
    #[doc = "Bits 15:17 - Select the output signal for test."]
    #[inline(always)]
    pub fn dtestsel(&self) -> DTESTSEL_R {
        DTESTSEL_R::new(((self.bits >> 15) & 7) as u8)
    }
    #[doc = "Bit 18 - Modify COT behavior."]
    #[inline(always)]
    pub fn iscaleenable(&self) -> ISCALEENABLE_R {
        ISCALEENABLE_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Force bypass mode."]
    #[inline(always)]
    pub fn forcebypass(&self) -> FORCEBYPASS_R {
        FORCEBYPASS_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:23 - Change the scaling ratio of the feedforward compensation."]
    #[inline(always)]
    pub fn trimautocot(&self) -> TRIMAUTOCOT_R {
        TRIMAUTOCOT_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - Force full PFM PMOS and NMOS cycle."]
    #[inline(always)]
    pub fn forcefullcycle(&self) -> FORCEFULLCYCLE_R {
        FORCEFULLCYCLE_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Change the range of the peak detector of current inside the inductor."]
    #[inline(always)]
    pub fn lcenable(&self) -> LCENABLE_R {
        LCENABLE_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bits 26:30 - Constant Off-Time calibration input."]
    #[inline(always)]
    pub fn toff(&self) -> TOFF_R {
        TOFF_R::new(((self.bits >> 26) & 0x1f) as u8)
    }
    #[doc = "Bit 31 - Enable Constant Off-Time feature."]
    #[inline(always)]
    pub fn toffenable(&self) -> TOFFENABLE_R {
        TOFFENABLE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Adjust the offset voltage of BJT based comparator."]
    #[inline(always)]
    pub fn rtrimoffet(&mut self) -> RTRIMOFFET_W<0> {
        RTRIMOFFET_W::new(self)
    }
    #[doc = "Bits 4:7 - Adjust Max inductor peak current limiting."]
    #[inline(always)]
    pub fn rsensetrim(&mut self) -> RSENSETRIM_W<4> {
        RSENSETRIM_W::new(self)
    }
    #[doc = "Bit 8 - Enable Digital test signals."]
    #[inline(always)]
    pub fn dtestenable(&mut self) -> DTESTENABLE_W<8> {
        DTESTENABLE_W::new(self)
    }
    #[doc = "Bits 9:10 - Bandgap calibration parameter."]
    #[inline(always)]
    pub fn setcurve(&mut self) -> SETCURVE_W<9> {
        SETCURVE_W::new(self)
    }
    #[doc = "Bits 11:14 - Bandgap calibration parameter."]
    #[inline(always)]
    pub fn setdc(&mut self) -> SETDC_W<11> {
        SETDC_W::new(self)
    }
    #[doc = "Bits 15:17 - Select the output signal for test."]
    #[inline(always)]
    pub fn dtestsel(&mut self) -> DTESTSEL_W<15> {
        DTESTSEL_W::new(self)
    }
    #[doc = "Bit 18 - Modify COT behavior."]
    #[inline(always)]
    pub fn iscaleenable(&mut self) -> ISCALEENABLE_W<18> {
        ISCALEENABLE_W::new(self)
    }
    #[doc = "Bit 19 - Force bypass mode."]
    #[inline(always)]
    pub fn forcebypass(&mut self) -> FORCEBYPASS_W<19> {
        FORCEBYPASS_W::new(self)
    }
    #[doc = "Bits 20:23 - Change the scaling ratio of the feedforward compensation."]
    #[inline(always)]
    pub fn trimautocot(&mut self) -> TRIMAUTOCOT_W<20> {
        TRIMAUTOCOT_W::new(self)
    }
    #[doc = "Bit 24 - Force full PFM PMOS and NMOS cycle."]
    #[inline(always)]
    pub fn forcefullcycle(&mut self) -> FORCEFULLCYCLE_W<24> {
        FORCEFULLCYCLE_W::new(self)
    }
    #[doc = "Bit 25 - Change the range of the peak detector of current inside the inductor."]
    #[inline(always)]
    pub fn lcenable(&mut self) -> LCENABLE_W<25> {
        LCENABLE_W::new(self)
    }
    #[doc = "Bits 26:30 - Constant Off-Time calibration input."]
    #[inline(always)]
    pub fn toff(&mut self) -> TOFF_W<26> {
        TOFF_W::new(self)
    }
    #[doc = "Bit 31 - Enable Constant Off-Time feature."]
    #[inline(always)]
    pub fn toffenable(&mut self) -> TOFFENABLE_W<31> {
        TOFFENABLE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "no description available\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcdc_power_profile_high_dcdc_power_profile_high_1](index.html) module"]
pub struct DCDC_POWER_PROFILE_HIGH_DCDC_POWER_PROFILE_HIGH_1_SPEC;
impl crate::RegisterSpec for DCDC_POWER_PROFILE_HIGH_DCDC_POWER_PROFILE_HIGH_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dcdc_power_profile_high_dcdc_power_profile_high_1::R](R) reader structure"]
impl crate::Readable for DCDC_POWER_PROFILE_HIGH_DCDC_POWER_PROFILE_HIGH_1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dcdc_power_profile_high_dcdc_power_profile_high_1::W](W) writer structure"]
impl crate::Writable for DCDC_POWER_PROFILE_HIGH_DCDC_POWER_PROFILE_HIGH_1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DCDC_POWER_PROFILE_HIGH_1 to value 0"]
impl crate::Resettable for DCDC_POWER_PROFILE_HIGH_DCDC_POWER_PROFILE_HIGH_1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
