#[doc = "Register `ENTROPY_INJECT` reader"]
pub struct R(crate::R<ENTROPY_INJECT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ENTROPY_INJECT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ENTROPY_INJECT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ENTROPY_INJECT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ENTROPY_INJECT` writer"]
pub struct W(crate::W<ENTROPY_INJECT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ENTROPY_INJECT_SPEC>;
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
impl From<crate::W<ENTROPY_INJECT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ENTROPY_INJECT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENTROPY` reader - Use this register to inject or restore entropy 32 bits at a time. Writing is blocking thus see to it to have analog clocks activated. Injection can be usefull to add contribution from an external source of entropy, like for example LSBs of an ADC or of a temperature sensor. Restore can be usefull to store N random numbers in central memory before going powerdown then restore this entropy to RNG IP after power-up. It is useless to inject or restore more than 1*(number of RNGs) 32b words consecutively. Recommendation is to inject 1*(number of RNGs) words, and possibly later (2*32 clock cycles of slowest analog clock) inject again 1*(number of RNGs) words. Then maximum capacity of restoration is reached: about 44 bits per RNG (not to be mistaken with maximum capacity of entropy accumulation which is about 100 bits per RNG). You can inject less than 32 bits words (let unused bits to 0). Injection cannot degrade overall performance due to the fact that some internal PRNGs are excluded on purpose from this external action."]
pub type ENTROPY_R = crate::FieldReader<u32, u32>;
#[doc = "Field `ENTROPY` writer - Use this register to inject or restore entropy 32 bits at a time. Writing is blocking thus see to it to have analog clocks activated. Injection can be usefull to add contribution from an external source of entropy, like for example LSBs of an ADC or of a temperature sensor. Restore can be usefull to store N random numbers in central memory before going powerdown then restore this entropy to RNG IP after power-up. It is useless to inject or restore more than 1*(number of RNGs) 32b words consecutively. Recommendation is to inject 1*(number of RNGs) words, and possibly later (2*32 clock cycles of slowest analog clock) inject again 1*(number of RNGs) words. Then maximum capacity of restoration is reached: about 44 bits per RNG (not to be mistaken with maximum capacity of entropy accumulation which is about 100 bits per RNG). You can inject less than 32 bits words (let unused bits to 0). Injection cannot degrade overall performance due to the fact that some internal PRNGs are excluded on purpose from this external action."]
pub type ENTROPY_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ENTROPY_INJECT_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Use this register to inject or restore entropy 32 bits at a time. Writing is blocking thus see to it to have analog clocks activated. Injection can be usefull to add contribution from an external source of entropy, like for example LSBs of an ADC or of a temperature sensor. Restore can be usefull to store N random numbers in central memory before going powerdown then restore this entropy to RNG IP after power-up. It is useless to inject or restore more than 1*(number of RNGs) 32b words consecutively. Recommendation is to inject 1*(number of RNGs) words, and possibly later (2*32 clock cycles of slowest analog clock) inject again 1*(number of RNGs) words. Then maximum capacity of restoration is reached: about 44 bits per RNG (not to be mistaken with maximum capacity of entropy accumulation which is about 100 bits per RNG). You can inject less than 32 bits words (let unused bits to 0). Injection cannot degrade overall performance due to the fact that some internal PRNGs are excluded on purpose from this external action."]
    #[inline(always)]
    pub fn entropy(&self) -> ENTROPY_R {
        ENTROPY_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Use this register to inject or restore entropy 32 bits at a time. Writing is blocking thus see to it to have analog clocks activated. Injection can be usefull to add contribution from an external source of entropy, like for example LSBs of an ADC or of a temperature sensor. Restore can be usefull to store N random numbers in central memory before going powerdown then restore this entropy to RNG IP after power-up. It is useless to inject or restore more than 1*(number of RNGs) 32b words consecutively. Recommendation is to inject 1*(number of RNGs) words, and possibly later (2*32 clock cycles of slowest analog clock) inject again 1*(number of RNGs) words. Then maximum capacity of restoration is reached: about 44 bits per RNG (not to be mistaken with maximum capacity of entropy accumulation which is about 100 bits per RNG). You can inject less than 32 bits words (let unused bits to 0). Injection cannot degrade overall performance due to the fact that some internal PRNGs are excluded on purpose from this external action."]
    #[inline(always)]
    pub fn entropy(&mut self) -> ENTROPY_W<0> {
        ENTROPY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "no description available\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [entropy_inject](index.html) module"]
pub struct ENTROPY_INJECT_SPEC;
impl crate::RegisterSpec for ENTROPY_INJECT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [entropy_inject::R](R) reader structure"]
impl crate::Readable for ENTROPY_INJECT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [entropy_inject::W](W) writer structure"]
impl crate::Writable for ENTROPY_INJECT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ENTROPY_INJECT to value 0"]
impl crate::Resettable for ENTROPY_INJECT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
