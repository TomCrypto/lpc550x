#[doc = "Register `DMA1_REQ_ENA` reader"]
pub struct R(crate::R<DMA1_REQ_ENA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA1_REQ_ENA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA1_REQ_ENA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA1_REQ_ENA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMA1_REQ_ENA` writer"]
pub struct W(crate::W<DMA1_REQ_ENA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMA1_REQ_ENA_SPEC>;
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
impl From<crate::W<DMA1_REQ_ENA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMA1_REQ_ENA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `REQ_ENA` reader - Controls the 10 request inputs of DMA1. If bit i is '1' the DMA request input #i is enabled."]
pub type REQ_ENA_R = crate::FieldReader<u16, REQ_ENA_A>;
#[doc = "Controls the 10 request inputs of DMA1. If bit i is '1' the DMA request input #i is enabled.\n\nValue on reset: 1023"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum REQ_ENA_A {
    #[doc = "0: Hash-Crypt input DMA request."]
    VAL0 = 0,
    #[doc = "2: High Speed SPI (Flexcomm 8) RX."]
    VAL2 = 2,
    #[doc = "3: High Speed SPI (Flexcomm 8) TX."]
    VAL3 = 3,
    #[doc = "4: Flexcomm Interface 0 RX /I2C Slave."]
    VAL4 = 4,
    #[doc = "5: Flexcomm Interface 0 TX / I2C Master."]
    VAL5 = 5,
    #[doc = "6: Flexcomm Interface 1 RX /I2C Slave."]
    VAL6 = 6,
    #[doc = "7: Flexcomm Interface 1 TX / I2C Master."]
    VAL7 = 7,
    #[doc = "8: Flexcomm Interface 3 RX / I2C Slave."]
    VAL8 = 8,
    #[doc = "9: Flexcomm Interface 3 TX / I2C Master."]
    VAL9 = 9,
}
impl From<REQ_ENA_A> for u16 {
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
}
#[doc = "Field `REQ_ENA` writer - Controls the 10 request inputs of DMA1. If bit i is '1' the DMA request input #i is enabled."]
pub type REQ_ENA_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DMA1_REQ_ENA_SPEC, u16, REQ_ENA_A, 10, O>;
impl<'a, const O: u8> REQ_ENA_W<'a, O> {
    #[doc = "Hash-Crypt input DMA request."]
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
    #[doc = "Flexcomm Interface 0 RX /I2C Slave."]
    #[inline(always)]
    pub fn val4(self) -> &'a mut W {
        self.variant(REQ_ENA_A::VAL4)
    }
    #[doc = "Flexcomm Interface 0 TX / I2C Master."]
    #[inline(always)]
    pub fn val5(self) -> &'a mut W {
        self.variant(REQ_ENA_A::VAL5)
    }
    #[doc = "Flexcomm Interface 1 RX /I2C Slave."]
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
}
impl R {
    #[doc = "Bits 0:9 - Controls the 10 request inputs of DMA1. If bit i is '1' the DMA request input #i is enabled."]
    #[inline(always)]
    pub fn req_ena(&self) -> REQ_ENA_R {
        REQ_ENA_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Controls the 10 request inputs of DMA1. If bit i is '1' the DMA request input #i is enabled."]
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
#[doc = "Enable DMA1 requests.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma1_req_ena](index.html) module"]
pub struct DMA1_REQ_ENA_SPEC;
impl crate::RegisterSpec for DMA1_REQ_ENA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma1_req_ena::R](R) reader structure"]
impl crate::Readable for DMA1_REQ_ENA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dma1_req_ena::W](W) writer structure"]
impl crate::Writable for DMA1_REQ_ENA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMA1_REQ_ENA to value 0x03ff"]
impl crate::Resettable for DMA1_REQ_ENA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x03ff
    }
}
