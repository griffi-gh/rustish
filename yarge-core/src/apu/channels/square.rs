use crate::{apu::{ApuChannel, wave::WaveDuty}, consts::audio_registers::*};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum SquareWaveChannelType {
  Channel1,
  Channel2
}

pub struct SquareWaveChannel {
  channel_type: SquareWaveChannelType,
  wave_duty: WaveDuty,
  wavelength: usize,
  freq_timer: usize,
  dac_enabled: bool,
  channel_enabled: bool,
}

impl SquareWaveChannel {
  pub fn new(channel_type: SquareWaveChannelType) -> Self {
    //TODO provide sensilble defaults?
    Self {
      channel_type,
      wave_duty: WaveDuty::new(),
      freq_timer: 8192, //or 0?
      wavelength: 0,
      dac_enabled: false,
      channel_enabled: false,
    }
  }
}

impl ApuChannel for SquareWaveChannel {
  fn tick(&mut self) {
    if !self.channel_enabled { return }
    //self.freq_timer -= 1;
    self.freq_timer = self.freq_timer.saturating_sub(1);
    if self.freq_timer == 0 {
      self.freq_timer = 4 * (2048 - self.wavelength);
      self.wave_duty.tick();
      self.channel_enabled = false;
    }
  }

  fn amplitude(&self) -> f32 {
    if !self.channel_enabled {
      return 0.
    }
    let data = self.wave_duty.get_data() as f32;
    //idk why /7.5 - 1 part is needed, I stole it from another emu
    (data / 7.5) - 1.0 
  }
  
  fn read(&self, mmio_addr: u16) -> u8 {
    0
  }

  fn write(&mut self, mmio_addr: u16, value: u8) {
    match mmio_addr { 
      R_NR10 => {
        //TODO
      }
      R_NR11 | R_NR21 => {
        // 0bAABBBBBB;
        //   I L- freq timer
        //   L- pat type
        self.wave_duty.set_pattern_type((value >> 6) as usize);
        //self.freq_timer = (value & 0x3f) as usize;
        //NO, length!
        //HACK: force enable channel!
        self.channel_enabled = true;
      }
      R_NR12 | R_NR22 => {
        //TODO
      }
      R_NR13 | R_NR23 => {
        //TODO
      }
      R_NR14 | R_NR24 => {
        //TODO
      }
      _ => ()
    }
  }
}
