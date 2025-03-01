use std::array;
use std::ptr;

/// Create a multi-channel audio buffer with the given dimensions.
pub fn create_buffer(channels: usize, frames: usize) -> Vec<Vec<f32>> {
    let mut buffer = Vec::with_capacity(channels);
    for _ in 0..channels {
        buffer.push(vec![0.0; frames]);
    }
    buffer
}

/// Convert a multi-channel buffer into an array of slices.
/// 
/// Useful for passing to API methods that expect fixed-size arrays.
pub fn get_channel_slices<'a, const C: usize>(buffer: &'a [Vec<f32>]) -> [&'a [f32]; C] {
    assert_eq!(buffer.len(), C, "Buffer channel count must match expected count");
    
    let mut slices = array::from_fn(|_| &[][..]);
    for i in 0..C {
        slices[i] = &buffer[i][..];
    }
    slices
}

/// Convert a mutable multi-channel buffer into an array of mutable slices.
pub fn get_channel_slices_mut<'a, const C: usize>(buffer: &'a mut [Vec<f32>]) -> [&'a mut [f32]; C] {
    assert_eq!(buffer.len(), C, "Buffer channel count must match expected count");
    
    // We need to create the array in a complex way because we can't directly
    // create an array of mutable references due to Rust's borrowing rules
    unsafe {
        // Create an array of raw pointers
        let mut raw_slices = [ptr::null_mut() as *mut f32; C];
        
        // Array to track slice lengths
        let mut lengths = [0usize; C];
        
        // Fill with pointers to each slice and track lengths
        for i in 0..C {
            raw_slices[i] = buffer[i].as_mut_ptr();
            lengths[i] = buffer[i].len();
        }
        
        // Convert back to array of mutable references
        let mut result = array::from_fn(|_| &mut [][..]);
        for i in 0..C {
            result[i] = std::slice::from_raw_parts_mut(raw_slices[i], lengths[i]);
        }
        result
    }
}