// This example demonstrates using the FFT component to analyze 
// the spectrum of an audio signal.

use ssstretch::dsp::fft::{FFT, RealFFT};
use ssstretch::ComplexFloat;
use std::f32::consts::PI;

fn main() {
    // FFT parameters
    let min_fft_size = 1024;
    let fft_size = FFT::optimal_size(min_fft_size);
    
    // Create test signal - a 100 Hz sine wave
    let sample_rate = 44100.0;
    let frequency = 100.0;
    
    println!("FFT Example");
    println!("===========");
    println!("Sample Rate: {} Hz", sample_rate);
    println!("FFT Size: {}", fft_size);
    println!("Test Signal: {} Hz sine wave", frequency);
    
    // Create input signal (sine wave)
    let mut input = vec![0.0; fft_size as usize];
    for i in 0..fft_size as usize {
        let t = i as f32 / sample_rate;
        input[i] = (2.0 * PI * frequency * t).sin();
    }
    
    // Create FFT output buffer
    let output_bins = (fft_size / 2 + 1) as usize;
    let mut output = vec![ComplexFloat::new(0.0, 0.0); output_bins];
    
    // Create and run the real FFT
    let mut fft = RealFFT::new(fft_size);
    fft.forward(&input, &mut output);
    
    // Find the peak bin
    let mut peak_bin = 0;
    let mut peak_magnitude = 0.0;
    
    for (i, complex) in output.iter().enumerate() {
        let magnitude = complex.norm();
        if magnitude > peak_magnitude {
            peak_magnitude = magnitude;
            peak_bin = i;
        }
    }
    
    // Calculate the frequency of the peak bin
    let bin_freq = peak_bin as f32 * sample_rate / fft_size as f32;
    
    println!();
    println!("Peak bin: {}", peak_bin);
    println!("Peak frequency: {:.2} Hz", bin_freq);
    println!("Peak magnitude: {:.6}", peak_magnitude);
    
    // Print spectrum magnitude for the first few bins
    println!();
    println!("Spectrum Analysis:");
    println!("{:<10} {:<15} {:<15}", "Bin", "Frequency (Hz)", "Magnitude");
    
    for i in 0..10 {
        let bin_freq = i as f32 * sample_rate / fft_size as f32;
        println!("{:<10} {:<15.2} {:<15.6}", i, bin_freq, output[i].norm());
    }
    
    // Now perform a complex FFT on a synthetic complex signal
    println!();
    println!("Complex FFT Example");
    println!("==================");
    
    // Create a complex exponential (perfect circle in complex plane)
    let mut complex_input = vec![ComplexFloat::new(0.0, 0.0); fft_size as usize];
    for i in 0..fft_size as usize {
        let t = i as f32 / sample_rate;
        let phase = 2.0 * PI * frequency * t;
        complex_input[i] = ComplexFloat::new(phase.cos(), phase.sin());
    }
    
    let mut complex_output = vec![ComplexFloat::new(0.0, 0.0); fft_size as usize];
    
    // Run the complex FFT
    let mut complex_fft = FFT::new(fft_size);
    complex_fft.forward(&complex_input, &mut complex_output);
    
    // Find the peak in the complex spectrum
    let mut complex_peak_bin = 0;
    let mut complex_peak_magnitude = 0.0;
    
    for (i, complex) in complex_output.iter().enumerate() {
        let magnitude = complex.norm();
        if magnitude > complex_peak_magnitude {
            complex_peak_magnitude = magnitude;
            complex_peak_bin = i;
        }
    }
    
    // Calculate the frequency of the peak bin
    let complex_bin_freq = if complex_peak_bin as usize <= fft_size as usize / 2 {
        complex_peak_bin as f32 * sample_rate / fft_size as f32
    } else {
        (complex_peak_bin as f32 - fft_size as f32) * sample_rate / fft_size as f32
    };
    
    println!("Complex Peak bin: {}", complex_peak_bin);
    println!("Complex Peak frequency: {:.2} Hz", complex_bin_freq);
    println!("Complex Peak magnitude: {:.6}", complex_peak_magnitude);
}