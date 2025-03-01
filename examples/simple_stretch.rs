use ssstretch::Stretch;

fn main() {
    // Create a simple input buffer with two channels of a sine wave
    let sample_rate = 44100.0;
    let frequency = 440.0; // A4 note
    let duration_seconds = 2.0;
    let sample_count = (sample_rate * duration_seconds) as usize;
    
    // Generate two channels of audio
    let mut input = vec![vec![0.0f32; sample_count], vec![0.0f32; sample_count]];
    for i in 0..sample_count {
        let t = i as f32 / sample_rate;
        let value = (t * frequency * 2.0 * std::f32::consts::PI).sin();
        
        // Channel 1 - original sine wave
        input[0][i] = value;
        
        // Channel 2 - phase-shifted version
        input[1][i] = ((t + 0.25) * frequency * 2.0 * std::f32::consts::PI).sin();
    }
    
    // Create a Stretch instance with 2 channels
    let mut stretch = Stretch::new();
    stretch.preset_default(2, sample_rate as f32);
    
    // Set pitch shift of 1 octave up (12 semitones)
    stretch.set_transpose_semitones(12.0, None);
    
    // Create output buffer that's half the length (2x speed)
    let output_samples = sample_count / 2;
    let mut output = vec![vec![0.0f32; output_samples], vec![0.0f32; output_samples]];
    
    // Process audio
    println!("Processing {} input samples into {} output samples...", sample_count, output_samples);
    
    stretch.process_vec(
        &input,
        sample_count as i32,
        &mut output,
        output_samples as i32,
    );
    
    // We'd normally write this to a file or audio device
    // For this example, we'll just report some statistics
    
    println!("Processing complete!");
    println!("Input amplitude: {:.4}", input[0].iter().map(|x| x.abs()).fold(0.0f32, |a, b| a.max(b)));
    println!("Output amplitude: {:.4}", output[0].iter().map(|x| x.abs()).fold(0.0f32, |a, b| a.max(b)));
    let latency = stretch.input_latency() as usize;
    println!("Input latency: {} samples", latency);
    println!("Output latency: {} samples", stretch.output_latency());
    println!("First few samples of output channel 1: {:?}", &output[0][0..10]);
    println!("Samples after latency: {:?}", &output[0][latency.min(output[0].len() - 1)..latency.min(output[0].len()) + 10.min(output[0].len() - latency)]);
}