use rodio::*;
use std::thread;
use std::time::Duration;

mod noise;
use noise::*;

fn main() {
    // get audio device
    let device = default_output_device().unwrap();

    // create noise source
    let source = Noise::new();

    // play noise source
    play_raw(&device, source.convert_samples());

    // for 1500 milliseconds
    thread::sleep(Duration::from_millis(1500));
}
