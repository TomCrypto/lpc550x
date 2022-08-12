#[doc = "Register `AUX_BIAS_CURVE_TEMP_1` reader"]
pub struct R(crate::R<AUX_BIAS_CURVE_TEMP_1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AUX_BIAS_CURVE_TEMP_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AUX_BIAS_CURVE_TEMP_1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AUX_BIAS_CURVE_TEMP_1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AUX_BIAS_CURVE_TEMP_1` writer"]
pub struct W(crate::W<AUX_BIAS_CURVE_TEMP_1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AUX_BIAS_CURVE_TEMP_1_SPEC>;
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
impl From<crate::W<AUX_BIAS_CURVE_TEMP_1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AUX_BIAS_CURVE_TEMP_1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VREF1VCURVETRIM_2` reader - VREF1VCURVETRIM_2 (unit: 100uV)"]
pub type VREF1VCURVETRIM_2_R = crate::FieldReader<u16, u16>;
#[doc = "Field `VREF1VCURVETRIM_2` writer - VREF1VCURVETRIM_2 (unit: 100uV)"]
pub type VREF1VCURVETRIM_2_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, AUX_BIAS_CURVE_TEMP_1_SPEC, u16, u16, 16, O>;
#[doc = "Field `VREF1VCURVETRIM_3` reader - VREF1VCURVETRIM_3 (unit: 100uV)"]
pub type VREF1VCURVETRIM_3_R = crate::FieldReader<u16, u16>;
#[doc = "Field `VREF1VCURVETRIM_3` writer - VREF1VCURVETRIM_3 (unit: 100uV)"]
pub type VREF1VCURVETRIM_3_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, AUX_BIAS_CURVE_TEMP_1_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - VREF1VCURVETRIM_2 (unit: 100uV)"]
    #[inline(always)]
    pub fn vref1vcurvetrim_2(&self) -> VREF1VCURVETRIM_2_R {
        VREF1VCURVETRIM_2_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - VREF1VCURVETRIM_3 (unit: 100uV)"]
    #[inline(always)]
    pub fn vref1vcurvetrim_3(&self) -> VREF1VCURVETRIM_3_R {
        VREF1VCURVETRIM_3_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - VREF1VCURVETRIM_2 (unit: 100uV)"]
    #[inline(always)]
    pub fn vref1vcurvetrim_2(&mut self) -> VREF1VCURVETRIM_2_W<0> {
        VREF1VCURVETRIM_2_W::new(self)
    }
    #[doc = "Bits 16:31 - VREF1VCURVETRIM_3 (unit: 100uV)"]
    #[inline(always)]
    pub fn vref1vcurvetrim_3(&mut self) -> VREF1VCURVETRIM_3_W<16> {
        VREF1VCURVETRIM_3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Aux Bias Curve TEMP (105degC)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aux_bias_curve_temp_1](index.html) module"]
pub struct AUX_BIAS_CURVE_TEMP_1_SPEC;
impl crate::RegisterSpec for AUX_BIAS_CURVE_TEMP_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [aux_bias_curve_temp_1::R](R) reader structure"]
impl crate::Readable for AUX_BIAS_CURVE_TEMP_1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [aux_bias_curve_temp_1::W](W) writer structure"]
impl crate::Writable for AUX_BIAS_CURVE_TEMP_1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AUX_BIAS_CURVE_TEMP_1 to value 0"]
impl crate::Resettable for AUX_BIAS_CURVE_TEMP_1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
