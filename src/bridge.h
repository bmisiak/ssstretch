#pragma once

#include "../src/signalsmith-stretch/signalsmith-stretch.h"
#include <memory>
#include <complex>
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

// Type aliases for templated C++ classes
using SignalsmithStretchFloat = signalsmith::stretch::SignalsmithStretch<float>;

// We won't try to access the private 'channels' field
// We'll track this in Rust instead

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
    return std::make_unique<SignalsmithStretchFloat>(static_cast<long>(seed));
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