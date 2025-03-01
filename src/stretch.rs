use crate::ffi;
use std::array;
use std::marker::PhantomData;

/// Configuration builder for Stretch.
///
/// Used to configure a Stretch instance with specific parameters.
///
/// The type parameter C specifies the number of audio channels.
pub struct StretchBuilder<const C: usize> {
    inner: cxx::UniquePtr<ffi::SignalsmithStretchFloat>,
}

impl<const C: usize> StretchBuilder<C> {
    /// Create a new builder with default settings for C channels.
    pub fn new() -> Self {
        Self {
            inner: ffi::new_signalsmith_stretch(),
        }
    }

    /// Create a builder with a specific random seed for C channels.
    pub fn with_seed(seed: i64) -> Self {
        Self {
            inner: ffi::new_signalsmith_stretch_with_seed(seed),
        }
    }

    /// Configure with default presets based on sample rate.
    pub fn preset_default(mut self, sample_rate: f32) -> Self {
        self.inner.pin_mut().presetDefault(C as i32, sample_rate);
        self
    }

    /// Configure with cheaper presets based on sample rate (less CPU intensive).
    pub fn preset_cheaper(mut self, sample_rate: f32) -> Self {
        self.inner.pin_mut().presetCheaper(C as i32, sample_rate);
        self
    }

    /// Manually configure the stretcher with specific parameters.
    pub fn configure(mut self, block_samples: i32, interval_samples: i32) -> Self {
        self.inner
            .pin_mut()
            .configure(C as i32, block_samples, interval_samples);
        self
    }

    /// Set the frequency multiplier and an optional tonality limit.
    pub fn transpose_factor(mut self, multiplier: f32, tonality_limit: Option<f32>) -> Self {
        self.inner
            .pin_mut()
            .setTransposeFactor(multiplier, tonality_limit.unwrap_or(0.0));
        self
    }

    /// Set the frequency shift in semitones and an optional tonality limit.
    pub fn transpose_semitones(mut self, semitones: f32, tonality_limit: Option<f32>) -> Self {
        self.inner
            .pin_mut()
            .setTransposeSemitones(semitones, tonality_limit.unwrap_or(0.0));
        self
    }

    /// Build a Stretch instance with the configured parameters.
    pub fn build(self) -> Stretch<C> {
        Stretch {
            inner: self.inner,
            _marker: PhantomData,
        }
    }
}

impl<const C: usize> Default for StretchBuilder<C> {
    fn default() -> Self {
        Self::new()
    }
}

/// Main struct for time-stretching and pitch-shifting audio.
///
/// This struct is generic over the number of channels, which is
/// statically known at compile time for better performance.
///
/// Use the `StretchBuilder` to configure and create instances.
pub struct Stretch<const CHANNELS: usize> {
    pub(crate) inner: cxx::UniquePtr<ffi::SignalsmithStretchFloat>,
    pub(crate) _marker: PhantomData<[(); CHANNELS]>,
}

impl<const CHANNELS: usize> Stretch<CHANNELS> {
    /// Create a new Stretch instance with default configuration for
    /// the specified number of channels and sample rate.
    pub fn new(sample_rate: f32) -> Self {
        StretchBuilder::<CHANNELS>::new()
            .preset_default(sample_rate)
            .build()
    }

    /// Create a new Stretch instance with a specified random seed.
    pub fn with_seed(seed: i64, sample_rate: f32) -> Self {
        StretchBuilder::<CHANNELS>::with_seed(seed)
            .preset_default(sample_rate)
            .build()
    }

    /// Reset the instance to its initial state.
    pub fn reset(&mut self) {
        self.inner.pin_mut().reset();
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
        self.inner
            .pin_mut()
            .setTransposeFactor(multiplier, tonality_limit.unwrap_or(0.0));
    }

    /// Set the frequency shift in semitones and an optional tonality limit.
    pub fn set_transpose_semitones(&mut self, semitones: f32, tonality_limit: Option<f32>) {
        self.inner
            .pin_mut()
            .setTransposeSemitones(semitones, tonality_limit.unwrap_or(0.0));
    }

    /// Process audio data, stretching time and/or shifting pitch.
    ///
    /// Takes arrays of input and output channel arrays. Each array represents one audio channel.
    /// The time stretch ratio is determined by the ratio of input to output lengths.
    ///
    /// # Panics
    ///
    /// Panics if the input arrays have different lengths, or if the output arrays have different lengths.
    pub fn process<'input, 'output: 'input>(
        &mut self,
        input_channels: [&'input [f32]; CHANNELS],
        output_channels: &mut [&'output mut [f32]; CHANNELS],
    ) {
        // Create stack-allocated arrays of pointers - no heap allocation
        let input_ptrs: [*const f32; CHANNELS] = array::from_fn(|i| input_channels[i].as_ptr());
        let mut output_ptrs: [*mut f32; CHANNELS] =
            array::from_fn(|i| output_channels[i].as_mut_ptr());

        let input_samples = input_channels[0].len() as i32;
        let output_samples = output_channels[0].len() as i32;

        debug_assert!(
            input_channels
                .iter()
                .all(|samples| samples.len() == input_samples as usize),
            "input channels vary in sample length"
        );

        debug_assert!(
            output_channels
                .iter()
                .all(|samples| samples.len() == output_samples as usize),
            "output channels vary in buffer length"
        );

        // Make the FFI call using our raw method
        unsafe {
            self.process_raw(
                input_ptrs.as_ptr(),
                input_samples,
                output_ptrs.as_mut_ptr(),
                output_samples,
            );
        }
    }

    /// Provide previous input ("pre-roll") without affecting speed calculation.
    ///
    /// # Panics
    ///
    /// Panics if the input arrays have different lengths.
    pub fn seek(&mut self, inputs: [&[f32]; CHANNELS], playback_rate: f64) {
        // Create stack-allocated arrays of pointers - no heap allocation
        let mut input_ptrs = [std::ptr::null(); CHANNELS];

        for i in 0..CHANNELS {
            input_ptrs[i] = inputs[i].as_ptr();
        }

        let input_samples = inputs[0].len() as i32;

        // Verify all channels have the same length
        for (i, channel) in inputs.iter().enumerate() {
            assert_eq!(
                channel.len(),
                input_samples as usize,
                "Input channel {} has different length than channel 0",
                i
            );
        }

        // Make the FFI call using our raw method
        self.seek_raw(input_ptrs.as_ptr(), input_samples, playback_rate);
    }

    /// Flush remaining output data.
    ///
    /// # Panics
    ///
    /// Panics if the output arrays have different lengths.
    pub fn flush(&mut self, outputs: [&mut [f32]; CHANNELS]) {
        // Create stack-allocated arrays of pointers - no heap allocation
        let mut output_ptrs = [std::ptr::null_mut(); CHANNELS];

        for i in 0..CHANNELS {
            output_ptrs[i] = outputs[i].as_mut_ptr();
        }

        let output_samples = outputs[0].len() as i32;

        // Verify all channels have the same length
        for (i, channel) in outputs.iter().enumerate() {
            assert_eq!(
                channel.len(),
                output_samples as usize,
                "Output channel {} has different length than channel 0",
                i
            );
        }

        // Make the FFI call using our raw method
        self.flush_raw(output_ptrs.as_mut_ptr(), output_samples);
    }
}

// For compatibility with Vec<Vec<f32>> format, we need a low-level processing interface
impl<const C: usize> Stretch<C> {
    // Low-level processing function that allows for safe handling of vectors
    unsafe fn process_raw(
        &mut self,
        input_ptrs: *const *const f32,
        input_samples: i32,
        output_ptrs: *mut *mut f32,
        output_samples: i32,
    ) {
        ffi::signalsmith_stretch_process(
            self.inner.pin_mut(),
            input_ptrs,
            input_samples,
            output_ptrs,
            output_samples,
            C as i32,
        );
    }

    fn seek_raw(&mut self, input_ptrs: *const *const f32, input_samples: i32, playback_rate: f64) {
        unsafe {
            ffi::signalsmith_stretch_seek(
                self.inner.pin_mut(),
                input_ptrs,
                input_samples,
                playback_rate,
                C as i32,
            );
        }
    }

    fn flush_raw(&mut self, output_ptrs: *mut *mut f32, output_samples: i32) {
        unsafe {
            ffi::signalsmith_stretch_flush(
                self.inner.pin_mut(),
                output_ptrs,
                output_samples,
                C as i32,
            );
        }
    }

    /// Process audio data with Vec<Vec<f32>> format (non-interleaved).
    ///
    /// Each inner Vec represents a channel of audio samples.
    /// The time stretch ratio is determined by the ratio of input_samples to output_samples.
    ///
    /// # Panics
    ///
    /// Panics if the number of input or output vectors is different from C.
    pub fn process_vec(
        &mut self,
        inputs: &[Vec<f32>],
        input_samples: i32,
        outputs: &mut [Vec<f32>],
        output_samples: i32,
    ) {
        assert_eq!(
            inputs.len(),
            C,
            "Expected {} input channels, got {}",
            C,
            inputs.len()
        );
        assert_eq!(
            outputs.len(),
            C,
            "Expected {} output channels, got {}",
            C,
            outputs.len()
        );

        // Ensure output vectors have enough capacity
        for channel in outputs.iter_mut() {
            channel.resize(output_samples as usize, 0.0);
        }

        // Create arrays of pointers to channel data
        let mut input_ptrs = Vec::with_capacity(C);
        let mut output_ptrs = Vec::with_capacity(C);

        for i in 0..C {
            input_ptrs.push(inputs[i].as_ptr());
        }

        for i in 0..C {
            output_ptrs.push(outputs[i].as_mut_ptr());
        }

        // Process using the low-level API
        unsafe {
            self.process_raw(
                input_ptrs.as_ptr(),
                input_samples,
                output_ptrs.as_mut_ptr(),
                output_samples,
            );
        }
    }

    /// Seek with Vec<Vec<f32>> format (non-interleaved).
    ///
    /// # Panics
    ///
    /// Panics if the number of input vectors is different from C.
    pub fn seek_vec(&mut self, inputs: &[Vec<f32>], input_samples: i32, playback_rate: f64) {
        assert_eq!(
            inputs.len(),
            C,
            "Expected {} input channels, got {}",
            C,
            inputs.len()
        );

        // Create arrays of pointers to channel data
        let mut input_ptrs = Vec::with_capacity(C);

        for i in 0..C {
            input_ptrs.push(inputs[i].as_ptr());
        }

        // Process using the low-level API
        self.seek_raw(input_ptrs.as_ptr(), input_samples, playback_rate);
    }

    /// Flush with Vec<Vec<f32>> format (non-interleaved).
    ///
    /// # Panics
    ///
    /// Panics if the number of output vectors is different from C.
    pub fn flush_vec(&mut self, outputs: &mut [Vec<f32>], output_samples: i32) {
        assert_eq!(
            outputs.len(),
            C,
            "Expected {} output channels, got {}",
            C,
            outputs.len()
        );

        // Ensure output vectors have enough capacity
        for channel in outputs.iter_mut() {
            channel.resize(output_samples as usize, 0.0);
        }

        // Create arrays of pointers to channel data
        let mut output_ptrs = Vec::with_capacity(C);

        for i in 0..C {
            output_ptrs.push(outputs[i].as_mut_ptr());
        }

        // Process using the low-level API
        self.flush_raw(output_ptrs.as_mut_ptr(), output_samples);
    }
}