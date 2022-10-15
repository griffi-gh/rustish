use crate::apu::ApuChannel;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum SquareWaveChannelType {
  Channel1,
  Channel2
}

pub struct SquareWaveChannel {
  channel_type: SquareWaveChannelType
}
impl SquareWaveChannel {
  pub fn new(channel_type: SquareWaveChannelType) -> Self {
    Self {
      channel_type
    }
  }
}
impl ApuChannel for SquareWaveChannel {
  fn tick(&mut self) {
    //TODO SquareWaveChannel::tick
  }
}
