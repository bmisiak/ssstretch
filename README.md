# SSStretch - Rust bindings for Signalsmith Stretch

This crate provides Rust bindings for the [Signalsmith Stretch](https://github.com/Signalsmith-Audio/signalsmith-stretch) time-stretching and pitch-shifting library.

## Features

- Time-stretch and pitch-shift audio with high quality
- Configurable parameters for sound quality vs. CPU usage
- Safe Rust API over the C++ implementation
- Convenience functions for common use cases

## Usage

```rust
use ssstretch::Stretch;

// Create and configure a stretcher instance
let mut stretch = Stretch::new();
stretch.preset_default(2, 44100.0); // 2 channels, 44.1kHz sample rate

// Optional: Set pitch shift (in semitones)
stretch.set_transpose_semitones(3.0, None); // Shift up by 3 semitones

// Process audio with time stretching
// Here we're creating output that's 1.5x longer than input (slower)
let output_len = (input_len as f32 * 1.5) as usize;
let mut output = vec![vec![0.0f32; output_len]; channels];

stretch.process_vec(
    &input,      // Input audio (array of channels)
    input_len,   // Input length in samples
    &mut output, // Output buffer
    output_len,  // Output length in samples
);
```

## Implementation Details

This crate uses the [cxx](https://crates.io/crates/cxx) crate to provide safe Rust bindings to the C++ library. The C++ template class `SignalsmithStretch<float>` is wrapped with a type alias and exposed through the cxx FFI boundary.

### Key Design Points:

1. Uses type aliases rather than C wrapper functions to maintain a direct connection to the C++ API
2. Handles template instantiation in C++ to avoid template issues in the FFI boundary
3. Provides both raw pointer APIs (for integration with audio systems) and convenient Vec-based APIs
4. Tracks channel count in Rust to work around private members in the C++ library

## Building

The library requires a C++ compiler that supports C++14 or newer.

```bash
cargo build
```

## Examples

See the `examples/` directory for sample code demonstrating how to use the library.

## License

This crate is available under the MIT License, see LICENSE for details.

## Acknowledgments

- [Signalsmith Audio](https://signalsmith-audio.co.uk/) for the original C++ library
- The [cxx](https://crates.io/crates/cxx) crate for making Rust/C++ interop safer and easier