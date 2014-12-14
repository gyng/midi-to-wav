# midi-to-wav

A small program to output *format0* MIDI files as a mono WAV with all instruments as square waves (until synthrs upgrades or changes its midi synthesis)

## Usage

Compile

    cargo build --release

Run

    midi-to-wav input.mid output.wav 120.0

Or compile and run

    cargo run input.mid output.wav [120.0 (BPM)]
