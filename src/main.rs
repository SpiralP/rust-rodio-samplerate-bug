use rodio::{
  default_output_device, play_raw,
  source::{ChannelVolume, Source},
  Decoder,
};
use std::{fs::File, thread::sleep, time::Duration};

fn main() {
  let file = File::open("music.ogg").unwrap();
  let source = Decoder::new(file).unwrap();

  // play louder in the right channel
  let source = ChannelVolume::new(source, vec![0.2, 1.0]).convert_samples();

  let device = default_output_device().unwrap();

  play_raw(&device, source);

  sleep(Duration::from_secs(20));
}
