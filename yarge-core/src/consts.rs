pub const VERSION: Option<&'static str> = option_env!("CARGO_PKG_VERSION");
pub const TILE_WIDTH: u8 = 8;
pub const WIDTH: usize = 160;
pub const HEIGHT: usize = 144;
pub const FB_SIZE: usize = WIDTH * HEIGHT;
pub(crate) const CYCLES_PER_FRAME: usize = 70684; //70224;
pub(crate) const VRAM_SIZE: usize = 0x2000;
pub(crate) const INT_JMP_VEC: [u16; 5] = [0x40, 0x48, 0x50, 0x58, 0x60];
pub(crate) const TIMER_CLOCKS: [u8; 4] = [9, 3, 5, 7];
pub const BIOS: [u8; 0x100] = [
  0x31, 0xFE, 0xFF, 0xAF, 0x21, 0xFF, 0x9F, 0x32, 0xCB, 0x7C, 0x20, 0xFB, 0x21, 0x26, 0xFF, 0x0E,
  0x11, 0x3E, 0x80, 0x32, 0xE2, 0x0C, 0x3E, 0xF3, 0xE2, 0x32, 0x3E, 0x77, 0x77, 0x3E, 0xFC, 0xE0,
  0x47, 0x11, 0x04, 0x01, 0x21, 0x10, 0x80, 0x1A, 0xCD, 0x95, 0x00, 0xCD, 0x96, 0x00, 0x13, 0x7B,
  0xFE, 0x34, 0x20, 0xF3, 0x11, 0xD8, 0x00, 0x06, 0x08, 0x1A, 0x13, 0x22, 0x23, 0x05, 0x20, 0xF9,
  0x3E, 0x19, 0xEA, 0x10, 0x99, 0x21, 0x2F, 0x99, 0x0E, 0x0C, 0x3D, 0x28, 0x08, 0x32, 0x0D, 0x20,
  0xF9, 0x2E, 0x0F, 0x18, 0xF3, 0x67, 0x3E, 0x64, 0x57, 0xE0, 0x42, 0x3E, 0x91, 0xE0, 0x40, 0x04,
  0x1E, 0x02, 0x0E, 0x0C, 0xF0, 0x44, 0xFE, 0x90, 0x20, 0xFA, 0x0D, 0x20, 0xF7, 0x1D, 0x20, 0xF2,
  0x0E, 0x13, 0x24, 0x7C, 0x1E, 0x83, 0xFE, 0x62, 0x28, 0x06, 0x1E, 0xC1, 0xFE, 0x64, 0x20, 0x06,
  0x7B, 0xE2, 0x0C, 0x3E, 0x87, 0xE2, 0xF0, 0x42, 0x90, 0xE0, 0x42, 0x15, 0x20, 0xD2, 0x05, 0x20,
  0x4F, 0x16, 0x20, 0x18, 0xCB, 0x4F, 0x06, 0x04, 0xC5, 0xCB, 0x11, 0x17, 0xC1, 0xCB, 0x11, 0x17,
  0x05, 0x20, 0xF5, 0x22, 0x23, 0x22, 0x23, 0xC9, 0xCE, 0xED, 0x66, 0x66, 0xCC, 0x0D, 0x00, 0x0B,
  0x03, 0x73, 0x00, 0x83, 0x00, 0x0C, 0x00, 0x0D, 0x00, 0x08, 0x11, 0x1F, 0x88, 0x89, 0x00, 0x0E,
  0xDC, 0xCC, 0x6E, 0xE6, 0xDD, 0xDD, 0xD9, 0x99, 0xBB, 0xBB, 0x67, 0x63, 0x6E, 0x0E, 0xEC, 0xCC,
  0xDD, 0xDC, 0x99, 0x9F, 0xBB, 0xB9, 0x33, 0x3E, 0x3C, 0x42, 0xB9, 0xA5, 0xB9, 0xA5, 0x42, 0x3C,
  0x21, 0x04, 0x01, 0x11, 0xA8, 0x00, 0x1A, 0x13, 0xBE, 0x20, 0xFE, 0x23, 0x7D, 0xFE, 0x34, 0x20,
  0xF5, 0x06, 0x19, 0x78, 0x86, 0x23, 0x05, 0x20, 0xFB, 0x86, 0x20, 0xFE, 0x3E, 0x01, 0xE0, 0x50,
];
pub(crate) const DEFAULT_HEADER: [u8; 80] = [
  0x00, 0x76, 0x37, 0x06, 0xCE, 0xED, 0x66, 0x66, 0xCC, 0x0D, 0x00, 0x0B, 0x03, 0x73, 0x00, 0x83,
  0x00, 0x0C, 0x00, 0x0D, 0x00, 0x08, 0x11, 0x1F, 0x88, 0x89, 0x00, 0x0E, 0xDC, 0xCC, 0x6E, 0xE6,
  0xDD, 0xDD, 0xD9, 0x99, 0xBB, 0xBB, 0x67, 0x63, 0x6E, 0x0E, 0xEC, 0xCC, 0xDD, 0xDC, 0x99, 0x9F,
  0xBB, 0xB9, 0x33, 0x3E, 0x43, 0x50, 0x55, 0x5F, 0x49, 0x4E, 0x53, 0x54, 0x52, 0x53, 0x00, 0x00,
  0x00, 0x00, 0x00, 0x80, 0x00, 0x00, 0x00, 0x01, 0x01, 0x00, 0x00, 0x00, 0x00, 0x3B, 0xF5, 0x30,
];

#[cfg(feature = "logging-file")]
pub const LOG_PATH: &str = "./gameboy.log";

pub const MBC_TYPE_LIST: &[(u8, &str)] = &[
  (0x00, "ROM ONLY"),
  (0x01, "MBC1"),
  (0x02, "MBC1+RAM"),
  (0x03, "MBC1+RAM+BATTERY"),
  (0x05, "MBC2"),
  (0x06, "MBC2+BATTERY"),
  (0x08, "ROM+RAM 1"),
  (0x09, "ROM+RAM+BATTERY 1"),
  (0x0B, "MMM01"),
  (0x0C, "MMM01+RAM"),
  (0x0D, "MMM01+RAM+BATTERY"),
  (0x0F, "MBC3+TIMER+BATTERY"),
  (0x10, "MBC3+TIMER+RAM+BATTERY 2"),
  (0x11, "MBC3"),
  (0x12, "MBC3+RAM 2"),
  (0x13, "MBC3+RAM+BATTERY 2"),
  (0x19, "MBC5"),
  (0x1A, "MBC5+RAM"),
  (0x1B, "MBC5+RAM+BATTERY"),
  (0x1C, "MBC5+RUMBLE"),
  (0x1D, "MBC5+RUMBLE+RAM"),
  (0x1E, "MBC5+RUMBLE+RAM+BATTERY"),
  (0x20, "MBC6"),
  (0x22, "MBC7+SENSOR+RUMBLE+RAM+BATTERY"),
  (0xFC, "POCKET CAMERA"),
  (0xFD, "BANDAI TAMA5"),
  (0xFE, "HuC3"),
  (0xFF, "HuC1+RAM+BATTERY"),
];

//static

use std::collections::HashMap;
use lazy_static::lazy_static;
lazy_static! {
  pub static ref MBC_TYPE_NAMES: HashMap<u8, &'static str> = {
    let mut map = HashMap::new();
    for v in MBC_TYPE_LIST { map.insert(v.0, v.1); }
    map.shrink_to_fit();
    map
  };
}
