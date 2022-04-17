use std::fmt;
use arrayvec::ArrayString;
use enum_dispatch::enum_dispatch;
use crate::{Res, YargeError, consts::DEFAULT_HEADER};

#[allow(unused_variables)]
#[enum_dispatch]
pub trait CartridgeImpl {
  fn index(&self) -> u8;
  fn name(&self) -> &str;
  fn read(&self, addr: u16) -> u8;
  fn write(&self, addr: u16, value: u8) {}
  fn read_eram(&self, addr: u16) -> u8 { 0xff }
  fn write_eram(&self, addr: u16, value: u8) {}
  fn load(&mut self, data: &[u8]) -> Res<()>;
}

#[enum_dispatch(CartridgeImpl)]
pub enum Cartridge {
  MockCartridge,
  CartridgeNone
}

pub struct MockCartridge {}
impl CartridgeImpl for MockCartridge {
  fn name(&self) -> &str { "NONE" }
  fn index(&self) -> u8 { 0 }
  fn load(&mut self, _: &[u8]) -> Res<()> { Ok(()) }
  fn read(&self, addr: u16) -> u8 {
    if (0x100..(0x100+80)).contains(&addr) {
      DEFAULT_HEADER[(addr - 0x100) as usize]
    } else {
      0x00
    }
  }
}

pub struct CartridgeNone {
  index: u8,
  rom: Box<[u8; 0x8000]>,
}
impl CartridgeNone {
  pub fn new(index: u8) -> Self {
    Self {
      index,
      rom: Box::new([0xFF; 0x8000]),
    }
  }
}
impl CartridgeImpl for CartridgeNone {
  fn name(&self) -> &str { "MBC0" }
  fn index(&self) -> u8 { self.index }
  fn load(&mut self, rom: &[u8]) -> Res<()> {
    if rom.len() != 0x8000 {
      return Err(
        YargeError::InvalidRomSize(rom.len())
      );
    }
    for (place, data) in self.rom.iter_mut().zip(rom.iter()) {
      *place = *data;
    }
    Ok(())
  }
  fn read(&self, addr: u16) -> u8 {
    //bitwise and allows the compiler to optimize away the bounds checks
    //...but I want to keep them on debug buils
    #[cfg(debug_assertions)]
    return self.rom[addr as usize];
    #[cfg(not(debug_assertions))]
    return self.rom[(addr & 0x7FFF) as usize];
  }
}

pub fn get_cartridge(cart_type: u8) -> Res<Cartridge> {
  match cart_type {
    0x00 => Ok(CartridgeNone::new(cart_type).into()),
    _ => Err(YargeError::InvalidMbcType(cart_type))
  }
}

#[derive(Clone, Copy, Default, Debug)]
pub struct RomHeader {
  pub name: ArrayString<16>,
  pub mbc_type: u8,
}
impl RomHeader {
  pub fn parse(rom: &[u8]) -> Self {
    Self {
      mbc_type: rom[0x147],
      name: {
        let mut str = ArrayString::<16>::new();
        for addr in 0x134..=0x143_usize {
          let byte = rom[addr];
          if byte == 0 {
            break;
          } else {
            str.push(char::from_u32(byte as u32).unwrap());
          }
        }
        str
      }
    }
  }
}
impl fmt::Display for RomHeader {
  fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
    let mbc_type = self.mbc_type;
    let name = &self.name[..];
    write!(formatter, "Name: {name}\nMBC Type: {mbc_type:#04X}")
  }
}
