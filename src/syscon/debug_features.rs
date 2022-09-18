#[doc = "Register `DEBUG_FEATURES` reader"]
pub struct R(crate::R<DEBUG_FEATURES_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DEBUG_FEATURES_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DEBUG_FEATURES_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DEBUG_FEATURES_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DEBUG_FEATURES` writer"]
pub struct W(crate::W<DEBUG_FEATURES_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DEBUG_FEATURES_SPEC>;
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
impl From<crate::W<DEBUG_FEATURES_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DEBUG_FEATURES_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CPU0_DBGEN` reader - CPU0 Invasive debug control:."]
pub type CPU0_DBGEN_R = crate::FieldReader<u8, CPU0_DBGEN_A>;
#[doc = "CPU0 Invasive debug control:.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CPU0_DBGEN_A {
    #[doc = "1: Any other value than b10: invasive debug is disable."]
    DISABLE = 1,
    #[doc = "2: 10: Invasive debug is enabled."]
    ENABLE = 2,
}
impl From<CPU0_DBGEN_A> for u8 {
    #[inline(always)]
    fn from(variant: CPU0_DBGEN_A) -> Self {
        variant as _
    }
}
impl CPU0_DBGEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CPU0_DBGEN_A> {
        match self.bits {
            1 => Some(CPU0_DBGEN_A::DISABLE),
            2 => Some(CPU0_DBGEN_A::ENABLE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CPU0_DBGEN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CPU0_DBGEN_A::ENABLE
    }
}
#[doc = "Field `CPU0_DBGEN` writer - CPU0 Invasive debug control:."]
pub type CPU0_DBGEN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DEBUG_FEATURES_SPEC, u8, CPU0_DBGEN_A, 2, O>;
impl<'a, const O: u8> CPU0_DBGEN_W<'a, O> {
    #[doc = "Any other value than b10: invasive debug is disable."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CPU0_DBGEN_A::DISABLE)
    }
    #[doc = "10: Invasive debug is enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CPU0_DBGEN_A::ENABLE)
    }
}
#[doc = "Field `CPU0_NIDEN` reader - CPU0 Non Invasive debug control:."]
pub type CPU0_NIDEN_R = crate::FieldReader<u8, CPU0_NIDEN_A>;
#[doc = "CPU0 Non Invasive debug control:.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CPU0_NIDEN_A {
    #[doc = "1: Any other value than b10: invasive debug is disable."]
    DISABLE = 1,
    #[doc = "2: 10: Invasive debug is enabled."]
    ENABLE = 2,
}
impl From<CPU0_NIDEN_A> for u8 {
    #[inline(always)]
    fn from(variant: CPU0_NIDEN_A) -> Self {
        variant as _
    }
}
impl CPU0_NIDEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CPU0_NIDEN_A> {
        match self.bits {
            1 => Some(CPU0_NIDEN_A::DISABLE),
            2 => Some(CPU0_NIDEN_A::ENABLE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CPU0_NIDEN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CPU0_NIDEN_A::ENABLE
    }
}
#[doc = "Field `CPU0_NIDEN` writer - CPU0 Non Invasive debug control:."]
pub type CPU0_NIDEN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DEBUG_FEATURES_SPEC, u8, CPU0_NIDEN_A, 2, O>;
impl<'a, const O: u8> CPU0_NIDEN_W<'a, O> {
    #[doc = "Any other value than b10: invasive debug is disable."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CPU0_NIDEN_A::DISABLE)
    }
    #[doc = "10: Invasive debug is enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CPU0_NIDEN_A::ENABLE)
    }
}
#[doc = "Field `CPU0_SPIDEN` reader - CPU0 Secure Invasive debug control:."]
pub type CPU0_SPIDEN_R = crate::FieldReader<u8, CPU0_SPIDEN_A>;
#[doc = "CPU0 Secure Invasive debug control:.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CPU0_SPIDEN_A {
    #[doc = "1: Any other value than b10: invasive debug is disable."]
    DISABLE = 1,
    #[doc = "2: 10: Invasive debug is enabled."]
    ENABLE = 2,
}
impl From<CPU0_SPIDEN_A> for u8 {
    #[inline(always)]
    fn from(variant: CPU0_SPIDEN_A) -> Self {
        variant as _
    }
}
impl CPU0_SPIDEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CPU0_SPIDEN_A> {
        match self.bits {
            1 => Some(CPU0_SPIDEN_A::DISABLE),
            2 => Some(CPU0_SPIDEN_A::ENABLE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CPU0_SPIDEN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CPU0_SPIDEN_A::ENABLE
    }
}
#[doc = "Field `CPU0_SPIDEN` writer - CPU0 Secure Invasive debug control:."]
pub type CPU0_SPIDEN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DEBUG_FEATURES_SPEC, u8, CPU0_SPIDEN_A, 2, O>;
impl<'a, const O: u8> CPU0_SPIDEN_W<'a, O> {
    #[doc = "Any other value than b10: invasive debug is disable."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CPU0_SPIDEN_A::DISABLE)
    }
    #[doc = "10: Invasive debug is enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CPU0_SPIDEN_A::ENABLE)
    }
}
#[doc = "Field `CPU0_SPNIDEN` reader - CPU0 Secure Non Invasive debug control:."]
pub type CPU0_SPNIDEN_R = crate::FieldReader<u8, CPU0_SPNIDEN_A>;
#[doc = "CPU0 Secure Non Invasive debug control:.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CPU0_SPNIDEN_A {
    #[doc = "1: Any other value than b10: invasive debug is disable."]
    DISABLE = 1,
    #[doc = "2: 10: Invasive debug is enabled."]
    ENABLE = 2,
}
impl From<CPU0_SPNIDEN_A> for u8 {
    #[inline(always)]
    fn from(variant: CPU0_SPNIDEN_A) -> Self {
        variant as _
    }
}
impl CPU0_SPNIDEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CPU0_SPNIDEN_A> {
        match self.bits {
            1 => Some(CPU0_SPNIDEN_A::DISABLE),
            2 => Some(CPU0_SPNIDEN_A::ENABLE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CPU0_SPNIDEN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CPU0_SPNIDEN_A::ENABLE
    }
}
#[doc = "Field `CPU0_SPNIDEN` writer - CPU0 Secure Non Invasive debug control:."]
pub type CPU0_SPNIDEN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DEBUG_FEATURES_SPEC, u8, CPU0_SPNIDEN_A, 2, O>;
impl<'a, const O: u8> CPU0_SPNIDEN_W<'a, O> {
    #[doc = "Any other value than b10: invasive debug is disable."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CPU0_SPNIDEN_A::DISABLE)
    }
    #[doc = "10: Invasive debug is enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CPU0_SPNIDEN_A::ENABLE)
    }
}
impl R {
    #[doc = "Bits 0:1 - CPU0 Invasive debug control:."]
    #[inline(always)]
    pub fn cpu0_dbgen(&self) -> CPU0_DBGEN_R {
        CPU0_DBGEN_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - CPU0 Non Invasive debug control:."]
    #[inline(always)]
    pub fn cpu0_niden(&self) -> CPU0_NIDEN_R {
        CPU0_NIDEN_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - CPU0 Secure Invasive debug control:."]
    #[inline(always)]
    pub fn cpu0_spiden(&self) -> CPU0_SPIDEN_R {
        CPU0_SPIDEN_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - CPU0 Secure Non Invasive debug control:."]
    #[inline(always)]
    pub fn cpu0_spniden(&self) -> CPU0_SPNIDEN_R {
        CPU0_SPNIDEN_R::new(((self.bits >> 6) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - CPU0 Invasive debug control:."]
    #[inline(always)]
    pub fn cpu0_dbgen(&mut self) -> CPU0_DBGEN_W<0> {
        CPU0_DBGEN_W::new(self)
    }
    #[doc = "Bits 2:3 - CPU0 Non Invasive debug control:."]
    #[inline(always)]
    pub fn cpu0_niden(&mut self) -> CPU0_NIDEN_W<2> {
        CPU0_NIDEN_W::new(self)
    }
    #[doc = "Bits 4:5 - CPU0 Secure Invasive debug control:."]
    #[inline(always)]
    pub fn cpu0_spiden(&mut self) -> CPU0_SPIDEN_W<4> {
        CPU0_SPIDEN_W::new(self)
    }
    #[doc = "Bits 6:7 - CPU0 Secure Non Invasive debug control:."]
    #[inline(always)]
    pub fn cpu0_spniden(&mut self) -> CPU0_SPNIDEN_W<6> {
        CPU0_SPNIDEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Cortex debug features control.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [debug_features](index.html) module"]
pub struct DEBUG_FEATURES_SPEC;
impl crate::RegisterSpec for DEBUG_FEATURES_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [debug_features::R](R) reader structure"]
impl crate::Readable for DEBUG_FEATURES_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [debug_features::W](W) writer structure"]
impl crate::Writable for DEBUG_FEATURES_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DEBUG_FEATURES to value 0"]
impl crate::Resettable for DEBUG_FEATURES_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
