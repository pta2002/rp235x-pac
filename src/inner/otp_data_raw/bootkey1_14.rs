#[doc = "Register `BOOTKEY1_14` reader"]
pub type R = crate::R<BOOTKEY1_14_SPEC>;
#[doc = "Register `BOOTKEY1_14` writer"]
pub type W = crate::W<BOOTKEY1_14_SPEC>;
#[doc = "Field `BOOTKEY1_14` reader - "]
pub type BOOTKEY1_14_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn bootkey1_14(&self) -> BOOTKEY1_14_R {
        BOOTKEY1_14_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {}
#[doc = "Bits 239:224 of SHA-256 hash of boot key 1 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`bootkey1_14::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bootkey1_14::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BOOTKEY1_14_SPEC;
impl crate::RegisterSpec for BOOTKEY1_14_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bootkey1_14::R`](R) reader structure"]
impl crate::Readable for BOOTKEY1_14_SPEC {}
#[doc = "`write(|w| ..)` method takes [`bootkey1_14::W`](W) writer structure"]
impl crate::Writable for BOOTKEY1_14_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BOOTKEY1_14 to value 0"]
impl crate::Resettable for BOOTKEY1_14_SPEC {
    const RESET_VALUE: u32 = 0;
}
