#[doc = "Register `PRINCE_BASE_ADDR` reader"]
pub struct R(crate::R<PRINCE_BASE_ADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRINCE_BASE_ADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRINCE_BASE_ADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRINCE_BASE_ADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PRINCE_BASE_ADDR` writer"]
pub struct W(crate::W<PRINCE_BASE_ADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRINCE_BASE_ADDR_SPEC>;
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
impl From<crate::W<PRINCE_BASE_ADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRINCE_BASE_ADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADDR0_PRG` reader - Programmable portion of the base address of region 0."]
pub type ADDR0_PRG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADDR0_PRG` writer - Programmable portion of the base address of region 0."]
pub type ADDR0_PRG_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PRINCE_BASE_ADDR_SPEC, u8, u8, 4, O>;
#[doc = "Field `ADDR1_PRG` reader - Programmable portion of the base address of region 1."]
pub type ADDR1_PRG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADDR1_PRG` writer - Programmable portion of the base address of region 1."]
pub type ADDR1_PRG_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PRINCE_BASE_ADDR_SPEC, u8, u8, 4, O>;
#[doc = "Field `ADDR2_PRG` reader - Programmable portion of the base address of region 2."]
pub type ADDR2_PRG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADDR2_PRG` writer - Programmable portion of the base address of region 2."]
pub type ADDR2_PRG_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PRINCE_BASE_ADDR_SPEC, u8, u8, 4, O>;
#[doc = "Field `LOCK_REG0` reader - Lock PRINCE region0 settings."]
pub type LOCK_REG0_R = crate::FieldReader<u8, LOCK_REG0_A>;
#[doc = "Lock PRINCE region0 settings.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LOCK_REG0_A {
    #[doc = "0: Region is not locked."]
    UNLOCK = 0,
    #[doc = "1: Region is locked."]
    LOCK = 1,
    #[doc = "2: Region is locked."]
    VALUE_2 = 2,
    #[doc = "3: Region is locked."]
    VALUE_3 = 3,
}
impl From<LOCK_REG0_A> for u8 {
    #[inline(always)]
    fn from(variant: LOCK_REG0_A) -> Self {
        variant as _
    }
}
impl LOCK_REG0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOCK_REG0_A {
        match self.bits {
            0 => LOCK_REG0_A::UNLOCK,
            1 => LOCK_REG0_A::LOCK,
            2 => LOCK_REG0_A::VALUE_2,
            3 => LOCK_REG0_A::VALUE_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `UNLOCK`"]
    #[inline(always)]
    pub fn is_unlock(&self) -> bool {
        *self == LOCK_REG0_A::UNLOCK
    }
    #[doc = "Checks if the value of the field is `LOCK`"]
    #[inline(always)]
    pub fn is_lock(&self) -> bool {
        *self == LOCK_REG0_A::LOCK
    }
    #[doc = "Checks if the value of the field is `VALUE_2`"]
    #[inline(always)]
    pub fn is_value_2(&self) -> bool {
        *self == LOCK_REG0_A::VALUE_2
    }
    #[doc = "Checks if the value of the field is `VALUE_3`"]
    #[inline(always)]
    pub fn is_value_3(&self) -> bool {
        *self == LOCK_REG0_A::VALUE_3
    }
}
#[doc = "Field `LOCK_REG0` writer - Lock PRINCE region0 settings."]
pub type LOCK_REG0_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, PRINCE_BASE_ADDR_SPEC, u8, LOCK_REG0_A, 2, O>;
impl<'a, const O: u8> LOCK_REG0_W<'a, O> {
    #[doc = "Region is not locked."]
    #[inline(always)]
    pub fn unlock(self) -> &'a mut W {
        self.variant(LOCK_REG0_A::UNLOCK)
    }
    #[doc = "Region is locked."]
    #[inline(always)]
    pub fn lock(self) -> &'a mut W {
        self.variant(LOCK_REG0_A::LOCK)
    }
    #[doc = "Region is locked."]
    #[inline(always)]
    pub fn value_2(self) -> &'a mut W {
        self.variant(LOCK_REG0_A::VALUE_2)
    }
    #[doc = "Region is locked."]
    #[inline(always)]
    pub fn value_3(self) -> &'a mut W {
        self.variant(LOCK_REG0_A::VALUE_3)
    }
}
#[doc = "Field `LOCK_REG1` reader - Lock PRINCE region1 settings."]
pub type LOCK_REG1_R = crate::FieldReader<u8, LOCK_REG1_A>;
#[doc = "Lock PRINCE region1 settings.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LOCK_REG1_A {
    #[doc = "0: Region is not locked."]
    UNLOCK = 0,
    #[doc = "1: Region is locked."]
    LOCK = 1,
    #[doc = "2: Region is locked."]
    VALUE_2 = 2,
    #[doc = "3: Region is locked."]
    VALUE_3 = 3,
}
impl From<LOCK_REG1_A> for u8 {
    #[inline(always)]
    fn from(variant: LOCK_REG1_A) -> Self {
        variant as _
    }
}
impl LOCK_REG1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOCK_REG1_A {
        match self.bits {
            0 => LOCK_REG1_A::UNLOCK,
            1 => LOCK_REG1_A::LOCK,
            2 => LOCK_REG1_A::VALUE_2,
            3 => LOCK_REG1_A::VALUE_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `UNLOCK`"]
    #[inline(always)]
    pub fn is_unlock(&self) -> bool {
        *self == LOCK_REG1_A::UNLOCK
    }
    #[doc = "Checks if the value of the field is `LOCK`"]
    #[inline(always)]
    pub fn is_lock(&self) -> bool {
        *self == LOCK_REG1_A::LOCK
    }
    #[doc = "Checks if the value of the field is `VALUE_2`"]
    #[inline(always)]
    pub fn is_value_2(&self) -> bool {
        *self == LOCK_REG1_A::VALUE_2
    }
    #[doc = "Checks if the value of the field is `VALUE_3`"]
    #[inline(always)]
    pub fn is_value_3(&self) -> bool {
        *self == LOCK_REG1_A::VALUE_3
    }
}
#[doc = "Field `LOCK_REG1` writer - Lock PRINCE region1 settings."]
pub type LOCK_REG1_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, PRINCE_BASE_ADDR_SPEC, u8, LOCK_REG1_A, 2, O>;
impl<'a, const O: u8> LOCK_REG1_W<'a, O> {
    #[doc = "Region is not locked."]
    #[inline(always)]
    pub fn unlock(self) -> &'a mut W {
        self.variant(LOCK_REG1_A::UNLOCK)
    }
    #[doc = "Region is locked."]
    #[inline(always)]
    pub fn lock(self) -> &'a mut W {
        self.variant(LOCK_REG1_A::LOCK)
    }
    #[doc = "Region is locked."]
    #[inline(always)]
    pub fn value_2(self) -> &'a mut W {
        self.variant(LOCK_REG1_A::VALUE_2)
    }
    #[doc = "Region is locked."]
    #[inline(always)]
    pub fn value_3(self) -> &'a mut W {
        self.variant(LOCK_REG1_A::VALUE_3)
    }
}
#[doc = "Field `REG0_ERASE_CHECK_EN` reader - For PRINCE region0 enable checking whether all encrypted pages are erased together."]
pub type REG0_ERASE_CHECK_EN_R = crate::FieldReader<u8, REG0_ERASE_CHECK_EN_A>;
#[doc = "For PRINCE region0 enable checking whether all encrypted pages are erased together.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum REG0_ERASE_CHECK_EN_A {
    #[doc = "0: Region is disabled."]
    DISABLE = 0,
    #[doc = "1: Region is enabled."]
    ENABLE = 1,
    #[doc = "2: Region is enabled."]
    VALUE_2 = 2,
    #[doc = "3: Region is enabled."]
    VALUE_3 = 3,
}
impl From<REG0_ERASE_CHECK_EN_A> for u8 {
    #[inline(always)]
    fn from(variant: REG0_ERASE_CHECK_EN_A) -> Self {
        variant as _
    }
}
impl REG0_ERASE_CHECK_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG0_ERASE_CHECK_EN_A {
        match self.bits {
            0 => REG0_ERASE_CHECK_EN_A::DISABLE,
            1 => REG0_ERASE_CHECK_EN_A::ENABLE,
            2 => REG0_ERASE_CHECK_EN_A::VALUE_2,
            3 => REG0_ERASE_CHECK_EN_A::VALUE_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == REG0_ERASE_CHECK_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == REG0_ERASE_CHECK_EN_A::ENABLE
    }
    #[doc = "Checks if the value of the field is `VALUE_2`"]
    #[inline(always)]
    pub fn is_value_2(&self) -> bool {
        *self == REG0_ERASE_CHECK_EN_A::VALUE_2
    }
    #[doc = "Checks if the value of the field is `VALUE_3`"]
    #[inline(always)]
    pub fn is_value_3(&self) -> bool {
        *self == REG0_ERASE_CHECK_EN_A::VALUE_3
    }
}
#[doc = "Field `REG0_ERASE_CHECK_EN` writer - For PRINCE region0 enable checking whether all encrypted pages are erased together."]
pub type REG0_ERASE_CHECK_EN_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, PRINCE_BASE_ADDR_SPEC, u8, REG0_ERASE_CHECK_EN_A, 2, O>;
impl<'a, const O: u8> REG0_ERASE_CHECK_EN_W<'a, O> {
    #[doc = "Region is disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(REG0_ERASE_CHECK_EN_A::DISABLE)
    }
    #[doc = "Region is enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(REG0_ERASE_CHECK_EN_A::ENABLE)
    }
    #[doc = "Region is enabled."]
    #[inline(always)]
    pub fn value_2(self) -> &'a mut W {
        self.variant(REG0_ERASE_CHECK_EN_A::VALUE_2)
    }
    #[doc = "Region is enabled."]
    #[inline(always)]
    pub fn value_3(self) -> &'a mut W {
        self.variant(REG0_ERASE_CHECK_EN_A::VALUE_3)
    }
}
#[doc = "Field `REG1_ERASE_CHECK_EN` reader - For PRINCE region1 enable checking whether all encrypted pages are erased together."]
pub type REG1_ERASE_CHECK_EN_R = crate::FieldReader<u8, REG1_ERASE_CHECK_EN_A>;
#[doc = "For PRINCE region1 enable checking whether all encrypted pages are erased together.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum REG1_ERASE_CHECK_EN_A {
    #[doc = "0: Region is disabled."]
    DISABLE = 0,
    #[doc = "1: Region is enabled."]
    ENABLE = 1,
    #[doc = "2: Region is enabled."]
    VALUE_2 = 2,
    #[doc = "3: Region is enabled."]
    VALUE_3 = 3,
}
impl From<REG1_ERASE_CHECK_EN_A> for u8 {
    #[inline(always)]
    fn from(variant: REG1_ERASE_CHECK_EN_A) -> Self {
        variant as _
    }
}
impl REG1_ERASE_CHECK_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG1_ERASE_CHECK_EN_A {
        match self.bits {
            0 => REG1_ERASE_CHECK_EN_A::DISABLE,
            1 => REG1_ERASE_CHECK_EN_A::ENABLE,
            2 => REG1_ERASE_CHECK_EN_A::VALUE_2,
            3 => REG1_ERASE_CHECK_EN_A::VALUE_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == REG1_ERASE_CHECK_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == REG1_ERASE_CHECK_EN_A::ENABLE
    }
    #[doc = "Checks if the value of the field is `VALUE_2`"]
    #[inline(always)]
    pub fn is_value_2(&self) -> bool {
        *self == REG1_ERASE_CHECK_EN_A::VALUE_2
    }
    #[doc = "Checks if the value of the field is `VALUE_3`"]
    #[inline(always)]
    pub fn is_value_3(&self) -> bool {
        *self == REG1_ERASE_CHECK_EN_A::VALUE_3
    }
}
#[doc = "Field `REG1_ERASE_CHECK_EN` writer - For PRINCE region1 enable checking whether all encrypted pages are erased together."]
pub type REG1_ERASE_CHECK_EN_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, PRINCE_BASE_ADDR_SPEC, u8, REG1_ERASE_CHECK_EN_A, 2, O>;
impl<'a, const O: u8> REG1_ERASE_CHECK_EN_W<'a, O> {
    #[doc = "Region is disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(REG1_ERASE_CHECK_EN_A::DISABLE)
    }
    #[doc = "Region is enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(REG1_ERASE_CHECK_EN_A::ENABLE)
    }
    #[doc = "Region is enabled."]
    #[inline(always)]
    pub fn value_2(self) -> &'a mut W {
        self.variant(REG1_ERASE_CHECK_EN_A::VALUE_2)
    }
    #[doc = "Region is enabled."]
    #[inline(always)]
    pub fn value_3(self) -> &'a mut W {
        self.variant(REG1_ERASE_CHECK_EN_A::VALUE_3)
    }
}
#[doc = "Field `REG2_ERASE_CHECK_EN` reader - For PRINCE region2 enable checking whether all encrypted pages are erased together."]
pub type REG2_ERASE_CHECK_EN_R = crate::FieldReader<u8, REG2_ERASE_CHECK_EN_A>;
#[doc = "For PRINCE region2 enable checking whether all encrypted pages are erased together.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum REG2_ERASE_CHECK_EN_A {
    #[doc = "0: Region is disabled."]
    DISABLE = 0,
    #[doc = "1: Region is enabled."]
    ENABLE = 1,
    #[doc = "2: Region is enabled."]
    VALUE_2 = 2,
    #[doc = "3: Region is enabled."]
    VALUE_3 = 3,
}
impl From<REG2_ERASE_CHECK_EN_A> for u8 {
    #[inline(always)]
    fn from(variant: REG2_ERASE_CHECK_EN_A) -> Self {
        variant as _
    }
}
impl REG2_ERASE_CHECK_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG2_ERASE_CHECK_EN_A {
        match self.bits {
            0 => REG2_ERASE_CHECK_EN_A::DISABLE,
            1 => REG2_ERASE_CHECK_EN_A::ENABLE,
            2 => REG2_ERASE_CHECK_EN_A::VALUE_2,
            3 => REG2_ERASE_CHECK_EN_A::VALUE_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == REG2_ERASE_CHECK_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == REG2_ERASE_CHECK_EN_A::ENABLE
    }
    #[doc = "Checks if the value of the field is `VALUE_2`"]
    #[inline(always)]
    pub fn is_value_2(&self) -> bool {
        *self == REG2_ERASE_CHECK_EN_A::VALUE_2
    }
    #[doc = "Checks if the value of the field is `VALUE_3`"]
    #[inline(always)]
    pub fn is_value_3(&self) -> bool {
        *self == REG2_ERASE_CHECK_EN_A::VALUE_3
    }
}
#[doc = "Field `REG2_ERASE_CHECK_EN` writer - For PRINCE region2 enable checking whether all encrypted pages are erased together."]
pub type REG2_ERASE_CHECK_EN_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, PRINCE_BASE_ADDR_SPEC, u8, REG2_ERASE_CHECK_EN_A, 2, O>;
impl<'a, const O: u8> REG2_ERASE_CHECK_EN_W<'a, O> {
    #[doc = "Region is disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(REG2_ERASE_CHECK_EN_A::DISABLE)
    }
    #[doc = "Region is enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(REG2_ERASE_CHECK_EN_A::ENABLE)
    }
    #[doc = "Region is enabled."]
    #[inline(always)]
    pub fn value_2(self) -> &'a mut W {
        self.variant(REG2_ERASE_CHECK_EN_A::VALUE_2)
    }
    #[doc = "Region is enabled."]
    #[inline(always)]
    pub fn value_3(self) -> &'a mut W {
        self.variant(REG2_ERASE_CHECK_EN_A::VALUE_3)
    }
}
impl R {
    #[doc = "Bits 0:3 - Programmable portion of the base address of region 0."]
    #[inline(always)]
    pub fn addr0_prg(&self) -> ADDR0_PRG_R {
        ADDR0_PRG_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Programmable portion of the base address of region 1."]
    #[inline(always)]
    pub fn addr1_prg(&self) -> ADDR1_PRG_R {
        ADDR1_PRG_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Programmable portion of the base address of region 2."]
    #[inline(always)]
    pub fn addr2_prg(&self) -> ADDR2_PRG_R {
        ADDR2_PRG_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 18:19 - Lock PRINCE region0 settings."]
    #[inline(always)]
    pub fn lock_reg0(&self) -> LOCK_REG0_R {
        LOCK_REG0_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Lock PRINCE region1 settings."]
    #[inline(always)]
    pub fn lock_reg1(&self) -> LOCK_REG1_R {
        LOCK_REG1_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 24:25 - For PRINCE region0 enable checking whether all encrypted pages are erased together."]
    #[inline(always)]
    pub fn reg0_erase_check_en(&self) -> REG0_ERASE_CHECK_EN_R {
        REG0_ERASE_CHECK_EN_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - For PRINCE region1 enable checking whether all encrypted pages are erased together."]
    #[inline(always)]
    pub fn reg1_erase_check_en(&self) -> REG1_ERASE_CHECK_EN_R {
        REG1_ERASE_CHECK_EN_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - For PRINCE region2 enable checking whether all encrypted pages are erased together."]
    #[inline(always)]
    pub fn reg2_erase_check_en(&self) -> REG2_ERASE_CHECK_EN_R {
        REG2_ERASE_CHECK_EN_R::new(((self.bits >> 28) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Programmable portion of the base address of region 0."]
    #[inline(always)]
    pub fn addr0_prg(&mut self) -> ADDR0_PRG_W<0> {
        ADDR0_PRG_W::new(self)
    }
    #[doc = "Bits 4:7 - Programmable portion of the base address of region 1."]
    #[inline(always)]
    pub fn addr1_prg(&mut self) -> ADDR1_PRG_W<4> {
        ADDR1_PRG_W::new(self)
    }
    #[doc = "Bits 8:11 - Programmable portion of the base address of region 2."]
    #[inline(always)]
    pub fn addr2_prg(&mut self) -> ADDR2_PRG_W<8> {
        ADDR2_PRG_W::new(self)
    }
    #[doc = "Bits 18:19 - Lock PRINCE region0 settings."]
    #[inline(always)]
    pub fn lock_reg0(&mut self) -> LOCK_REG0_W<18> {
        LOCK_REG0_W::new(self)
    }
    #[doc = "Bits 20:21 - Lock PRINCE region1 settings."]
    #[inline(always)]
    pub fn lock_reg1(&mut self) -> LOCK_REG1_W<20> {
        LOCK_REG1_W::new(self)
    }
    #[doc = "Bits 24:25 - For PRINCE region0 enable checking whether all encrypted pages are erased together."]
    #[inline(always)]
    pub fn reg0_erase_check_en(&mut self) -> REG0_ERASE_CHECK_EN_W<24> {
        REG0_ERASE_CHECK_EN_W::new(self)
    }
    #[doc = "Bits 26:27 - For PRINCE region1 enable checking whether all encrypted pages are erased together."]
    #[inline(always)]
    pub fn reg1_erase_check_en(&mut self) -> REG1_ERASE_CHECK_EN_W<26> {
        REG1_ERASE_CHECK_EN_W::new(self)
    }
    #[doc = "Bits 28:29 - For PRINCE region2 enable checking whether all encrypted pages are erased together."]
    #[inline(always)]
    pub fn reg2_erase_check_en(&mut self) -> REG2_ERASE_CHECK_EN_W<28> {
        REG2_ERASE_CHECK_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "no description available.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prince_base_addr](index.html) module"]
pub struct PRINCE_BASE_ADDR_SPEC;
impl crate::RegisterSpec for PRINCE_BASE_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [prince_base_addr::R](R) reader structure"]
impl crate::Readable for PRINCE_BASE_ADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [prince_base_addr::W](W) writer structure"]
impl crate::Writable for PRINCE_BASE_ADDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PRINCE_BASE_ADDR to value 0"]
impl crate::Resettable for PRINCE_BASE_ADDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
