#[doc = "Register `GPO0_2` reader"]
pub struct R(crate::R<GPO0_2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPO0_2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPO0_2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPO0_2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPO0_2` writer"]
pub struct W(crate::W<GPO0_2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPO0_2_SPEC>;
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
impl From<crate::W<GPO0_2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPO0_2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYSTEM_SPEED_CODE` reader - 00 : FRO12MHz 01 : FRO24MHz 10 : FRO48MHz 11 : FRO96MHz."]
pub type SYSTEM_SPEED_CODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SYSTEM_SPEED_CODE` writer - 00 : FRO12MHz 01 : FRO24MHz 10 : FRO48MHz 11 : FRO96MHz."]
pub type SYSTEM_SPEED_CODE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GPO0_2_SPEC, u8, u8, 2, O>;
#[doc = "Field `FLASH_CTRL_OPMODE` reader - 00 : Delay Line 01 : RCLK (back up clock) 10 : PCLK (back up clock)"]
pub type FLASH_CTRL_OPMODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FLASH_CTRL_OPMODE` writer - 00 : Delay Line 01 : RCLK (back up clock) 10 : PCLK (back up clock)"]
pub type FLASH_CTRL_OPMODE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GPO0_2_SPEC, u8, u8, 2, O>;
#[doc = "Field `FIELD` reader - no description available."]
pub type FIELD_R = crate::FieldReader<u32, u32>;
#[doc = "Field `FIELD` writer - no description available."]
pub type FIELD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GPO0_2_SPEC, u32, u32, 28, O>;
impl R {
    #[doc = "Bits 0:1 - 00 : FRO12MHz 01 : FRO24MHz 10 : FRO48MHz 11 : FRO96MHz."]
    #[inline(always)]
    pub fn system_speed_code(&self) -> SYSTEM_SPEED_CODE_R {
        SYSTEM_SPEED_CODE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - 00 : Delay Line 01 : RCLK (back up clock) 10 : PCLK (back up clock)"]
    #[inline(always)]
    pub fn flash_ctrl_opmode(&self) -> FLASH_CTRL_OPMODE_R {
        FLASH_CTRL_OPMODE_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:31 - no description available."]
    #[inline(always)]
    pub fn field(&self) -> FIELD_R {
        FIELD_R::new(((self.bits >> 4) & 0x0fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:1 - 00 : FRO12MHz 01 : FRO24MHz 10 : FRO48MHz 11 : FRO96MHz."]
    #[inline(always)]
    pub fn system_speed_code(&mut self) -> SYSTEM_SPEED_CODE_W<0> {
        SYSTEM_SPEED_CODE_W::new(self)
    }
    #[doc = "Bits 2:3 - 00 : Delay Line 01 : RCLK (back up clock) 10 : PCLK (back up clock)"]
    #[inline(always)]
    pub fn flash_ctrl_opmode(&mut self) -> FLASH_CTRL_OPMODE_W<2> {
        FLASH_CTRL_OPMODE_W::new(self)
    }
    #[doc = "Bits 4:31 - no description available."]
    #[inline(always)]
    pub fn field(&mut self) -> FIELD_W<4> {
        FIELD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPO0 register 2 description.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpo0_2](index.html) module"]
pub struct GPO0_2_SPEC;
impl crate::RegisterSpec for GPO0_2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpo0_2::R](R) reader structure"]
impl crate::Readable for GPO0_2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpo0_2::W](W) writer structure"]
impl crate::Writable for GPO0_2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GPO0_2 to value 0"]
impl crate::Resettable for GPO0_2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
