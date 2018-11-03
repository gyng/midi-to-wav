# midi-to-wav

A small program to output MIDI files as a mono WAV with all instruments changed to whatever [synthrs](https://github.com/gyng/synthrs) has provided

Sample output:
[Mountain King](docs/assets/mountainking.ogg),
[Mountain King (square)](docs/assets/mountainking-puresquare.ogg),
[Rustle of Spring](docs/assets/rustle.ogg)

## Usage

```
midi-to-wav - synthesise MIDI files

midi-to-wav convert input.mid --output output.wav --instrument=square_karplus --envelope

Usage:
  midi-to-wav convert <input> [--instrument=<sine_wave>] [--output=<path>] [--envelope]
  midi-to-wav instruments
  midi-to-wav (-h | --help)
  midi-to-wav --version

Options:
  -h --help                  Show this screen.
  --version                  Show version.
  --input                    Path to input MIDI file
  --output=<path>            Path to output MIDI file, defaults to input.ext.wav
  --envelope                 Whether to use an attack-decay envelope
  --instrument=<sine_wave>   Which waveform generator to use (defaults to sine_wave)
```

Example

```
convert tests/assets/octave.mid --output output.wav --instrument=square_wave --envelope
```

## Building

```
cargo build --release
```
