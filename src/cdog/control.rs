#[doc = "Register `CONTROL` reader"]
pub struct R(crate::R<CONTROL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONTROL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONTROL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONTROL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CONTROL` writer"]
pub struct W(crate::W<CONTROL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONTROL_SPEC>;
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
impl From<crate::W<CONTROL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONTROL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LOCK_CTRL` reader - Lock control field."]
pub type LOCK_CTRL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LOCK_CTRL` writer - Lock control field."]
pub type LOCK_CTRL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CONTROL_SPEC, u8, u8, 2, O>;
#[doc = "Field `TIMEOUT_CTRL` reader - TIMEOUT control."]
pub type TIMEOUT_CTRL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TIMEOUT_CTRL` writer - TIMEOUT control."]
pub type TIMEOUT_CTRL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CONTROL_SPEC, u8, u8, 3, O>;
#[doc = "Field `MISCOMPARE_CTRL` reader - MISCOMPARE control field."]
pub type MISCOMPARE_CTRL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MISCOMPARE_CTRL` writer - MISCOMPARE control field."]
pub type MISCOMPARE_CTRL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CONTROL_SPEC, u8, u8, 3, O>;
#[doc = "Field `SEQUENCE_CTRL` reader - SEQUENCE control field."]
pub type SEQUENCE_CTRL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEQUENCE_CTRL` writer - SEQUENCE control field."]
pub type SEQUENCE_CTRL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CONTROL_SPEC, u8, u8, 3, O>;
#[doc = "Field `CONTROL_CTRL` reader - CONTROL control field."]
pub type CONTROL_CTRL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CONTROL_CTRL` writer - CONTROL control field."]
pub type CONTROL_CTRL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CONTROL_SPEC, u8, u8, 3, O>;
#[doc = "Field `STATE_CTRL` reader - STATE control field."]
pub type STATE_CTRL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `STATE_CTRL` writer - STATE control field."]
pub type STATE_CTRL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CONTROL_SPEC, u8, u8, 3, O>;
#[doc = "Field `ADDRESS_CTRL` reader - ADDRESS control field."]
pub type ADDRESS_CTRL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADDRESS_CTRL` writer - ADDRESS control field."]
pub type ADDRESS_CTRL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CONTROL_SPEC, u8, u8, 3, O>;
#[doc = "Field `IRQ_PAUSE` reader - IRQ pause control field."]
pub type IRQ_PAUSE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IRQ_PAUSE` writer - IRQ pause control field."]
pub type IRQ_PAUSE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CONTROL_SPEC, u8, u8, 2, O>;
#[doc = "Field `DEBUG_HALT_CTRL` reader - DEBUG_HALT control field."]
pub type DEBUG_HALT_CTRL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DEBUG_HALT_CTRL` writer - DEBUG_HALT control field."]
pub type DEBUG_HALT_CTRL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CONTROL_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:1 - Lock control field."]
    #[inline(always)]
    pub fn lock_ctrl(&self) -> LOCK_CTRL_R {
        LOCK_CTRL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:4 - TIMEOUT control."]
    #[inline(always)]
    pub fn timeout_ctrl(&self) -> TIMEOUT_CTRL_R {
        TIMEOUT_CTRL_R::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bits 5:7 - MISCOMPARE control field."]
    #[inline(always)]
    pub fn miscompare_ctrl(&self) -> MISCOMPARE_CTRL_R {
        MISCOMPARE_CTRL_R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bits 8:10 - SEQUENCE control field."]
    #[inline(always)]
    pub fn sequence_ctrl(&self) -> SEQUENCE_CTRL_R {
        SEQUENCE_CTRL_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:13 - CONTROL control field."]
    #[inline(always)]
    pub fn control_ctrl(&self) -> CONTROL_CTRL_R {
        CONTROL_CTRL_R::new(((self.bits >> 11) & 7) as u8)
    }
    #[doc = "Bits 14:16 - STATE control field."]
    #[inline(always)]
    pub fn state_ctrl(&self) -> STATE_CTRL_R {
        STATE_CTRL_R::new(((self.bits >> 14) & 7) as u8)
    }
    #[doc = "Bits 17:19 - ADDRESS control field."]
    #[inline(always)]
    pub fn address_ctrl(&self) -> ADDRESS_CTRL_R {
        ADDRESS_CTRL_R::new(((self.bits >> 17) & 7) as u8)
    }
    #[doc = "Bits 28:29 - IRQ pause control field."]
    #[inline(always)]
    pub fn irq_pause(&self) -> IRQ_PAUSE_R {
        IRQ_PAUSE_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - DEBUG_HALT control field."]
    #[inline(always)]
    pub fn debug_halt_ctrl(&self) -> DEBUG_HALT_CTRL_R {
        DEBUG_HALT_CTRL_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Lock control field."]
    #[inline(always)]
    pub fn lock_ctrl(&mut self) -> LOCK_CTRL_W<0> {
        LOCK_CTRL_W::new(self)
    }
    #[doc = "Bits 2:4 - TIMEOUT control."]
    #[inline(always)]
    pub fn timeout_ctrl(&mut self) -> TIMEOUT_CTRL_W<2> {
        TIMEOUT_CTRL_W::new(self)
    }
    #[doc = "Bits 5:7 - MISCOMPARE control field."]
    #[inline(always)]
    pub fn miscompare_ctrl(&mut self) -> MISCOMPARE_CTRL_W<5> {
        MISCOMPARE_CTRL_W::new(self)
    }
    #[doc = "Bits 8:10 - SEQUENCE control field."]
    #[inline(always)]
    pub fn sequence_ctrl(&mut self) -> SEQUENCE_CTRL_W<8> {
        SEQUENCE_CTRL_W::new(self)
    }
    #[doc = "Bits 11:13 - CONTROL control field."]
    #[inline(always)]
    pub fn control_ctrl(&mut self) -> CONTROL_CTRL_W<11> {
        CONTROL_CTRL_W::new(self)
    }
    #[doc = "Bits 14:16 - STATE control field."]
    #[inline(always)]
    pub fn state_ctrl(&mut self) -> STATE_CTRL_W<14> {
        STATE_CTRL_W::new(self)
    }
    #[doc = "Bits 17:19 - ADDRESS control field."]
    #[inline(always)]
    pub fn address_ctrl(&mut self) -> ADDRESS_CTRL_W<17> {
        ADDRESS_CTRL_W::new(self)
    }
    #[doc = "Bits 28:29 - IRQ pause control field."]
    #[inline(always)]
    pub fn irq_pause(&mut self) -> IRQ_PAUSE_W<28> {
        IRQ_PAUSE_W::new(self)
    }
    #[doc = "Bits 30:31 - DEBUG_HALT control field."]
    #[inline(always)]
    pub fn debug_halt_ctrl(&mut self) -> DEBUG_HALT_CTRL_W<30> {
        DEBUG_HALT_CTRL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The control fields, which constitute CONTROL, control all controllable attributes of the module, including those of CONTROL itself.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [control](index.html) module"]
pub struct CONTROL_SPEC;
impl crate::RegisterSpec for CONTROL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [control::R](R) reader structure"]
impl crate::Readable for CONTROL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [control::W](W) writer structure"]
impl crate::Writable for CONTROL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CONTROL to value 0x5009_2492"]
impl crate::Resettable for CONTROL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x5009_2492
    }
}
