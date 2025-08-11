#pragma once

#include "./signalsmith-stretch/signalsmith-stretch.h"
#include "./signalsmith-stretch/dsp/filters.h"
#include <memory>
#include <vector>
#include <utility>

// C++11 version of make_unique for older compilers
#if __cplusplus < 201402L
namespace std {
    template<typename T, typename... Args>
    unique_ptr<T> make_unique(Args&&... args) {
        return unique_ptr<T>(new T(std::forward<Args>(args)...));
    }
}
#endif

///////////////////////////////////////////////////////////////////////////////
// Type aliases for templated C++ classes
///////////////////////////////////////////////////////////////////////////////

// Time stretch
using SignalsmithStretchFloat = signalsmith::stretch::SignalsmithStretch<float>;

// DSP - Filters
using BiquadStaticFloat = signalsmith::filters::BiquadStatic<float>;

///////////////////////////////////////////////////////////////////////////////
// TimStretch API
///////////////////////////////////////////////////////////////////////////////

inline int blockSamples(const SignalsmithStretchFloat& stretch) {
    return stretch.blockSamples();
}

inline int intervalSamples(const SignalsmithStretchFloat& stretch) {
    return stretch.intervalSamples();
}

inline int inputLatency(const SignalsmithStretchFloat& stretch) {
    return stretch.inputLatency();
}

inline int outputLatency(const SignalsmithStretchFloat& stretch) {
    return stretch.outputLatency();
}

// Factory functions
inline std::unique_ptr<SignalsmithStretchFloat> new_signalsmith_stretch() {
    return std::make_unique<SignalsmithStretchFloat>();
}

inline std::unique_ptr<SignalsmithStretchFloat> new_signalsmith_stretch_with_seed(int64_t seed) {
    // Construct with a 64-bit seed to avoid truncation on platforms where long is 32-bit
    return std::make_unique<SignalsmithStretchFloat>(static_cast<long long>(seed));
}

// Helper for handling buffer views
class FloatBufferView {
private:
    const float* const* buffers;

public:
    FloatBufferView(const float* const* bufs, int /* channels */) : buffers(bufs) {}
    
    const float* operator[](size_t channel) const {
        return buffers[channel];
    }
};

class FloatBufferMutView {
private:
    float* const* buffers;

public:
    FloatBufferMutView(float* const* bufs, int /* channels */) : buffers(bufs) {}
    
    float* operator[](size_t channel) const {
        return buffers[channel];
    }
};

// Process wrapper for handling the templated process method
inline void signalsmith_stretch_process(
    SignalsmithStretchFloat& stretch,
    const float* const* inputs, int inputSamples,
    float** outputs, int outputSamples,
    int channels
) {
    FloatBufferView inputView(inputs, channels);
    FloatBufferMutView outputView(outputs, channels);
    stretch.process(inputView, inputSamples, outputView, outputSamples);
}

// Seek wrapper
inline void signalsmith_stretch_seek(
    SignalsmithStretchFloat& stretch,
    const float* const* inputs, int inputSamples,
    double playbackRate,
    int channels
) {
    FloatBufferView inputView(inputs, channels);
    stretch.seek(inputView, inputSamples, playbackRate);
}

// Flush wrapper
inline void signalsmith_stretch_flush(
    SignalsmithStretchFloat& stretch,
    float** outputs, int outputSamples,
    int channels
) {
    FloatBufferMutView outputView(outputs, channels);
    stretch.flush(outputView, outputSamples);
}

///////////////////////////////////////////////////////////////////////////////
// Biquad Filter Methods
///////////////////////////////////////////////////////////////////////////////

// Factory function
inline std::unique_ptr<BiquadStaticFloat> new_biquad() {
    return std::make_unique<BiquadStaticFloat>();
}

// Biquad functions - these are non-member functions that modify the filter
inline void biquad_lowpass(
    BiquadStaticFloat& filter, 
    float freq, 
    float q, 
    int design = 1 // 1 = cookbook (from enum BiquadDesign)
) {
    if (design == 0) { // Bilinear
        filter.lowpassQ(freq, q, signalsmith::filters::BiquadDesign::bilinear);
    } else if (design == 1) { // Cookbook
        filter.lowpassQ(freq, q, signalsmith::filters::BiquadDesign::cookbook);
    } else if (design == 2) { // OneSided
        filter.lowpassQ(freq, q, signalsmith::filters::BiquadDesign::oneSided);
    } else { // Vicanek
        filter.lowpassQ(freq, q, signalsmith::filters::BiquadDesign::vicanek);
    }
}

inline void biquad_highpass(
    BiquadStaticFloat& filter, 
    float freq, 
    float q, 
    int design = 1 // 1 = cookbook (from enum BiquadDesign)
) {
    if (design == 0) { // Bilinear
        filter.highpassQ(freq, q, signalsmith::filters::BiquadDesign::bilinear);
    } else if (design == 1) { // Cookbook
        filter.highpassQ(freq, q, signalsmith::filters::BiquadDesign::cookbook);
    } else if (design == 2) { // OneSided
        filter.highpassQ(freq, q, signalsmith::filters::BiquadDesign::oneSided);
    } else { // Vicanek
        filter.highpassQ(freq, q, signalsmith::filters::BiquadDesign::vicanek);
    }
}

inline void biquad_bandpass(
    BiquadStaticFloat& filter, 
    float freq, 
    float bandwidth, 
    int design = 1 // 1 = cookbook (from enum BiquadDesign)
) {
    if (design == 0) { // Bilinear
        filter.bandpass(freq, bandwidth, signalsmith::filters::BiquadDesign::bilinear);
    } else if (design == 1) { // Cookbook
        filter.bandpass(freq, bandwidth, signalsmith::filters::BiquadDesign::cookbook);
    } else if (design == 2) { // OneSided
        filter.bandpass(freq, bandwidth, signalsmith::filters::BiquadDesign::oneSided);
    } else { // Vicanek
        filter.bandpass(freq, bandwidth, signalsmith::filters::BiquadDesign::vicanek);
    }
}

inline void biquad_notch(
    BiquadStaticFloat& filter, 
    float freq, 
    float bandwidth, 
    int design = 1 // 1 = cookbook (from enum BiquadDesign)
) {
    if (design == 0) { // Bilinear
        filter.notch(freq, bandwidth, signalsmith::filters::BiquadDesign::bilinear);
    } else if (design == 1) { // Cookbook
        filter.notch(freq, bandwidth, signalsmith::filters::BiquadDesign::cookbook);
    } else if (design == 2) { // OneSided
        filter.notch(freq, bandwidth, signalsmith::filters::BiquadDesign::oneSided);
    } else { // Vicanek
        filter.notch(freq, bandwidth, signalsmith::filters::BiquadDesign::vicanek);
    }
}

inline void biquad_peak(
    BiquadStaticFloat& filter, 
    float freq, 
    float bandwidth, 
    float gainDB, 
    int design = 1 // 1 = cookbook (from enum BiquadDesign)
) {
    if (design == 0) { // Bilinear
        filter.peakDb(freq, gainDB, bandwidth, signalsmith::filters::BiquadDesign::bilinear);
    } else if (design == 1) { // Cookbook
        filter.peakDb(freq, gainDB, bandwidth, signalsmith::filters::BiquadDesign::cookbook);
    } else if (design == 2) { // OneSided
        filter.peakDb(freq, gainDB, bandwidth, signalsmith::filters::BiquadDesign::oneSided);
    } else { // Vicanek
        filter.peakDb(freq, gainDB, bandwidth, signalsmith::filters::BiquadDesign::vicanek);
    }
}

inline void biquad_low_shelf(
    BiquadStaticFloat& filter, 
    float freq, 
    float gainDB, 
    int design = 1 // 1 = cookbook (from enum BiquadDesign)
) {
    double defaultBandwidth = 2.0;
    
    if (design == 0) { // Bilinear
        filter.lowShelfDb(freq, gainDB, defaultBandwidth, signalsmith::filters::BiquadDesign::bilinear);
    } else if (design == 1) { // Cookbook
        filter.lowShelfDb(freq, gainDB, defaultBandwidth, signalsmith::filters::BiquadDesign::cookbook);
    } else if (design == 2) { // OneSided
        filter.lowShelfDb(freq, gainDB, defaultBandwidth, signalsmith::filters::BiquadDesign::oneSided);
    } else { // Vicanek
        filter.lowShelfDb(freq, gainDB, defaultBandwidth, signalsmith::filters::BiquadDesign::vicanek);
    }
}

inline void biquad_high_shelf(
    BiquadStaticFloat& filter, 
    float freq, 
    float gainDB, 
    int design = 1 // 1 = cookbook (from enum BiquadDesign)
) {
    double defaultBandwidth = 1.8999686269529916;
    
    if (design == 0) { // Bilinear
        filter.highShelfDb(freq, gainDB, defaultBandwidth, signalsmith::filters::BiquadDesign::bilinear);
    } else if (design == 1) { // Cookbook
        filter.highShelfDb(freq, gainDB, defaultBandwidth, signalsmith::filters::BiquadDesign::cookbook);
    } else if (design == 2) { // OneSided
        filter.highShelfDb(freq, gainDB, defaultBandwidth, signalsmith::filters::BiquadDesign::oneSided);
    } else { // Vicanek
        filter.highShelfDb(freq, gainDB, defaultBandwidth, signalsmith::filters::BiquadDesign::vicanek);
    }
}

inline void biquad_allpass(
    BiquadStaticFloat& filter, 
    float freq, 
    float q, 
    int design = 1 // 1 = cookbook (from enum BiquadDesign)
) {
    if (design == 0) { // Bilinear
        filter.allpassQ(freq, q, signalsmith::filters::BiquadDesign::bilinear);
    } else if (design == 1) { // Cookbook
        filter.allpassQ(freq, q, signalsmith::filters::BiquadDesign::cookbook);
    } else if (design == 2) { // OneSided
        filter.allpassQ(freq, q, signalsmith::filters::BiquadDesign::oneSided);
    } else { // Vicanek
        filter.allpassQ(freq, q, signalsmith::filters::BiquadDesign::vicanek);
    }
}

// Filter processing method
inline float biquad_process_sample(BiquadStaticFloat& filter, float sample) {
    return filter(sample);
}

// Custom wrapper to process a buffer of samples
inline void biquad_process_buffer(
    BiquadStaticFloat& filter, 
    const float* input, 
    float* output, 
    int samples
) {
    for (int i = 0; i < samples; i++) {
        output[i] = filter(input[i]);
    }
}

// Reset filter state
inline void biquad_reset(BiquadStaticFloat& filter) {
    filter.reset();
}