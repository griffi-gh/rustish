#![forbid(unsafe_code)]
pub mod consts;
pub mod errors;
mod mmu;
mod cpu;
mod ppu;
pub use mmu::MMU;
pub use cpu::CPU;
pub use ppu::PPU;
use mmu::cartridge::RomHeader;
use std::{thread, sync::{Arc, Mutex}, error::Error};
use consts::{CYCLES_PER_FRAME, FB_SIZE};

#[cfg(feature = "logging-file")]
use std::fs::File;
#[cfg(feature = "logging-file")]
use consts::LOG_PATH;

pub struct GameboyBuilder {
  gb: Gameboy,
}
type Res<T> = Result<T, Box<dyn Error>>;
impl GameboyBuilder {
  pub fn new() -> Self {
    Self {
      gb: Gameboy::new(),
    }
  }
  pub fn init(mut self, cond: bool) -> Self {
    if cond {
      (&mut self).gb.init();
    }
    return self;
  }
  pub fn skip_bootrom(mut self, cond: bool) -> Self {
    if cond {
      (&mut self).gb.skip_bootrom();
    }
    return self;
  }
  #[allow(dead_code)]
  pub fn load_rom(mut self, data: &[u8]) -> Res<Self> {
    (&mut self).gb.load_rom(data)?;
    return Ok(self);
  }
  pub fn load_rom_file(mut self, path: &str) -> Res<Self> {
    match (&mut self).gb.load_rom_file(path) {
      Ok(()) => Ok(self),
      Err(e) => Err(e)
    }
  }
  pub fn build(self) -> Gameboy { self.gb }
}

#[derive(Clone)]
#[deprecated]
pub struct ThreadInfo {
  pub instrs: u64,
  pub time: std::time::Instant,
  pub error: Option<String>
}
#[allow(deprecated)]
impl Default for ThreadInfo {
  fn default() -> Self {
    Self {
      instrs: 0,
      time: std::time::Instant::now(),
      error: None,
    }
  }
}

///Gameboy emulator
#[allow(deprecated)]
pub struct Gameboy {
  pub running: bool,
  pub cpu: CPU,
  #[cfg(feature = "logging-file")] 
  log_file: Option<File>,
  #[deprecated = "Use custom thread runners instead"] 
  pub thread_info: Option<ThreadInfo>,
}
impl Gameboy {
  pub fn new() -> Self {
    #[allow(deprecated)]
    Self {
      running: true,
      cpu: CPU::new(),
      #[cfg(feature = "logging-file")]
      log_file: None,
      thread_info: None,
    }
  }

  pub fn init(&mut self) {
    #[cfg(feature = "logging-file")] {
      use std::{fs, io::Write};
      let mut file = fs::File::create(LOG_PATH).unwrap();
      file.write_all(b"").unwrap();
      drop(file);
      self.log_file = Some(fs::OpenOptions::new()
        .write(true)
        .append(true)
        .open(LOG_PATH)
        .unwrap());
      println!(
        "Writing trace log to file: \"{}\"\n\
        Warning! The log files can get as big as a few gigabytes in size!\n\
        Build without the 'logging-file' feature to disable.", 
        LOG_PATH
      )
    }
  }
  
  //Shorthands
  #[inline] pub fn pause(&mut self) {
    self.running = false;
  }
  #[inline] pub fn resume(&mut self) {
    self.running = true;
  }
  #[inline] pub fn load_rom_file(&mut self, path: &str) -> Res<()> {
    self.cpu.mmu.load_file(path)
  }
  #[inline] pub fn load_rom(&mut self, data: &[u8]) -> Res<()> {
    self.cpu.mmu.load_rom(data)
  }
  #[inline] pub fn load_rom_no_mbc(&mut self, data: &[u8]) -> Res<()> {
    self.cpu.mmu.load_rom_no_mbc(data)
  }
  #[inline] pub fn get_mbc_name(&self) -> &str {
    self.cpu.mmu.mbc_type_name()
  }
  #[inline] pub fn get_mbc_type(&self) -> u8 {
    self.cpu.mmu.mbc_index()
  }
  #[inline] pub fn get_rom_header(&self) -> RomHeader {
    self.cpu.mmu.header()
  }
  
  pub fn skip_bootrom(&mut self) {
    if self.cpu.mmu.bios_disabled {
      panic!("Attempt to skip bios while not in bootrom");
    }
    let reg = &mut self.cpu.reg;
    reg.pc = 0x0100;
    reg.sp = 0xFFFE;
    reg.set_af(0x01B0);
    reg.set_bc(0x0013);
    reg.set_de(0x00D8);
    reg.set_hl(0x014D);
    self.cpu.mmu.bios_disabled = true;
  }

  pub fn reset(&mut self) {
    self.cpu = CPU::new();
    #[allow(deprecated)] {
      if self.thread_info.is_some() {
        self.thread_info = Some(ThreadInfo::default());
      }
    }
  }

  pub fn get_display_data(&self) -> [u8; FB_SIZE] {
    self.cpu.mmu.ppu.display
  }

  #[cfg(feature = "logging")]
  fn log_step(&mut self) {
    let r = &self.cpu.reg;
    let m = &self.cpu.mmu;
    let string = format!(
      "A: {:02X} F: {:02X} B: {:02X} C: {:02X} D: {:02X} E: {:02X} H: {:02X} L: {:02X} SP: {:04X} PC: 00:{:04X} ({:02X} {:02X} {:02X} {:02X})", 
      r.a(), r.f(), r.b(), r.c(), r.d(), r.e(), r.h(), r.l(), r.sp, r.pc,
      m.rb(r.pc), m.rb(r.pc + 1), m.rb(r.pc + 2), m.rb(r.pc + 3)
    );
    #[cfg(feature = "logging-file")] {
      if self.log_file.is_none() {
        panic!("File not inited! Call <Gameboy>.init() or <GameboyBuilder>.init()");
      } else {
        use std::io::Write;
        write!(self.log_file.as_mut().unwrap(), "{}\n", string).unwrap();
      }
    }
    #[cfg(feature = "logging-stdout")] {
      println!("{}", string);
    }
  }

  pub fn step(&mut self) -> Result<usize, Box<dyn Error>> {
    if !self.running { return Ok(0); }
    #[cfg(feature = "logging")] self.log_step();
    let cycles = self.cpu.step()?;
    Ok(cycles)
  }

  pub fn run_for_frame(&mut self) -> Res<()> {
    if !self.running {
      return Ok(());
    }
    let mut t = 0;
    while t < CYCLES_PER_FRAME {
      t += self.step()?;
    }
    Ok(())
  }
  pub fn run(&mut self) -> Res<()> {
    loop { self.step()?; }
  }

  #[deprecated = "Use custom thread runners instead"]
  #[allow(deprecated)]
  pub fn run_thread(gb: &Arc<Mutex<Gameboy>>) -> thread::JoinHandle<()> { 
    println!("[ WARNING ] Gameboy::run_thread(&Arc<Mutex<Gameboy>>) is deprecated!");  
    let gb = Arc::clone(&*gb);
    thread::spawn(move || {
      use std::time;
      let paused_sleep_duration = time::Duration::from_millis(100);
      {
        let mut gb = gb.lock().unwrap();
        gb.thread_info = Some(ThreadInfo::default());
      }
      'main: loop {
        let mut should_sleep = false;
        {
          let gb = gb.try_lock();
          if let Ok(mut gb) = gb {
            should_sleep = !gb.running;
            if gb.thread_info.is_some() {
              let stp_res = gb.step();
              let info = gb.thread_info.as_mut().unwrap();
              if stp_res.is_err() {
                let error = stp_res.unwrap_err();
                info.error = Some(error.to_string());
                println!("Rustish Error: {}", error);
                break 'main;
              }
              info.instrs += 1;
            } else {
              panic!("No thread_info but the thread is still running");
            }
          }
        }
        if should_sleep {
          //Reduce CPU usage while paused
          thread::sleep(paused_sleep_duration);
        }
      }
    })
  }
}
