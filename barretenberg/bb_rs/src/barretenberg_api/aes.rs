use super::{
    bindgen,
    Buffer,
};
use std::ptr;

/// AES-128 CBC encryption
/// Expects input to be already padded (PKCS#7 padding should be applied at the TypeScript layer)
pub unsafe fn aes_encrypt_buffer_cbc(
    input: &[u8],
    iv: &[u8; 16],
    key: &[u8; 16],
) -> Buffer {
    // Verify input is properly padded (must be multiple of 16 bytes)
    if input.len() % 16 != 0 {
        panic!("Input length must be a multiple of 16 bytes (pre-padded), got: {}", input.len());
    }
    
    // Create mutable copies since the C++ function modifies both input and IV in-place
    let mut input_copy = input.to_vec();
    let mut iv_copy = *iv;
    
    // Convert to network byte order as expected by the C++ function
    let length = (input_copy.len() as u32).to_be();
    let mut result_ptr: *mut u8 = ptr::null_mut();
    
    bindgen::aes_encrypt_buffer_cbc(
        input_copy.as_mut_ptr(),
        iv_copy.as_mut_ptr(),
        key.as_ptr(),
        &length,
        &mut result_ptr,
    );
    
    let buffer = Buffer::from_ptr(result_ptr as *const u8).expect("AES encryption failed");
    
    // The buffer contains double-length-prefix from to_heap_buffer:
    // [4 bytes: inner length][actual encrypted data]
    // We need to extract just the actual encrypted data
    let buffer_data = buffer.as_slice();
    if buffer_data.len() < 4 {
        panic!("Invalid buffer format from C++ function");
    }
    
    // Skip the inner length prefix (first 4 bytes) and get the actual encrypted data
    let actual_encrypted_data = &buffer_data[4..];
    
    Buffer::from_data(actual_encrypted_data.to_vec())
}

/// AES-128 CBC decryption
/// Returns the decrypted data with padding intact (padding removal should be handled at the TypeScript layer)
pub unsafe fn aes_decrypt_buffer_cbc(
    input: &[u8],
    iv: &[u8; 16],
    key: &[u8; 16],
) -> Buffer {
    // The input here should be the actual encrypted data (multiple of 16 bytes)
    if input.len() % 16 != 0 {
        panic!("Input length must be a multiple of 16 bytes for AES decryption, got: {}", input.len());
    }
    
    // Create mutable copies since the C++ function modifies both input and IV in-place
    let mut input_copy = input.to_vec();
    let mut iv_copy = *iv;
    
    // Convert to network byte order as expected by the C++ function
    let length = (input_copy.len() as u32).to_be();
    let mut result_ptr: *mut u8 = ptr::null_mut();
    
    bindgen::aes_decrypt_buffer_cbc(
        input_copy.as_mut_ptr(),
        iv_copy.as_mut_ptr(),
        key.as_ptr(),
        &length,
        &mut result_ptr,
    );
    
    let decrypted_buffer = Buffer::from_ptr(result_ptr as *const u8).expect("AES decryption failed");
    
    // The decrypted buffer also has the double-length-prefix issue
    // Extract the actual decrypted data (skip the inner length prefix)
    let buffer_data = decrypted_buffer.as_slice();
    if buffer_data.len() < 4 {
        panic!("Invalid decrypted buffer format from C++ function");
    }
    
    let actual_decrypted_data = &buffer_data[4..];
    
    // Return the decrypted data with padding intact
    // Padding removal should be handled at the TypeScript layer
    Buffer::from_data(actual_decrypted_data.to_vec())
}
