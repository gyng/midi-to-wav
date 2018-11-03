#[macro_use]
extern crate serde_derive;

use docopt::Docopt;
use synthrs::synthesizer::{make_samples_from_midi_file, quantize_samples};
use synthrs::wave;
use synthrs::writer::write_wav;

const USAGE: &'static str = "
midi-to-wav - synthesise MIDI files

Example:
  midi-to-wav convert input.mid --output output.wav --instrument=square_karplus --envelope

Usage:
  midi-to-wav convert <input> [--instrument=<square_wave>] [--output=<path>] [--envelope]
  midi-to-wav instruments
  midi-to-wav (-h | --help)
  midi-to-wav --version

Options:
  -h --help                  Show this screen.
  --version                  Show version.
  --input                    Path to input MIDI file
  --output=<path>            Path to output MIDI file, defaults to input.ext.wav
  --envelope                 Whether to use an attack-decay envelope
  --instrument=<square_wave> Which waveform generator to use (defaults to square_wave)
";

#[derive(Debug, Deserialize, Clone)]
struct Args {
    arg_input: String,
    flag_output: Option<String>,
    flag_instrument: Option<String>,
    flag_envelope: bool,
    cmd_instruments: bool,
    cmd_convert: bool,
}

// Really awful, should Box up closures
// Cannot do `instrument: Fn(f64) -> impl Fn(f64) -> f64`
fn make_samples(
    path: &str,
    instrument: &str,
    envelope: bool,
) -> Result<Vec<f64>, synthrs::errors::SynthrsError> {
    match instrument {
        "tangent_wave" => make_samples_from_midi_file(wave::tangent_wave, 44100, envelope, path),
        "sawtooth_wave" => make_samples_from_midi_file(wave::sawtooth_wave, 44100, envelope, path),
        "triangle_wave" => make_samples_from_midi_file(wave::triangle_wave, 44100, envelope, path),
        "bell" => make_samples_from_midi_file(
            |frequency: f64| wave::bell(frequency, 0.003, 0.5),
            44100,
            envelope,
            path,
        ),
        "organ_karplus" => make_samples_from_midi_file(
            |frequency: f64| wave::karplus_strong(wave::organ(frequency), 0.01, 1.0, 0.9, 44_100.0),
            44100,
            envelope,
            path,
        ),
        "organ" => make_samples_from_midi_file(wave::organ, 44100, envelope, path),
        "sine_wave" => make_samples_from_midi_file(wave::sine_wave, 44100, envelope, path),
        "sine_wave_karplus" => make_samples_from_midi_file(
            |frequency: f64| {
                wave::karplus_strong(wave::sine_wave(frequency), 0.01, 1.0, 0.9, 44_100.0)
            },
            44100,
            envelope,
            path,
        ),
        "square_wave_karplus" => make_samples_from_midi_file(
            |frequency: f64| {
                wave::karplus_strong(wave::square_wave(frequency), 0.01, 1.0, 0.9, 44_100.0)
            },
            44100,
            envelope,
            path,
        ),
        "square_wave" | _ => make_samples_from_midi_file(wave::square_wave, 44100, envelope, path),
    }
}

fn list_instruments() {
    let list = vec![
        "bell",
        "organ_karplus",
        "organ",
        "sawtooth_wave",
        "sine_wave_karplus",
        "sine_wave",
        "square_wave_karplus",
        "square_wave (default)",
        "tangent_wave",
        "triangle_wave",
    ];
    println!("{:#?}", list);
}

fn main() -> Result<(), synthrs::errors::SynthrsError> {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());

    if args.cmd_instruments {
        list_instruments();
    } else if args.cmd_convert {
        let output_path = &args
            .flag_output
            .unwrap_or(format!("{}{}", &args.arg_input, ".wav"));

        let instrument = &args.flag_instrument.unwrap_or("square_wave".to_string());

        write_wav(
            output_path,
            44100,
            &quantize_samples::<i16>(&make_samples(
                &args.arg_input,
                &instrument,
                args.flag_envelope,
            )?),
        ).ok()
        .expect("failed to synthesize MIDI file");
    }

    Ok(())
}
