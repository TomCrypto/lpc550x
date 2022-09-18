#[doc = "Register `CMDL4` reader"]
pub struct R(crate::R<CMDL4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMDL4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMDL4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMDL4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CMDL4` writer"]
pub struct W(crate::W<CMDL4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMDL4_SPEC>;
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
impl From<crate::W<CMDL4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMDL4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADCH` reader - Input channel select."]
pub type ADCH_R = crate::FieldReader<u8, ADCH_A>;
#[doc = "Input channel select.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ADCH_A {
    #[doc = "0: Select CH0A/ADC0_0 or CH0B/ADC0_8."]
    CHANNEL_0 = 0,
    #[doc = "1: Select CH1A/ADC0_1 or CH1B/ADC0_9."]
    CHANNEL_1 = 1,
    #[doc = "2: Select CH2A/ADC0_2 or CH2B/ADC0_10."]
    CHANNEL_2 = 2,
    #[doc = "3: Select CH3A/ADC0_3 or CH3B/ADC0_11."]
    CHANNEL_3 = 3,
    #[doc = "4: Select CH4B/ADC0_12."]
    CHANNEL_4 = 4,
    #[doc = "12: Select VDDA."]
    VDDA = 12,
    #[doc = "13: Select internal ADC bandgap reference."]
    BANDGAP = 13,
    #[doc = "26: Select temperature sensor."]
    TEMP_SENSOR = 26,
}
impl From<ADCH_A> for u8 {
    #[inline(always)]
    fn from(variant: ADCH_A) -> Self {
        variant as _
    }
}
impl ADCH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ADCH_A> {
        match self.bits {
            0 => Some(ADCH_A::CHANNEL_0),
            1 => Some(ADCH_A::CHANNEL_1),
            2 => Some(ADCH_A::CHANNEL_2),
            3 => Some(ADCH_A::CHANNEL_3),
            4 => Some(ADCH_A::CHANNEL_4),
            12 => Some(ADCH_A::VDDA),
            13 => Some(ADCH_A::BANDGAP),
            26 => Some(ADCH_A::TEMP_SENSOR),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `CHANNEL_0`"]
    #[inline(always)]
    pub fn is_channel_0(&self) -> bool {
        *self == ADCH_A::CHANNEL_0
    }
    #[doc = "Checks if the value of the field is `CHANNEL_1`"]
    #[inline(always)]
    pub fn is_channel_1(&self) -> bool {
        *self == ADCH_A::CHANNEL_1
    }
    #[doc = "Checks if the value of the field is `CHANNEL_2`"]
    #[inline(always)]
    pub fn is_channel_2(&self) -> bool {
        *self == ADCH_A::CHANNEL_2
    }
    #[doc = "Checks if the value of the field is `CHANNEL_3`"]
    #[inline(always)]
    pub fn is_channel_3(&self) -> bool {
        *self == ADCH_A::CHANNEL_3
    }
    #[doc = "Checks if the value of the field is `CHANNEL_4`"]
    #[inline(always)]
    pub fn is_channel_4(&self) -> bool {
        *self == ADCH_A::CHANNEL_4
    }
    #[doc = "Checks if the value of the field is `VDDA`"]
    #[inline(always)]
    pub fn is_vdda(&self) -> bool {
        *self == ADCH_A::VDDA
    }
    #[doc = "Checks if the value of the field is `BANDGAP`"]
    #[inline(always)]
    pub fn is_bandgap(&self) -> bool {
        *self == ADCH_A::BANDGAP
    }
    #[doc = "Checks if the value of the field is `TEMP_SENSOR`"]
    #[inline(always)]
    pub fn is_temp_sensor(&self) -> bool {
        *self == ADCH_A::TEMP_SENSOR
    }
}
#[doc = "Field `ADCH` writer - Input channel select."]
pub type ADCH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CMDL4_SPEC, u8, ADCH_A, 5, O>;
impl<'a, const O: u8> ADCH_W<'a, O> {
    #[doc = "Select CH0A/ADC0_0 or CH0B/ADC0_8."]
    #[inline(always)]
    pub fn channel_0(self) -> &'a mut W {
        self.variant(ADCH_A::CHANNEL_0)
    }
    #[doc = "Select CH1A/ADC0_1 or CH1B/ADC0_9."]
    #[inline(always)]
    pub fn channel_1(self) -> &'a mut W {
        self.variant(ADCH_A::CHANNEL_1)
    }
    #[doc = "Select CH2A/ADC0_2 or CH2B/ADC0_10."]
    #[inline(always)]
    pub fn channel_2(self) -> &'a mut W {
        self.variant(ADCH_A::CHANNEL_2)
    }
    #[doc = "Select CH3A/ADC0_3 or CH3B/ADC0_11."]
    #[inline(always)]
    pub fn channel_3(self) -> &'a mut W {
        self.variant(ADCH_A::CHANNEL_3)
    }
    #[doc = "Select CH4B/ADC0_12."]
    #[inline(always)]
    pub fn channel_4(self) -> &'a mut W {
        self.variant(ADCH_A::CHANNEL_4)
    }
    #[doc = "Select VDDA."]
    #[inline(always)]
    pub fn vdda(self) -> &'a mut W {
        self.variant(ADCH_A::VDDA)
    }
    #[doc = "Select internal ADC bandgap reference."]
    #[inline(always)]
    pub fn bandgap(self) -> &'a mut W {
        self.variant(ADCH_A::BANDGAP)
    }
    #[doc = "Select temperature sensor."]
    #[inline(always)]
    pub fn temp_sensor(self) -> &'a mut W {
        self.variant(ADCH_A::TEMP_SENSOR)
    }
}
#[doc = "Field `CTYPE` reader - Conversion Type."]
pub type CTYPE_R = crate::FieldReader<u8, CTYPE_A>;
#[doc = "Conversion Type.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CTYPE_A {
    #[doc = "0: Single-Ended Mode. Only A side channel is converted."]
    SINGLE_ENDED_A = 0,
    #[doc = "1: Single-Ended Mode. Only B side channel is converted."]
    SINGLE_ENDED_B = 1,
    #[doc = "2: Differential Mode. A-B."]
    DIFFERENTIAL = 2,
    #[doc = "3: Dual-Single-Ended Mode. Both A side and B side channels are converted independently."]
    DUAL_SINGLE_ENDED = 3,
}
impl From<CTYPE_A> for u8 {
    #[inline(always)]
    fn from(variant: CTYPE_A) -> Self {
        variant as _
    }
}
impl CTYPE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTYPE_A {
        match self.bits {
            0 => CTYPE_A::SINGLE_ENDED_A,
            1 => CTYPE_A::SINGLE_ENDED_B,
            2 => CTYPE_A::DIFFERENTIAL,
            3 => CTYPE_A::DUAL_SINGLE_ENDED,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SINGLE_ENDED_A`"]
    #[inline(always)]
    pub fn is_single_ended_a(&self) -> bool {
        *self == CTYPE_A::SINGLE_ENDED_A
    }
    #[doc = "Checks if the value of the field is `SINGLE_ENDED_B`"]
    #[inline(always)]
    pub fn is_single_ended_b(&self) -> bool {
        *self == CTYPE_A::SINGLE_ENDED_B
    }
    #[doc = "Checks if the value of the field is `DIFFERENTIAL`"]
    #[inline(always)]
    pub fn is_differential(&self) -> bool {
        *self == CTYPE_A::DIFFERENTIAL
    }
    #[doc = "Checks if the value of the field is `DUAL_SINGLE_ENDED`"]
    #[inline(always)]
    pub fn is_dual_single_ended(&self) -> bool {
        *self == CTYPE_A::DUAL_SINGLE_ENDED
    }
}
#[doc = "Field `CTYPE` writer - Conversion Type."]
pub type CTYPE_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CMDL4_SPEC, u8, CTYPE_A, 2, O>;
impl<'a, const O: u8> CTYPE_W<'a, O> {
    #[doc = "Single-Ended Mode. Only A side channel is converted."]
    #[inline(always)]
    pub fn single_ended_a(self) -> &'a mut W {
        self.variant(CTYPE_A::SINGLE_ENDED_A)
    }
    #[doc = "Single-Ended Mode. Only B side channel is converted."]
    #[inline(always)]
    pub fn single_ended_b(self) -> &'a mut W {
        self.variant(CTYPE_A::SINGLE_ENDED_B)
    }
    #[doc = "Differential Mode. A-B."]
    #[inline(always)]
    pub fn differential(self) -> &'a mut W {
        self.variant(CTYPE_A::DIFFERENTIAL)
    }
    #[doc = "Dual-Single-Ended Mode. Both A side and B side channels are converted independently."]
    #[inline(always)]
    pub fn dual_single_ended(self) -> &'a mut W {
        self.variant(CTYPE_A::DUAL_SINGLE_ENDED)
    }
}
#[doc = "Field `MODE` reader - Select resolution of conversions."]
pub type MODE_R = crate::BitReader<MODE_A>;
#[doc = "Select resolution of conversions.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MODE_A {
    #[doc = "0: Standard resolution. Single-ended 12-bit conversion; Differential 13-bit conversion with 2's complement output."]
    STANDARD_RESOLUTION = 0,
    #[doc = "1: High resolution. Single-ended 16-bit conversion; Differential 16-bit conversion with 2's complement output."]
    HIGH_RESOLUTION = 1,
}
impl From<MODE_A> for bool {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        variant as u8 != 0
    }
}
impl MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODE_A {
        match self.bits {
            false => MODE_A::STANDARD_RESOLUTION,
            true => MODE_A::HIGH_RESOLUTION,
        }
    }
    #[doc = "Checks if the value of the field is `STANDARD_RESOLUTION`"]
    #[inline(always)]
    pub fn is_standard_resolution(&self) -> bool {
        *self == MODE_A::STANDARD_RESOLUTION
    }
    #[doc = "Checks if the value of the field is `HIGH_RESOLUTION`"]
    #[inline(always)]
    pub fn is_high_resolution(&self) -> bool {
        *self == MODE_A::HIGH_RESOLUTION
    }
}
#[doc = "Field `MODE` writer - Select resolution of conversions."]
pub type MODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMDL4_SPEC, MODE_A, O>;
impl<'a, const O: u8> MODE_W<'a, O> {
    #[doc = "Standard resolution. Single-ended 12-bit conversion; Differential 13-bit conversion with 2's complement output."]
    #[inline(always)]
    pub fn standard_resolution(self) -> &'a mut W {
        self.variant(MODE_A::STANDARD_RESOLUTION)
    }
    #[doc = "High resolution. Single-ended 16-bit conversion; Differential 16-bit conversion with 2's complement output."]
    #[inline(always)]
    pub fn high_resolution(self) -> &'a mut W {
        self.variant(MODE_A::HIGH_RESOLUTION)
    }
}
impl R {
    #[doc = "Bits 0:4 - Input channel select."]
    #[inline(always)]
    pub fn adch(&self) -> ADCH_R {
        ADCH_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:6 - Conversion Type."]
    #[inline(always)]
    pub fn ctype(&self) -> CTYPE_R {
        CTYPE_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - Select resolution of conversions."]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Input channel select."]
    #[inline(always)]
    pub fn adch(&mut self) -> ADCH_W<0> {
        ADCH_W::new(self)
    }
    #[doc = "Bits 5:6 - Conversion Type."]
    #[inline(always)]
    pub fn ctype(&mut self) -> CTYPE_W<5> {
        CTYPE_W::new(self)
    }
    #[doc = "Bit 7 - Select resolution of conversions."]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W<7> {
        MODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC Command Low Buffer Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmdl4](index.html) module"]
pub struct CMDL4_SPEC;
impl crate::RegisterSpec for CMDL4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cmdl4::R](R) reader structure"]
impl crate::Readable for CMDL4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cmdl4::W](W) writer structure"]
impl crate::Writable for CMDL4_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CMDL4 to value 0"]
impl crate::Resettable for CMDL4_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
