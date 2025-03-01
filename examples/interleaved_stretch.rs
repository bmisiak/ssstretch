use ssstretch::Stretch;

fn main() {
    // Create a simple interleaved input buffer with two channels
    let sample_rate = 44100.0;
    let frequency = 440.0; // A4 note
    let duration_seconds = 2.0;
    let sample_count = (sample_rate * duration_seconds) as usize;
    let channels = 2;
    
    // Generate interleaved audio (stereo - 2 channels)
    // Format is [L, R, L, R, L, R, ...] where L=left, R=right
    let mut input = Vec::with_capacity(sample_count * channels);
    for i in 0..sample_count {
        let t = i as f32 / sample_rate;
        
        // Left channel (sine wave)
        input.push((t * frequency * 2.0 * std::f32::consts::PI).sin());
        
        // Right channel (phase-shifted version)
        input.push(((t + 0.25) * frequency * 2.0 * std::f32::consts::PI).sin());
    }
    
    // Create a Stretch instance with 2 channels using the builder
    let mut stretch = Stretch::<2>::new(sample_rate as f32);
    
    // Set pitch shift of 1 octave up (12 semitones)
    stretch.set_transpose_semitones(12.0, None);
    
    // Create output buffer that's half the length (2x speed)
    let output_frames = sample_count / 2;
    
    // Use Vec<Vec<f32>> format instead of interleaved
    let mut input_channels = vec![vec![0.0f32; sample_count], vec![0.0f32; sample_count]];
    let mut output_channels = vec![vec![0.0f32; output_frames], vec![0.0f32; output_frames]];
    
    // De-interleave input
    for i in 0..sample_count {
        input_channels[0][i] = input[i * channels];
        input_channels[1][i] = input[i * channels + 1];
    }
    
    // Process audio
    println!("Processing {} input frames into {} output frames...", 
             sample_count, output_frames);
    
    stretch.process_vec(
        &input_channels,
        sample_count as i32,
        &mut output_channels,
        output_frames as i32,
    );
    
    // Re-interleave output for consistency with the rest of the example
    let mut output = vec![0.0f32; output_frames * channels];
    for i in 0..output_frames {
        output[i * channels] = output_channels[0][i];
        output[i * channels + 1] = output_channels[1][i];
    }
    
    // We'd normally write this to a file or audio device
    // For this example, we'll just report some statistics
    
    println!("Processing complete!");
    println!("Input size: {} samples ({} per channel)", input.len(), sample_count);
    println!("Output size: {} samples ({} per channel)", output.len(), output_frames);
    
    let input_max = input.iter().fold(0.0f32, |a, b| a.max(b.abs()));
    let output_max = output.iter().fold(0.0f32, |a, b| a.max(b.abs()));
    
    println!("Input amplitude: {:.4}", input_max);
    println!("Output amplitude: {:.4}", output_max);
    
    // Show a few samples from the start after latency
    let latency = stretch.input_latency() as usize;
    println!("Input latency: {} frames", latency);
    
    // Show a few samples of output
    let output_offset = latency;
    if output_offset < output_frames {
        println!("First few frames after latency:");
        for i in 0..4.min(output_frames - output_offset) {
            println!("  Frame {}: Left={:.4}, Right={:.4}", 
                    i, 
                    output[(i + output_offset) * channels], 
                    output[(i + output_offset) * channels + 1]);
        }
    }
}