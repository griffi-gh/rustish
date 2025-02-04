#[derive(Default, Clone, Copy, Debug)]
pub struct Terminal {
  //pub vin: bool,
  pub volume: u8,
  pub enabled_channels: (bool, bool, bool, bool),
}

impl Terminal {
  pub fn new() -> Self {
    Self {
      volume: 7,
      enabled_channels: (true, true, true, true),
    }
  }

  /// This is ridicuosly over-optimized but this greatly improves the generated assembly
  /// mixes channels together (averages them) with an option to 
  /// disable individual channels (Self.enabled_channels)
  pub fn mix_outputs(&self, channels: (f32, f32, f32, f32)) -> f32 {
    //Compute sum of all enabled channels
    let amplitude = {
      f32::from_bits(channels.0.to_bits() * (self.enabled_channels.0 as u32)) +
      f32::from_bits(channels.1.to_bits() * (self.enabled_channels.1 as u32)) +
      f32::from_bits(channels.2.to_bits() * (self.enabled_channels.2 as u32)) +
      f32::from_bits(channels.3.to_bits() * (self.enabled_channels.3 as u32))
    };

    // compute volume:
    // 1 + (self.volume as f32) / 7
    // but precomputed
    // ! volume also does division by 4 to compute average of amplitude instead of sum
    let volume = {
      const VOLUME_LUT: [f32; 8] = [
        0.25 * (1. / 8.),
        0.25 * (2. / 8.),
        0.25 * (3. / 8.),
        0.25 * (4. / 8.),
        0.25 * (5. / 8.),
        0.25 * (6. / 8.),
        0.25 * (7. / 8.),
        0.25,
      ];
      VOLUME_LUT[(self.volume & 7) as usize]
    };
    volume * amplitude
  }
}
