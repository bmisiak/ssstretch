use crate::ffi;

/// Biquad filter design methods
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BiquadDesign {
    /// Standard bilinear transform
    Bilinear,
    /// Robert Bristow-Johnson's cookbook
    Cookbook,
    /// One-sided designs with better phase
    OneSided,
    /// Urs Vicanek's method
    Vicanek,
}

impl From<BiquadDesign> for i32 {
    fn from(design: BiquadDesign) -> Self {
        match design {
            BiquadDesign::Bilinear => 0, // Values match C++ enum order
            BiquadDesign::Cookbook => 1,
            BiquadDesign::OneSided => 2,
            BiquadDesign::Vicanek => 3,
        }
    }
}

/// Filter types for biquad filters
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FilterType {
    /// Low-pass filter (passes low frequencies)
    LowPass,
    /// High-pass filter (passes high frequencies)
    HighPass,
    /// Band-pass filter (passes a band of frequencies)
    BandPass,
    /// Notch filter (rejects a band of frequencies)
    Notch,
    /// Peak filter (boosts or cuts a band of frequencies)
    Peak,
    /// Low shelf filter (boosts or cuts low frequencies)
    LowShelf,
    /// High shelf filter (boosts or cuts high frequencies)
    HighShelf,
    /// All-pass filter (changes phase, not magnitude)
    AllPass,
}

/// Biquad filter implementation with static coefficients
pub struct BiquadFilter {
    inner: cxx::UniquePtr<ffi::BiquadStaticFloat>,
}

impl BiquadFilter {
    /// Create a new biquad filter with default coefficients
    pub fn new() -> Self {
        Self {
            inner: ffi::new_biquad(),
        }
    }
    
    /// Configure as a low-pass filter
    pub fn lowpass(
        &mut self, 
        freq: f32, 
        q: f32, 
        design: Option<BiquadDesign>
    ) -> &mut Self {
        let design_int: i32 = design.unwrap_or(BiquadDesign::Cookbook).into();
        ffi::biquad_lowpass(self.inner.pin_mut(), freq, q, design_int);
        self
    }
    
    /// Configure as a high-pass filter
    pub fn highpass(
        &mut self, 
        freq: f32, 
        q: f32, 
        design: Option<BiquadDesign>
    ) -> &mut Self {
        let design_int: i32 = design.unwrap_or(BiquadDesign::Cookbook).into();
        ffi::biquad_highpass(self.inner.pin_mut(), freq, q, design_int);
        self
    }
    
    /// Configure as a band-pass filter
    pub fn bandpass(
        &mut self, 
        freq: f32, 
        bandwidth: f32, 
        design: Option<BiquadDesign>
    ) -> &mut Self {
        let design_int: i32 = design.unwrap_or(BiquadDesign::Cookbook).into();
        ffi::biquad_bandpass(self.inner.pin_mut(), freq, bandwidth, design_int);
        self
    }
    
    /// Configure as a notch filter
    pub fn notch(
        &mut self, 
        freq: f32, 
        bandwidth: f32, 
        design: Option<BiquadDesign>
    ) -> &mut Self {
        let design_int: i32 = design.unwrap_or(BiquadDesign::Cookbook).into();
        ffi::biquad_notch(self.inner.pin_mut(), freq, bandwidth, design_int);
        self
    }
    
    /// Configure as a peak filter
    pub fn peak(
        &mut self, 
        freq: f32, 
        bandwidth: f32, 
        gain_db: f32,
        design: Option<BiquadDesign>
    ) -> &mut Self {
        let design_int: i32 = design.unwrap_or(BiquadDesign::Cookbook).into();
        ffi::biquad_peak(self.inner.pin_mut(), freq, bandwidth, gain_db, design_int);
        self
    }
    
    /// Configure as a low shelf filter
    pub fn low_shelf(
        &mut self, 
        freq: f32, 
        gain_db: f32,
        design: Option<BiquadDesign>
    ) -> &mut Self {
        let design_int: i32 = design.unwrap_or(BiquadDesign::Cookbook).into();
        ffi::biquad_low_shelf(self.inner.pin_mut(), freq, gain_db, design_int);
        self
    }
    
    /// Configure as a high shelf filter
    pub fn high_shelf(
        &mut self, 
        freq: f32, 
        gain_db: f32,
        design: Option<BiquadDesign>
    ) -> &mut Self {
        let design_int: i32 = design.unwrap_or(BiquadDesign::Cookbook).into();
        ffi::biquad_high_shelf(self.inner.pin_mut(), freq, gain_db, design_int);
        self
    }
    
    /// Configure as an all-pass filter
    pub fn allpass(
        &mut self, 
        freq: f32, 
        q: f32, 
        design: Option<BiquadDesign>
    ) -> &mut Self {
        let design_int: i32 = design.unwrap_or(BiquadDesign::Cookbook).into();
        ffi::biquad_allpass(self.inner.pin_mut(), freq, q, design_int);
        self
    }
    
    /// Process a single sample through the filter
    pub fn process_sample(&mut self, sample: f32) -> f32 {
        ffi::biquad_process_sample(self.inner.pin_mut(), sample)
    }
    
    /// Process a buffer of samples through the filter
    pub fn process_buffer(&mut self, input: &[f32], output: &mut [f32]) {
        let len = input.len().min(output.len()) as i32;
        
        unsafe {
            ffi::biquad_process_buffer(
                self.inner.pin_mut(),
                input.as_ptr(),
                output.as_mut_ptr(),
                len,
            );
        }
    }
    
    /// Reset the filter state
    pub fn reset(&mut self) {
        ffi::biquad_reset(self.inner.pin_mut());
    }
}

impl Default for BiquadFilter {
    fn default() -> Self {
        Self::new()
    }
}