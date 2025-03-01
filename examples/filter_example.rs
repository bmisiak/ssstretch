// This example demonstrates using the biquad filter component
// to create a simple filter chain for audio processing.

use std::f32::consts::PI;
use ssstretch::dsp::filters::{BiquadFilter, BiquadDesign};

fn main() {
    // Sample parameters
    let sample_rate = 44100.0;
    let duration_sec = 2.0;
    let num_samples = (duration_sec * sample_rate) as usize;
    
    // Create a test signal - a 440 Hz sine wave plus a 2000 Hz sine wave
    let mut input = generate_test_signal(440.0, 2000.0, sample_rate, num_samples);
    let mut output = vec![0.0; num_samples];
    
    // Create a lowpass filter at 1000 Hz with a Q of 0.7
    let mut filter = BiquadFilter::new();
    
    // Configure filter using normalized frequency (freq / sample_rate)
    let normalized_freq = 1000.0 / sample_rate;
    filter.lowpass(normalized_freq, 0.7, Some(BiquadDesign::Cookbook));
    
    // Process the audio through the filter
    filter.process_buffer(&input, &mut output);
    
    // Print the first few samples of input and output
    println!("Filter Example - Lowpass at 1000 Hz");
    println!("===================================");
    println!("Sample Rate: {} Hz", sample_rate);
    println!("Input: 440 Hz + 2000 Hz sine waves");
    println!();
    println!("First 10 samples:");
    println!("{:<10} {:<15} {:<15}", "Sample #", "Input", "Output");
    
    for i in 0..10 {
        println!("{:<10} {:<15.6} {:<15.6}", i, input[i], output[i]);
    }
    
    // Calculate signal power before and after filtering
    let input_power = calculate_signal_power(&input);
    let output_power = calculate_signal_power(&output);
    
    println!();
    println!("Signal power analysis:");
    println!("Input power:  {:.6}", input_power);
    println!("Output power: {:.6}", output_power);
    println!("Power ratio:  {:.6} dB", 10.0 * (output_power / input_power).log10());
}

// Generate a test signal consisting of two sine waves
fn generate_test_signal(freq1: f32, freq2: f32, sample_rate: f32, num_samples: usize) -> Vec<f32> {
    let mut signal = vec![0.0; num_samples];
    
    for i in 0..num_samples {
        let t = i as f32 / sample_rate;
        let sample1 = (2.0 * PI * freq1 * t).sin() * 0.5;
        let sample2 = (2.0 * PI * freq2 * t).sin() * 0.5;
        signal[i] = sample1 + sample2;
    }
    
    signal
}

// Calculate signal power (mean square)
fn calculate_signal_power(signal: &[f32]) -> f32 {
    let sum_squared: f32 = signal.iter().map(|&x| x * x).sum();
    sum_squared / signal.len() as f32
}