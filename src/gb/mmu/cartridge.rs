use std::{fs, error::Error};

#[allow(unused_variables)]
pub trait Cartridge {
    fn read(&self, addr: u16) -> u8;
    fn write(&self, addr: u16, value: u8) {}
    fn load(&mut self, data: &[u8]) {}
    fn load_file(&mut self, path: &str) -> Result<(), Box<dyn Error + 'static>> {
        let data: &[u8] = &(fs::read(path)?)[..];
        self.load(data);
        Ok(())
    }
}
pub type DynCartridge = Box<(dyn Cartridge + Send)>;

pub struct CartridgeNone { rom: [u8; 0x8000] }
impl CartridgeNone {
    pub fn new() -> Self {
        Self { rom: [0; 0x8000] }
    }
}
impl Cartridge for CartridgeNone {
    fn load(&mut self, rom: &[u8]) {
        if rom.len() != 0x8000 {
            panic!("Invalid ROM size: {:#X}", rom.len());
        }
        for (place, data) in self.rom.iter_mut().zip(rom.iter()) {
            *place = *data;
        }
    }
    #[inline(always)]
    fn read(&self, addr: u16) -> u8 {
        //bitwise and allows the compiler to optimize away the bounds checks
        //...but I want to keep them on debug buils
        #[cfg(debug_assertions)]
        return self.rom[addr as usize];
        #[cfg(not(debug_assertions))]
        return self.rom[(addr & 0x7FFF) as usize];
    }
}

pub fn _parse_header(_rom: &[u8]) {
    todo!(); // TODO parse_header()
}
pub fn get_cartridge(cart_type: u8) -> DynCartridge {
    match cart_type {
        0x00 => Box::new(CartridgeNone::new()),
        _ => panic!("Cartridge type not supported {:#04X}", cart_type)
    }
}