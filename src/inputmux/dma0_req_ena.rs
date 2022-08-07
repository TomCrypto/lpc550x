#[doc = "Register `DMA0_REQ_ENA` reader"]
pub struct R(crate::R<DMA0_REQ_ENA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA0_REQ_ENA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA0_REQ_ENA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA0_REQ_ENA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMA0_REQ_ENA` writer"]
pub struct W(crate::W<DMA0_REQ_ENA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMA0_REQ_ENA_SPEC>;
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
impl From<crate::W<DMA0_REQ_ENA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMA0_REQ_ENA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `REQ_ENA` reader - Controls the 23 request inputs of DMA0. If bit i is '1' the DMA request input #i is enabled."]
pub type REQ_ENA_R = crate::FieldReader<u32, REQ_ENA_A>;
#[doc = "Controls the 23 request inputs of DMA0. If bit i is '1' the DMA request input #i is enabled.\n\nValue on reset: 8388607"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum REQ_ENA_A {
    #[doc = "0: Hash-Crypt DMA request."]
    VAL0 = 0,
    #[doc = "2: High Speed SPI (Flexcomm 8) RX."]
    VAL2 = 2,
    #[doc = "3: High Speed SPI (Flexcomm 8) TX."]
    VAL3 = 3,
    #[doc = "4: Flexcomm Interface 0 RX / I2C Slave."]
    VAL4 = 4,
    #[doc = "5: Flexcomm Interface 0 TX / I2C Master."]
    VAL5 = 5,
    #[doc = "6: Flexcomm Interface 1 RX / I2C Slave."]
    VAL6 = 6,
    #[doc = "7: Flexcomm Interface 1 TX / I2C Master."]
    VAL7 = 7,
    #[doc = "8: Flexcomm Interface 3 RX / I2C Slave."]
    VAL8 = 8,
    #[doc = "9: Flexcomm Interface 3 TX / I2C Master."]
    VAL9 = 9,
    #[doc = "10: Flexcomm Interface 2 RX / I2C Slave."]
    VAL10 = 10,
    #[doc = "11: Flexcomm Interface 2 TX / I2C Master."]
    VAL11 = 11,
    #[doc = "12: Flexcomm Interface 4 RX / I2C Slave."]
    VAL12 = 12,
    #[doc = "13: Flexcomm Interface 4 TX / I2C Master."]
    VAL13 = 13,
    #[doc = "14: Flexcomm Interface 5 RX / I2C Slave."]
    VAL14 = 14,
    #[doc = "15: Flexcomm Interface 5 TX / I2C Master."]
    VAL15 = 15,
    #[doc = "16: Flexcomm Interface 6 RX / I2C Slave."]
    VAL16 = 16,
    #[doc = "17: Flexcomm Interface 6 TX / I2C Master."]
    VAL17 = 17,
    #[doc = "18: Flexcomm Interface 7 RX / I2C Slave."]
    VAL18 = 18,
    #[doc = "19: Flexcomm Interface 6 TX / I2C Master."]
    VAL19 = 19,
    #[doc = "21: ADC0 FIFO 0."]
    VAL21 = 21,
    #[doc = "22: ADC0 FIFO 1."]
    VAL22 = 22,
}
impl From<REQ_ENA_A> for u32 {
    #[inline(always)]
    fn from(variant: REQ_ENA_A) -> Self {
        variant as _
    }
}
impl REQ_ENA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<REQ_ENA_A> {
        match self.bits {
            0 => Some(REQ_ENA_A::VAL0),
            2 => Some(REQ_ENA_A::VAL2),
            3 => Some(REQ_ENA_A::VAL3),
            4 => Some(REQ_ENA_A::VAL4),
            5 => Some(REQ_ENA_A::VAL5),
            6 => Some(REQ_ENA_A::VAL6),
            7 => Some(REQ_ENA_A::VAL7),
            8 => Some(REQ_ENA_A::VAL8),
            9 => Some(REQ_ENA_A::VAL9),
            10 => Some(REQ_ENA_A::VAL10),
            11 => Some(REQ_ENA_A::VAL11),
            12 => Some(REQ_ENA_A::VAL12),
            13 => Some(REQ_ENA_A::VAL13),
            14 => Some(REQ_ENA_A::VAL14),
            15 => Some(REQ_ENA_A::VAL15),
            16 => Some(REQ_ENA_A::VAL16),
            17 => Some(REQ_ENA_A::VAL17),
            18 => Some(REQ_ENA_A::VAL18),
            19 => Some(REQ_ENA_A::VAL19),
            21 => Some(REQ_ENA_A::VAL21),
            22 => Some(REQ_ENA_A::VAL22),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VAL0`"]
    #[inline(always)]
    pub fn is_val0(&self) -> bool {
        *self == REQ_ENA_A::VAL0
    }
    #[doc = "Checks if the value of the field is `VAL2`"]
    #[inline(always)]
    pub fn is_val2(&self) -> bool {
        *self == REQ_ENA_A::VAL2
    }
    #[doc = "Checks if the value of the field is `VAL3`"]
    #[inline(always)]
    pub fn is_val3(&self) -> bool {
        *self == REQ_ENA_A::VAL3
    }
    #[doc = "Checks if the value of the field is `VAL4`"]
    #[inline(always)]
    pub fn is_val4(&self) -> bool {
        *self == REQ_ENA_A::VAL4
    }
    #[doc = "Checks if the value of the field is `VAL5`"]
    #[inline(always)]
    pub fn is_val5(&self) -> bool {
        *self == REQ_ENA_A::VAL5
    }
    #[doc = "Checks if the value of the field is `VAL6`"]
    #[inline(always)]
    pub fn is_val6(&self) -> bool {
        *self == REQ_ENA_A::VAL6
    }
    #[doc = "Checks if the value of the field is `VAL7`"]
    #[inline(always)]
    pub fn is_val7(&self) -> bool {
        *self == REQ_ENA_A::VAL7
    }
    #[doc = "Checks if the value of the field is `VAL8`"]
    #[inline(always)]
    pub fn is_val8(&self) -> bool {
        *self == REQ_ENA_A::VAL8
    }
    #[doc = "Checks if the value of the field is `VAL9`"]
    #[inline(always)]
    pub fn is_val9(&self) -> bool {
        *self == REQ_ENA_A::VAL9
    }
    #[doc = "Checks if the value of the field is `VAL10`"]
    #[inline(always)]
    pub fn is_val10(&self) -> bool {
        *self == REQ_ENA_A::VAL10
    }
    #[doc = "Checks if the value of the field is `VAL11`"]
    #[inline(always)]
    pub fn is_val11(&self) -> bool {
        *self == REQ_ENA_A::VAL11
    }
    #[doc = "Checks if the value of the field is `VAL12`"]
    #[inline(always)]
    pub fn is_val12(&self) -> bool {
        *self == REQ_ENA_A::VAL12
    }
    #[doc = "Checks if the value of the field is `VAL13`"]
    #[inline(always)]
    pub fn is_val13(&self) -> bool {
        *self == REQ_ENA_A::VAL13
    }
    #[doc = "Checks if the value of the field is `VAL14`"]
    #[inline(always)]
    pub fn is_val14(&self) -> bool {
        *self == REQ_ENA_A::VAL14
    }
    #[doc = "Checks if the value of the field is `VAL15`"]
    #[inline(always)]
    pub fn is_val15(&self) -> bool {
        *self == REQ_ENA_A::VAL15
    }
    #[doc = "Checks if the value of the field is `VAL16`"]
    #[inline(always)]
    pub fn is_val16(&self) -> bool {
        *self == REQ_ENA_A::VAL16
    }
    #[doc = "Checks if the value of the field is `VAL17`"]
    #[inline(always)]
    pub fn is_val17(&self) -> bool {
        *self == REQ_ENA_A::VAL17
    }
    #[doc = "Checks if the value of the field is `VAL18`"]
    #[inline(always)]
    pub fn is_val18(&self) -> bool {
        *self == REQ_ENA_A::VAL18
    }
    #[doc = "Checks if the value of the field is `VAL19`"]
    #[inline(always)]
    pub fn is_val19(&self) -> bool {
        *self == REQ_ENA_A::VAL19
    }
    #[doc = "Checks if the value of the field is `VAL21`"]
    #[inline(always)]
    pub fn is_val21(&self) -> bool {
        *self == REQ_ENA_A::VAL21
    }
    #[doc = "Checks if the value of the field is `VAL22`"]
    #[inline(always)]
    pub fn is_val22(&self) -> bool {
        *self == REQ_ENA_A::VAL22
    }
}
#[doc = "Field `REQ_ENA` writer - Controls the 23 request inputs of DMA0. If bit i is '1' the DMA request input #i is enabled."]
pub type REQ_ENA_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DMA0_REQ_ENA_SPEC, u32, REQ_ENA_A, 23, O>;
impl<'a, const O: u8> REQ_ENA_W<'a, O> {
    #[doc = "Hash-Crypt DMA request."]
    #[inline(always)]
    pub fn val0(self) -> &'a mut W {
        self.variant(REQ_ENA_A::VAL0)
    }
    #[doc = "High Speed SPI (Flexcomm 8) RX."]
    #[inline(always)]
    pub fn val2(self) -> &'a mut W {
        self.variant(REQ_ENA_A::VAL2)
    }
    #[doc = "High Speed SPI (Flexcomm 8) TX."]
    #[inline(always)]
    pub fn val3(self) -> &'a mut W {
        self.variant(REQ_ENA_A::VAL3)
    }
    #[doc = "Flexcomm Interface 0 RX / I2C Slave."]
    #[inline(always)]
    pub fn val4(self) -> &'a mut W {
        self.variant(REQ_ENA_A::VAL4)
    }
    #[doc = "Flexcomm Interface 0 TX / I2C Master."]
    #[inline(always)]
    pub fn val5(self) -> &'a mut W {
        self.variant(REQ_ENA_A::VAL5)
    }
    #[doc = "Flexcomm Interface 1 RX / I2C Slave."]
    #[inline(always)]
    pub fn val6(self) -> &'a mut W {
        self.variant(REQ_ENA_A::VAL6)
    }
    #[doc = "Flexcomm Interface 1 TX / I2C Master."]
    #[inline(always)]
    pub fn val7(self) -> &'a mut W {
        self.variant(REQ_ENA_A::VAL7)
    }
    #[doc = "Flexcomm Interface 3 RX / I2C Slave."]
    #[inline(always)]
    pub fn val8(self) -> &'a mut W {
        self.variant(REQ_ENA_A::VAL8)
    }
    #[doc = "Flexcomm Interface 3 TX / I2C Master."]
    #[inline(always)]
    pub fn val9(self) -> &'a mut W {
        self.variant(REQ_ENA_A::VAL9)
    }
    #[doc = "Flexcomm Interface 2 RX / I2C Slave."]
    #[inline(always)]
    pub fn val10(self) -> &'a mut W {
        self.variant(REQ_ENA_A::VAL10)
    }
    #[doc = "Flexcomm Interface 2 TX / I2C Master."]
    #[inline(always)]
    pub fn val11(self) -> &'a mut W {
        self.variant(REQ_ENA_A::VAL11)
    }
    #[doc = "Flexcomm Interface 4 RX / I2C Slave."]
    #[inline(always)]
    pub fn val12(self) -> &'a mut W {
        self.variant(REQ_ENA_A::VAL12)
    }
    #[doc = "Flexcomm Interface 4 TX / I2C Master."]
    #[inline(always)]
    pub fn val13(self) -> &'a mut W {
        self.variant(REQ_ENA_A::VAL13)
    }
    #[doc = "Flexcomm Interface 5 RX / I2C Slave."]
    #[inline(always)]
    pub fn val14(self) -> &'a mut W {
        self.variant(REQ_ENA_A::VAL14)
    }
    #[doc = "Flexcomm Interface 5 TX / I2C Master."]
    #[inline(always)]
    pub fn val15(self) -> &'a mut W {
        self.variant(REQ_ENA_A::VAL15)
    }
    #[doc = "Flexcomm Interface 6 RX / I2C Slave."]
    #[inline(always)]
    pub fn val16(self) -> &'a mut W {
        self.variant(REQ_ENA_A::VAL16)
    }
    #[doc = "Flexcomm Interface 6 TX / I2C Master."]
    #[inline(always)]
    pub fn val17(self) -> &'a mut W {
        self.variant(REQ_ENA_A::VAL17)
    }
    #[doc = "Flexcomm Interface 7 RX / I2C Slave."]
    #[inline(always)]
    pub fn val18(self) -> &'a mut W {
        self.variant(REQ_ENA_A::VAL18)
    }
    #[doc = "Flexcomm Interface 6 TX / I2C Master."]
    #[inline(always)]
    pub fn val19(self) -> &'a mut W {
        self.variant(REQ_ENA_A::VAL19)
    }
    #[doc = "ADC0 FIFO 0."]
    #[inline(always)]
    pub fn val21(self) -> &'a mut W {
        self.variant(REQ_ENA_A::VAL21)
    }
    #[doc = "ADC0 FIFO 1."]
    #[inline(always)]
    pub fn val22(self) -> &'a mut W {
        self.variant(REQ_ENA_A::VAL22)
    }
}
impl R {
    #[doc = "Bits 0:22 - Controls the 23 request inputs of DMA0. If bit i is '1' the DMA request input #i is enabled."]
    #[inline(always)]
    pub fn req_ena(&self) -> REQ_ENA_R {
        REQ_ENA_R::new((self.bits & 0x007f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:22 - Controls the 23 request inputs of DMA0. If bit i is '1' the DMA request input #i is enabled."]
    #[inline(always)]
    pub fn req_ena(&mut self) -> REQ_ENA_W<0> {
        REQ_ENA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Enable DMA0 requests\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma0_req_ena](index.html) module"]
pub struct DMA0_REQ_ENA_SPEC;
impl crate::RegisterSpec for DMA0_REQ_ENA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma0_req_ena::R](R) reader structure"]
impl crate::Readable for DMA0_REQ_ENA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dma0_req_ena::W](W) writer structure"]
impl crate::Writable for DMA0_REQ_ENA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMA0_REQ_ENA to value 0x007f_ffff"]
impl crate::Resettable for DMA0_REQ_ENA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x007f_ffff
    }
}
