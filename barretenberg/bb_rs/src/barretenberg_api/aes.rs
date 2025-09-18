use super::{
    bindgen,
    Buffer,
};
use std::ptr;

/// Apply PKCS#7 padding to input data
fn apply_pkcs7_padding(input: &[u8]) -> Vec<u8> {
    let block_size = 16;
    let padding_len = block_size - (input.len() % block_size);
    let mut padded = input.to_vec();
    padded.extend(vec![padding_len as u8; padding_len]);
    padded
}

/// Remove PKCS#7 padding from decrypted data
fn remove_pkcs7_padding(data: &[u8]) -> Result<Vec<u8>, &'static str> {
    if data.is_empty() {
        return Err("Empty data");
    }
    
    let padding_len = data[data.len() - 1] as usize;
    if padding_len == 0 || padding_len > 16 {
        return Err("Invalid padding");
    }
    
    if data.len() < padding_len {
        return Err("Data too short for padding");
    }
    
    // Verify all padding bytes are correct
    for i in 0..padding_len {
        if data[data.len() - 1 - i] != padding_len as u8 {
            return Err("Invalid padding bytes");
        }
    }
    
    Ok(data[..data.len() - padding_len].to_vec())
}

/// AES-128 CBC encryption
pub unsafe fn aes_encrypt_buffer_cbc(
    input: &[u8],
    iv: &[u8; 16],
    key: &[u8; 16],
) -> Buffer {
    // Apply PKCS#7 padding
    let padded_input = apply_pkcs7_padding(input);
    
    // Create mutable copies since the C++ function modifies both input and IV in-place
    let mut input_copy = padded_input;
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
    
    // Remove PKCS#7 padding
    let unpadded_data = remove_pkcs7_padding(actual_decrypted_data)
        .expect("Failed to remove padding");
    
    // Create a new buffer with the unpadded data
    Buffer::from_data(unpadded_data)
}
