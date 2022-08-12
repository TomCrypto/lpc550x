#[doc = "Register `GPO2_0` reader"]
pub struct R(crate::R<GPO2_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPO2_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPO2_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPO2_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPO2_0` writer"]
pub struct W(crate::W<GPO2_0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPO2_0_SPEC>;
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
impl From<crate::W<GPO2_0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPO2_0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USBHS_PHY_TRIM_VALID` reader - no description available."]
pub type USBHS_PHY_TRIM_VALID_R = crate::BitReader<bool>;
#[doc = "Field `USBHS_PHY_TRIM_VALID` writer - no description available."]
pub type USBHS_PHY_TRIM_VALID_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPO2_0_SPEC, bool, O>;
#[doc = "Field `TRIM_USB_REG_ENV_TAIL_ADJ_VD` reader - no description available."]
pub type TRIM_USB_REG_ENV_TAIL_ADJ_VD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TRIM_USB_REG_ENV_TAIL_ADJ_VD` writer - no description available."]
pub type TRIM_USB_REG_ENV_TAIL_ADJ_VD_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GPO2_0_SPEC, u8, u8, 2, O>;
#[doc = "Field `TRIM_USBPHY_TX_D_CAL` reader - no description available."]
pub type TRIM_USBPHY_TX_D_CAL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TRIM_USBPHY_TX_D_CAL` writer - no description available."]
pub type TRIM_USBPHY_TX_D_CAL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GPO2_0_SPEC, u8, u8, 4, O>;
#[doc = "Field `TRIM_USBPHY_TX_CAL45DP` reader - no description available."]
pub type TRIM_USBPHY_TX_CAL45DP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TRIM_USBPHY_TX_CAL45DP` writer - no description available."]
pub type TRIM_USBPHY_TX_CAL45DP_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GPO2_0_SPEC, u8, u8, 5, O>;
#[doc = "Field `TRIM_USBPHY_TX_CAL45DN` reader - no description available."]
pub type TRIM_USBPHY_TX_CAL45DN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TRIM_USBPHY_TX_CAL45DN` writer - no description available."]
pub type TRIM_USBPHY_TX_CAL45DN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GPO2_0_SPEC, u8, u8, 5, O>;
#[doc = "Field `TRIM_USB2_REFBIAS_TST` reader - no description available."]
pub type TRIM_USB2_REFBIAS_TST_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TRIM_USB2_REFBIAS_TST` writer - no description available."]
pub type TRIM_USB2_REFBIAS_TST_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GPO2_0_SPEC, u8, u8, 2, O>;
#[doc = "Field `TRIM_USB2_REFBIAS_VBGADJ` reader - no description available."]
pub type TRIM_USB2_REFBIAS_VBGADJ_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TRIM_USB2_REFBIAS_VBGADJ` writer - no description available."]
pub type TRIM_USB2_REFBIAS_VBGADJ_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GPO2_0_SPEC, u8, u8, 3, O>;
#[doc = "Field `TRIM_PLL_CTRL0_DIV_SEL` reader - no description available."]
pub type TRIM_PLL_CTRL0_DIV_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TRIM_PLL_CTRL0_DIV_SEL` writer - no description available."]
pub type TRIM_PLL_CTRL0_DIV_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GPO2_0_SPEC, u8, u8, 3, O>;
#[doc = "Field `FLASH_SIZE` reader - (For Niobe4) 000 : 640 KB 001 : 512 KB 010 : 256 KB 011 : 128 KB 100 : 0 KB All others : RESERVED (For Niobe4 Mini) FLASH_SIZE\\[2:0\\]
000 : 256 KB 001 : 128 KB 010 : 80 KB (reserved) 011 : 64 KB 100 : 0 kB (reserved) All others : RESERVED."]
pub type FLASH_SIZE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FLASH_SIZE` writer - (For Niobe4) 000 : 640 KB 001 : 512 KB 010 : 256 KB 011 : 128 KB 100 : 0 KB All others : RESERVED (For Niobe4 Mini) FLASH_SIZE\\[2:0\\]
000 : 256 KB 001 : 128 KB 010 : 80 KB (reserved) 011 : 64 KB 100 : 0 kB (reserved) All others : RESERVED."]
pub type FLASH_SIZE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GPO2_0_SPEC, u8, u8, 3, O>;
#[doc = "Field `CPU0_SECURITY_EXTENSION_DISABLE` reader - CPU0_SECURITY_EXTENSION_DISABLE\\[3:0\\]: 1010 : CPU0 Security Extension is disabled. All Other values: CPU0 Security Extension is enabled."]
pub type CPU0_SECURITY_EXTENSION_DISABLE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CPU0_SECURITY_EXTENSION_DISABLE` writer - CPU0_SECURITY_EXTENSION_DISABLE\\[3:0\\]: 1010 : CPU0 Security Extension is disabled. All Other values: CPU0 Security Extension is enabled."]
pub type CPU0_SECURITY_EXTENSION_DISABLE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GPO2_0_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bit 0 - no description available."]
    #[inline(always)]
    pub fn usbhs_phy_trim_valid(&self) -> USBHS_PHY_TRIM_VALID_R {
        USBHS_PHY_TRIM_VALID_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - no description available."]
    #[inline(always)]
    pub fn trim_usb_reg_env_tail_adj_vd(&self) -> TRIM_USB_REG_ENV_TAIL_ADJ_VD_R {
        TRIM_USB_REG_ENV_TAIL_ADJ_VD_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bits 3:6 - no description available."]
    #[inline(always)]
    pub fn trim_usbphy_tx_d_cal(&self) -> TRIM_USBPHY_TX_D_CAL_R {
        TRIM_USBPHY_TX_D_CAL_R::new(((self.bits >> 3) & 0x0f) as u8)
    }
    #[doc = "Bits 7:11 - no description available."]
    #[inline(always)]
    pub fn trim_usbphy_tx_cal45dp(&self) -> TRIM_USBPHY_TX_CAL45DP_R {
        TRIM_USBPHY_TX_CAL45DP_R::new(((self.bits >> 7) & 0x1f) as u8)
    }
    #[doc = "Bits 12:16 - no description available."]
    #[inline(always)]
    pub fn trim_usbphy_tx_cal45dn(&self) -> TRIM_USBPHY_TX_CAL45DN_R {
        TRIM_USBPHY_TX_CAL45DN_R::new(((self.bits >> 12) & 0x1f) as u8)
    }
    #[doc = "Bits 17:18 - no description available."]
    #[inline(always)]
    pub fn trim_usb2_refbias_tst(&self) -> TRIM_USB2_REFBIAS_TST_R {
        TRIM_USB2_REFBIAS_TST_R::new(((self.bits >> 17) & 3) as u8)
    }
    #[doc = "Bits 19:21 - no description available."]
    #[inline(always)]
    pub fn trim_usb2_refbias_vbgadj(&self) -> TRIM_USB2_REFBIAS_VBGADJ_R {
        TRIM_USB2_REFBIAS_VBGADJ_R::new(((self.bits >> 19) & 7) as u8)
    }
    #[doc = "Bits 22:24 - no description available."]
    #[inline(always)]
    pub fn trim_pll_ctrl0_div_sel(&self) -> TRIM_PLL_CTRL0_DIV_SEL_R {
        TRIM_PLL_CTRL0_DIV_SEL_R::new(((self.bits >> 22) & 7) as u8)
    }
    #[doc = "Bits 25:27 - (For Niobe4) 000 : 640 KB 001 : 512 KB 010 : 256 KB 011 : 128 KB 100 : 0 KB All others : RESERVED (For Niobe4 Mini) FLASH_SIZE\\[2:0\\]
000 : 256 KB 001 : 128 KB 010 : 80 KB (reserved) 011 : 64 KB 100 : 0 kB (reserved) All others : RESERVED."]
    #[inline(always)]
    pub fn flash_size(&self) -> FLASH_SIZE_R {
        FLASH_SIZE_R::new(((self.bits >> 25) & 7) as u8)
    }
    #[doc = "Bits 28:31 - CPU0_SECURITY_EXTENSION_DISABLE\\[3:0\\]: 1010 : CPU0 Security Extension is disabled. All Other values: CPU0 Security Extension is enabled."]
    #[inline(always)]
    pub fn cpu0_security_extension_disable(&self) -> CPU0_SECURITY_EXTENSION_DISABLE_R {
        CPU0_SECURITY_EXTENSION_DISABLE_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - no description available."]
    #[inline(always)]
    pub fn usbhs_phy_trim_valid(&mut self) -> USBHS_PHY_TRIM_VALID_W<0> {
        USBHS_PHY_TRIM_VALID_W::new(self)
    }
    #[doc = "Bits 1:2 - no description available."]
    #[inline(always)]
    pub fn trim_usb_reg_env_tail_adj_vd(&mut self) -> TRIM_USB_REG_ENV_TAIL_ADJ_VD_W<1> {
        TRIM_USB_REG_ENV_TAIL_ADJ_VD_W::new(self)
    }
    #[doc = "Bits 3:6 - no description available."]
    #[inline(always)]
    pub fn trim_usbphy_tx_d_cal(&mut self) -> TRIM_USBPHY_TX_D_CAL_W<3> {
        TRIM_USBPHY_TX_D_CAL_W::new(self)
    }
    #[doc = "Bits 7:11 - no description available."]
    #[inline(always)]
    pub fn trim_usbphy_tx_cal45dp(&mut self) -> TRIM_USBPHY_TX_CAL45DP_W<7> {
        TRIM_USBPHY_TX_CAL45DP_W::new(self)
    }
    #[doc = "Bits 12:16 - no description available."]
    #[inline(always)]
    pub fn trim_usbphy_tx_cal45dn(&mut self) -> TRIM_USBPHY_TX_CAL45DN_W<12> {
        TRIM_USBPHY_TX_CAL45DN_W::new(self)
    }
    #[doc = "Bits 17:18 - no description available."]
    #[inline(always)]
    pub fn trim_usb2_refbias_tst(&mut self) -> TRIM_USB2_REFBIAS_TST_W<17> {
        TRIM_USB2_REFBIAS_TST_W::new(self)
    }
    #[doc = "Bits 19:21 - no description available."]
    #[inline(always)]
    pub fn trim_usb2_refbias_vbgadj(&mut self) -> TRIM_USB2_REFBIAS_VBGADJ_W<19> {
        TRIM_USB2_REFBIAS_VBGADJ_W::new(self)
    }
    #[doc = "Bits 22:24 - no description available."]
    #[inline(always)]
    pub fn trim_pll_ctrl0_div_sel(&mut self) -> TRIM_PLL_CTRL0_DIV_SEL_W<22> {
        TRIM_PLL_CTRL0_DIV_SEL_W::new(self)
    }
    #[doc = "Bits 25:27 - (For Niobe4) 000 : 640 KB 001 : 512 KB 010 : 256 KB 011 : 128 KB 100 : 0 KB All others : RESERVED (For Niobe4 Mini) FLASH_SIZE\\[2:0\\]
000 : 256 KB 001 : 128 KB 010 : 80 KB (reserved) 011 : 64 KB 100 : 0 kB (reserved) All others : RESERVED."]
    #[inline(always)]
    pub fn flash_size(&mut self) -> FLASH_SIZE_W<25> {
        FLASH_SIZE_W::new(self)
    }
    #[doc = "Bits 28:31 - CPU0_SECURITY_EXTENSION_DISABLE\\[3:0\\]: 1010 : CPU0 Security Extension is disabled. All Other values: CPU0 Security Extension is enabled."]
    #[inline(always)]
    pub fn cpu0_security_extension_disable(&mut self) -> CPU0_SECURITY_EXTENSION_DISABLE_W<28> {
        CPU0_SECURITY_EXTENSION_DISABLE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPO2 register 0 description.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpo2_0](index.html) module"]
pub struct GPO2_0_SPEC;
impl crate::RegisterSpec for GPO2_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpo2_0::R](R) reader structure"]
impl crate::Readable for GPO2_0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpo2_0::W](W) writer structure"]
impl crate::Writable for GPO2_0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GPO2_0 to value 0"]
impl crate::Resettable for GPO2_0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
