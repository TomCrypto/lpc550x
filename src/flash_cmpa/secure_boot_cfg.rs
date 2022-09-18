#[doc = "Register `SECURE_BOOT_CFG` reader"]
pub struct R(crate::R<SECURE_BOOT_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SECURE_BOOT_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SECURE_BOOT_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SECURE_BOOT_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SECURE_BOOT_CFG` writer"]
pub struct W(crate::W<SECURE_BOOT_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SECURE_BOOT_CFG_SPEC>;
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
impl From<crate::W<SECURE_BOOT_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SECURE_BOOT_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RSA4K` reader - Use RSA4096 keys only."]
pub type RSA4K_R = crate::FieldReader<u8, RSA4K_A>;
#[doc = "Use RSA4096 keys only.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RSA4K_A {
    #[doc = "0: Allow RSA2048 and higher."]
    VALUE_0 = 0,
    #[doc = "1: RSA4096 only."]
    VALUE_1 = 1,
    #[doc = "2: RSA4096 only."]
    VALUE_2 = 2,
    #[doc = "3: RSA4096 only."]
    VALUE_3 = 3,
}
impl From<RSA4K_A> for u8 {
    #[inline(always)]
    fn from(variant: RSA4K_A) -> Self {
        variant as _
    }
}
impl RSA4K_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RSA4K_A {
        match self.bits {
            0 => RSA4K_A::VALUE_0,
            1 => RSA4K_A::VALUE_1,
            2 => RSA4K_A::VALUE_2,
            3 => RSA4K_A::VALUE_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE_0`"]
    #[inline(always)]
    pub fn is_value_0(&self) -> bool {
        *self == RSA4K_A::VALUE_0
    }
    #[doc = "Checks if the value of the field is `VALUE_1`"]
    #[inline(always)]
    pub fn is_value_1(&self) -> bool {
        *self == RSA4K_A::VALUE_1
    }
    #[doc = "Checks if the value of the field is `VALUE_2`"]
    #[inline(always)]
    pub fn is_value_2(&self) -> bool {
        *self == RSA4K_A::VALUE_2
    }
    #[doc = "Checks if the value of the field is `VALUE_3`"]
    #[inline(always)]
    pub fn is_value_3(&self) -> bool {
        *self == RSA4K_A::VALUE_3
    }
}
#[doc = "Field `RSA4K` writer - Use RSA4096 keys only."]
pub type RSA4K_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, SECURE_BOOT_CFG_SPEC, u8, RSA4K_A, 2, O>;
impl<'a, const O: u8> RSA4K_W<'a, O> {
    #[doc = "Allow RSA2048 and higher."]
    #[inline(always)]
    pub fn value_0(self) -> &'a mut W {
        self.variant(RSA4K_A::VALUE_0)
    }
    #[doc = "RSA4096 only."]
    #[inline(always)]
    pub fn value_1(self) -> &'a mut W {
        self.variant(RSA4K_A::VALUE_1)
    }
    #[doc = "RSA4096 only."]
    #[inline(always)]
    pub fn value_2(self) -> &'a mut W {
        self.variant(RSA4K_A::VALUE_2)
    }
    #[doc = "RSA4096 only."]
    #[inline(always)]
    pub fn value_3(self) -> &'a mut W {
        self.variant(RSA4K_A::VALUE_3)
    }
}
#[doc = "Field `DICE_INC_NXP_CFG` reader - Include NXP area in DICE computation."]
pub type DICE_INC_NXP_CFG_R = crate::FieldReader<u8, DICE_INC_NXP_CFG_A>;
#[doc = "Include NXP area in DICE computation.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DICE_INC_NXP_CFG_A {
    #[doc = "0: not included."]
    NOT_INCLUD = 0,
    #[doc = "1: included."]
    INCLUD = 1,
    #[doc = "2: included."]
    VALUE_2 = 2,
    #[doc = "3: included."]
    VALUE_3 = 3,
}
impl From<DICE_INC_NXP_CFG_A> for u8 {
    #[inline(always)]
    fn from(variant: DICE_INC_NXP_CFG_A) -> Self {
        variant as _
    }
}
impl DICE_INC_NXP_CFG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DICE_INC_NXP_CFG_A {
        match self.bits {
            0 => DICE_INC_NXP_CFG_A::NOT_INCLUD,
            1 => DICE_INC_NXP_CFG_A::INCLUD,
            2 => DICE_INC_NXP_CFG_A::VALUE_2,
            3 => DICE_INC_NXP_CFG_A::VALUE_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NOT_INCLUD`"]
    #[inline(always)]
    pub fn is_not_includ(&self) -> bool {
        *self == DICE_INC_NXP_CFG_A::NOT_INCLUD
    }
    #[doc = "Checks if the value of the field is `INCLUD`"]
    #[inline(always)]
    pub fn is_includ(&self) -> bool {
        *self == DICE_INC_NXP_CFG_A::INCLUD
    }
    #[doc = "Checks if the value of the field is `VALUE_2`"]
    #[inline(always)]
    pub fn is_value_2(&self) -> bool {
        *self == DICE_INC_NXP_CFG_A::VALUE_2
    }
    #[doc = "Checks if the value of the field is `VALUE_3`"]
    #[inline(always)]
    pub fn is_value_3(&self) -> bool {
        *self == DICE_INC_NXP_CFG_A::VALUE_3
    }
}
#[doc = "Field `DICE_INC_NXP_CFG` writer - Include NXP area in DICE computation."]
pub type DICE_INC_NXP_CFG_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, SECURE_BOOT_CFG_SPEC, u8, DICE_INC_NXP_CFG_A, 2, O>;
impl<'a, const O: u8> DICE_INC_NXP_CFG_W<'a, O> {
    #[doc = "not included."]
    #[inline(always)]
    pub fn not_includ(self) -> &'a mut W {
        self.variant(DICE_INC_NXP_CFG_A::NOT_INCLUD)
    }
    #[doc = "included."]
    #[inline(always)]
    pub fn includ(self) -> &'a mut W {
        self.variant(DICE_INC_NXP_CFG_A::INCLUD)
    }
    #[doc = "included."]
    #[inline(always)]
    pub fn value_2(self) -> &'a mut W {
        self.variant(DICE_INC_NXP_CFG_A::VALUE_2)
    }
    #[doc = "included."]
    #[inline(always)]
    pub fn value_3(self) -> &'a mut W {
        self.variant(DICE_INC_NXP_CFG_A::VALUE_3)
    }
}
#[doc = "Field `DICE_CUST_CFG` reader - Include Customer factory area (including keys) in DICE computation."]
pub type DICE_CUST_CFG_R = crate::FieldReader<u8, DICE_CUST_CFG_A>;
#[doc = "Include Customer factory area (including keys) in DICE computation.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DICE_CUST_CFG_A {
    #[doc = "0: not included."]
    NOT_INCLUD = 0,
    #[doc = "1: included."]
    UNCLUD = 1,
    #[doc = "2: included."]
    VALUE_2 = 2,
    #[doc = "3: included."]
    VALUE_3 = 3,
}
impl From<DICE_CUST_CFG_A> for u8 {
    #[inline(always)]
    fn from(variant: DICE_CUST_CFG_A) -> Self {
        variant as _
    }
}
impl DICE_CUST_CFG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DICE_CUST_CFG_A {
        match self.bits {
            0 => DICE_CUST_CFG_A::NOT_INCLUD,
            1 => DICE_CUST_CFG_A::UNCLUD,
            2 => DICE_CUST_CFG_A::VALUE_2,
            3 => DICE_CUST_CFG_A::VALUE_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NOT_INCLUD`"]
    #[inline(always)]
    pub fn is_not_includ(&self) -> bool {
        *self == DICE_CUST_CFG_A::NOT_INCLUD
    }
    #[doc = "Checks if the value of the field is `UNCLUD`"]
    #[inline(always)]
    pub fn is_unclud(&self) -> bool {
        *self == DICE_CUST_CFG_A::UNCLUD
    }
    #[doc = "Checks if the value of the field is `VALUE_2`"]
    #[inline(always)]
    pub fn is_value_2(&self) -> bool {
        *self == DICE_CUST_CFG_A::VALUE_2
    }
    #[doc = "Checks if the value of the field is `VALUE_3`"]
    #[inline(always)]
    pub fn is_value_3(&self) -> bool {
        *self == DICE_CUST_CFG_A::VALUE_3
    }
}
#[doc = "Field `DICE_CUST_CFG` writer - Include Customer factory area (including keys) in DICE computation."]
pub type DICE_CUST_CFG_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, SECURE_BOOT_CFG_SPEC, u8, DICE_CUST_CFG_A, 2, O>;
impl<'a, const O: u8> DICE_CUST_CFG_W<'a, O> {
    #[doc = "not included."]
    #[inline(always)]
    pub fn not_includ(self) -> &'a mut W {
        self.variant(DICE_CUST_CFG_A::NOT_INCLUD)
    }
    #[doc = "included."]
    #[inline(always)]
    pub fn unclud(self) -> &'a mut W {
        self.variant(DICE_CUST_CFG_A::UNCLUD)
    }
    #[doc = "included."]
    #[inline(always)]
    pub fn value_2(self) -> &'a mut W {
        self.variant(DICE_CUST_CFG_A::VALUE_2)
    }
    #[doc = "included."]
    #[inline(always)]
    pub fn value_3(self) -> &'a mut W {
        self.variant(DICE_CUST_CFG_A::VALUE_3)
    }
}
#[doc = "Field `SKIP_DICE` reader - Skip DICE computation."]
pub type SKIP_DICE_R = crate::FieldReader<u8, SKIP_DICE_A>;
#[doc = "Skip DICE computation.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SKIP_DICE_A {
    #[doc = "0: Enable DICE."]
    ENABLE = 0,
    #[doc = "1: Disable DICE."]
    DISABLE = 1,
    #[doc = "2: Disable DICE."]
    VALUE_2 = 2,
    #[doc = "3: Disable DICE."]
    VALUE_3 = 3,
}
impl From<SKIP_DICE_A> for u8 {
    #[inline(always)]
    fn from(variant: SKIP_DICE_A) -> Self {
        variant as _
    }
}
impl SKIP_DICE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SKIP_DICE_A {
        match self.bits {
            0 => SKIP_DICE_A::ENABLE,
            1 => SKIP_DICE_A::DISABLE,
            2 => SKIP_DICE_A::VALUE_2,
            3 => SKIP_DICE_A::VALUE_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == SKIP_DICE_A::ENABLE
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == SKIP_DICE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `VALUE_2`"]
    #[inline(always)]
    pub fn is_value_2(&self) -> bool {
        *self == SKIP_DICE_A::VALUE_2
    }
    #[doc = "Checks if the value of the field is `VALUE_3`"]
    #[inline(always)]
    pub fn is_value_3(&self) -> bool {
        *self == SKIP_DICE_A::VALUE_3
    }
}
#[doc = "Field `SKIP_DICE` writer - Skip DICE computation."]
pub type SKIP_DICE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, SECURE_BOOT_CFG_SPEC, u8, SKIP_DICE_A, 2, O>;
impl<'a, const O: u8> SKIP_DICE_W<'a, O> {
    #[doc = "Enable DICE."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SKIP_DICE_A::ENABLE)
    }
    #[doc = "Disable DICE."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SKIP_DICE_A::DISABLE)
    }
    #[doc = "Disable DICE."]
    #[inline(always)]
    pub fn value_2(self) -> &'a mut W {
        self.variant(SKIP_DICE_A::VALUE_2)
    }
    #[doc = "Disable DICE."]
    #[inline(always)]
    pub fn value_3(self) -> &'a mut W {
        self.variant(SKIP_DICE_A::VALUE_3)
    }
}
#[doc = "Field `TZM_IMAGE_TYPE` reader - TrustZone-M mode."]
pub type TZM_IMAGE_TYPE_R = crate::FieldReader<u8, TZM_IMAGE_TYPE_A>;
#[doc = "TrustZone-M mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TZM_IMAGE_TYPE_A {
    #[doc = "0: TZ-M image mode is taken from application image header."]
    VALUE_0 = 0,
    #[doc = "1: TZ-M disabled image, boots to non-secure mode."]
    VALUE_1 = 1,
    #[doc = "2: TZ-M enabled image, boots to secure mode."]
    VALUE_2 = 2,
    #[doc = "3: TZ-M enabled image with TZ-M preset, boot to secure mode TZ-M pre-configured by data from application image header."]
    VALUE_3 = 3,
}
impl From<TZM_IMAGE_TYPE_A> for u8 {
    #[inline(always)]
    fn from(variant: TZM_IMAGE_TYPE_A) -> Self {
        variant as _
    }
}
impl TZM_IMAGE_TYPE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TZM_IMAGE_TYPE_A {
        match self.bits {
            0 => TZM_IMAGE_TYPE_A::VALUE_0,
            1 => TZM_IMAGE_TYPE_A::VALUE_1,
            2 => TZM_IMAGE_TYPE_A::VALUE_2,
            3 => TZM_IMAGE_TYPE_A::VALUE_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE_0`"]
    #[inline(always)]
    pub fn is_value_0(&self) -> bool {
        *self == TZM_IMAGE_TYPE_A::VALUE_0
    }
    #[doc = "Checks if the value of the field is `VALUE_1`"]
    #[inline(always)]
    pub fn is_value_1(&self) -> bool {
        *self == TZM_IMAGE_TYPE_A::VALUE_1
    }
    #[doc = "Checks if the value of the field is `VALUE_2`"]
    #[inline(always)]
    pub fn is_value_2(&self) -> bool {
        *self == TZM_IMAGE_TYPE_A::VALUE_2
    }
    #[doc = "Checks if the value of the field is `VALUE_3`"]
    #[inline(always)]
    pub fn is_value_3(&self) -> bool {
        *self == TZM_IMAGE_TYPE_A::VALUE_3
    }
}
#[doc = "Field `TZM_IMAGE_TYPE` writer - TrustZone-M mode."]
pub type TZM_IMAGE_TYPE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, SECURE_BOOT_CFG_SPEC, u8, TZM_IMAGE_TYPE_A, 2, O>;
impl<'a, const O: u8> TZM_IMAGE_TYPE_W<'a, O> {
    #[doc = "TZ-M image mode is taken from application image header."]
    #[inline(always)]
    pub fn value_0(self) -> &'a mut W {
        self.variant(TZM_IMAGE_TYPE_A::VALUE_0)
    }
    #[doc = "TZ-M disabled image, boots to non-secure mode."]
    #[inline(always)]
    pub fn value_1(self) -> &'a mut W {
        self.variant(TZM_IMAGE_TYPE_A::VALUE_1)
    }
    #[doc = "TZ-M enabled image, boots to secure mode."]
    #[inline(always)]
    pub fn value_2(self) -> &'a mut W {
        self.variant(TZM_IMAGE_TYPE_A::VALUE_2)
    }
    #[doc = "TZ-M enabled image with TZ-M preset, boot to secure mode TZ-M pre-configured by data from application image header."]
    #[inline(always)]
    pub fn value_3(self) -> &'a mut W {
        self.variant(TZM_IMAGE_TYPE_A::VALUE_3)
    }
}
#[doc = "Field `BLOCK_SET_KEY` reader - Block PUF key code generation."]
pub type BLOCK_SET_KEY_R = crate::FieldReader<u8, BLOCK_SET_KEY_A>;
#[doc = "Block PUF key code generation.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BLOCK_SET_KEY_A {
    #[doc = "0: Allow PUF Key Code generation."]
    ALLOW = 0,
    #[doc = "1: Disable PUF Key Code generation."]
    DISABLE = 1,
    #[doc = "2: Disable PUF Key Code generation."]
    VALUE_2 = 2,
    #[doc = "3: Disable PUF Key Code generation."]
    VALUE_3 = 3,
}
impl From<BLOCK_SET_KEY_A> for u8 {
    #[inline(always)]
    fn from(variant: BLOCK_SET_KEY_A) -> Self {
        variant as _
    }
}
impl BLOCK_SET_KEY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BLOCK_SET_KEY_A {
        match self.bits {
            0 => BLOCK_SET_KEY_A::ALLOW,
            1 => BLOCK_SET_KEY_A::DISABLE,
            2 => BLOCK_SET_KEY_A::VALUE_2,
            3 => BLOCK_SET_KEY_A::VALUE_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ALLOW`"]
    #[inline(always)]
    pub fn is_allow(&self) -> bool {
        *self == BLOCK_SET_KEY_A::ALLOW
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == BLOCK_SET_KEY_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `VALUE_2`"]
    #[inline(always)]
    pub fn is_value_2(&self) -> bool {
        *self == BLOCK_SET_KEY_A::VALUE_2
    }
    #[doc = "Checks if the value of the field is `VALUE_3`"]
    #[inline(always)]
    pub fn is_value_3(&self) -> bool {
        *self == BLOCK_SET_KEY_A::VALUE_3
    }
}
#[doc = "Field `BLOCK_SET_KEY` writer - Block PUF key code generation."]
pub type BLOCK_SET_KEY_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, SECURE_BOOT_CFG_SPEC, u8, BLOCK_SET_KEY_A, 2, O>;
impl<'a, const O: u8> BLOCK_SET_KEY_W<'a, O> {
    #[doc = "Allow PUF Key Code generation."]
    #[inline(always)]
    pub fn allow(self) -> &'a mut W {
        self.variant(BLOCK_SET_KEY_A::ALLOW)
    }
    #[doc = "Disable PUF Key Code generation."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(BLOCK_SET_KEY_A::DISABLE)
    }
    #[doc = "Disable PUF Key Code generation."]
    #[inline(always)]
    pub fn value_2(self) -> &'a mut W {
        self.variant(BLOCK_SET_KEY_A::VALUE_2)
    }
    #[doc = "Disable PUF Key Code generation."]
    #[inline(always)]
    pub fn value_3(self) -> &'a mut W {
        self.variant(BLOCK_SET_KEY_A::VALUE_3)
    }
}
#[doc = "Field `BLOCK_ENROLL` reader - Block PUF enrollement."]
pub type BLOCK_ENROLL_R = crate::FieldReader<u8, BLOCK_ENROLL_A>;
#[doc = "Block PUF enrollement.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BLOCK_ENROLL_A {
    #[doc = "0: Allow PUF enroll operation."]
    ALLOW = 0,
    #[doc = "1: Disable PUF enroll operation."]
    DISABLE = 1,
    #[doc = "2: Disable PUF enroll operation."]
    VALUE_2 = 2,
    #[doc = "3: Disable PUF enroll operation."]
    VALUE_3 = 3,
}
impl From<BLOCK_ENROLL_A> for u8 {
    #[inline(always)]
    fn from(variant: BLOCK_ENROLL_A) -> Self {
        variant as _
    }
}
impl BLOCK_ENROLL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BLOCK_ENROLL_A {
        match self.bits {
            0 => BLOCK_ENROLL_A::ALLOW,
            1 => BLOCK_ENROLL_A::DISABLE,
            2 => BLOCK_ENROLL_A::VALUE_2,
            3 => BLOCK_ENROLL_A::VALUE_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ALLOW`"]
    #[inline(always)]
    pub fn is_allow(&self) -> bool {
        *self == BLOCK_ENROLL_A::ALLOW
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == BLOCK_ENROLL_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `VALUE_2`"]
    #[inline(always)]
    pub fn is_value_2(&self) -> bool {
        *self == BLOCK_ENROLL_A::VALUE_2
    }
    #[doc = "Checks if the value of the field is `VALUE_3`"]
    #[inline(always)]
    pub fn is_value_3(&self) -> bool {
        *self == BLOCK_ENROLL_A::VALUE_3
    }
}
#[doc = "Field `BLOCK_ENROLL` writer - Block PUF enrollement."]
pub type BLOCK_ENROLL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, SECURE_BOOT_CFG_SPEC, u8, BLOCK_ENROLL_A, 2, O>;
impl<'a, const O: u8> BLOCK_ENROLL_W<'a, O> {
    #[doc = "Allow PUF enroll operation."]
    #[inline(always)]
    pub fn allow(self) -> &'a mut W {
        self.variant(BLOCK_ENROLL_A::ALLOW)
    }
    #[doc = "Disable PUF enroll operation."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(BLOCK_ENROLL_A::DISABLE)
    }
    #[doc = "Disable PUF enroll operation."]
    #[inline(always)]
    pub fn value_2(self) -> &'a mut W {
        self.variant(BLOCK_ENROLL_A::VALUE_2)
    }
    #[doc = "Disable PUF enroll operation."]
    #[inline(always)]
    pub fn value_3(self) -> &'a mut W {
        self.variant(BLOCK_ENROLL_A::VALUE_3)
    }
}
#[doc = "Field `DICE_INC_SEC_EPOCH` reader - Include security EPOCH in DICE."]
pub type DICE_INC_SEC_EPOCH_R = crate::FieldReader<u8, DICE_INC_SEC_EPOCH_A>;
#[doc = "Include security EPOCH in DICE.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DICE_INC_SEC_EPOCH_A {
    #[doc = "0: not included."]
    NOT_INCLUD = 0,
    #[doc = "1: included."]
    INCLUD = 1,
    #[doc = "2: included."]
    VALUE_2 = 2,
    #[doc = "3: included."]
    VALUE_3 = 3,
}
impl From<DICE_INC_SEC_EPOCH_A> for u8 {
    #[inline(always)]
    fn from(variant: DICE_INC_SEC_EPOCH_A) -> Self {
        variant as _
    }
}
impl DICE_INC_SEC_EPOCH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DICE_INC_SEC_EPOCH_A {
        match self.bits {
            0 => DICE_INC_SEC_EPOCH_A::NOT_INCLUD,
            1 => DICE_INC_SEC_EPOCH_A::INCLUD,
            2 => DICE_INC_SEC_EPOCH_A::VALUE_2,
            3 => DICE_INC_SEC_EPOCH_A::VALUE_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NOT_INCLUD`"]
    #[inline(always)]
    pub fn is_not_includ(&self) -> bool {
        *self == DICE_INC_SEC_EPOCH_A::NOT_INCLUD
    }
    #[doc = "Checks if the value of the field is `INCLUD`"]
    #[inline(always)]
    pub fn is_includ(&self) -> bool {
        *self == DICE_INC_SEC_EPOCH_A::INCLUD
    }
    #[doc = "Checks if the value of the field is `VALUE_2`"]
    #[inline(always)]
    pub fn is_value_2(&self) -> bool {
        *self == DICE_INC_SEC_EPOCH_A::VALUE_2
    }
    #[doc = "Checks if the value of the field is `VALUE_3`"]
    #[inline(always)]
    pub fn is_value_3(&self) -> bool {
        *self == DICE_INC_SEC_EPOCH_A::VALUE_3
    }
}
#[doc = "Field `DICE_INC_SEC_EPOCH` writer - Include security EPOCH in DICE."]
pub type DICE_INC_SEC_EPOCH_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, SECURE_BOOT_CFG_SPEC, u8, DICE_INC_SEC_EPOCH_A, 2, O>;
impl<'a, const O: u8> DICE_INC_SEC_EPOCH_W<'a, O> {
    #[doc = "not included."]
    #[inline(always)]
    pub fn not_includ(self) -> &'a mut W {
        self.variant(DICE_INC_SEC_EPOCH_A::NOT_INCLUD)
    }
    #[doc = "included."]
    #[inline(always)]
    pub fn includ(self) -> &'a mut W {
        self.variant(DICE_INC_SEC_EPOCH_A::INCLUD)
    }
    #[doc = "included."]
    #[inline(always)]
    pub fn value_2(self) -> &'a mut W {
        self.variant(DICE_INC_SEC_EPOCH_A::VALUE_2)
    }
    #[doc = "included."]
    #[inline(always)]
    pub fn value_3(self) -> &'a mut W {
        self.variant(DICE_INC_SEC_EPOCH_A::VALUE_3)
    }
}
#[doc = "Field `SKIP_BOOT_SEED` reader - Skip boot seed computation."]
pub type SKIP_BOOT_SEED_R = crate::FieldReader<u8, SKIP_BOOT_SEED_A>;
#[doc = "Skip boot seed computation.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SKIP_BOOT_SEED_A {
    #[doc = "0: Enable BOOT_SEED."]
    ENABLE = 0,
    #[doc = "1: Disable BOOT_SEED."]
    DISABLE = 1,
    #[doc = "2: Disable BOOT_SEED."]
    VALUE_2 = 2,
    #[doc = "3: Disable BOOT_SEED."]
    VALUE_3 = 3,
}
impl From<SKIP_BOOT_SEED_A> for u8 {
    #[inline(always)]
    fn from(variant: SKIP_BOOT_SEED_A) -> Self {
        variant as _
    }
}
impl SKIP_BOOT_SEED_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SKIP_BOOT_SEED_A {
        match self.bits {
            0 => SKIP_BOOT_SEED_A::ENABLE,
            1 => SKIP_BOOT_SEED_A::DISABLE,
            2 => SKIP_BOOT_SEED_A::VALUE_2,
            3 => SKIP_BOOT_SEED_A::VALUE_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == SKIP_BOOT_SEED_A::ENABLE
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == SKIP_BOOT_SEED_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `VALUE_2`"]
    #[inline(always)]
    pub fn is_value_2(&self) -> bool {
        *self == SKIP_BOOT_SEED_A::VALUE_2
    }
    #[doc = "Checks if the value of the field is `VALUE_3`"]
    #[inline(always)]
    pub fn is_value_3(&self) -> bool {
        *self == SKIP_BOOT_SEED_A::VALUE_3
    }
}
#[doc = "Field `SKIP_BOOT_SEED` writer - Skip boot seed computation."]
pub type SKIP_BOOT_SEED_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, SECURE_BOOT_CFG_SPEC, u8, SKIP_BOOT_SEED_A, 2, O>;
impl<'a, const O: u8> SKIP_BOOT_SEED_W<'a, O> {
    #[doc = "Enable BOOT_SEED."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SKIP_BOOT_SEED_A::ENABLE)
    }
    #[doc = "Disable BOOT_SEED."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SKIP_BOOT_SEED_A::DISABLE)
    }
    #[doc = "Disable BOOT_SEED."]
    #[inline(always)]
    pub fn value_2(self) -> &'a mut W {
        self.variant(SKIP_BOOT_SEED_A::VALUE_2)
    }
    #[doc = "Disable BOOT_SEED."]
    #[inline(always)]
    pub fn value_3(self) -> &'a mut W {
        self.variant(SKIP_BOOT_SEED_A::VALUE_3)
    }
}
#[doc = "Field `BOOT_SEED_INC_NXP_CFG` reader - Include NXP area in BOOT SEED computation."]
pub type BOOT_SEED_INC_NXP_CFG_R = crate::FieldReader<u8, BOOT_SEED_INC_NXP_CFG_A>;
#[doc = "Include NXP area in BOOT SEED computation.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BOOT_SEED_INC_NXP_CFG_A {
    #[doc = "0: not included."]
    NOT_INCLUD = 0,
    #[doc = "1: included."]
    INCLUD = 1,
    #[doc = "2: included."]
    VALUE_2 = 2,
    #[doc = "3: included."]
    VALUE_3 = 3,
}
impl From<BOOT_SEED_INC_NXP_CFG_A> for u8 {
    #[inline(always)]
    fn from(variant: BOOT_SEED_INC_NXP_CFG_A) -> Self {
        variant as _
    }
}
impl BOOT_SEED_INC_NXP_CFG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BOOT_SEED_INC_NXP_CFG_A {
        match self.bits {
            0 => BOOT_SEED_INC_NXP_CFG_A::NOT_INCLUD,
            1 => BOOT_SEED_INC_NXP_CFG_A::INCLUD,
            2 => BOOT_SEED_INC_NXP_CFG_A::VALUE_2,
            3 => BOOT_SEED_INC_NXP_CFG_A::VALUE_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NOT_INCLUD`"]
    #[inline(always)]
    pub fn is_not_includ(&self) -> bool {
        *self == BOOT_SEED_INC_NXP_CFG_A::NOT_INCLUD
    }
    #[doc = "Checks if the value of the field is `INCLUD`"]
    #[inline(always)]
    pub fn is_includ(&self) -> bool {
        *self == BOOT_SEED_INC_NXP_CFG_A::INCLUD
    }
    #[doc = "Checks if the value of the field is `VALUE_2`"]
    #[inline(always)]
    pub fn is_value_2(&self) -> bool {
        *self == BOOT_SEED_INC_NXP_CFG_A::VALUE_2
    }
    #[doc = "Checks if the value of the field is `VALUE_3`"]
    #[inline(always)]
    pub fn is_value_3(&self) -> bool {
        *self == BOOT_SEED_INC_NXP_CFG_A::VALUE_3
    }
}
#[doc = "Field `BOOT_SEED_INC_NXP_CFG` writer - Include NXP area in BOOT SEED computation."]
pub type BOOT_SEED_INC_NXP_CFG_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, SECURE_BOOT_CFG_SPEC, u8, BOOT_SEED_INC_NXP_CFG_A, 2, O>;
impl<'a, const O: u8> BOOT_SEED_INC_NXP_CFG_W<'a, O> {
    #[doc = "not included."]
    #[inline(always)]
    pub fn not_includ(self) -> &'a mut W {
        self.variant(BOOT_SEED_INC_NXP_CFG_A::NOT_INCLUD)
    }
    #[doc = "included."]
    #[inline(always)]
    pub fn includ(self) -> &'a mut W {
        self.variant(BOOT_SEED_INC_NXP_CFG_A::INCLUD)
    }
    #[doc = "included."]
    #[inline(always)]
    pub fn value_2(self) -> &'a mut W {
        self.variant(BOOT_SEED_INC_NXP_CFG_A::VALUE_2)
    }
    #[doc = "included."]
    #[inline(always)]
    pub fn value_3(self) -> &'a mut W {
        self.variant(BOOT_SEED_INC_NXP_CFG_A::VALUE_3)
    }
}
#[doc = "Field `BOOT_SEED_CUST_CFG` reader - Include CMPA area in BOOT SEED computation."]
pub type BOOT_SEED_CUST_CFG_R = crate::FieldReader<u8, BOOT_SEED_CUST_CFG_A>;
#[doc = "Include CMPA area in BOOT SEED computation.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BOOT_SEED_CUST_CFG_A {
    #[doc = "0: not included."]
    NOT_INCLUD = 0,
    #[doc = "1: included."]
    INCLUD = 1,
    #[doc = "2: included."]
    VALUE_2 = 2,
    #[doc = "3: included."]
    VALUE_3 = 3,
}
impl From<BOOT_SEED_CUST_CFG_A> for u8 {
    #[inline(always)]
    fn from(variant: BOOT_SEED_CUST_CFG_A) -> Self {
        variant as _
    }
}
impl BOOT_SEED_CUST_CFG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BOOT_SEED_CUST_CFG_A {
        match self.bits {
            0 => BOOT_SEED_CUST_CFG_A::NOT_INCLUD,
            1 => BOOT_SEED_CUST_CFG_A::INCLUD,
            2 => BOOT_SEED_CUST_CFG_A::VALUE_2,
            3 => BOOT_SEED_CUST_CFG_A::VALUE_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NOT_INCLUD`"]
    #[inline(always)]
    pub fn is_not_includ(&self) -> bool {
        *self == BOOT_SEED_CUST_CFG_A::NOT_INCLUD
    }
    #[doc = "Checks if the value of the field is `INCLUD`"]
    #[inline(always)]
    pub fn is_includ(&self) -> bool {
        *self == BOOT_SEED_CUST_CFG_A::INCLUD
    }
    #[doc = "Checks if the value of the field is `VALUE_2`"]
    #[inline(always)]
    pub fn is_value_2(&self) -> bool {
        *self == BOOT_SEED_CUST_CFG_A::VALUE_2
    }
    #[doc = "Checks if the value of the field is `VALUE_3`"]
    #[inline(always)]
    pub fn is_value_3(&self) -> bool {
        *self == BOOT_SEED_CUST_CFG_A::VALUE_3
    }
}
#[doc = "Field `BOOT_SEED_CUST_CFG` writer - Include CMPA area in BOOT SEED computation."]
pub type BOOT_SEED_CUST_CFG_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, SECURE_BOOT_CFG_SPEC, u8, BOOT_SEED_CUST_CFG_A, 2, O>;
impl<'a, const O: u8> BOOT_SEED_CUST_CFG_W<'a, O> {
    #[doc = "not included."]
    #[inline(always)]
    pub fn not_includ(self) -> &'a mut W {
        self.variant(BOOT_SEED_CUST_CFG_A::NOT_INCLUD)
    }
    #[doc = "included."]
    #[inline(always)]
    pub fn includ(self) -> &'a mut W {
        self.variant(BOOT_SEED_CUST_CFG_A::INCLUD)
    }
    #[doc = "included."]
    #[inline(always)]
    pub fn value_2(self) -> &'a mut W {
        self.variant(BOOT_SEED_CUST_CFG_A::VALUE_2)
    }
    #[doc = "included."]
    #[inline(always)]
    pub fn value_3(self) -> &'a mut W {
        self.variant(BOOT_SEED_CUST_CFG_A::VALUE_3)
    }
}
#[doc = "Field `BOOT_SEED_INC_EPOCH` reader - Include security epoch area in BOOT_SEED computation."]
pub type BOOT_SEED_INC_EPOCH_R = crate::FieldReader<u8, BOOT_SEED_INC_EPOCH_A>;
#[doc = "Include security epoch area in BOOT_SEED computation.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BOOT_SEED_INC_EPOCH_A {
    #[doc = "0: not included."]
    NOT_INCLUD = 0,
    #[doc = "1: included."]
    INCLUD = 1,
    #[doc = "2: included."]
    VALUE_2 = 2,
    #[doc = "3: included."]
    VALUE_3 = 3,
}
impl From<BOOT_SEED_INC_EPOCH_A> for u8 {
    #[inline(always)]
    fn from(variant: BOOT_SEED_INC_EPOCH_A) -> Self {
        variant as _
    }
}
impl BOOT_SEED_INC_EPOCH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BOOT_SEED_INC_EPOCH_A {
        match self.bits {
            0 => BOOT_SEED_INC_EPOCH_A::NOT_INCLUD,
            1 => BOOT_SEED_INC_EPOCH_A::INCLUD,
            2 => BOOT_SEED_INC_EPOCH_A::VALUE_2,
            3 => BOOT_SEED_INC_EPOCH_A::VALUE_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NOT_INCLUD`"]
    #[inline(always)]
    pub fn is_not_includ(&self) -> bool {
        *self == BOOT_SEED_INC_EPOCH_A::NOT_INCLUD
    }
    #[doc = "Checks if the value of the field is `INCLUD`"]
    #[inline(always)]
    pub fn is_includ(&self) -> bool {
        *self == BOOT_SEED_INC_EPOCH_A::INCLUD
    }
    #[doc = "Checks if the value of the field is `VALUE_2`"]
    #[inline(always)]
    pub fn is_value_2(&self) -> bool {
        *self == BOOT_SEED_INC_EPOCH_A::VALUE_2
    }
    #[doc = "Checks if the value of the field is `VALUE_3`"]
    #[inline(always)]
    pub fn is_value_3(&self) -> bool {
        *self == BOOT_SEED_INC_EPOCH_A::VALUE_3
    }
}
#[doc = "Field `BOOT_SEED_INC_EPOCH` writer - Include security epoch area in BOOT_SEED computation."]
pub type BOOT_SEED_INC_EPOCH_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, SECURE_BOOT_CFG_SPEC, u8, BOOT_SEED_INC_EPOCH_A, 2, O>;
impl<'a, const O: u8> BOOT_SEED_INC_EPOCH_W<'a, O> {
    #[doc = "not included."]
    #[inline(always)]
    pub fn not_includ(self) -> &'a mut W {
        self.variant(BOOT_SEED_INC_EPOCH_A::NOT_INCLUD)
    }
    #[doc = "included."]
    #[inline(always)]
    pub fn includ(self) -> &'a mut W {
        self.variant(BOOT_SEED_INC_EPOCH_A::INCLUD)
    }
    #[doc = "included."]
    #[inline(always)]
    pub fn value_2(self) -> &'a mut W {
        self.variant(BOOT_SEED_INC_EPOCH_A::VALUE_2)
    }
    #[doc = "included."]
    #[inline(always)]
    pub fn value_3(self) -> &'a mut W {
        self.variant(BOOT_SEED_INC_EPOCH_A::VALUE_3)
    }
}
#[doc = "Field `SEC_BOOT_EN` reader - Secure boot enable."]
pub type SEC_BOOT_EN_R = crate::FieldReader<u8, SEC_BOOT_EN_A>;
#[doc = "Secure boot enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SEC_BOOT_EN_A {
    #[doc = "0: Plain image (internal flash with or without CRC)"]
    DISABLE = 0,
    #[doc = "1: Boot signed images. (internal flash, RSA signed)"]
    ENABLE = 1,
    #[doc = "2: Boot signed images. (internal flash, RSA signed)"]
    VALUE_2 = 2,
    #[doc = "3: Boot signed images. (internal flash, RSA signed)"]
    VALUE_3 = 3,
}
impl From<SEC_BOOT_EN_A> for u8 {
    #[inline(always)]
    fn from(variant: SEC_BOOT_EN_A) -> Self {
        variant as _
    }
}
impl SEC_BOOT_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SEC_BOOT_EN_A {
        match self.bits {
            0 => SEC_BOOT_EN_A::DISABLE,
            1 => SEC_BOOT_EN_A::ENABLE,
            2 => SEC_BOOT_EN_A::VALUE_2,
            3 => SEC_BOOT_EN_A::VALUE_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == SEC_BOOT_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == SEC_BOOT_EN_A::ENABLE
    }
    #[doc = "Checks if the value of the field is `VALUE_2`"]
    #[inline(always)]
    pub fn is_value_2(&self) -> bool {
        *self == SEC_BOOT_EN_A::VALUE_2
    }
    #[doc = "Checks if the value of the field is `VALUE_3`"]
    #[inline(always)]
    pub fn is_value_3(&self) -> bool {
        *self == SEC_BOOT_EN_A::VALUE_3
    }
}
#[doc = "Field `SEC_BOOT_EN` writer - Secure boot enable."]
pub type SEC_BOOT_EN_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, SECURE_BOOT_CFG_SPEC, u8, SEC_BOOT_EN_A, 2, O>;
impl<'a, const O: u8> SEC_BOOT_EN_W<'a, O> {
    #[doc = "Plain image (internal flash with or without CRC)"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SEC_BOOT_EN_A::DISABLE)
    }
    #[doc = "Boot signed images. (internal flash, RSA signed)"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SEC_BOOT_EN_A::ENABLE)
    }
    #[doc = "Boot signed images. (internal flash, RSA signed)"]
    #[inline(always)]
    pub fn value_2(self) -> &'a mut W {
        self.variant(SEC_BOOT_EN_A::VALUE_2)
    }
    #[doc = "Boot signed images. (internal flash, RSA signed)"]
    #[inline(always)]
    pub fn value_3(self) -> &'a mut W {
        self.variant(SEC_BOOT_EN_A::VALUE_3)
    }
}
impl R {
    #[doc = "Bits 0:1 - Use RSA4096 keys only."]
    #[inline(always)]
    pub fn rsa4k(&self) -> RSA4K_R {
        RSA4K_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Include NXP area in DICE computation."]
    #[inline(always)]
    pub fn dice_inc_nxp_cfg(&self) -> DICE_INC_NXP_CFG_R {
        DICE_INC_NXP_CFG_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Include Customer factory area (including keys) in DICE computation."]
    #[inline(always)]
    pub fn dice_cust_cfg(&self) -> DICE_CUST_CFG_R {
        DICE_CUST_CFG_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Skip DICE computation."]
    #[inline(always)]
    pub fn skip_dice(&self) -> SKIP_DICE_R {
        SKIP_DICE_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - TrustZone-M mode."]
    #[inline(always)]
    pub fn tzm_image_type(&self) -> TZM_IMAGE_TYPE_R {
        TZM_IMAGE_TYPE_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Block PUF key code generation."]
    #[inline(always)]
    pub fn block_set_key(&self) -> BLOCK_SET_KEY_R {
        BLOCK_SET_KEY_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Block PUF enrollement."]
    #[inline(always)]
    pub fn block_enroll(&self) -> BLOCK_ENROLL_R {
        BLOCK_ENROLL_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Include security EPOCH in DICE."]
    #[inline(always)]
    pub fn dice_inc_sec_epoch(&self) -> DICE_INC_SEC_EPOCH_R {
        DICE_INC_SEC_EPOCH_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Skip boot seed computation."]
    #[inline(always)]
    pub fn skip_boot_seed(&self) -> SKIP_BOOT_SEED_R {
        SKIP_BOOT_SEED_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Include NXP area in BOOT SEED computation."]
    #[inline(always)]
    pub fn boot_seed_inc_nxp_cfg(&self) -> BOOT_SEED_INC_NXP_CFG_R {
        BOOT_SEED_INC_NXP_CFG_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Include CMPA area in BOOT SEED computation."]
    #[inline(always)]
    pub fn boot_seed_cust_cfg(&self) -> BOOT_SEED_CUST_CFG_R {
        BOOT_SEED_CUST_CFG_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Include security epoch area in BOOT_SEED computation."]
    #[inline(always)]
    pub fn boot_seed_inc_epoch(&self) -> BOOT_SEED_INC_EPOCH_R {
        BOOT_SEED_INC_EPOCH_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 30:31 - Secure boot enable."]
    #[inline(always)]
    pub fn sec_boot_en(&self) -> SEC_BOOT_EN_R {
        SEC_BOOT_EN_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Use RSA4096 keys only."]
    #[inline(always)]
    pub fn rsa4k(&mut self) -> RSA4K_W<0> {
        RSA4K_W::new(self)
    }
    #[doc = "Bits 2:3 - Include NXP area in DICE computation."]
    #[inline(always)]
    pub fn dice_inc_nxp_cfg(&mut self) -> DICE_INC_NXP_CFG_W<2> {
        DICE_INC_NXP_CFG_W::new(self)
    }
    #[doc = "Bits 4:5 - Include Customer factory area (including keys) in DICE computation."]
    #[inline(always)]
    pub fn dice_cust_cfg(&mut self) -> DICE_CUST_CFG_W<4> {
        DICE_CUST_CFG_W::new(self)
    }
    #[doc = "Bits 6:7 - Skip DICE computation."]
    #[inline(always)]
    pub fn skip_dice(&mut self) -> SKIP_DICE_W<6> {
        SKIP_DICE_W::new(self)
    }
    #[doc = "Bits 8:9 - TrustZone-M mode."]
    #[inline(always)]
    pub fn tzm_image_type(&mut self) -> TZM_IMAGE_TYPE_W<8> {
        TZM_IMAGE_TYPE_W::new(self)
    }
    #[doc = "Bits 10:11 - Block PUF key code generation."]
    #[inline(always)]
    pub fn block_set_key(&mut self) -> BLOCK_SET_KEY_W<10> {
        BLOCK_SET_KEY_W::new(self)
    }
    #[doc = "Bits 12:13 - Block PUF enrollement."]
    #[inline(always)]
    pub fn block_enroll(&mut self) -> BLOCK_ENROLL_W<12> {
        BLOCK_ENROLL_W::new(self)
    }
    #[doc = "Bits 14:15 - Include security EPOCH in DICE."]
    #[inline(always)]
    pub fn dice_inc_sec_epoch(&mut self) -> DICE_INC_SEC_EPOCH_W<14> {
        DICE_INC_SEC_EPOCH_W::new(self)
    }
    #[doc = "Bits 16:17 - Skip boot seed computation."]
    #[inline(always)]
    pub fn skip_boot_seed(&mut self) -> SKIP_BOOT_SEED_W<16> {
        SKIP_BOOT_SEED_W::new(self)
    }
    #[doc = "Bits 18:19 - Include NXP area in BOOT SEED computation."]
    #[inline(always)]
    pub fn boot_seed_inc_nxp_cfg(&mut self) -> BOOT_SEED_INC_NXP_CFG_W<18> {
        BOOT_SEED_INC_NXP_CFG_W::new(self)
    }
    #[doc = "Bits 20:21 - Include CMPA area in BOOT SEED computation."]
    #[inline(always)]
    pub fn boot_seed_cust_cfg(&mut self) -> BOOT_SEED_CUST_CFG_W<20> {
        BOOT_SEED_CUST_CFG_W::new(self)
    }
    #[doc = "Bits 22:23 - Include security epoch area in BOOT_SEED computation."]
    #[inline(always)]
    pub fn boot_seed_inc_epoch(&mut self) -> BOOT_SEED_INC_EPOCH_W<22> {
        BOOT_SEED_INC_EPOCH_W::new(self)
    }
    #[doc = "Bits 30:31 - Secure boot enable."]
    #[inline(always)]
    pub fn sec_boot_en(&mut self) -> SEC_BOOT_EN_W<30> {
        SEC_BOOT_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Secure boot configuration flags.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [secure_boot_cfg](index.html) module"]
pub struct SECURE_BOOT_CFG_SPEC;
impl crate::RegisterSpec for SECURE_BOOT_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [secure_boot_cfg::R](R) reader structure"]
impl crate::Readable for SECURE_BOOT_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [secure_boot_cfg::W](W) writer structure"]
impl crate::Writable for SECURE_BOOT_CFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SECURE_BOOT_CFG to value 0"]
impl crate::Resettable for SECURE_BOOT_CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
