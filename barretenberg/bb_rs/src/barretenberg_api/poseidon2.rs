use super::{
    bindgen,
    models::Fr,
    traits::{DeserializeBuffer, SerializeBuffer},
    Buffer,
};

pub unsafe fn poseidon2_hash(inputs: &[Fr]) -> Fr {
    let mut output: <Fr as DeserializeBuffer>::Slice = [0; 32];
    bindgen::poseidon2_hash(inputs.to_buffer().as_slice().as_ptr(), output.as_mut_ptr());
    Fr::from_buffer(output)
}

pub unsafe fn poseidon2_hashes(inputs: &[Fr]) -> Vec<Fr> {
    // The C++ function processes inputs in pairs, so we expect half as many results as inputs
    let expected_results = inputs.len() / 2;
    // Allocate buffer for vector serialization: 4 bytes (count) + expected_results * 32 bytes (Fr data)
    let mut output_buffer = vec![0u8; 4 + expected_results * 32];
    
    bindgen::poseidon2_hashes(
        inputs.to_buffer().as_slice().as_ptr(),
        output_buffer.as_mut_ptr(),
    );
    
    // Parse the output buffer: first 4 bytes are the count, then Fr values
    let count = u32::from_be_bytes([
        output_buffer[0],
        output_buffer[1], 
        output_buffer[2],
        output_buffer[3]
    ]) as usize;
    
    let mut results = Vec::new();
    for i in 0..count {
        let start = 4 + i * 32; // Skip the 4-byte count prefix
        let end = start + 32;
        let mut fr_data = [0u8; 32];
        fr_data.copy_from_slice(&output_buffer[start..end]);
        results.push(Fr::from_buffer(fr_data));
    }
    
    results
}

pub unsafe fn poseidon2_permutation(inputs: &[Fr]) -> Vec<Fr> {
    let mut result_ptr: *mut u8 = std::ptr::null_mut();
    
    bindgen::poseidon2_permutation(
        inputs.to_buffer().as_slice().as_ptr(),
        &mut result_ptr as *mut *mut u8,
    );
    
    if result_ptr.is_null() {
        return Vec::new();
    }
    
    let buffer = Buffer::from_ptr(result_ptr).expect("Failed to create buffer from pointer");
    let buffer_data = buffer.as_slice();
    
    // Parse the output buffer: first 4 bytes are the count, then Fr values
    if buffer_data.len() < 4 {
        return Vec::new();
    }
    
    let count = u32::from_be_bytes([
        buffer_data[0],
        buffer_data[1], 
        buffer_data[2],
        buffer_data[3]
    ]) as usize;
    
    let mut results = Vec::new();
    for i in 0..count {
        let start = 4 + i * 32; // Skip the 4-byte count prefix
        let end = start + 32;
        if end <= buffer_data.len() {
            let mut fr_data = [0u8; 32];
            fr_data.copy_from_slice(&buffer_data[start..end]);
            results.push(Fr::from_buffer(fr_data));
        }
    }
    
    results
}
