# midi-to-wav

A small program to output MIDI files as a mono WAV with all instruments as whatever [synthrs](https://github.com/gyng/synthrs) has for a default (until synthrs upgrades or changes its MIDI synthesis)

Sample output:
[Mountain King](https://dl.dropboxusercontent.com/u/38256631/mountainking.ogg),
[Hungarian No. 2](https://dl.dropboxusercontent.com/u/38256631/liszt-hungarian2.ogg)

## Usage

Compile

    cargo build --release

Run

    midi-to-wav input.mid output.wav

Or compile and run

    cargo run --release input.mid output.wav

The `--release` flag is used only for speed.
