#[doc = "Register `TRCCIDR1` reader"]
pub type R = crate::R<TRCCIDR1_SPEC>;
#[doc = "Register `TRCCIDR1` writer"]
pub type W = crate::W<TRCCIDR1_SPEC>;
#[doc = "Field `PRMBL_1` reader - reads as 0b0000"]
pub type PRMBL_1_R = crate::FieldReader;
#[doc = "Field `CLASS` reader - reads as 0b1001"]
pub type CLASS_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - reads as 0b0000"]
    #[inline(always)]
    pub fn prmbl_1(&self) -> PRMBL_1_R {
        PRMBL_1_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - reads as 0b1001"]
    #[inline(always)]
    pub fn class(&self) -> CLASS_R {
        CLASS_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {}
#[doc = "TRCCIDR1  

You can [`read`](crate::Reg::read) this register and get [`trccidr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trccidr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TRCCIDR1_SPEC;
impl crate::RegisterSpec for TRCCIDR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`trccidr1::R`](R) reader structure"]
impl crate::Readable for TRCCIDR1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`trccidr1::W`](W) writer structure"]
impl crate::Writable for TRCCIDR1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TRCCIDR1 to value 0x90"]
impl crate::Resettable for TRCCIDR1_SPEC {
    const RESET_VALUE: u32 = 0x90;
}
