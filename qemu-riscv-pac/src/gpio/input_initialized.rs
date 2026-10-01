#[doc = "Register `INPUT_INITIALIZED` reader"]
pub type R = crate::R<InputInitializedSpec>;
#[doc = "Field `INPUT_INITIALIZED` reader - Input level has been explicitly initialized"]
pub type InputInitializedR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Input level has been explicitly initialized"]
    #[inline(always)]
    pub fn input_initialized(&self) -> InputInitializedR {
        InputInitializedR::new(self.bits)
    }
}
#[doc = "Initialized external input mask (one bit per pin)\n\nYou can [`read`](crate::Reg::read) this register and get [`input_initialized::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct InputInitializedSpec;
impl crate::RegisterSpec for InputInitializedSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`input_initialized::R`](R) reader structure"]
impl crate::Readable for InputInitializedSpec {}
#[doc = "`reset()` method sets INPUT_INITIALIZED to value 0"]
impl crate::Resettable for InputInitializedSpec {}
