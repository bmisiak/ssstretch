//! Rust bindings for the Signalsmith Stretch time-stretching and pitch-shifting library.
//! 
//! This crate provides idiomatic Rust bindings for the C++ library, allowing for 
//! time stretching and pitch shifting of audio samples.


#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("ssstretch/src/bridge.h");
        
        // Opaque C++ type
        type SignalsmithStretchFloat;
        
        // Factory functions
        fn new_signalsmith_stretch() -> UniquePtr<SignalsmithStretchFloat>;
        fn new_signalsmith_stretch_with_seed(seed: i64) -> UniquePtr<SignalsmithStretchFloat>;
        
        // Object properties
        fn blockSamples(self: &SignalsmithStretchFloat) -> i32;
        fn intervalSamples(self: &SignalsmithStretchFloat) -> i32;
        fn inputLatency(self: &SignalsmithStretchFloat) -> i32;
        fn outputLatency(self: &SignalsmithStretchFloat) -> i32;
        
        // Configuration methods
        fn reset(self: Pin<&mut SignalsmithStretchFloat>);
        fn presetDefault(self: Pin<&mut SignalsmithStretchFloat>, nChannels: i32, sampleRate: f32);
        fn presetCheaper(self: Pin<&mut SignalsmithStretchFloat>, nChannels: i32, sampleRate: f32);
        fn configure(self: Pin<&mut SignalsmithStretchFloat>, nChannels: i32, blockSamples: i32, intervalSamples: i32);
        
        // Pitch shifting methods
        fn setTransposeFactor(self: Pin<&mut SignalsmithStretchFloat>, multiplier: f32, tonalityLimit: f32);
        fn setTransposeSemitones(self: Pin<&mut SignalsmithStretchFloat>, semitones: f32, tonalityLimit: f32);
        
        // Processing wrapper functions
        unsafe fn signalsmith_stretch_process(
            stretch: Pin<&mut SignalsmithStretchFloat>, 
            inputs: *const *const f32, 
            inputSamples: i32,
            outputs: *mut *mut f32, 
            outputSamples: i32,
            channels: i32
        );
        
        unsafe fn signalsmith_stretch_seek(
            stretch: Pin<&mut SignalsmithStretchFloat>,
            inputs: *const *const f32,
            inputSamples: i32,
            playbackRate: f64,
            channels: i32
        );
        
        unsafe fn signalsmith_stretch_flush(
            stretch: Pin<&mut SignalsmithStretchFloat>,
            outputs: *mut *mut f32,
            outputSamples: i32,
            channels: i32
        );
    }
}

/// Main struct for time-stretching and pitch-shifting audio.
/// 
/// This struct wraps the C++ SignalsmithStretch class and provides
/// an idiomatic Rust interface.
pub struct Stretch {
    inner: cxx::UniquePtr<ffi::SignalsmithStretchFloat>,
    channels: i32, // We need to track this ourselves
}

impl Stretch {
    /// Create a new Stretch instance with default parameters.
    pub fn new() -> Self {
        Self {
            inner: ffi::new_signalsmith_stretch(),
            channels: 0,
        }
    }
    
    /// Create a new Stretch instance with a specified random seed.
    pub fn with_seed(seed: i64) -> Self {
        Self {
            inner: ffi::new_signalsmith_stretch_with_seed(seed),
            channels: 0,
        }
    }
    
    /// Reset the instance to its initial state.
    pub fn reset(&mut self) {
        self.inner.pin_mut().reset();
    }
    
    /// Configure with default presets based on sample rate.
    pub fn preset_default(&mut self, channels: i32, sample_rate: f32) {
        self.channels = channels;
        self.inner.pin_mut().presetDefault(channels, sample_rate);
    }
    
    /// Configure with cheaper presets based on sample rate (less CPU intensive).
    pub fn preset_cheaper(&mut self, channels: i32, sample_rate: f32) {
        self.channels = channels;
        self.inner.pin_mut().presetCheaper(channels, sample_rate);
    }
    
    /// Manually configure the stretcher.
    pub fn configure(&mut self, channels: i32, block_samples: i32, interval_samples: i32) {
        self.channels = channels;
        self.inner.pin_mut().configure(channels, block_samples, interval_samples);
    }
    
    /// Get the number of channels.
    pub fn channels(&self) -> i32 {
        self.channels
    }
    
    /// Get the block size in samples.
    pub fn block_samples(&self) -> i32 {
        self.inner.blockSamples()
    }
    
    /// Get the interval size in samples.
    pub fn interval_samples(&self) -> i32 {
        self.inner.intervalSamples()
    }
    
    /// Get the input latency in samples.
    pub fn input_latency(&self) -> i32 {
        self.inner.inputLatency()
    }
    
    /// Get the output latency in samples.
    pub fn output_latency(&self) -> i32 {
        self.inner.outputLatency()
    }
    
    /// Set the frequency multiplier and an optional tonality limit.
    pub fn set_transpose_factor(&mut self, multiplier: f32, tonality_limit: Option<f32>) {
        self.inner.pin_mut().setTransposeFactor(multiplier, tonality_limit.unwrap_or(0.0));
    }
    
    /// Set the frequency shift in semitones and an optional tonality limit.
    pub fn set_transpose_semitones(&mut self, semitones: f32, tonality_limit: Option<f32>) {
        self.inner.pin_mut().setTransposeSemitones(semitones, tonality_limit.unwrap_or(0.0));
    }
    
    /// Process audio data, stretching time and/or shifting pitch.
    /// 
    /// Input and output must be properly sized arrays of channel pointers.
    /// Each channel should have the specified number of samples.
    /// 
    /// # Safety
    /// 
    /// This method is unsafe because it works with raw pointers. The caller must ensure:
    /// - `inputs` points to an array of `self.channels()` valid pointers to channel data
    /// - `outputs` points to an array of `self.channels()` valid pointers to output buffers
    /// - Each input channel has at least `input_samples` elements
    /// - Each output channel has at least `output_samples` elements
    pub unsafe fn process(
        &mut self,
        inputs: &[*const f32],
        input_samples: i32,
        outputs: &mut [*mut f32],
        output_samples: i32,
    ) {
        assert!(inputs.len() >= self.channels as usize, "Not enough input channels");
        assert!(outputs.len() >= self.channels as usize, "Not enough output channels");
        
        ffi::signalsmith_stretch_process(
            self.inner.pin_mut(),
            inputs.as_ptr(),
            input_samples,
            outputs.as_mut_ptr(),
            output_samples,
            self.channels,
        );
    }
    
    /// Provide previous input ("pre-roll") without affecting speed calculation.
    /// 
    /// # Safety
    /// 
    /// Same safety requirements as `process`.
    pub unsafe fn seek(
        &mut self,
        inputs: &[*const f32],
        input_samples: i32,
        playback_rate: f64,
    ) {
        assert!(inputs.len() >= self.channels as usize, "Not enough input channels");
        
        ffi::signalsmith_stretch_seek(
            self.inner.pin_mut(),
            inputs.as_ptr(),
            input_samples,
            playback_rate,
            self.channels,
        );
    }
    
    /// Flush remaining output data.
    /// 
    /// # Safety
    /// 
    /// Same safety requirements as `process` for the output buffers.
    pub unsafe fn flush(
        &mut self,
        outputs: &mut [*mut f32],
        output_samples: i32,
    ) {
        assert!(outputs.len() >= self.channels as usize, "Not enough output channels");
        
        ffi::signalsmith_stretch_flush(
            self.inner.pin_mut(),
            outputs.as_mut_ptr(),
            output_samples,
            self.channels,
        );
    }
}

impl Default for Stretch {
    fn default() -> Self {
        Self::new()
    }
}

// Helper for working with audio data in Vec<Vec<f32>> format
impl Stretch {
    /// Process audio data with Vec<Vec<f32>> format.
    /// 
    /// Each inner Vec represents a channel of audio samples.
    /// The time stretch ratio is determined by the ratio of input_samples to output_samples.
    pub fn process_vec(
        &mut self,
        inputs: &[Vec<f32>],
        input_samples: i32,
        outputs: &mut [Vec<f32>],
        output_samples: i32,
    ) {
        let channels = self.channels() as usize;
        assert!(inputs.len() >= channels, "Not enough input channels");
        assert!(outputs.len() >= channels, "Not enough output channels");
        
        // Ensure output vectors have enough capacity
        for channel in outputs.iter_mut().take(channels) {
            channel.resize(output_samples as usize, 0.0);
        }
        
        // Create arrays of pointers to channel data
        let input_ptrs: Vec<*const f32> = inputs.iter()
            .map(|channel| channel.as_ptr())
            .collect();
            
        let mut output_ptrs: Vec<*mut f32> = outputs.iter_mut()
            .map(|channel| channel.as_mut_ptr())
            .collect();
        
        // Process the audio
        unsafe {
            self.process(
                &input_ptrs,
                input_samples,
                &mut output_ptrs,
                output_samples,
            );
        }
    }
    
    /// Seek with Vec<Vec<f32>> format.
    pub fn seek_vec(
        &mut self,
        inputs: &[Vec<f32>],
        input_samples: i32,
        playback_rate: f64,
    ) {
        let channels = self.channels() as usize;
        assert!(inputs.len() >= channels, "Not enough input channels");
        
        // Create arrays of pointers to channel data
        let input_ptrs: Vec<*const f32> = inputs.iter()
            .map(|channel| channel.as_ptr())
            .collect();
        
        // Process the seek
        unsafe {
            self.seek(
                &input_ptrs,
                input_samples,
                playback_rate,
            );
        }
    }
    
    /// Flush with Vec<Vec<f32>> format.
    pub fn flush_vec(
        &mut self,
        outputs: &mut [Vec<f32>],
        output_samples: i32,
    ) {
        let channels = self.channels() as usize;
        assert!(outputs.len() >= channels, "Not enough output channels");
        
        // Ensure output vectors have enough capacity
        for channel in outputs.iter_mut().take(channels) {
            channel.resize(output_samples as usize, 0.0);
        }
        
        // Create arrays of pointers to channel data
        let mut output_ptrs: Vec<*mut f32> = outputs.iter_mut()
            .map(|channel| channel.as_mut_ptr())
            .collect();
        
        // Flush the output
        unsafe {
            self.flush(
                &mut output_ptrs,
                output_samples,
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_stretch() {
        let mut stretch = Stretch::new();
        stretch.preset_default(2, 44100.0);
        assert_eq!(stretch.channels(), 2);
    }
}
