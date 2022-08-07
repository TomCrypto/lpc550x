#[doc = "Register `BOOT_CFG` reader"]
pub struct R(crate::R<BOOT_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BOOT_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BOOT_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BOOT_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BOOT_CFG` writer"]
pub struct W(crate::W<BOOT_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BOOT_CFG_SPEC>;
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
impl From<crate::W<BOOT_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BOOT_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DEFAULT_ISP_MODE` reader - Default ISP mode:"]
pub type DEFAULT_ISP_MODE_R = crate::FieldReader<u8, DEFAULT_ISP_MODE_A>;
#[doc = "Default ISP mode:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DEFAULT_ISP_MODE_A {
    #[doc = "0: Auto ISP"]
    AUTO_ISP = 0,
    #[doc = "1: USB_HID_ISP"]
    USB_HID_ISP = 1,
    #[doc = "2: UART ISP"]
    UART_ISP = 2,
    #[doc = "3: SPI Slave ISP"]
    SPI_ISP = 3,
    #[doc = "4: I2C Slave ISP"]
    I2C_ISP = 4,
    #[doc = "7: Disable ISP fall through"]
    DISABLE = 7,
}
impl From<DEFAULT_ISP_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: DEFAULT_ISP_MODE_A) -> Self {
        variant as _
    }
}
impl DEFAULT_ISP_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DEFAULT_ISP_MODE_A> {
        match self.bits {
            0 => Some(DEFAULT_ISP_MODE_A::AUTO_ISP),
            1 => Some(DEFAULT_ISP_MODE_A::USB_HID_ISP),
            2 => Some(DEFAULT_ISP_MODE_A::UART_ISP),
            3 => Some(DEFAULT_ISP_MODE_A::SPI_ISP),
            4 => Some(DEFAULT_ISP_MODE_A::I2C_ISP),
            7 => Some(DEFAULT_ISP_MODE_A::DISABLE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `AUTO_ISP`"]
    #[inline(always)]
    pub fn is_auto_isp(&self) -> bool {
        *self == DEFAULT_ISP_MODE_A::AUTO_ISP
    }
    #[doc = "Checks if the value of the field is `USB_HID_ISP`"]
    #[inline(always)]
    pub fn is_usb_hid_isp(&self) -> bool {
        *self == DEFAULT_ISP_MODE_A::USB_HID_ISP
    }
    #[doc = "Checks if the value of the field is `UART_ISP`"]
    #[inline(always)]
    pub fn is_uart_isp(&self) -> bool {
        *self == DEFAULT_ISP_MODE_A::UART_ISP
    }
    #[doc = "Checks if the value of the field is `SPI_ISP`"]
    #[inline(always)]
    pub fn is_spi_isp(&self) -> bool {
        *self == DEFAULT_ISP_MODE_A::SPI_ISP
    }
    #[doc = "Checks if the value of the field is `I2C_ISP`"]
    #[inline(always)]
    pub fn is_i2c_isp(&self) -> bool {
        *self == DEFAULT_ISP_MODE_A::I2C_ISP
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == DEFAULT_ISP_MODE_A::DISABLE
    }
}
#[doc = "Field `DEFAULT_ISP_MODE` writer - Default ISP mode:"]
pub type DEFAULT_ISP_MODE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, BOOT_CFG_SPEC, u8, DEFAULT_ISP_MODE_A, 3, O>;
impl<'a, const O: u8> DEFAULT_ISP_MODE_W<'a, O> {
    #[doc = "Auto ISP"]
    #[inline(always)]
    pub fn auto_isp(self) -> &'a mut W {
        self.variant(DEFAULT_ISP_MODE_A::AUTO_ISP)
    }
    #[doc = "USB_HID_ISP"]
    #[inline(always)]
    pub fn usb_hid_isp(self) -> &'a mut W {
        self.variant(DEFAULT_ISP_MODE_A::USB_HID_ISP)
    }
    #[doc = "UART ISP"]
    #[inline(always)]
    pub fn uart_isp(self) -> &'a mut W {
        self.variant(DEFAULT_ISP_MODE_A::UART_ISP)
    }
    #[doc = "SPI Slave ISP"]
    #[inline(always)]
    pub fn spi_isp(self) -> &'a mut W {
        self.variant(DEFAULT_ISP_MODE_A::SPI_ISP)
    }
    #[doc = "I2C Slave ISP"]
    #[inline(always)]
    pub fn i2c_isp(self) -> &'a mut W {
        self.variant(DEFAULT_ISP_MODE_A::I2C_ISP)
    }
    #[doc = "Disable ISP fall through"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(DEFAULT_ISP_MODE_A::DISABLE)
    }
}
#[doc = "Field `BOOT_SPEED` reader - Core clock:"]
pub type BOOT_SPEED_R = crate::FieldReader<u8, BOOT_SPEED_A>;
#[doc = "Core clock:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BOOT_SPEED_A {
    #[doc = "0: Defined by NMPA.SYSTEM_SPEED_CODE"]
    VALUE_0 = 0,
    #[doc = "2: 48MHz FRO"]
    VALUE_2 = 2,
}
impl From<BOOT_SPEED_A> for u8 {
    #[inline(always)]
    fn from(variant: BOOT_SPEED_A) -> Self {
        variant as _
    }
}
impl BOOT_SPEED_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<BOOT_SPEED_A> {
        match self.bits {
            0 => Some(BOOT_SPEED_A::VALUE_0),
            2 => Some(BOOT_SPEED_A::VALUE_2),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE_0`"]
    #[inline(always)]
    pub fn is_value_0(&self) -> bool {
        *self == BOOT_SPEED_A::VALUE_0
    }
    #[doc = "Checks if the value of the field is `VALUE_2`"]
    #[inline(always)]
    pub fn is_value_2(&self) -> bool {
        *self == BOOT_SPEED_A::VALUE_2
    }
}
#[doc = "Field `BOOT_SPEED` writer - Core clock:"]
pub type BOOT_SPEED_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, BOOT_CFG_SPEC, u8, BOOT_SPEED_A, 2, O>;
impl<'a, const O: u8> BOOT_SPEED_W<'a, O> {
    #[doc = "Defined by NMPA.SYSTEM_SPEED_CODE"]
    #[inline(always)]
    pub fn value_0(self) -> &'a mut W {
        self.variant(BOOT_SPEED_A::VALUE_0)
    }
    #[doc = "48MHz FRO"]
    #[inline(always)]
    pub fn value_2(self) -> &'a mut W {
        self.variant(BOOT_SPEED_A::VALUE_2)
    }
}
#[doc = "Field `BOOT_FAILURE_PIN` reader - GPIO port and pin number to use for indicating failure reason. The toggle rate of the pin is used to decode the error type. \\[2:0\\]
- Defines GPIO port \\[7:3\\]
- Defines GPIO pin"]
pub type BOOT_FAILURE_PIN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BOOT_FAILURE_PIN` writer - GPIO port and pin number to use for indicating failure reason. The toggle rate of the pin is used to decode the error type. \\[2:0\\]
- Defines GPIO port \\[7:3\\]
- Defines GPIO pin"]
pub type BOOT_FAILURE_PIN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, BOOT_CFG_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 4:6 - Default ISP mode:"]
    #[inline(always)]
    pub fn default_isp_mode(&self) -> DEFAULT_ISP_MODE_R {
        DEFAULT_ISP_MODE_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 7:8 - Core clock:"]
    #[inline(always)]
    pub fn boot_speed(&self) -> BOOT_SPEED_R {
        BOOT_SPEED_R::new(((self.bits >> 7) & 3) as u8)
    }
    #[doc = "Bits 24:31 - GPIO port and pin number to use for indicating failure reason. The toggle rate of the pin is used to decode the error type. \\[2:0\\]
- Defines GPIO port \\[7:3\\]
- Defines GPIO pin"]
    #[inline(always)]
    pub fn boot_failure_pin(&self) -> BOOT_FAILURE_PIN_R {
        BOOT_FAILURE_PIN_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 4:6 - Default ISP mode:"]
    #[inline(always)]
    pub fn default_isp_mode(&mut self) -> DEFAULT_ISP_MODE_W<4> {
        DEFAULT_ISP_MODE_W::new(self)
    }
    #[doc = "Bits 7:8 - Core clock:"]
    #[inline(always)]
    pub fn boot_speed(&mut self) -> BOOT_SPEED_W<7> {
        BOOT_SPEED_W::new(self)
    }
    #[doc = "Bits 24:31 - GPIO port and pin number to use for indicating failure reason. The toggle rate of the pin is used to decode the error type. \\[2:0\\]
- Defines GPIO port \\[7:3\\]
- Defines GPIO pin"]
    #[inline(always)]
    pub fn boot_failure_pin(&mut self) -> BOOT_FAILURE_PIN_W<24> {
        BOOT_FAILURE_PIN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "no description available\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [boot_cfg](index.html) module"]
pub struct BOOT_CFG_SPEC;
impl crate::RegisterSpec for BOOT_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [boot_cfg::R](R) reader structure"]
impl crate::Readable for BOOT_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [boot_cfg::W](W) writer structure"]
impl crate::Writable for BOOT_CFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BOOT_CFG to value 0"]
impl crate::Resettable for BOOT_CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
