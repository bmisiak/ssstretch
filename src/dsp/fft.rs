use crate::ComplexFloat;

/// Naive FFT/DFT utilities sufficient for examples and testing.
///
/// These implementations prioritize clarity over performance.
pub struct FFT {
    size: usize,
}

impl FFT {
    /// Create a complex FFT handler for a given size
    pub fn new(size: usize) -> Self {
        Self { size }
    }

    /// Return an efficient size for FFT operations. Here we return the next power of two.
    pub fn optimal_size(min_size: usize) -> usize {
        min_size.next_power_of_two()
    }

    /// Forward transform: time-domain complex → frequency-domain complex
    pub fn forward(&mut self, input: &[ComplexFloat], output: &mut [ComplexFloat]) {
        let n = self.size;
        assert_eq!(input.len(), n, "input length must equal FFT size");
        assert_eq!(output.len(), n, "output length must equal FFT size");

        let two_pi_over_n = 2.0_f32 * std::f32::consts::PI / n as f32;
        for k in 0..n {
            let mut acc = ComplexFloat::new(0.0, 0.0);
            for (n_idx, x) in input.iter().enumerate() {
                let angle = two_pi_over_n * (k as f32) * (n_idx as f32);
                let w = ComplexFloat::from_polar(1.0, -angle);
                acc += *x * w;
            }
            output[k] = acc;
        }
    }

    /// Inverse transform: frequency-domain complex → time-domain complex
    pub fn inverse(&mut self, input: &[ComplexFloat], output: &mut [ComplexFloat]) {
        let n = self.size;
        assert_eq!(input.len(), n, "input length must equal FFT size");
        assert_eq!(output.len(), n, "output length must equal FFT size");

        let two_pi_over_n = 2.0_f32 * std::f32::consts::PI / n as f32;
        for n_idx in 0..n {
            let mut acc = ComplexFloat::new(0.0, 0.0);
            for (k, x) in input.iter().enumerate() {
                let angle = two_pi_over_n * (k as f32) * (n_idx as f32);
                let w = ComplexFloat::from_polar(1.0, angle);
                acc += *x * w;
            }
            output[n_idx] = ComplexFloat::new(acc.re / n as f32, acc.im / n as f32);
        }
    }
}

/// Real-valued FFT convenience wrapper.
pub struct RealFFT {
    size: usize,
}

impl RealFFT {
    pub fn new(size: usize) -> Self {
        Self { size }
    }

    /// Forward transform: real input → complex spectrum of length size/2+1
    pub fn forward(&mut self, input: &[f32], output: &mut [ComplexFloat]) {
        let n = self.size;
        assert_eq!(input.len(), n, "input length must equal FFT size");
        assert_eq!(output.len(), n / 2 + 1, "output length must be size/2+1");

        // Promote to complex and run complex DFT
        let mut tmp_in = vec![ComplexFloat::new(0.0, 0.0); n];
        for (i, &v) in input.iter().enumerate() {
            tmp_in[i] = ComplexFloat::new(v, 0.0);
        }
        let mut tmp_out = vec![ComplexFloat::new(0.0, 0.0); n];
        FFT::new(n).forward(&tmp_in, &mut tmp_out);
        output.copy_from_slice(&tmp_out[..(n / 2 + 1)]);
    }

    /// Inverse transform: complex spectrum (size/2+1) → real time signal (size)
    pub fn inverse(&mut self, input: &[ComplexFloat], output: &mut [f32]) {
        let n = self.size;
        assert_eq!(input.len(), n / 2 + 1, "input length must be size/2+1");
        assert_eq!(output.len(), n, "output length must equal FFT size");

        // Reconstruct full complex spectrum using Hermitian symmetry
        let mut full_spec = vec![ComplexFloat::new(0.0, 0.0); n];
        full_spec[..(n / 2 + 1)].copy_from_slice(input);
        for k in (1..(n / 2)).rev() {
            full_spec[n - k] = ComplexFloat::new(full_spec[k].re, -full_spec[k].im);
        }

        let mut tmp_time = vec![ComplexFloat::new(0.0, 0.0); n];
        FFT::new(n).inverse(&full_spec, &mut tmp_time);
        for (i, c) in tmp_time.into_iter().enumerate() {
            output[i] = c.re;
        }
    }
}


