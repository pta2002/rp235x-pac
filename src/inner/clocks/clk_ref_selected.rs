#[doc = "Register `CLK_REF_SELECTED` reader"]
pub type R = crate::R<CLK_REF_SELECTED_SPEC>;
#[doc = "Register `CLK_REF_SELECTED` writer"]
pub type W = crate::W<CLK_REF_SELECTED_SPEC>;
#[doc = "Field `CLK_REF_SELECTED` reader - The glitchless multiplexer does not switch instantaneously (to avoid glitches), so software should poll this register to wait for the switch to complete. This register contains one decoded bit for each of the clock sources enumerated in the CTRL SRC field. At most one of these bits will be set at any time, indicating that clock is currently present at the output of the glitchless mux. Whilst switching is in progress, this register may briefly show all-0s."]
pub type CLK_REF_SELECTED_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - The glitchless multiplexer does not switch instantaneously (to avoid glitches), so software should poll this register to wait for the switch to complete. This register contains one decoded bit for each of the clock sources enumerated in the CTRL SRC field. At most one of these bits will be set at any time, indicating that clock is currently present at the output of the glitchless mux. Whilst switching is in progress, this register may briefly show all-0s."]
    #[inline(always)]
    pub fn clk_ref_selected(&self) -> CLK_REF_SELECTED_R {
        CLK_REF_SELECTED_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {}
#[doc = "Indicates which src is currently selected (one-hot)  

You can [`read`](crate::Reg::read) this register and get [`clk_ref_selected::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk_ref_selected::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLK_REF_SELECTED_SPEC;
impl crate::RegisterSpec for CLK_REF_SELECTED_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk_ref_selected::R`](R) reader structure"]
impl crate::Readable for CLK_REF_SELECTED_SPEC {}
#[doc = "`write(|w| ..)` method takes [`clk_ref_selected::W`](W) writer structure"]
impl crate::Writable for CLK_REF_SELECTED_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLK_REF_SELECTED to value 0x01"]
impl crate::Resettable for CLK_REF_SELECTED_SPEC {
    const RESET_VALUE: u32 = 0x01;
}
