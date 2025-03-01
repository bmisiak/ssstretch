# SSStretch - Rust bindings for Signalsmith Stretch

This crate provides Rust bindings for the [Signalsmith Stretch](https://github.com/Signalsmith-Audio/signalsmith-stretch) time-stretching and pitch-shifting library.

## Features

- Time-stretch and pitch-shift audio with high quality
- Configurable parameters for sound quality vs. CPU usage
- Safe Rust API over the C++ implementation
- Convenience functions for common use cases

## Usage

The library provides a type-safe, generic API with the number of channels known at compile time.

### Basic Usage

```rust
use ssstretch::Stretch;

// Create a stereo Stretch instance (2 channels)
let mut stretch = Stretch::<2>::new(44100.0); // 44.1kHz sample rate

// Optional: Set pitch shift (in semitones)
stretch.set_transpose_semitones(3.0, None); // Shift up by 3 semitones

// Process audio with time stretching
// Here we're creating output that's 1.5x longer than input (slower)
let output_samples = (input_samples as f32 * 1.5) as usize;
let mut output = [
    vec![0.0f32; output_samples],
    vec![0.0f32; output_samples]
];

// Process using Vec<Vec<f32>> format
stretch.process_vec(
    &input,                // Input audio (array of channels)
    input_samples as i32,  // Input length in samples
    &mut output,           // Output buffer 
    output_samples as i32  // Output length in samples
);
```

### Builder Pattern

For more complex configuration, use the builder pattern:

```rust
use ssstretch::StretchBuilder;

// Create a 5.1 surround configuration with custom parameters
let mut stretch = StretchBuilder::<6>::new()
    .preset_cheaper(44100.0)                    // Use cheaper preset at 44.1kHz
    .transpose_semitones(-2.0, Some(0.5))       // Shift down 2 semitones
    .build();                                   // Build the Stretch<6> instance

// Process 6-channel audio
stretch.process_vec(
    &input,                // 6-channel input
    input_samples as i32,
    &mut output,           // 6-channel output
    output_samples as i32
);
```

### Advanced Usage with Raw Slices

For direct processing with fixed-size arrays:

```rust
use ssstretch::Stretch;

let mut stretch = Stretch::<2>::new(44100.0);

// Create input and output slices directly
let input_slices = [&left_channel[..], &right_channel[..]];
let mut output_slices = [&mut left_output[..], &mut right_output[..]];

// Process with direct slice references
stretch.process(input_slices, output_slices);
```

## Implementation Details

This crate uses the [cxx](https://crates.io/crates/cxx) crate to provide safe Rust bindings to the C++ library. The C++ template class `SignalsmithStretch<float>` is wrapped with a type alias and exposed through the cxx FFI boundary.

### Key Design Points:

1. **Type Safety**: Uses const generics (`Stretch<C>`) to enforce correct channel count at compile time
2. **Builder Pattern**: Provides a fluent `StretchBuilder<C>` API for clean configuration
3. **Zero Allocation Processing**: The core API uses fixed-size arrays with no heap allocations during processing
4. **Direct FFI**: Uses type aliases rather than C wrapper functions for efficient interop with C++
5. **Flexible API Layers**:
   - Low-level API with fixed-size arrays for maximum performance
   - Helper methods for working with Vec-based audio data
6. **Thread Safety**: The main Stretch type can be safely used across threads

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