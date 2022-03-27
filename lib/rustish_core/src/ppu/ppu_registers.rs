#[repr(u8)]
#[derive(Clone, Copy, PartialEq)]
pub enum PPUMode {
  HBlank     = 0,
  VBlank     = 1,
  OamSearch  = 2,
  PxTransfer = 3,
}

impl Default for PPUMode {
  fn default() -> Self { Self::HBlank }
}

#[derive(Default, Clone, Copy)]
pub struct LCDC {
  pub enable_bg: bool,
  pub enable_obj: bool,      
  pub obj_size: bool,         // 0: 8x8, 1: 8x16
  pub bg_tilemap_addr: bool,  // 0: 0x9800-0x9BFF, 1: 0x9C00-0x9FFF
  pub bg_tiledata_addr: bool, // 0: 0x8800-0x97FF, 1: 0x8000-0x87FF
  pub enable_win: bool,
  pub win_tilemap_addr: bool, // 0: 0x9800-0x9BFF, 1: 0x9C00-0x9FFF
  pub enable_display: bool,
}
impl LCDC {
  pub fn from_u8(val: u8) -> Self {
    let mut new = Self::default();
    new.set_from_u8(val);
    return new;
  }
  pub fn set_from_u8(&mut self, val: u8) {
    self.enable_bg        = (val & 0x01) != 0;
    self.enable_obj       = (val & 0x02) != 0;
    self.obj_size         = (val & 0x04) != 0;
    self.bg_tilemap_addr  = (val & 0x08) != 0;
    self.bg_tiledata_addr = (val & 0x10) != 0;
    self.enable_win       = (val & 0x20) != 0;
    self.win_tilemap_addr = (val & 0x40) != 0;
    self.enable_display   = (val & 0x80) != 0;
  }
  pub fn into_u8(&self) -> u8 {
    (self.enable_bg         as u8)       | 
    ((self.enable_obj       as u8) << 1) |
    ((self.obj_size         as u8) << 2) |
    ((self.bg_tilemap_addr  as u8) << 3) |
    ((self.bg_tiledata_addr as u8) << 4) |
    ((self.enable_win       as u8) << 5) |
    ((self.win_tilemap_addr as u8) << 6) |
    ((self.enable_display   as u8) << 7)
  }
}
impl Into<u8> for LCDC {
  fn into(self) -> u8 { self.into_u8() }
}
impl From<u8> for LCDC {
  fn from(v: u8) -> Self { Self::from_u8(v) }
}
