use super::{
    bindgen,
    models::Fr,
    traits::{DeserializeBuffer, SerializeBuffer},
};

/// Compute the square root of a field element in BN254's Fr field
/// Returns Some(sqrt) if the square root exists, None otherwise
pub unsafe fn bn254_fr_sqrt(input: &Fr) -> Option<Fr> {
    let mut result_buf = [0u8; 33]; // 1 byte for boolean + 32 bytes for Fr
    bindgen::bn254_fr_sqrt(
        input.to_buffer().as_slice().as_ptr(),
        result_buf.as_mut_ptr(),
    );
    
    // First byte indicates whether a square root exists
    let is_sqrt = result_buf[0] == 1;
    
    if is_sqrt {
        // Extract the square root from the remaining 32 bytes
        let mut sqrt_buf = [0u8; 32];
        sqrt_buf.copy_from_slice(&result_buf[1..33]);
        Some(Fr::from_buffer(sqrt_buf))
    } else {
        None
    }
} 
