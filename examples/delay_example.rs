// This example demonstrates using the delay components for audio processing.

use ssstretch::dsp::delay::{Delay, MultiDelay};
use std::f32::consts::PI;

fn main() {
    // Sample parameters
    let sample_rate = 44100.0;
    
    // Create a simple delay line with maximum delay of 1 second
    let max_delay_samples = sample_rate as i32;
    let mut delay = Delay::new(max_delay_samples);
    
    // Process a simple impulse with different delay times
    println!("Single Channel Delay Example");
    println!("===========================");
    println!("Sample Rate: {} Hz", sample_rate);
    println!("Maximum Delay: {} samples ({:.2} seconds)", max_delay_samples, max_delay_samples as f32 / sample_rate);
    
    // Creating an impulse
    let impulse_input = [1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
    
    // Process with a 10 sample delay
    process_and_print_delay(&mut delay, &impulse_input, 10.0, "10 samples");
    
    // Process with a 5.5 sample delay (fractional delay with interpolation)
    process_and_print_delay(&mut delay, &impulse_input, 5.5, "5.5 samples (interpolated)");
    
    // Multi-channel delay example
    println!("\nMulti-Channel Delay Example");
    println!("===========================");
    
    // Create a stereo delay line
    let channels = 2;
    let mut multi_delay = MultiDelay::new(channels, max_delay_samples);
    
    // Create a stereo input signal - impulse on left, silence on right
    process_and_print_multi_delay(&mut multi_delay, &[1.0, 0.0], 10.0, "10 samples");
    
    // Now process a short musical phrase with echo
    println!("\nMusical Phrase with Echo");
    println!("=======================");
    
    // Generate a short musical phrase
    let phrase_length = sample_rate as usize / 2; // 0.5 seconds
    let mut phrase = Vec::with_capacity(phrase_length);
    
    // Generate a simple sine wave melody
    let notes = [440.0, 493.88, 523.25, 587.33]; // A4, B4, C5, D5
    let note_duration = phrase_length / notes.len();
    
    for (i, &note) in notes.iter().enumerate() {
        for j in 0..note_duration {
            let t = (i * note_duration + j) as f32 / sample_rate;
            phrase.push((2.0 * PI * note * t).sin() * 0.5);
        }
    }
    
    // Add 0.5 seconds of silence for the echo to fade
    let mut output = vec![0.0; phrase_length * 2];
    
    // Reset the delay line
    let mut echo_delay = Delay::new(max_delay_samples);
    
    // Process the phrase with a 250ms echo
    let echo_delay_samples = (0.25 * sample_rate) as f32;
    let echo_feedback = 0.5; // 50% feedback
    
    // Process each sample through the delay line with feedback
    for i in 0..phrase_length * 2 {
        let input_sample = if i < phrase_length { phrase[i] } else { 0.0 };
        
        // Get the delayed output
        let delayed = echo_delay.process(input_sample, echo_delay_samples);
        
        // Apply feedback
        let output_sample = input_sample + delayed * echo_feedback;
        output[i] = output_sample;
        
        // The output becomes the next input with feedback
        echo_delay.process(output_sample * echo_feedback, echo_delay_samples);
    }
    
    // Print a few samples from the beginning and middle of the processed phrase
    println!("Original phrase with 250ms echo and 50% feedback:");
    println!("{:<10} {:<15}", "Sample #", "Output");
    
    for i in 0..5 {
        println!("{:<10} {:<15.6}", i, output[i]);
    }
    
    println!("...");
    
    // Sample near the echo point
    let echo_point = phrase_length + (echo_delay_samples as usize) - 5;
    for i in echo_point..echo_point + 10 {
        if i < output.len() {
            println!("{:<10} {:<15.6}", i, output[i]);
        }
    }
}

// Process a simple impulse through a delay line and print the results
fn process_and_print_delay(delay: &mut Delay, impulse: &[f32], delay_samples: f32, label: &str) {
    println!("\nDelay Time: {} ({})", delay_samples, label);
    println!("{:<10} {:<15} {:<15}", "Sample #", "Input", "Output");
    
    let mut output = vec![0.0; impulse.len()];
    
    // Process each sample
    for i in 0..impulse.len() {
        output[i] = delay.process(impulse[i], delay_samples);
        println!("{:<10} {:<15.1} {:<15.6}", i, impulse[i], output[i]);
    }
}

// Process a stereo impulse through a multi-channel delay line
fn process_and_print_multi_delay(delay: &mut MultiDelay, impulse: &[f32], delay_samples: f32, label: &str) {
    println!("\nStereo Delay Time: {} ({})", delay_samples, label);
    println!("{:<10} {:<15} {:<15} {:<15}", "Sample #", "Input L", "Input R", "Output");
    
    let mut output = vec![0.0, 0.0];
    
    // Process 10 samples
    for i in 0..10 {
        let input = if i == 0 { impulse } else { &[0.0, 0.0] };
        delay.process(input, &mut output, delay_samples);
        println!("{:<10} {:<15.1} {:<15.1} {:<15.6} {:<15.6}", 
                i, input[0], input[1], output[0], output[1]);
    }
}