/// Simple fractional-delay line for single channel audio.
pub struct Delay {
    buffer: Vec<f32>,
    write_index: usize,
}

impl Delay {
    /// Create a delay line with a given maximum delay (in samples)
    pub fn new(max_delay_samples: i32) -> Self {
        let capacity = max_delay_samples.max(1) as usize + 1;
        Self {
            buffer: vec![0.0; capacity],
            write_index: 0,
        }
    }

    /// Process one sample, returning the delayed sample for the given delay length.
    /// Supports fractional delay using linear interpolation.
    pub fn process(&mut self, input: f32, delay_samples: f32) -> f32 {
        let len = self.buffer.len();
        // Write input into buffer
        self.buffer[self.write_index] = input;

        // Compute read index with wrap-around, with fractional part
        let delay = delay_samples.max(0.0);
        let read_pos = self.write_index as f32 - delay;
        let read_pos = if read_pos >= 0.0 { read_pos } else { read_pos + len as f32 };

        let i0 = read_pos.floor() as usize % len;
        let i1 = (i0 + 1) % len;
        let frac = read_pos - read_pos.floor();
        let y = self.buffer[i0] * (1.0 - frac) + self.buffer[i1] * frac;

        // Advance write index
        self.write_index = (self.write_index + 1) % len;
        y
    }
}

/// Multi-channel wrapper around `Delay` with independent state per channel.
pub struct MultiDelay {
    channels: usize,
    delays: Vec<Delay>,
}

impl MultiDelay {
    pub fn new(channels: usize, max_delay_samples: i32) -> Self {
        let delays = (0..channels).map(|_| Delay::new(max_delay_samples)).collect();
        Self { channels, delays }
    }

    /// Process one frame across all channels.
    pub fn process(&mut self, input: &[f32], output: &mut [f32], delay_samples: f32) {
        assert_eq!(input.len(), self.channels, "input channels mismatch");
        assert_eq!(output.len(), self.channels, "output channels mismatch");
        for ch in 0..self.channels {
            output[ch] = self.delays[ch].process(input[ch], delay_samples);
        }
    }
}


