use super::{
    bindgen,
    models::{Fr, Point},
    traits::{DeserializeBuffer, SerializeBuffer},
};

/// Scalar multiplication on Grumpkin curve: point * scalar
pub unsafe fn ecc_grumpkin__mul(point: &Point, scalar: &Fr) -> Point {
    let mut result_buf = [0; 64];
    bindgen::ecc_grumpkin__mul(
        point.to_buffer().as_slice().as_ptr(),
        scalar.to_buffer().as_slice().as_ptr(),
        result_buf.as_mut_ptr(),
    );
    Point::from_buffer(result_buf)
}

/// Point addition on Grumpkin curve: point_a + point_b
pub unsafe fn ecc_grumpkin__add(point_a: &Point, point_b: &Point) -> Point {
    let mut result_buf = [0; 64];
    bindgen::ecc_grumpkin__add(
        point_a.to_buffer().as_slice().as_ptr(),
        point_b.to_buffer().as_slice().as_ptr(),
        result_buf.as_mut_ptr(),
    );
    Point::from_buffer(result_buf)
}

/// Batch scalar multiplication: multiply each point by the same scalar
pub unsafe fn ecc_grumpkin__batch_mul(points: &[Point], scalar: &Fr) -> Vec<Point> {
    let num_points = points.len() as u32;
    
    // Serialize all points into a single buffer
    let mut points_buf = Vec::with_capacity(points.len() * 64);
    for point in points {
        points_buf.extend_from_slice(&point.to_buffer());
    }
    
    // Prepare result buffer
    let mut result_buf = vec![0u8; points.len() * 64];
    
    bindgen::ecc_grumpkin__batch_mul(
        points_buf.as_ptr(),
        scalar.to_buffer().as_slice().as_ptr(),
        num_points,
        result_buf.as_mut_ptr(),
    );
    
    // Deserialize results back into Points
    let mut results = Vec::with_capacity(points.len());
    for i in 0..points.len() {
        let start = i * 64;
        let end = start + 64;
        let mut point_buf = [0; 64];
        point_buf.copy_from_slice(&result_buf[start..end]);
        results.push(Point::from_buffer(point_buf));
    }
    
    results
}

/// Generate a random scalar modulo the circuit modulus
pub unsafe fn ecc_grumpkin__get_random_scalar_mod_circuit_modulus() -> Fr {
    let mut result_buf = [0; 32];
    bindgen::ecc_grumpkin__get_random_scalar_mod_circuit_modulus(result_buf.as_mut_ptr());
    Fr::from_buffer(result_buf)
}

/// Reduce a 512-bit buffer modulo the circuit modulus
pub unsafe fn ecc_grumpkin__reduce512_buffer_mod_circuit_modulus(input: &[u8; 64]) -> Fr {
    let mut result_buf = [0; 32];
    let mut input_copy = *input;
    bindgen::ecc_grumpkin__reduce512_buffer_mod_circuit_modulus(
        input_copy.as_mut_ptr(),
        result_buf.as_mut_ptr(),
    );
    Fr::from_buffer(result_buf)
}
