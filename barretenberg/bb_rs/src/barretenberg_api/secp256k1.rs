use super::{
    bindgen,
    models::{Fr, Point},
    traits::{DeserializeBuffer, SerializeBuffer},
};

/// Scalar multiplication on Secp256k1 curve: point * scalar
pub unsafe fn ecc_secp256k1__mul(point: &Point, scalar: &Fr) -> Point {
    let mut result_buf = [0; 64];
    bindgen::ecc_secp256k1__mul(
        point.to_buffer().as_slice().as_ptr(),
        scalar.to_buffer().as_slice().as_ptr(),
        result_buf.as_mut_ptr(),
    );
    Point::from_buffer(result_buf)
}

/// Generate a random scalar modulo the circuit modulus
pub unsafe fn ecc_secp256k1__get_random_scalar_mod_circuit_modulus() -> Fr {
    let mut result_buf = [0; 32];
    bindgen::ecc_secp256k1__get_random_scalar_mod_circuit_modulus(result_buf.as_mut_ptr());
    Fr::from_buffer(result_buf)
}

/// Reduce a 512-bit buffer modulo the circuit modulus
pub unsafe fn ecc_secp256k1__reduce512_buffer_mod_circuit_modulus(input: &[u8; 64]) -> Fr {
    let mut result_buf = [0; 32];
    let mut input_copy = *input;
    bindgen::ecc_secp256k1__reduce512_buffer_mod_circuit_modulus(
        input_copy.as_mut_ptr(),
        result_buf.as_mut_ptr(),
    );
    Fr::from_buffer(result_buf)
} 
