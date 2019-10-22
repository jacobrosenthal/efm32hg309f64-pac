#[doc = "Reader of register DIEPTXF3"]
pub type R = crate::R<u32, super::DIEPTXF3>;
#[doc = "Writer for register DIEPTXF3"]
pub type W = crate::W<u32, super::DIEPTXF3>;
#[doc = "Register DIEPTXF3 `reset()`'s with value 0x0200_0800"]
impl crate::ResetValue for super::DIEPTXF3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0200_0800
    }
}
#[doc = "Reader of field `INEPNTXFSTADDR`"]
pub type INEPNTXFSTADDR_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `INEPNTXFSTADDR`"]
pub struct INEPNTXFSTADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> INEPNTXFSTADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
#[doc = "Reader of field `INEPNTXFDEP`"]
pub type INEPNTXFDEP_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `INEPNTXFDEP`"]
pub struct INEPNTXFDEP_W<'a> {
    w: &'a mut W,
}
impl<'a> INEPNTXFDEP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 16)) | (((value as u32) & 0x03ff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:11 - IN Endpoint FIFO 3 Transmit RAM Start Address"]
    #[inline(always)]
    pub fn inepntxfstaddr(&self) -> INEPNTXFSTADDR_R {
        INEPNTXFSTADDR_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:25 - IN Endpoint TxFIFO Depth"]
    #[inline(always)]
    pub fn inepntxfdep(&self) -> INEPNTXFDEP_R {
        INEPNTXFDEP_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - IN Endpoint FIFO 3 Transmit RAM Start Address"]
    #[inline(always)]
    pub fn inepntxfstaddr(&mut self) -> INEPNTXFSTADDR_W {
        INEPNTXFSTADDR_W { w: self }
    }
    #[doc = "Bits 16:25 - IN Endpoint TxFIFO Depth"]
    #[inline(always)]
    pub fn inepntxfdep(&mut self) -> INEPNTXFDEP_W {
        INEPNTXFDEP_W { w: self }
    }
}
