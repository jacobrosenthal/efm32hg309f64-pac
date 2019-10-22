#[doc = "Reader of register COMP0"]
pub type R = crate::R<u32, super::COMP0>;
#[doc = "Writer for register COMP0"]
pub type W = crate::W<u32, super::COMP0>;
#[doc = "Register COMP0 `reset()`'s with value 0"]
impl crate::ResetValue for super::COMP0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `COMP0`"]
pub type COMP0_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `COMP0`"]
pub struct COMP0_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x00ff_ffff) | ((value as u32) & 0x00ff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:23 - Compare Value 0"]
    #[inline(always)]
    pub fn comp0(&self) -> COMP0_R {
        COMP0_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:23 - Compare Value 0"]
    #[inline(always)]
    pub fn comp0(&mut self) -> COMP0_W {
        COMP0_W { w: self }
    }
}
