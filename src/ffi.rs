#[cxx::bridge]
pub mod bindings {
    unsafe extern "C++" {
        include!("bridge.h");

        //////////////////////////
        // Type aliases for the DSP components
        //////////////////////////
        
        // Main TimeStretch type
        type SignalsmithStretchFloat;
        
        // Filter Types
        type BiquadStaticFloat;
        
        //////////////////////////
        // TimeStretch Factory + Methods
        //////////////////////////
        
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
        fn configure(
            self: Pin<&mut SignalsmithStretchFloat>,
            nChannels: i32,
            blockSamples: i32,
            intervalSamples: i32,
        );

        // Pitch shifting methods
        fn setTransposeFactor(
            self: Pin<&mut SignalsmithStretchFloat>,
            multiplier: f32,
            tonalityLimit: f32,
        );
        fn setTransposeSemitones(
            self: Pin<&mut SignalsmithStretchFloat>,
            semitones: f32,
            tonalityLimit: f32,
        );

        // Processing wrapper functions
        unsafe fn signalsmith_stretch_process(
            stretch: Pin<&mut SignalsmithStretchFloat>,
            inputs: *const *const f32,
            inputSamples: i32,
            outputs: *mut *mut f32,
            outputSamples: i32,
            channels: i32,
        );

        unsafe fn signalsmith_stretch_seek(
            stretch: Pin<&mut SignalsmithStretchFloat>,
            inputs: *const *const f32,
            inputSamples: i32,
            playbackRate: f64,
            channels: i32,
        );

        unsafe fn signalsmith_stretch_flush(
            stretch: Pin<&mut SignalsmithStretchFloat>,
            outputs: *mut *mut f32,
            outputSamples: i32,
            channels: i32,
        );
                
        //////////////////////////
        // Biquad Filter Methods
        //////////////////////////
        
        // Factory
        fn new_biquad() -> UniquePtr<BiquadStaticFloat>;
        
        // These are free functions in bridge.h, not methods of BiquadStaticFloat
        fn biquad_lowpass(filter: Pin<&mut BiquadStaticFloat>, freq: f32, q: f32, design: i32);
        fn biquad_highpass(filter: Pin<&mut BiquadStaticFloat>, freq: f32, q: f32, design: i32);
        fn biquad_bandpass(filter: Pin<&mut BiquadStaticFloat>, freq: f32, bandwidth: f32, design: i32);
        fn biquad_notch(filter: Pin<&mut BiquadStaticFloat>, freq: f32, bandwidth: f32, design: i32);
        fn biquad_peak(filter: Pin<&mut BiquadStaticFloat>, freq: f32, bandwidth: f32, gainDB: f32, design: i32);
        fn biquad_low_shelf(filter: Pin<&mut BiquadStaticFloat>, freq: f32, gainDB: f32, design: i32);
        fn biquad_high_shelf(filter: Pin<&mut BiquadStaticFloat>, freq: f32, gainDB: f32, design: i32);
        fn biquad_allpass(filter: Pin<&mut BiquadStaticFloat>, freq: f32, q: f32, design: i32);
        
        // Filter processing - also free functions
        fn biquad_process_sample(filter: Pin<&mut BiquadStaticFloat>, sample: f32) -> f32;
        unsafe fn biquad_process_buffer(filter: Pin<&mut BiquadStaticFloat>, input: *const f32, output: *mut f32, samples: i32);
        fn biquad_reset(filter: Pin<&mut BiquadStaticFloat>);
    }
}

// Re-export the FFI bindings for use by the rest of the crate
pub use bindings::*;