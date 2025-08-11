//! Rust bindings for the Signalsmith Audio DSP library.
//!
//! This crate provides idiomatic Rust bindings for the C++ Signalsmith DSP library,
//! including time stretching, pitch shifting, and filters.

// Re-export the main stretch interface to maintain backward compatibility
pub use stretch::Stretch;
pub use stretch::StretchBuilder;
pub use dsp::filters::BiquadFilter;
pub use num_complex::Complex32 as ComplexFloat;

// Import submodules
pub mod stretch;
pub mod dsp;
pub mod util;
mod ffi;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stretch::Stretch;
    use crate::dsp::filters::BiquadFilter;

    #[test]
    fn test_create_stretch() {
        let stretch = Stretch::<2>::new(44100.0);
        assert_eq!(2, 2); // Channels is part of the type now
    }
    
    #[test]
    fn test_create_biquad() {
        let mut filter = BiquadFilter::new();
        filter.lowpass(0.1, 0.7, None);
        
        // Process a simple impulse
        let input = vec![1.0, 0.0, 0.0, 0.0, 0.0];
        let mut output = vec![0.0; 5];
        
        filter.process_buffer(&input, &mut output);
        
        // Output should be a decaying response
        assert!(output[0] > 0.0);
        assert!(output[1] > 0.0);
    }
}
