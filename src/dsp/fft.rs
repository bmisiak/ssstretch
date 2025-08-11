use crate::ComplexFloat;

// Feature-gated Rust FFT backend for examples and optional users
// Uses realfft for real transforms and rustfft for complex transforms
#[cfg(feature = "fft-rust")]
mod backend {
    use super::ComplexFloat;
    use realfft::{RealFftPlanner, RealToComplex, ComplexToReal};
    use rustfft::{FftPlanner, num_complex::Complex32};

    pub struct FFT {
        size: usize,
        forward: std::sync::Arc<dyn rustfft::Fft<Complex32>>,
        inverse: std::sync::Arc<dyn rustfft::Fft<Complex32>>,
        scratch: Vec<Complex32>,
    }

    impl FFT {
        pub fn new(size: usize) -> Self {
            let mut planner = FftPlanner::new();
            let forward = planner.plan_fft_forward(size);
            let inverse = planner.plan_fft_inverse(size);
            let scratch = vec![Complex32::new(0.0, 0.0); size.max(1)];
            Self { size, forward, inverse, scratch }
        }

        pub fn optimal_size(min_size: usize) -> usize { min_size.next_power_of_two() }

        pub fn forward(&mut self, input: &[ComplexFloat], output: &mut [ComplexFloat]) {
            assert_eq!(input.len(), self.size);
            assert_eq!(output.len(), self.size);
            let mut buf: Vec<Complex32> = input.iter().map(|c| Complex32::new(c.re, c.im)).collect();
            self.forward.process(&mut buf);
            for (o, c) in output.iter_mut().zip(buf.into_iter()) {
                *o = ComplexFloat::new(c.re, c.im);
            }
        }

        pub fn inverse(&mut self, input: &[ComplexFloat], output: &mut [ComplexFloat]) {
            assert_eq!(input.len(), self.size);
            assert_eq!(output.len(), self.size);
            let mut buf: Vec<Complex32> = input.iter().map(|c| Complex32::new(c.re, c.im)).collect();
            self.inverse.process(&mut buf);
            for (o, c) in output.iter_mut().zip(buf.into_iter()) {
                *o = ComplexFloat::new(c.re / self.size as f32, c.im / self.size as f32);
            }
        }
    }

    pub struct RealFFT {
        size: usize,
        r2c: std::sync::Arc<dyn RealToComplex<f32>>, 
        c2r: std::sync::Arc<dyn ComplexToReal<f32>>, 
        scratch_fwd: Vec<f32>,
        scratch_inv: Vec<f32>,
    }

    impl RealFFT {
        pub fn new(size: usize) -> Self {
            let mut planner = RealFftPlanner::<f32>::new();
            let r2c = planner.plan_fft_forward(size);
            let c2r = planner.plan_fft_inverse(size);
            let scratch_fwd = r2c.make_scratch_vec();
            let scratch_inv = c2r.make_scratch_vec();
            Self { size, r2c, c2r, scratch_fwd, scratch_inv }
        }

        pub fn forward(&mut self, input: &[f32], output: &mut [ComplexFloat]) {
            assert_eq!(input.len(), self.size);
            assert_eq!(output.len(), self.size / 2 + 1);
            let mut in_buf = self.r2c.make_input_vec();
            in_buf.copy_from_slice(input);
            let mut out_buf = self.r2c.make_output_vec();
            self.r2c.process_with_scratch(&mut in_buf, &mut out_buf, &mut self.scratch_fwd).unwrap();
            for (o, c) in output.iter_mut().zip(out_buf.into_iter()) {
                *o = ComplexFloat::new(c.re, c.im);
            }
        }

        pub fn inverse(&mut self, input: &[ComplexFloat], output: &mut [f32]) {
            assert_eq!(input.len(), self.size / 2 + 1);
            assert_eq!(output.len(), self.size);
            let mut in_buf = self.c2r.make_input_vec();
            for (i, c) in input.iter().enumerate() { in_buf[i] = rustfft::num_complex::Complex32::new(c.re, c.im); }
            let mut out_buf = self.c2r.make_output_vec();
            self.c2r.process_with_scratch(&mut in_buf, &mut out_buf, &mut self.scratch_inv).unwrap();
            let scale = 1.0 / self.size as f32;
            for (o, v) in output.iter_mut().zip(out_buf.into_iter()) { *o = v * scale; }
        }
    }

    pub use FFT as FFTImpl;
    pub use RealFFT as RealFFTImpl;
}

// Fallback naive backend (only used if feature disabled, though example is gated on the feature)
#[cfg(not(feature = "fft-rust"))]
mod backend {
    use super::ComplexFloat;
    pub struct FFT { size: usize }
    impl FFT {
        pub fn new(size: usize) -> Self { Self { size } }
        pub fn optimal_size(min_size: usize) -> usize { min_size.next_power_of_two() }
        pub fn forward(&mut self, input: &[ComplexFloat], output: &mut [ComplexFloat]) {
            let n = self.size; assert_eq!(input.len(), n); assert_eq!(output.len(), n);
            let two_pi_over_n = 2.0_f32 * std::f32::consts::PI / n as f32;
            for k in 0..n { let mut acc = ComplexFloat::new(0.0,0.0); for (n_idx, x) in input.iter().enumerate() {
                let angle = two_pi_over_n * (k as f32) * (n_idx as f32);
                let w = ComplexFloat::from_polar(1.0, -angle); acc += *x * w; } output[k] = acc; }
        }
        pub fn inverse(&mut self, input: &[ComplexFloat], output: &mut [ComplexFloat]) {
            let n = self.size; assert_eq!(input.len(), n); assert_eq!(output.len(), n);
            let two_pi_over_n = 2.0_f32 * std::f32::consts::PI / n as f32;
            for n_idx in 0..n { let mut acc = ComplexFloat::new(0.0,0.0); for (k, x) in input.iter().enumerate() {
                let angle = two_pi_over_n * (k as f32) * (n_idx as f32); let w = ComplexFloat::from_polar(1.0, angle); acc += *x * w; }
                output[n_idx] = ComplexFloat::new(acc.re / n as f32, acc.im / n as f32); }
        }
    }
    pub struct RealFFT { size: usize }
    impl RealFFT {
        pub fn new(size: usize) -> Self { Self { size } }
        pub fn forward(&mut self, input: &[f32], output: &mut [ComplexFloat]) {
            let n = self.size; assert_eq!(input.len(), n); assert_eq!(output.len(), n/2 + 1);
            let mut tmp_in = vec![ComplexFloat::new(0.0,0.0); n]; for (i,&v) in input.iter().enumerate(){ tmp_in[i]=ComplexFloat::new(v,0.0);} 
            let mut tmp_out = vec![ComplexFloat::new(0.0,0.0); n];
            FFT::new(n).forward(&tmp_in, &mut tmp_out);
            output.copy_from_slice(&tmp_out[..(n/2+1)]);
        }
        pub fn inverse(&mut self, input: &[ComplexFloat], output: &mut [f32]) {
            let n = self.size; assert_eq!(input.len(), n/2 + 1); assert_eq!(output.len(), n);
            let mut full_spec = vec![ComplexFloat::new(0.0,0.0); n]; full_spec[..(n/2+1)].copy_from_slice(input);
            for k in (1..(n/2)).rev(){ full_spec[n-k] = ComplexFloat::new(full_spec[k].re, -full_spec[k].im); }
            let mut tmp_time = vec![ComplexFloat::new(0.0,0.0); n]; FFT::new(n).inverse(&full_spec, &mut tmp_time);
            for (i,c) in tmp_time.into_iter().enumerate(){ output[i]=c.re; }
        }
    }
    pub use FFT as FFTImpl; pub use RealFFT as RealFFTImpl;
}

pub use backend::FFTImpl as FFT;
pub use backend::RealFFTImpl as RealFFT;


