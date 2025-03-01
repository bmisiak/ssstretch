use ssstretch::Stretch;

fn main() {
    // Define audio parameters
    const CHANNELS: usize = 2;
    const SAMPLE_RATE: f32 = 44100.0;
    const FREQUENCY: f32 = 440.0; // A4 note
    const DURATION_SECONDS: f32 = 2.0;
    const SAMPLE_COUNT: usize = (SAMPLE_RATE * DURATION_SECONDS) as usize;
    
    // Generate test audio (stereo sine waves)
    let mut input = [
        vec![0.0f32; SAMPLE_COUNT],
        vec![0.0f32; SAMPLE_COUNT]
    ];
    
    for i in 0..SAMPLE_COUNT {
        let t = i as f32 / SAMPLE_RATE;
        
        // Left channel - original sine wave
        input[0][i] = (t * FREQUENCY * 2.0 * std::f32::consts::PI).sin();
        
        // Right channel - phase-shifted version
        input[1][i] = ((t + 0.25) * FREQUENCY * 2.0 * std::f32::consts::PI).sin();
    }
    
    // Create a Stretch instance using the builder pattern
    let mut stretch = Stretch::<CHANNELS>::new(SAMPLE_RATE);
    
    // Set pitch shift of 1 octave up (12 semitones)
    stretch.set_transpose_semitones(12.0, None);
    
    // Calculate output size (2x speed = half the length)
    let output_samples = SAMPLE_COUNT / 2;
    let mut output = [
        vec![0.0f32; output_samples],
        vec![0.0f32; output_samples]
    ];
    
    // Process audio with our generic Stretch<2> implementation
    println!("Processing {} input samples into {} output samples...", SAMPLE_COUNT, output_samples);
    
    // Process using process_vec to avoid borrow checker issues
    stretch.process_vec(
        &input,
        SAMPLE_COUNT as i32,
        &mut output,
        output_samples as i32
    );
    
    // Report results
    println!("Processing complete!");
    println!("Input latency: {} samples", stretch.input_latency());
    println!("Output latency: {} samples", stretch.output_latency());
    
    // Show a few samples of output after latency
    let latency = stretch.input_latency() as usize;
    
    if latency < output_samples {
        println!("First few samples after latency:");
        for i in 0..4.min(output_samples - latency) {
            println!("  Sample {}: Left={:.4}, Right={:.4}", 
                     i, 
                     output[0][latency + i], 
                     output[1][latency + i]);
        }
    }
    
    // We don't need a separate test anymore as we're already using process_vec
    println!("\nProcessing complete!");
}