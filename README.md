# midi-to-wav

A small program to output MIDI files as a mono WAV with all instruments as square waves (until synthrs upgrades or changes its midi synthesis)

## Usage

Compile

    cargo build --release

Run

    midi-to-wav input.mid output.wav

Or compile and run

    cargo run --release input.mid output.wav

The `--release` flag is used only for speed.
