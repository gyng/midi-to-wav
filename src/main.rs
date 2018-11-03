extern crate synthrs;

use std::env;

use synthrs::synthesizer::{make_samples_from_midi_file, quantize_samples};
use synthrs::wave;
use synthrs::writer::write_wav;

fn main() -> Result<(), synthrs::errors::SynthrsError> {
    let args: Vec<String> = env::args().collect();
    assert!(args.len() == 3);

    let input_path = &args[1][..];
    let output_path = &args[2][..];

    write_wav(
        output_path,
        44100,
        &quantize_samples::<i16>(&make_samples_from_midi_file(
            wave::square_wave,
            44100,
            true,
            input_path,
        )?),
    ).ok()
    .expect("failed to synthesize MIDI file");

    Ok(())
}
