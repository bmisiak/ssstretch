# ssstretch - Rust bindings for Signalsmith Audio DSP

This crate provides Rust bindings for the [Signalsmith Audio DSP](https://github.com/Signalsmith-Audio/signalsmith-stretch) library, including time-stretching, pitch-shifting, filters, FFT, and other audio processing components.

## Features

### Core Features
- High-quality time-stretching and pitch-shifting with the Signalsmith Stretch algorithm
- Audio DSP components including:
  - Fast Fourier Transform (FFT) with complex and real variants
  - Biquad filters with multiple filter types and design methods
  - Delay lines with fractional sample interpolation
  - Window functions for spectral processing
- Type-safe Rust API with const generics for compile-time safety
- Zero-allocation processing options for real-time audio
- Builder patterns for clean configuration

### Implementation Quality
- Configurable parameters for sound quality vs. CPU usage
- Safe Rust API over the C++ implementation
- Convenience functions for common use cases

## Usage

The library provides modular components that can be used independently or together.

### Time Stretching and Pitch Shifting

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

### Biquad Filter Example

```rust
use ssstretch::dsp::filters::{BiquadFilter, BiquadDesign};

// Create a biquad filter
let mut filter = BiquadFilter::new();

// Configure as lowpass filter at 1kHz with Q of 0.7
// (normalized frequency = frequency / sample_rate)
let sample_rate = 44100.0;
let normalized_freq = 1000.0 / sample_rate;
filter.lowpass(normalized_freq, 0.7, Some(BiquadDesign::Cookbook));

// Process a buffer of audio
let input = vec![1.0, 0.0, 0.0, 0.0, 0.0]; // Impulse 
let mut output = vec![0.0; 5];
filter.process_buffer(&input, &mut output);
```

### FFT Example

```rust
use ssstretch::dsp::fft::RealFFT;
use ssstretch::ComplexFloat;

// Create a real FFT processor
let fft_size = 1024;
let mut fft = RealFFT::new(fft_size);

// Create input/output buffers
let mut input = vec![0.0; fft_size as usize];
let mut output = vec![ComplexFloat::new(0.0, 0.0); (fft_size/2 + 1) as usize];

// Fill input with audio data...

// Perform forward FFT (time domain to frequency domain)
fft.forward(&input, &mut output);

// Process the spectrum...

// Convert back to time domain with inverse FFT
fft.inverse(&output, &mut input);
```

### Delay Line Example

```rust
use ssstretch::dsp::delay::Delay;

// Create a delay line with maximum 1 second delay at 44.1kHz
let mut delay = Delay::new(44100);

// Process 500ms echo with 50% feedback
let echo_samples = 22050.0; // 500ms at 44.1kHz
let feedback = 0.5;

// Process input sample with echo
for input_sample in input_signal {
    // Get the delayed output
    let delayed = delay.process(input_sample, echo_samples);
    
    // Mix original with echo
    let output_sample = input_sample + delayed * feedback;
    
    // Feed back into the delay line
    delay.process(output_sample * feedback, echo_samples);
    
    // Output the mixed signal
    output_signal.push(output_sample);
}
```

## Implementation Details

This crate uses the [cxx](https://crates.io/crates/cxx) crate to provide safe Rust bindings to the C++ library. The C++ template classes are wrapped with type aliases and exposed through the cxx FFI boundary.

### Key Design Points:

1. **Modular Components**: Each DSP component can be used independently
2. **Type Safety**: Uses const generics for compile-time safety (e.g., `Stretch<C>`)
3. **Builder Pattern**: Provides fluent APIs for clean configuration
4. **Zero Allocation Processing**: Core APIs use fixed-size arrays with no heap allocations during processing
5. **Direct FFI**: Uses type aliases rather than C wrapper functions for efficient interop with C++
6. **Flexible API Layers**:
   - Low-level APIs with fixed-size arrays for maximum performance
   - Helper methods for working with Vec-based audio data
7. **Thread Safety**: Most types can be safely used across threads

## Building

The library requires a C++ compiler that supports C++14 or newer.

```bash
cargo build
```

## Examples

See the `examples/` directory for sample code demonstrating how to use the library:

- `simple_stretch.rs` - Basic time stretching
- `filter_example.rs` - Using biquad filters
- `fft_example.rs` - Performing spectral analysis with FFT
- `delay_example.rs` - Creating echo effects with delay lines

## License

This crate is available under the MIT License, see LICENSE for details.

## Acknowledgments

- [Signalsmith Audio](https://signalsmith-audio.co.uk/) for the original C++ DSP library
- The [cxx](https://crates.io/crates/cxx) crate for making Rust/C++ interop safer and easier
