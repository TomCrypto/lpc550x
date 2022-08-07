#[doc = "Register `CFG` reader"]
pub struct R(crate::R<CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFG` writer"]
pub struct W(crate::W<CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFG_SPEC>;
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
impl From<crate::W<CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TPRICTRL` reader - ADC trigger priority control"]
pub type TPRICTRL_R = crate::FieldReader<u8, TPRICTRL_A>;
#[doc = "ADC trigger priority control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TPRICTRL_A {
    #[doc = "0: If a higher priority trigger is detected during command processing, the current conversion is aborted and the new command specified by the trigger is started."]
    TPRICTRL_0 = 0,
    #[doc = "1: If a higher priority trigger is received during command processing, the current command is stopped after after completing the current conversion. If averaging is enabled, the averaging loop will be completed. However, CMDHa\\[LOOP\\]
will be ignored and the higher priority trigger will be serviced."]
    TPRICTRL_1 = 1,
    #[doc = "2: If a higher priority trigger is received during command processing, the current command will be completed (averaging, looping, compare) before servicing the higher priority trigger."]
    TPRICTRL_2 = 2,
}
impl From<TPRICTRL_A> for u8 {
    #[inline(always)]
    fn from(variant: TPRICTRL_A) -> Self {
        variant as _
    }
}
impl TPRICTRL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TPRICTRL_A> {
        match self.bits {
            0 => Some(TPRICTRL_A::TPRICTRL_0),
            1 => Some(TPRICTRL_A::TPRICTRL_1),
            2 => Some(TPRICTRL_A::TPRICTRL_2),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `TPRICTRL_0`"]
    #[inline(always)]
    pub fn is_tprictrl_0(&self) -> bool {
        *self == TPRICTRL_A::TPRICTRL_0
    }
    #[doc = "Checks if the value of the field is `TPRICTRL_1`"]
    #[inline(always)]
    pub fn is_tprictrl_1(&self) -> bool {
        *self == TPRICTRL_A::TPRICTRL_1
    }
    #[doc = "Checks if the value of the field is `TPRICTRL_2`"]
    #[inline(always)]
    pub fn is_tprictrl_2(&self) -> bool {
        *self == TPRICTRL_A::TPRICTRL_2
    }
}
#[doc = "Field `TPRICTRL` writer - ADC trigger priority control"]
pub type TPRICTRL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFG_SPEC, u8, TPRICTRL_A, 2, O>;
impl<'a, const O: u8> TPRICTRL_W<'a, O> {
    #[doc = "If a higher priority trigger is detected during command processing, the current conversion is aborted and the new command specified by the trigger is started."]
    #[inline(always)]
    pub fn tprictrl_0(self) -> &'a mut W {
        self.variant(TPRICTRL_A::TPRICTRL_0)
    }
    #[doc = "If a higher priority trigger is received during command processing, the current command is stopped after after completing the current conversion. If averaging is enabled, the averaging loop will be completed. However, CMDHa\\[LOOP\\]
will be ignored and the higher priority trigger will be serviced."]
    #[inline(always)]
    pub fn tprictrl_1(self) -> &'a mut W {
        self.variant(TPRICTRL_A::TPRICTRL_1)
    }
    #[doc = "If a higher priority trigger is received during command processing, the current command will be completed (averaging, looping, compare) before servicing the higher priority trigger."]
    #[inline(always)]
    pub fn tprictrl_2(self) -> &'a mut W {
        self.variant(TPRICTRL_A::TPRICTRL_2)
    }
}
#[doc = "Field `PWRSEL` reader - Power Configuration Select"]
pub type PWRSEL_R = crate::FieldReader<u8, PWRSEL_A>;
#[doc = "Power Configuration Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PWRSEL_A {
    #[doc = "0: Lowest power setting."]
    PWRSEL_0 = 0,
    #[doc = "1: Higher power setting than 0b0."]
    PWRSEL_1 = 1,
    #[doc = "2: Higher power setting than 0b1."]
    PWRSEL_2 = 2,
    #[doc = "3: Highest power setting."]
    PWRSEL_3 = 3,
}
impl From<PWRSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PWRSEL_A) -> Self {
        variant as _
    }
}
impl PWRSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWRSEL_A {
        match self.bits {
            0 => PWRSEL_A::PWRSEL_0,
            1 => PWRSEL_A::PWRSEL_1,
            2 => PWRSEL_A::PWRSEL_2,
            3 => PWRSEL_A::PWRSEL_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PWRSEL_0`"]
    #[inline(always)]
    pub fn is_pwrsel_0(&self) -> bool {
        *self == PWRSEL_A::PWRSEL_0
    }
    #[doc = "Checks if the value of the field is `PWRSEL_1`"]
    #[inline(always)]
    pub fn is_pwrsel_1(&self) -> bool {
        *self == PWRSEL_A::PWRSEL_1
    }
    #[doc = "Checks if the value of the field is `PWRSEL_2`"]
    #[inline(always)]
    pub fn is_pwrsel_2(&self) -> bool {
        *self == PWRSEL_A::PWRSEL_2
    }
    #[doc = "Checks if the value of the field is `PWRSEL_3`"]
    #[inline(always)]
    pub fn is_pwrsel_3(&self) -> bool {
        *self == PWRSEL_A::PWRSEL_3
    }
}
#[doc = "Field `PWRSEL` writer - Power Configuration Select"]
pub type PWRSEL_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CFG_SPEC, u8, PWRSEL_A, 2, O>;
impl<'a, const O: u8> PWRSEL_W<'a, O> {
    #[doc = "Lowest power setting."]
    #[inline(always)]
    pub fn pwrsel_0(self) -> &'a mut W {
        self.variant(PWRSEL_A::PWRSEL_0)
    }
    #[doc = "Higher power setting than 0b0."]
    #[inline(always)]
    pub fn pwrsel_1(self) -> &'a mut W {
        self.variant(PWRSEL_A::PWRSEL_1)
    }
    #[doc = "Higher power setting than 0b1."]
    #[inline(always)]
    pub fn pwrsel_2(self) -> &'a mut W {
        self.variant(PWRSEL_A::PWRSEL_2)
    }
    #[doc = "Highest power setting."]
    #[inline(always)]
    pub fn pwrsel_3(self) -> &'a mut W {
        self.variant(PWRSEL_A::PWRSEL_3)
    }
}
#[doc = "Field `REFSEL` reader - Voltage Reference Selection"]
pub type REFSEL_R = crate::FieldReader<u8, REFSEL_A>;
#[doc = "Voltage Reference Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum REFSEL_A {
    #[doc = "0: (Default) Option 1 setting."]
    REFSEL_0 = 0,
    #[doc = "1: Option 2 setting."]
    REFSEL_1 = 1,
    #[doc = "2: Option 3 setting."]
    REFSEL_2 = 2,
}
impl From<REFSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: REFSEL_A) -> Self {
        variant as _
    }
}
impl REFSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<REFSEL_A> {
        match self.bits {
            0 => Some(REFSEL_A::REFSEL_0),
            1 => Some(REFSEL_A::REFSEL_1),
            2 => Some(REFSEL_A::REFSEL_2),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `REFSEL_0`"]
    #[inline(always)]
    pub fn is_refsel_0(&self) -> bool {
        *self == REFSEL_A::REFSEL_0
    }
    #[doc = "Checks if the value of the field is `REFSEL_1`"]
    #[inline(always)]
    pub fn is_refsel_1(&self) -> bool {
        *self == REFSEL_A::REFSEL_1
    }
    #[doc = "Checks if the value of the field is `REFSEL_2`"]
    #[inline(always)]
    pub fn is_refsel_2(&self) -> bool {
        *self == REFSEL_A::REFSEL_2
    }
}
#[doc = "Field `REFSEL` writer - Voltage Reference Selection"]
pub type REFSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFG_SPEC, u8, REFSEL_A, 2, O>;
impl<'a, const O: u8> REFSEL_W<'a, O> {
    #[doc = "(Default) Option 1 setting."]
    #[inline(always)]
    pub fn refsel_0(self) -> &'a mut W {
        self.variant(REFSEL_A::REFSEL_0)
    }
    #[doc = "Option 2 setting."]
    #[inline(always)]
    pub fn refsel_1(self) -> &'a mut W {
        self.variant(REFSEL_A::REFSEL_1)
    }
    #[doc = "Option 3 setting."]
    #[inline(always)]
    pub fn refsel_2(self) -> &'a mut W {
        self.variant(REFSEL_A::REFSEL_2)
    }
}
#[doc = "Field `TRES` reader - Trigger Resume Enable"]
pub type TRES_R = crate::BitReader<TRES_A>;
#[doc = "Trigger Resume Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRES_A {
    #[doc = "0: Trigger sequences interrupted by a high priority trigger exception will not be automatically resumed or restarted."]
    TRES_0 = 0,
    #[doc = "1: Trigger sequences interrupted by a high priority trigger exception will be automatically resumed or restarted."]
    TRES_1 = 1,
}
impl From<TRES_A> for bool {
    #[inline(always)]
    fn from(variant: TRES_A) -> Self {
        variant as u8 != 0
    }
}
impl TRES_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRES_A {
        match self.bits {
            false => TRES_A::TRES_0,
            true => TRES_A::TRES_1,
        }
    }
    #[doc = "Checks if the value of the field is `TRES_0`"]
    #[inline(always)]
    pub fn is_tres_0(&self) -> bool {
        *self == TRES_A::TRES_0
    }
    #[doc = "Checks if the value of the field is `TRES_1`"]
    #[inline(always)]
    pub fn is_tres_1(&self) -> bool {
        *self == TRES_A::TRES_1
    }
}
#[doc = "Field `TRES` writer - Trigger Resume Enable"]
pub type TRES_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, TRES_A, O>;
impl<'a, const O: u8> TRES_W<'a, O> {
    #[doc = "Trigger sequences interrupted by a high priority trigger exception will not be automatically resumed or restarted."]
    #[inline(always)]
    pub fn tres_0(self) -> &'a mut W {
        self.variant(TRES_A::TRES_0)
    }
    #[doc = "Trigger sequences interrupted by a high priority trigger exception will be automatically resumed or restarted."]
    #[inline(always)]
    pub fn tres_1(self) -> &'a mut W {
        self.variant(TRES_A::TRES_1)
    }
}
#[doc = "Field `TCMDRES` reader - Trigger Command Resume"]
pub type TCMDRES_R = crate::BitReader<TCMDRES_A>;
#[doc = "Trigger Command Resume\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TCMDRES_A {
    #[doc = "0: Trigger sequences interrupted by a high priority trigger exception will be automatically restarted."]
    TCMDRES_0 = 0,
    #[doc = "1: Trigger sequences interrupted by a high priority trigger exception will be resumed from the command executing before the exception."]
    TCMDRES_1 = 1,
}
impl From<TCMDRES_A> for bool {
    #[inline(always)]
    fn from(variant: TCMDRES_A) -> Self {
        variant as u8 != 0
    }
}
impl TCMDRES_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TCMDRES_A {
        match self.bits {
            false => TCMDRES_A::TCMDRES_0,
            true => TCMDRES_A::TCMDRES_1,
        }
    }
    #[doc = "Checks if the value of the field is `TCMDRES_0`"]
    #[inline(always)]
    pub fn is_tcmdres_0(&self) -> bool {
        *self == TCMDRES_A::TCMDRES_0
    }
    #[doc = "Checks if the value of the field is `TCMDRES_1`"]
    #[inline(always)]
    pub fn is_tcmdres_1(&self) -> bool {
        *self == TCMDRES_A::TCMDRES_1
    }
}
#[doc = "Field `TCMDRES` writer - Trigger Command Resume"]
pub type TCMDRES_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, TCMDRES_A, O>;
impl<'a, const O: u8> TCMDRES_W<'a, O> {
    #[doc = "Trigger sequences interrupted by a high priority trigger exception will be automatically restarted."]
    #[inline(always)]
    pub fn tcmdres_0(self) -> &'a mut W {
        self.variant(TCMDRES_A::TCMDRES_0)
    }
    #[doc = "Trigger sequences interrupted by a high priority trigger exception will be resumed from the command executing before the exception."]
    #[inline(always)]
    pub fn tcmdres_1(self) -> &'a mut W {
        self.variant(TCMDRES_A::TCMDRES_1)
    }
}
#[doc = "Field `HPT_EXDI` reader - High Priority Trigger Exception Disable"]
pub type HPT_EXDI_R = crate::BitReader<HPT_EXDI_A>;
#[doc = "High Priority Trigger Exception Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HPT_EXDI_A {
    #[doc = "0: High priority trigger exceptions are enabled."]
    HPT_EXDI_0 = 0,
    #[doc = "1: High priority trigger exceptions are disabled."]
    HPT_EXDI_1 = 1,
}
impl From<HPT_EXDI_A> for bool {
    #[inline(always)]
    fn from(variant: HPT_EXDI_A) -> Self {
        variant as u8 != 0
    }
}
impl HPT_EXDI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HPT_EXDI_A {
        match self.bits {
            false => HPT_EXDI_A::HPT_EXDI_0,
            true => HPT_EXDI_A::HPT_EXDI_1,
        }
    }
    #[doc = "Checks if the value of the field is `HPT_EXDI_0`"]
    #[inline(always)]
    pub fn is_hpt_exdi_0(&self) -> bool {
        *self == HPT_EXDI_A::HPT_EXDI_0
    }
    #[doc = "Checks if the value of the field is `HPT_EXDI_1`"]
    #[inline(always)]
    pub fn is_hpt_exdi_1(&self) -> bool {
        *self == HPT_EXDI_A::HPT_EXDI_1
    }
}
#[doc = "Field `HPT_EXDI` writer - High Priority Trigger Exception Disable"]
pub type HPT_EXDI_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, HPT_EXDI_A, O>;
impl<'a, const O: u8> HPT_EXDI_W<'a, O> {
    #[doc = "High priority trigger exceptions are enabled."]
    #[inline(always)]
    pub fn hpt_exdi_0(self) -> &'a mut W {
        self.variant(HPT_EXDI_A::HPT_EXDI_0)
    }
    #[doc = "High priority trigger exceptions are disabled."]
    #[inline(always)]
    pub fn hpt_exdi_1(self) -> &'a mut W {
        self.variant(HPT_EXDI_A::HPT_EXDI_1)
    }
}
#[doc = "Field `PUDLY` reader - Power Up Delay"]
pub type PUDLY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PUDLY` writer - Power Up Delay"]
pub type PUDLY_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFG_SPEC, u8, u8, 8, O>;
#[doc = "Field `PWREN` reader - ADC Analog Pre-Enable"]
pub type PWREN_R = crate::BitReader<PWREN_A>;
#[doc = "ADC Analog Pre-Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWREN_A {
    #[doc = "0: ADC analog circuits are only enabled while conversions are active. Performance is affected due to analog startup delays."]
    PWREN_0 = 0,
    #[doc = "1: ADC analog circuits are pre-enabled and ready to execute conversions without startup delays (at the cost of higher DC current consumption). A single power up delay (CFG\\[PUDLY\\]) is executed immediately once PWREN is set, and any detected trigger does not begin ADC operation until the power up delay time has passed. After this initial delay expires the analog will remain pre-enabled, and no additional delays will be executed."]
    PWREN_1 = 1,
}
impl From<PWREN_A> for bool {
    #[inline(always)]
    fn from(variant: PWREN_A) -> Self {
        variant as u8 != 0
    }
}
impl PWREN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWREN_A {
        match self.bits {
            false => PWREN_A::PWREN_0,
            true => PWREN_A::PWREN_1,
        }
    }
    #[doc = "Checks if the value of the field is `PWREN_0`"]
    #[inline(always)]
    pub fn is_pwren_0(&self) -> bool {
        *self == PWREN_A::PWREN_0
    }
    #[doc = "Checks if the value of the field is `PWREN_1`"]
    #[inline(always)]
    pub fn is_pwren_1(&self) -> bool {
        *self == PWREN_A::PWREN_1
    }
}
#[doc = "Field `PWREN` writer - ADC Analog Pre-Enable"]
pub type PWREN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, PWREN_A, O>;
impl<'a, const O: u8> PWREN_W<'a, O> {
    #[doc = "ADC analog circuits are only enabled while conversions are active. Performance is affected due to analog startup delays."]
    #[inline(always)]
    pub fn pwren_0(self) -> &'a mut W {
        self.variant(PWREN_A::PWREN_0)
    }
    #[doc = "ADC analog circuits are pre-enabled and ready to execute conversions without startup delays (at the cost of higher DC current consumption). A single power up delay (CFG\\[PUDLY\\]) is executed immediately once PWREN is set, and any detected trigger does not begin ADC operation until the power up delay time has passed. After this initial delay expires the analog will remain pre-enabled, and no additional delays will be executed."]
    #[inline(always)]
    pub fn pwren_1(self) -> &'a mut W {
        self.variant(PWREN_A::PWREN_1)
    }
}
impl R {
    #[doc = "Bits 0:1 - ADC trigger priority control"]
    #[inline(always)]
    pub fn tprictrl(&self) -> TPRICTRL_R {
        TPRICTRL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:5 - Power Configuration Select"]
    #[inline(always)]
    pub fn pwrsel(&self) -> PWRSEL_R {
        PWRSEL_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Voltage Reference Selection"]
    #[inline(always)]
    pub fn refsel(&self) -> REFSEL_R {
        REFSEL_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - Trigger Resume Enable"]
    #[inline(always)]
    pub fn tres(&self) -> TRES_R {
        TRES_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Trigger Command Resume"]
    #[inline(always)]
    pub fn tcmdres(&self) -> TCMDRES_R {
        TCMDRES_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - High Priority Trigger Exception Disable"]
    #[inline(always)]
    pub fn hpt_exdi(&self) -> HPT_EXDI_R {
        HPT_EXDI_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 16:23 - Power Up Delay"]
    #[inline(always)]
    pub fn pudly(&self) -> PUDLY_R {
        PUDLY_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 28 - ADC Analog Pre-Enable"]
    #[inline(always)]
    pub fn pwren(&self) -> PWREN_R {
        PWREN_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - ADC trigger priority control"]
    #[inline(always)]
    pub fn tprictrl(&mut self) -> TPRICTRL_W<0> {
        TPRICTRL_W::new(self)
    }
    #[doc = "Bits 4:5 - Power Configuration Select"]
    #[inline(always)]
    pub fn pwrsel(&mut self) -> PWRSEL_W<4> {
        PWRSEL_W::new(self)
    }
    #[doc = "Bits 6:7 - Voltage Reference Selection"]
    #[inline(always)]
    pub fn refsel(&mut self) -> REFSEL_W<6> {
        REFSEL_W::new(self)
    }
    #[doc = "Bit 8 - Trigger Resume Enable"]
    #[inline(always)]
    pub fn tres(&mut self) -> TRES_W<8> {
        TRES_W::new(self)
    }
    #[doc = "Bit 9 - Trigger Command Resume"]
    #[inline(always)]
    pub fn tcmdres(&mut self) -> TCMDRES_W<9> {
        TCMDRES_W::new(self)
    }
    #[doc = "Bit 10 - High Priority Trigger Exception Disable"]
    #[inline(always)]
    pub fn hpt_exdi(&mut self) -> HPT_EXDI_W<10> {
        HPT_EXDI_W::new(self)
    }
    #[doc = "Bits 16:23 - Power Up Delay"]
    #[inline(always)]
    pub fn pudly(&mut self) -> PUDLY_W<16> {
        PUDLY_W::new(self)
    }
    #[doc = "Bit 28 - ADC Analog Pre-Enable"]
    #[inline(always)]
    pub fn pwren(&mut self) -> PWREN_W<28> {
        PWREN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFG_SPEC;
impl crate::RegisterSpec for CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfg::R](R) reader structure"]
impl crate::Readable for CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfg::W](W) writer structure"]
impl crate::Writable for CFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CFG to value 0x0080_0000"]
impl crate::Resettable for CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0080_0000
    }
}
