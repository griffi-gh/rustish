#![forbid(unsafe_code)]

mod mmu;
mod cpu;
pub use mmu::MMU;
pub use cpu::CPU;

use std::{thread, sync::{Arc, Mutex}};

///Gameboy emulator
pub struct Gameboy {
    pub cpu: CPU,
}
impl Gameboy {
    pub fn new() -> Self {
        Self{
            cpu: CPU::new(),
        }
    }
    pub fn step(&mut self) {
        let _t = self.cpu.step();
        //TODO Tick other components
    }

    pub fn run(gb: &mut Gameboy) {
        loop { gb.step(); }
    }
    pub fn run_thread(gb: &Arc<Mutex<Gameboy>>) -> thread::JoinHandle<()> {   
        let gb = Arc::clone(&*gb);
        thread::spawn(move || {
            loop {
                let mut gb = gb.lock().unwrap();
                gb.step();
                drop(gb);
            }
        })
    }
}