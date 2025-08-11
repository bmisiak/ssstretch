ssstretch
=========

Rust bindings for the Signalsmith Stretch library: high‑quality time‑stretching and pitch‑shifting, with a small binding to its biquad filters.

- Core: safe Rust API over the C++ Signalsmith Stretch implementation
- Also available: `BiquadFilter` for simple IIR filtering
- Optional: a Rust‑native FFT backend (for examples and convenience) behind a feature flag

Links
- Signalsmith Stretch: [github.com/Signalsmith-Audio/signalsmith-stretch](https://github.com/Signalsmith-Audio/signalsmith-stretch)
- cxx (Rust/C++ interop): [crates.io/crates/cxx](https://crates.io/crates/cxx)

Install
-------

Add to your Cargo.toml:

```toml
[dependencies]
ssstretch = "0.1"
```

Optional FFT support (pure Rust, not via C++):

```toml
[dependencies]
ssstretch = { version = "0.1", features = ["fft-rust"] }
```

Requirements
------------

- A C++14 (or newer) compiler toolchain (needed to build the C++ Stretch library via cxx)
- If you’re developing from a git clone of this repo: initialize the submodule once

```bash
git submodule update --init --recursive
```

Quick start: time‑stretch + pitch‑shift
---------------------------------------

```rust
use ssstretch::Stretch;

// Stereo stretcher for 44.1 kHz
let mut stretch = Stretch::<2>::new(44_100.0);

// Optional pitch shift: +3 semitones (tonality limit 0.0 = off)
stretch.set_transpose_semitones(3.0, None);

// Input: 2 channels of N samples
let input: [Vec<f32>; 2] = [left, right];

// Choose output length; this sets the time‑stretch ratio
let out_len = (input[0].len() as f32 * 1.5) as i32; // 1.5× slower
let mut output = [vec![0.0; out_len as usize], vec![0.0; out_len as usize]];

stretch.process_vec(&input, input[0].len() as i32, &mut output, out_len);

// Optional: inspect algorithm latencies
let in_lat = stretch.input_latency();
let out_lat = stretch.output_latency();
```

Pitch control
-------------

```rust
// Frequency multiplier (1.0 = unchanged)
stretch.set_transpose_factor(1.1225, None); // ~= +2 semitones

// Or semitones directly
stretch.set_transpose_semitones(-7.0, Some(0.5)); // down a fifth, with a tonality limit
```

Biquad filter (binding)
-----------------------

```rust
use ssstretch::dsp::filters::{BiquadFilter, BiquadDesign};

let mut filter = BiquadFilter::new();

// Note: frequency is normalized (Hz / sample_rate)
let fs = 44_100.0;
filter.lowpass(1_000.0 / fs, 0.7, Some(BiquadDesign::Cookbook));

let input = vec![1.0, 0.0, 0.0, 0.0];
let mut output = vec![0.0; input.len()];
filter.process_buffer(&input, &mut output);
```

Optional FFT (Rust backend)
---------------------------

FFT is not bound from the C++ library. If you enable the `fft-rust` feature, the crate exposes a small adapter over `rustfft`/`realfft` for convenience in examples:

```bash
cargo run --example fft_example --features fft-rust
```

Examples
--------

- `simple_stretch.rs`: basic stretch + pitch shift
- `interleaved_stretch.rs`: de‑interleave, process, re‑interleave
- `builder_pattern.rs`: custom configuration
- `filter_example.rs`: biquad usage
- `fft_example.rs`: requires `--features fft-rust`
- `delay_example.rs`: demo of a small Rust delay (not part of the C++ binding)

API at a glance
---------------

- `Stretch<C>`: main processor for `C` channels
  - `new(sample_rate)`, `with_seed(seed, sample_rate)`
  - `process(&[&[f32]; C], &mut [&mut [f32]; C])`
  - `process_vec(&[Vec<f32>], in_samples, &mut [Vec<f32>], out_samples)`
  - `seek`, `flush`, `reset`
  - `set_transpose_factor`, `set_transpose_semitones`
  - `block_samples`, `interval_samples`, `input_latency`, `output_latency`
- `BiquadFilter`: lowpass/highpass/bandpass/notch/peak/low_shelf/high_shelf/allpass

Notes on safety and buffers
---------------------------

- Channel count is a compile‑time constant (`Stretch::<C>`). The API checks channel counts and per‑channel lengths.
- All input channels must have the same length; likewise for output channels. Mismatches will panic with a clear message.
- The time‑stretch ratio is defined by your chosen in/out lengths (there’s no separate “ratio” parameter).

Build and test
--------------

```bash
cargo build
cargo build --examples
cargo test

# If you are building from a fresh git clone
git submodule update --init --recursive
```

License
-------

MIT

Acknowledgements
----------------

- Signalsmith Audio for the original C++ Stretch implementation
- The `cxx` project for safe Rust/C++ interop
