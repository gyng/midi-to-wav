extern crate synthrs;

use std::os;

use synthrs::synthesizer::{ make_samples_from_midi, quantize_samples };
use synthrs::writer::write_wav;

fn main() {
    let args = os::args();
    assert!(args.len() == 4);
    let input_path = args[1].as_slice();
    let output_path = args[2].as_slice();
    let bpm: f64 = from_str(args[3].as_slice()).unwrap();

    write_wav(output_path, 44100,
        quantize_samples::<i16>(
            make_samples_from_midi(44100, bpm, input_path)
        )
    ).ok().expect("failed to synthesize MIDI file");
}