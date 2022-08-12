#[doc = "Register `CLR[%s]` writer"]
pub struct W(crate::W<CLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLR_SPEC>;
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
impl From<crate::W<CLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Sets pin state to low for pin PIOn_0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIO0_AW {
    #[doc = "0: Pin state is unchanged."]
    UNCHANGED = 0,
    #[doc = "1: Pin state is set to low."]
    SET_LOW = 1,
}
impl From<PIO0_AW> for bool {
    #[inline(always)]
    fn from(variant: PIO0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIO0` writer - Sets pin state to low for pin PIOn_0."]
pub type PIO0_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLR_SPEC, PIO0_AW, O>;
impl<'a, const O: u8> PIO0_W<'a, O> {
    #[doc = "Pin state is unchanged."]
    #[inline(always)]
    pub fn unchanged(self) -> &'a mut W {
        self.variant(PIO0_AW::UNCHANGED)
    }
    #[doc = "Pin state is set to low."]
    #[inline(always)]
    pub fn set_low(self) -> &'a mut W {
        self.variant(PIO0_AW::SET_LOW)
    }
}
#[doc = "Sets pin state to low for pin PIOn_1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIO1_AW {
    #[doc = "0: Pin state is unchanged."]
    UNCHANGED = 0,
    #[doc = "1: Pin state is set to low."]
    SET_LOW = 1,
}
impl From<PIO1_AW> for bool {
    #[inline(always)]
    fn from(variant: PIO1_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIO1` writer - Sets pin state to low for pin PIOn_1."]
pub type PIO1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLR_SPEC, PIO1_AW, O>;
impl<'a, const O: u8> PIO1_W<'a, O> {
    #[doc = "Pin state is unchanged."]
    #[inline(always)]
    pub fn unchanged(self) -> &'a mut W {
        self.variant(PIO1_AW::UNCHANGED)
    }
    #[doc = "Pin state is set to low."]
    #[inline(always)]
    pub fn set_low(self) -> &'a mut W {
        self.variant(PIO1_AW::SET_LOW)
    }
}
#[doc = "Sets pin state to low for pin PIOn_2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIO2_AW {
    #[doc = "0: Pin state is unchanged."]
    UNCHANGED = 0,
    #[doc = "1: Pin state is set to low."]
    SET_LOW = 1,
}
impl From<PIO2_AW> for bool {
    #[inline(always)]
    fn from(variant: PIO2_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIO2` writer - Sets pin state to low for pin PIOn_2."]
pub type PIO2_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLR_SPEC, PIO2_AW, O>;
impl<'a, const O: u8> PIO2_W<'a, O> {
    #[doc = "Pin state is unchanged."]
    #[inline(always)]
    pub fn unchanged(self) -> &'a mut W {
        self.variant(PIO2_AW::UNCHANGED)
    }
    #[doc = "Pin state is set to low."]
    #[inline(always)]
    pub fn set_low(self) -> &'a mut W {
        self.variant(PIO2_AW::SET_LOW)
    }
}
#[doc = "Sets pin state to low for pin PIOn_3.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIO3_AW {
    #[doc = "0: Pin state is unchanged."]
    UNCHANGED = 0,
    #[doc = "1: Pin state is set to low."]
    SET_LOW = 1,
}
impl From<PIO3_AW> for bool {
    #[inline(always)]
    fn from(variant: PIO3_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIO3` writer - Sets pin state to low for pin PIOn_3."]
pub type PIO3_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLR_SPEC, PIO3_AW, O>;
impl<'a, const O: u8> PIO3_W<'a, O> {
    #[doc = "Pin state is unchanged."]
    #[inline(always)]
    pub fn unchanged(self) -> &'a mut W {
        self.variant(PIO3_AW::UNCHANGED)
    }
    #[doc = "Pin state is set to low."]
    #[inline(always)]
    pub fn set_low(self) -> &'a mut W {
        self.variant(PIO3_AW::SET_LOW)
    }
}
#[doc = "Sets pin state to low for pin PIOn_4.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIO4_AW {
    #[doc = "0: Pin state is unchanged."]
    UNCHANGED = 0,
    #[doc = "1: Pin state is set to low."]
    SET_LOW = 1,
}
impl From<PIO4_AW> for bool {
    #[inline(always)]
    fn from(variant: PIO4_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIO4` writer - Sets pin state to low for pin PIOn_4."]
pub type PIO4_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLR_SPEC, PIO4_AW, O>;
impl<'a, const O: u8> PIO4_W<'a, O> {
    #[doc = "Pin state is unchanged."]
    #[inline(always)]
    pub fn unchanged(self) -> &'a mut W {
        self.variant(PIO4_AW::UNCHANGED)
    }
    #[doc = "Pin state is set to low."]
    #[inline(always)]
    pub fn set_low(self) -> &'a mut W {
        self.variant(PIO4_AW::SET_LOW)
    }
}
#[doc = "Sets pin state to low for pin PIOn_5.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIO5_AW {
    #[doc = "0: Pin state is unchanged."]
    UNCHANGED = 0,
    #[doc = "1: Pin state is set to low."]
    SET_LOW = 1,
}
impl From<PIO5_AW> for bool {
    #[inline(always)]
    fn from(variant: PIO5_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIO5` writer - Sets pin state to low for pin PIOn_5."]
pub type PIO5_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLR_SPEC, PIO5_AW, O>;
impl<'a, const O: u8> PIO5_W<'a, O> {
    #[doc = "Pin state is unchanged."]
    #[inline(always)]
    pub fn unchanged(self) -> &'a mut W {
        self.variant(PIO5_AW::UNCHANGED)
    }
    #[doc = "Pin state is set to low."]
    #[inline(always)]
    pub fn set_low(self) -> &'a mut W {
        self.variant(PIO5_AW::SET_LOW)
    }
}
#[doc = "Sets pin state to low for pin PIOn_6.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIO6_AW {
    #[doc = "0: Pin state is unchanged."]
    UNCHANGED = 0,
    #[doc = "1: Pin state is set to low."]
    SET_LOW = 1,
}
impl From<PIO6_AW> for bool {
    #[inline(always)]
    fn from(variant: PIO6_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIO6` writer - Sets pin state to low for pin PIOn_6."]
pub type PIO6_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLR_SPEC, PIO6_AW, O>;
impl<'a, const O: u8> PIO6_W<'a, O> {
    #[doc = "Pin state is unchanged."]
    #[inline(always)]
    pub fn unchanged(self) -> &'a mut W {
        self.variant(PIO6_AW::UNCHANGED)
    }
    #[doc = "Pin state is set to low."]
    #[inline(always)]
    pub fn set_low(self) -> &'a mut W {
        self.variant(PIO6_AW::SET_LOW)
    }
}
#[doc = "Sets pin state to low for pin PIOn_7.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIO7_AW {
    #[doc = "0: Pin state is unchanged."]
    UNCHANGED = 0,
    #[doc = "1: Pin state is set to low."]
    SET_LOW = 1,
}
impl From<PIO7_AW> for bool {
    #[inline(always)]
    fn from(variant: PIO7_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIO7` writer - Sets pin state to low for pin PIOn_7."]
pub type PIO7_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLR_SPEC, PIO7_AW, O>;
impl<'a, const O: u8> PIO7_W<'a, O> {
    #[doc = "Pin state is unchanged."]
    #[inline(always)]
    pub fn unchanged(self) -> &'a mut W {
        self.variant(PIO7_AW::UNCHANGED)
    }
    #[doc = "Pin state is set to low."]
    #[inline(always)]
    pub fn set_low(self) -> &'a mut W {
        self.variant(PIO7_AW::SET_LOW)
    }
}
#[doc = "Sets pin state to low for pin PIOn_8.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIO8_AW {
    #[doc = "0: Pin state is unchanged."]
    UNCHANGED = 0,
    #[doc = "1: Pin state is set to low."]
    SET_LOW = 1,
}
impl From<PIO8_AW> for bool {
    #[inline(always)]
    fn from(variant: PIO8_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIO8` writer - Sets pin state to low for pin PIOn_8."]
pub type PIO8_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLR_SPEC, PIO8_AW, O>;
impl<'a, const O: u8> PIO8_W<'a, O> {
    #[doc = "Pin state is unchanged."]
    #[inline(always)]
    pub fn unchanged(self) -> &'a mut W {
        self.variant(PIO8_AW::UNCHANGED)
    }
    #[doc = "Pin state is set to low."]
    #[inline(always)]
    pub fn set_low(self) -> &'a mut W {
        self.variant(PIO8_AW::SET_LOW)
    }
}
#[doc = "Sets pin state to low for pin PIOn_9.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIO9_AW {
    #[doc = "0: Pin state is unchanged."]
    UNCHANGED = 0,
    #[doc = "1: Pin state is set to low."]
    SET_LOW = 1,
}
impl From<PIO9_AW> for bool {
    #[inline(always)]
    fn from(variant: PIO9_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIO9` writer - Sets pin state to low for pin PIOn_9."]
pub type PIO9_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLR_SPEC, PIO9_AW, O>;
impl<'a, const O: u8> PIO9_W<'a, O> {
    #[doc = "Pin state is unchanged."]
    #[inline(always)]
    pub fn unchanged(self) -> &'a mut W {
        self.variant(PIO9_AW::UNCHANGED)
    }
    #[doc = "Pin state is set to low."]
    #[inline(always)]
    pub fn set_low(self) -> &'a mut W {
        self.variant(PIO9_AW::SET_LOW)
    }
}
#[doc = "Sets pin state to low for pin PIOn_10.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIO10_AW {
    #[doc = "0: Pin state is unchanged."]
    UNCHANGED = 0,
    #[doc = "1: Pin state is set to low."]
    SET_LOW = 1,
}
impl From<PIO10_AW> for bool {
    #[inline(always)]
    fn from(variant: PIO10_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIO10` writer - Sets pin state to low for pin PIOn_10."]
pub type PIO10_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLR_SPEC, PIO10_AW, O>;
impl<'a, const O: u8> PIO10_W<'a, O> {
    #[doc = "Pin state is unchanged."]
    #[inline(always)]
    pub fn unchanged(self) -> &'a mut W {
        self.variant(PIO10_AW::UNCHANGED)
    }
    #[doc = "Pin state is set to low."]
    #[inline(always)]
    pub fn set_low(self) -> &'a mut W {
        self.variant(PIO10_AW::SET_LOW)
    }
}
#[doc = "Sets pin state to low for pin PIOn_11.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIO11_AW {
    #[doc = "0: Pin state is unchanged."]
    UNCHANGED = 0,
    #[doc = "1: Pin state is set to low."]
    SET_LOW = 1,
}
impl From<PIO11_AW> for bool {
    #[inline(always)]
    fn from(variant: PIO11_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIO11` writer - Sets pin state to low for pin PIOn_11."]
pub type PIO11_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLR_SPEC, PIO11_AW, O>;
impl<'a, const O: u8> PIO11_W<'a, O> {
    #[doc = "Pin state is unchanged."]
    #[inline(always)]
    pub fn unchanged(self) -> &'a mut W {
        self.variant(PIO11_AW::UNCHANGED)
    }
    #[doc = "Pin state is set to low."]
    #[inline(always)]
    pub fn set_low(self) -> &'a mut W {
        self.variant(PIO11_AW::SET_LOW)
    }
}
#[doc = "Sets pin state to low for pin PIOn_12.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIO12_AW {
    #[doc = "0: Pin state is unchanged."]
    UNCHANGED = 0,
    #[doc = "1: Pin state is set to low."]
    SET_LOW = 1,
}
impl From<PIO12_AW> for bool {
    #[inline(always)]
    fn from(variant: PIO12_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIO12` writer - Sets pin state to low for pin PIOn_12."]
pub type PIO12_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLR_SPEC, PIO12_AW, O>;
impl<'a, const O: u8> PIO12_W<'a, O> {
    #[doc = "Pin state is unchanged."]
    #[inline(always)]
    pub fn unchanged(self) -> &'a mut W {
        self.variant(PIO12_AW::UNCHANGED)
    }
    #[doc = "Pin state is set to low."]
    #[inline(always)]
    pub fn set_low(self) -> &'a mut W {
        self.variant(PIO12_AW::SET_LOW)
    }
}
#[doc = "Sets pin state to low for pin PIOn_13.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIO13_AW {
    #[doc = "0: Pin state is unchanged."]
    UNCHANGED = 0,
    #[doc = "1: Pin state is set to low."]
    SET_LOW = 1,
}
impl From<PIO13_AW> for bool {
    #[inline(always)]
    fn from(variant: PIO13_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIO13` writer - Sets pin state to low for pin PIOn_13."]
pub type PIO13_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLR_SPEC, PIO13_AW, O>;
impl<'a, const O: u8> PIO13_W<'a, O> {
    #[doc = "Pin state is unchanged."]
    #[inline(always)]
    pub fn unchanged(self) -> &'a mut W {
        self.variant(PIO13_AW::UNCHANGED)
    }
    #[doc = "Pin state is set to low."]
    #[inline(always)]
    pub fn set_low(self) -> &'a mut W {
        self.variant(PIO13_AW::SET_LOW)
    }
}
#[doc = "Sets pin state to low for pin PIOn_14.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIO14_AW {
    #[doc = "0: Pin state is unchanged."]
    UNCHANGED = 0,
    #[doc = "1: Pin state is set to low."]
    SET_LOW = 1,
}
impl From<PIO14_AW> for bool {
    #[inline(always)]
    fn from(variant: PIO14_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIO14` writer - Sets pin state to low for pin PIOn_14."]
pub type PIO14_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLR_SPEC, PIO14_AW, O>;
impl<'a, const O: u8> PIO14_W<'a, O> {
    #[doc = "Pin state is unchanged."]
    #[inline(always)]
    pub fn unchanged(self) -> &'a mut W {
        self.variant(PIO14_AW::UNCHANGED)
    }
    #[doc = "Pin state is set to low."]
    #[inline(always)]
    pub fn set_low(self) -> &'a mut W {
        self.variant(PIO14_AW::SET_LOW)
    }
}
#[doc = "Sets pin state to low for pin PIOn_15.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIO15_AW {
    #[doc = "0: Pin state is unchanged."]
    UNCHANGED = 0,
    #[doc = "1: Pin state is set to low."]
    SET_LOW = 1,
}
impl From<PIO15_AW> for bool {
    #[inline(always)]
    fn from(variant: PIO15_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIO15` writer - Sets pin state to low for pin PIOn_15."]
pub type PIO15_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLR_SPEC, PIO15_AW, O>;
impl<'a, const O: u8> PIO15_W<'a, O> {
    #[doc = "Pin state is unchanged."]
    #[inline(always)]
    pub fn unchanged(self) -> &'a mut W {
        self.variant(PIO15_AW::UNCHANGED)
    }
    #[doc = "Pin state is set to low."]
    #[inline(always)]
    pub fn set_low(self) -> &'a mut W {
        self.variant(PIO15_AW::SET_LOW)
    }
}
#[doc = "Sets pin state to low for pin PIOn_16.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIO16_AW {
    #[doc = "0: Pin state is unchanged."]
    UNCHANGED = 0,
    #[doc = "1: Pin state is set to low."]
    SET_LOW = 1,
}
impl From<PIO16_AW> for bool {
    #[inline(always)]
    fn from(variant: PIO16_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIO16` writer - Sets pin state to low for pin PIOn_16."]
pub type PIO16_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLR_SPEC, PIO16_AW, O>;
impl<'a, const O: u8> PIO16_W<'a, O> {
    #[doc = "Pin state is unchanged."]
    #[inline(always)]
    pub fn unchanged(self) -> &'a mut W {
        self.variant(PIO16_AW::UNCHANGED)
    }
    #[doc = "Pin state is set to low."]
    #[inline(always)]
    pub fn set_low(self) -> &'a mut W {
        self.variant(PIO16_AW::SET_LOW)
    }
}
#[doc = "Sets pin state to low for pin PIOn_17.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIO17_AW {
    #[doc = "0: Pin state is unchanged."]
    UNCHANGED = 0,
    #[doc = "1: Pin state is set to low."]
    SET_LOW = 1,
}
impl From<PIO17_AW> for bool {
    #[inline(always)]
    fn from(variant: PIO17_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIO17` writer - Sets pin state to low for pin PIOn_17."]
pub type PIO17_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLR_SPEC, PIO17_AW, O>;
impl<'a, const O: u8> PIO17_W<'a, O> {
    #[doc = "Pin state is unchanged."]
    #[inline(always)]
    pub fn unchanged(self) -> &'a mut W {
        self.variant(PIO17_AW::UNCHANGED)
    }
    #[doc = "Pin state is set to low."]
    #[inline(always)]
    pub fn set_low(self) -> &'a mut W {
        self.variant(PIO17_AW::SET_LOW)
    }
}
#[doc = "Sets pin state to low for pin PIOn_18.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIO18_AW {
    #[doc = "0: Pin state is unchanged."]
    UNCHANGED = 0,
    #[doc = "1: Pin state is set to low."]
    SET_LOW = 1,
}
impl From<PIO18_AW> for bool {
    #[inline(always)]
    fn from(variant: PIO18_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIO18` writer - Sets pin state to low for pin PIOn_18."]
pub type PIO18_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLR_SPEC, PIO18_AW, O>;
impl<'a, const O: u8> PIO18_W<'a, O> {
    #[doc = "Pin state is unchanged."]
    #[inline(always)]
    pub fn unchanged(self) -> &'a mut W {
        self.variant(PIO18_AW::UNCHANGED)
    }
    #[doc = "Pin state is set to low."]
    #[inline(always)]
    pub fn set_low(self) -> &'a mut W {
        self.variant(PIO18_AW::SET_LOW)
    }
}
#[doc = "Sets pin state to low for pin PIOn_19.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIO19_AW {
    #[doc = "0: Pin state is unchanged."]
    UNCHANGED = 0,
    #[doc = "1: Pin state is set to low."]
    SET_LOW = 1,
}
impl From<PIO19_AW> for bool {
    #[inline(always)]
    fn from(variant: PIO19_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIO19` writer - Sets pin state to low for pin PIOn_19."]
pub type PIO19_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLR_SPEC, PIO19_AW, O>;
impl<'a, const O: u8> PIO19_W<'a, O> {
    #[doc = "Pin state is unchanged."]
    #[inline(always)]
    pub fn unchanged(self) -> &'a mut W {
        self.variant(PIO19_AW::UNCHANGED)
    }
    #[doc = "Pin state is set to low."]
    #[inline(always)]
    pub fn set_low(self) -> &'a mut W {
        self.variant(PIO19_AW::SET_LOW)
    }
}
#[doc = "Sets pin state to low for pin PIOn_20.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIO20_AW {
    #[doc = "0: Pin state is unchanged."]
    UNCHANGED = 0,
    #[doc = "1: Pin state is set to low."]
    SET_LOW = 1,
}
impl From<PIO20_AW> for bool {
    #[inline(always)]
    fn from(variant: PIO20_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIO20` writer - Sets pin state to low for pin PIOn_20."]
pub type PIO20_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLR_SPEC, PIO20_AW, O>;
impl<'a, const O: u8> PIO20_W<'a, O> {
    #[doc = "Pin state is unchanged."]
    #[inline(always)]
    pub fn unchanged(self) -> &'a mut W {
        self.variant(PIO20_AW::UNCHANGED)
    }
    #[doc = "Pin state is set to low."]
    #[inline(always)]
    pub fn set_low(self) -> &'a mut W {
        self.variant(PIO20_AW::SET_LOW)
    }
}
#[doc = "Sets pin state to low for pin PIOn_21.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIO21_AW {
    #[doc = "0: Pin state is unchanged."]
    UNCHANGED = 0,
    #[doc = "1: Pin state is set to low."]
    SET_LOW = 1,
}
impl From<PIO21_AW> for bool {
    #[inline(always)]
    fn from(variant: PIO21_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIO21` writer - Sets pin state to low for pin PIOn_21."]
pub type PIO21_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLR_SPEC, PIO21_AW, O>;
impl<'a, const O: u8> PIO21_W<'a, O> {
    #[doc = "Pin state is unchanged."]
    #[inline(always)]
    pub fn unchanged(self) -> &'a mut W {
        self.variant(PIO21_AW::UNCHANGED)
    }
    #[doc = "Pin state is set to low."]
    #[inline(always)]
    pub fn set_low(self) -> &'a mut W {
        self.variant(PIO21_AW::SET_LOW)
    }
}
#[doc = "Sets pin state to low for pin PIOn_22.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIO22_AW {
    #[doc = "0: Pin state is unchanged."]
    UNCHANGED = 0,
    #[doc = "1: Pin state is set to low."]
    SET_LOW = 1,
}
impl From<PIO22_AW> for bool {
    #[inline(always)]
    fn from(variant: PIO22_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIO22` writer - Sets pin state to low for pin PIOn_22."]
pub type PIO22_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLR_SPEC, PIO22_AW, O>;
impl<'a, const O: u8> PIO22_W<'a, O> {
    #[doc = "Pin state is unchanged."]
    #[inline(always)]
    pub fn unchanged(self) -> &'a mut W {
        self.variant(PIO22_AW::UNCHANGED)
    }
    #[doc = "Pin state is set to low."]
    #[inline(always)]
    pub fn set_low(self) -> &'a mut W {
        self.variant(PIO22_AW::SET_LOW)
    }
}
#[doc = "Sets pin state to low for pin PIOn_23.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIO23_AW {
    #[doc = "0: Pin state is unchanged."]
    UNCHANGED = 0,
    #[doc = "1: Pin state is set to low."]
    SET_LOW = 1,
}
impl From<PIO23_AW> for bool {
    #[inline(always)]
    fn from(variant: PIO23_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIO23` writer - Sets pin state to low for pin PIOn_23."]
pub type PIO23_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLR_SPEC, PIO23_AW, O>;
impl<'a, const O: u8> PIO23_W<'a, O> {
    #[doc = "Pin state is unchanged."]
    #[inline(always)]
    pub fn unchanged(self) -> &'a mut W {
        self.variant(PIO23_AW::UNCHANGED)
    }
    #[doc = "Pin state is set to low."]
    #[inline(always)]
    pub fn set_low(self) -> &'a mut W {
        self.variant(PIO23_AW::SET_LOW)
    }
}
#[doc = "Sets pin state to low for pin PIOn_24.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIO24_AW {
    #[doc = "0: Pin state is unchanged."]
    UNCHANGED = 0,
    #[doc = "1: Pin state is set to low."]
    SET_LOW = 1,
}
impl From<PIO24_AW> for bool {
    #[inline(always)]
    fn from(variant: PIO24_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIO24` writer - Sets pin state to low for pin PIOn_24."]
pub type PIO24_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLR_SPEC, PIO24_AW, O>;
impl<'a, const O: u8> PIO24_W<'a, O> {
    #[doc = "Pin state is unchanged."]
    #[inline(always)]
    pub fn unchanged(self) -> &'a mut W {
        self.variant(PIO24_AW::UNCHANGED)
    }
    #[doc = "Pin state is set to low."]
    #[inline(always)]
    pub fn set_low(self) -> &'a mut W {
        self.variant(PIO24_AW::SET_LOW)
    }
}
#[doc = "Sets pin state to low for pin PIOn_25.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIO25_AW {
    #[doc = "0: Pin state is unchanged."]
    UNCHANGED = 0,
    #[doc = "1: Pin state is set to low."]
    SET_LOW = 1,
}
impl From<PIO25_AW> for bool {
    #[inline(always)]
    fn from(variant: PIO25_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIO25` writer - Sets pin state to low for pin PIOn_25."]
pub type PIO25_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLR_SPEC, PIO25_AW, O>;
impl<'a, const O: u8> PIO25_W<'a, O> {
    #[doc = "Pin state is unchanged."]
    #[inline(always)]
    pub fn unchanged(self) -> &'a mut W {
        self.variant(PIO25_AW::UNCHANGED)
    }
    #[doc = "Pin state is set to low."]
    #[inline(always)]
    pub fn set_low(self) -> &'a mut W {
        self.variant(PIO25_AW::SET_LOW)
    }
}
#[doc = "Sets pin state to low for pin PIOn_26.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIO26_AW {
    #[doc = "0: Pin state is unchanged."]
    UNCHANGED = 0,
    #[doc = "1: Pin state is set to low."]
    SET_LOW = 1,
}
impl From<PIO26_AW> for bool {
    #[inline(always)]
    fn from(variant: PIO26_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIO26` writer - Sets pin state to low for pin PIOn_26."]
pub type PIO26_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLR_SPEC, PIO26_AW, O>;
impl<'a, const O: u8> PIO26_W<'a, O> {
    #[doc = "Pin state is unchanged."]
    #[inline(always)]
    pub fn unchanged(self) -> &'a mut W {
        self.variant(PIO26_AW::UNCHANGED)
    }
    #[doc = "Pin state is set to low."]
    #[inline(always)]
    pub fn set_low(self) -> &'a mut W {
        self.variant(PIO26_AW::SET_LOW)
    }
}
#[doc = "Sets pin state to low for pin PIOn_27.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIO27_AW {
    #[doc = "0: Pin state is unchanged."]
    UNCHANGED = 0,
    #[doc = "1: Pin state is set to low."]
    SET_LOW = 1,
}
impl From<PIO27_AW> for bool {
    #[inline(always)]
    fn from(variant: PIO27_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIO27` writer - Sets pin state to low for pin PIOn_27."]
pub type PIO27_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLR_SPEC, PIO27_AW, O>;
impl<'a, const O: u8> PIO27_W<'a, O> {
    #[doc = "Pin state is unchanged."]
    #[inline(always)]
    pub fn unchanged(self) -> &'a mut W {
        self.variant(PIO27_AW::UNCHANGED)
    }
    #[doc = "Pin state is set to low."]
    #[inline(always)]
    pub fn set_low(self) -> &'a mut W {
        self.variant(PIO27_AW::SET_LOW)
    }
}
#[doc = "Sets pin state to low for pin PIOn_28.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIO28_AW {
    #[doc = "0: Pin state is unchanged."]
    UNCHANGED = 0,
    #[doc = "1: Pin state is set to low."]
    SET_LOW = 1,
}
impl From<PIO28_AW> for bool {
    #[inline(always)]
    fn from(variant: PIO28_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIO28` writer - Sets pin state to low for pin PIOn_28."]
pub type PIO28_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLR_SPEC, PIO28_AW, O>;
impl<'a, const O: u8> PIO28_W<'a, O> {
    #[doc = "Pin state is unchanged."]
    #[inline(always)]
    pub fn unchanged(self) -> &'a mut W {
        self.variant(PIO28_AW::UNCHANGED)
    }
    #[doc = "Pin state is set to low."]
    #[inline(always)]
    pub fn set_low(self) -> &'a mut W {
        self.variant(PIO28_AW::SET_LOW)
    }
}
#[doc = "Sets pin state to low for pin PIOn_29.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIO29_AW {
    #[doc = "0: Pin state is unchanged."]
    UNCHANGED = 0,
    #[doc = "1: Pin state is set to low."]
    SET_LOW = 1,
}
impl From<PIO29_AW> for bool {
    #[inline(always)]
    fn from(variant: PIO29_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIO29` writer - Sets pin state to low for pin PIOn_29."]
pub type PIO29_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLR_SPEC, PIO29_AW, O>;
impl<'a, const O: u8> PIO29_W<'a, O> {
    #[doc = "Pin state is unchanged."]
    #[inline(always)]
    pub fn unchanged(self) -> &'a mut W {
        self.variant(PIO29_AW::UNCHANGED)
    }
    #[doc = "Pin state is set to low."]
    #[inline(always)]
    pub fn set_low(self) -> &'a mut W {
        self.variant(PIO29_AW::SET_LOW)
    }
}
#[doc = "Sets pin state to low for pin PIOn_30.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIO30_AW {
    #[doc = "0: Pin state is unchanged."]
    UNCHANGED = 0,
    #[doc = "1: Pin state is set to low."]
    SET_LOW = 1,
}
impl From<PIO30_AW> for bool {
    #[inline(always)]
    fn from(variant: PIO30_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIO30` writer - Sets pin state to low for pin PIOn_30."]
pub type PIO30_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLR_SPEC, PIO30_AW, O>;
impl<'a, const O: u8> PIO30_W<'a, O> {
    #[doc = "Pin state is unchanged."]
    #[inline(always)]
    pub fn unchanged(self) -> &'a mut W {
        self.variant(PIO30_AW::UNCHANGED)
    }
    #[doc = "Pin state is set to low."]
    #[inline(always)]
    pub fn set_low(self) -> &'a mut W {
        self.variant(PIO30_AW::SET_LOW)
    }
}
#[doc = "Sets pin state to low for pin PIOn_31.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIO31_AW {
    #[doc = "0: Pin state is unchanged."]
    UNCHANGED = 0,
    #[doc = "1: Pin state is set to low."]
    SET_LOW = 1,
}
impl From<PIO31_AW> for bool {
    #[inline(always)]
    fn from(variant: PIO31_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIO31` writer - Sets pin state to low for pin PIOn_31."]
pub type PIO31_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLR_SPEC, PIO31_AW, O>;
impl<'a, const O: u8> PIO31_W<'a, O> {
    #[doc = "Pin state is unchanged."]
    #[inline(always)]
    pub fn unchanged(self) -> &'a mut W {
        self.variant(PIO31_AW::UNCHANGED)
    }
    #[doc = "Pin state is set to low."]
    #[inline(always)]
    pub fn set_low(self) -> &'a mut W {
        self.variant(PIO31_AW::SET_LOW)
    }
}
impl W {
    #[doc = "Bit 0 - Sets pin state to low for pin PIOn_0."]
    #[inline(always)]
    pub fn pio0(&mut self) -> PIO0_W<0> {
        PIO0_W::new(self)
    }
    #[doc = "Bit 1 - Sets pin state to low for pin PIOn_1."]
    #[inline(always)]
    pub fn pio1(&mut self) -> PIO1_W<1> {
        PIO1_W::new(self)
    }
    #[doc = "Bit 2 - Sets pin state to low for pin PIOn_2."]
    #[inline(always)]
    pub fn pio2(&mut self) -> PIO2_W<2> {
        PIO2_W::new(self)
    }
    #[doc = "Bit 3 - Sets pin state to low for pin PIOn_3."]
    #[inline(always)]
    pub fn pio3(&mut self) -> PIO3_W<3> {
        PIO3_W::new(self)
    }
    #[doc = "Bit 4 - Sets pin state to low for pin PIOn_4."]
    #[inline(always)]
    pub fn pio4(&mut self) -> PIO4_W<4> {
        PIO4_W::new(self)
    }
    #[doc = "Bit 5 - Sets pin state to low for pin PIOn_5."]
    #[inline(always)]
    pub fn pio5(&mut self) -> PIO5_W<5> {
        PIO5_W::new(self)
    }
    #[doc = "Bit 6 - Sets pin state to low for pin PIOn_6."]
    #[inline(always)]
    pub fn pio6(&mut self) -> PIO6_W<6> {
        PIO6_W::new(self)
    }
    #[doc = "Bit 7 - Sets pin state to low for pin PIOn_7."]
    #[inline(always)]
    pub fn pio7(&mut self) -> PIO7_W<7> {
        PIO7_W::new(self)
    }
    #[doc = "Bit 8 - Sets pin state to low for pin PIOn_8."]
    #[inline(always)]
    pub fn pio8(&mut self) -> PIO8_W<8> {
        PIO8_W::new(self)
    }
    #[doc = "Bit 9 - Sets pin state to low for pin PIOn_9."]
    #[inline(always)]
    pub fn pio9(&mut self) -> PIO9_W<9> {
        PIO9_W::new(self)
    }
    #[doc = "Bit 10 - Sets pin state to low for pin PIOn_10."]
    #[inline(always)]
    pub fn pio10(&mut self) -> PIO10_W<10> {
        PIO10_W::new(self)
    }
    #[doc = "Bit 11 - Sets pin state to low for pin PIOn_11."]
    #[inline(always)]
    pub fn pio11(&mut self) -> PIO11_W<11> {
        PIO11_W::new(self)
    }
    #[doc = "Bit 12 - Sets pin state to low for pin PIOn_12."]
    #[inline(always)]
    pub fn pio12(&mut self) -> PIO12_W<12> {
        PIO12_W::new(self)
    }
    #[doc = "Bit 13 - Sets pin state to low for pin PIOn_13."]
    #[inline(always)]
    pub fn pio13(&mut self) -> PIO13_W<13> {
        PIO13_W::new(self)
    }
    #[doc = "Bit 14 - Sets pin state to low for pin PIOn_14."]
    #[inline(always)]
    pub fn pio14(&mut self) -> PIO14_W<14> {
        PIO14_W::new(self)
    }
    #[doc = "Bit 15 - Sets pin state to low for pin PIOn_15."]
    #[inline(always)]
    pub fn pio15(&mut self) -> PIO15_W<15> {
        PIO15_W::new(self)
    }
    #[doc = "Bit 16 - Sets pin state to low for pin PIOn_16."]
    #[inline(always)]
    pub fn pio16(&mut self) -> PIO16_W<16> {
        PIO16_W::new(self)
    }
    #[doc = "Bit 17 - Sets pin state to low for pin PIOn_17."]
    #[inline(always)]
    pub fn pio17(&mut self) -> PIO17_W<17> {
        PIO17_W::new(self)
    }
    #[doc = "Bit 18 - Sets pin state to low for pin PIOn_18."]
    #[inline(always)]
    pub fn pio18(&mut self) -> PIO18_W<18> {
        PIO18_W::new(self)
    }
    #[doc = "Bit 19 - Sets pin state to low for pin PIOn_19."]
    #[inline(always)]
    pub fn pio19(&mut self) -> PIO19_W<19> {
        PIO19_W::new(self)
    }
    #[doc = "Bit 20 - Sets pin state to low for pin PIOn_20."]
    #[inline(always)]
    pub fn pio20(&mut self) -> PIO20_W<20> {
        PIO20_W::new(self)
    }
    #[doc = "Bit 21 - Sets pin state to low for pin PIOn_21."]
    #[inline(always)]
    pub fn pio21(&mut self) -> PIO21_W<21> {
        PIO21_W::new(self)
    }
    #[doc = "Bit 22 - Sets pin state to low for pin PIOn_22."]
    #[inline(always)]
    pub fn pio22(&mut self) -> PIO22_W<22> {
        PIO22_W::new(self)
    }
    #[doc = "Bit 23 - Sets pin state to low for pin PIOn_23."]
    #[inline(always)]
    pub fn pio23(&mut self) -> PIO23_W<23> {
        PIO23_W::new(self)
    }
    #[doc = "Bit 24 - Sets pin state to low for pin PIOn_24."]
    #[inline(always)]
    pub fn pio24(&mut self) -> PIO24_W<24> {
        PIO24_W::new(self)
    }
    #[doc = "Bit 25 - Sets pin state to low for pin PIOn_25."]
    #[inline(always)]
    pub fn pio25(&mut self) -> PIO25_W<25> {
        PIO25_W::new(self)
    }
    #[doc = "Bit 26 - Sets pin state to low for pin PIOn_26."]
    #[inline(always)]
    pub fn pio26(&mut self) -> PIO26_W<26> {
        PIO26_W::new(self)
    }
    #[doc = "Bit 27 - Sets pin state to low for pin PIOn_27."]
    #[inline(always)]
    pub fn pio27(&mut self) -> PIO27_W<27> {
        PIO27_W::new(self)
    }
    #[doc = "Bit 28 - Sets pin state to low for pin PIOn_28."]
    #[inline(always)]
    pub fn pio28(&mut self) -> PIO28_W<28> {
        PIO28_W::new(self)
    }
    #[doc = "Bit 29 - Sets pin state to low for pin PIOn_29."]
    #[inline(always)]
    pub fn pio29(&mut self) -> PIO29_W<29> {
        PIO29_W::new(self)
    }
    #[doc = "Bit 30 - Sets pin state to low for pin PIOn_30."]
    #[inline(always)]
    pub fn pio30(&mut self) -> PIO30_W<30> {
        PIO30_W::new(self)
    }
    #[doc = "Bit 31 - Sets pin state to low for pin PIOn_31."]
    #[inline(always)]
    pub fn pio31(&mut self) -> PIO31_W<31> {
        PIO31_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clear register for port.\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clr](index.html) module"]
pub struct CLR_SPEC;
impl crate::RegisterSpec for CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [clr::W](W) writer structure"]
impl crate::Writable for CLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLR[%s]
to value 0"]
impl crate::Resettable for CLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
