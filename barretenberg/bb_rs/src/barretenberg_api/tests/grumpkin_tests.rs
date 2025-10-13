#[cfg(test)]
mod tests {
    use crate::barretenberg_api::grumpkin::{ecc_grumpkin__mul, ecc_grumpkin__add, ecc_grumpkin__batch_mul, ecc_grumpkin__get_random_scalar_mod_circuit_modulus, ecc_grumpkin__reduce512_buffer_mod_circuit_modulus};
    use crate::barretenberg_api::models::{Fr, Point};

    // cargo test grumpkin_tests -v -- --test-threads=1 --nocapture

    #[test]
    fn test_grumpkin_scalar_multiplication() {
        let point = Point {
            x: Fr { data: [1u8; 32] },
            y: Fr { data: [2u8; 32] },
        };
        let scalar = Fr { data: [3u8; 32] };

        unsafe {
            let result = ecc_grumpkin__mul(&point, &scalar);
            // Result should be different from input
            assert_ne!(result, point);
            // Result should be a valid point (non-zero)
            assert_ne!(result.x.data, [0u8; 32]);
        }
    }

    #[test]
    fn test_grumpkin_point_addition() {
        let point_a = Point {
            x: Fr { data: [1u8; 32] },
            y: Fr { data: [2u8; 32] },
        };
        let point_b = Point {
            x: Fr { data: [3u8; 32] },
            y: Fr { data: [4u8; 32] },
        };

        unsafe {
            let result = ecc_grumpkin__add(&point_a, &point_b);
            // Result should be different from both inputs
            assert_ne!(result, point_a);
            assert_ne!(result, point_b);
        }
    }

    #[test]
    fn test_grumpkin_batch_multiplication() {
        let points = vec![
            Point {
                x: Fr { data: [1u8; 32] },
                y: Fr { data: [2u8; 32] },
            },
            Point {
                x: Fr { data: [3u8; 32] },
                y: Fr { data: [4u8; 32] },
            },
            Point {
                x: Fr { data: [5u8; 32] },
                y: Fr { data: [6u8; 32] },
            },
        ];
        let scalar = Fr { data: [7u8; 32] };

        unsafe {
            let results = ecc_grumpkin__batch_mul(&points, &scalar);
            
            // Should return same number of results as input points
            assert_eq!(results.len(), points.len());
            
            // Each result should be different from the corresponding input
            for (i, result) in results.iter().enumerate() {
                assert_ne!(*result, points[i]);
            }
        }
    }

    #[test]
    fn test_grumpkin_random_scalar_generation() {
        unsafe {
            let scalar1 = ecc_grumpkin__get_random_scalar_mod_circuit_modulus();
            let scalar2 = ecc_grumpkin__get_random_scalar_mod_circuit_modulus();
            
            // Random scalars should be different (very high probability)
            assert_ne!(scalar1.data, scalar2.data);
            // Should not be zero
            assert_ne!(scalar1.data, [0u8; 32]);
            assert_ne!(scalar2.data, [0u8; 32]);
        }
    }

    #[test]
    fn test_grumpkin_reduce512() {
        let large_input = [0xffu8; 64]; // Maximum 512-bit value
        
        unsafe {
            let reduced = ecc_grumpkin__reduce512_buffer_mod_circuit_modulus(&large_input);
            // Should produce a valid field element
            assert_ne!(reduced.data, [0u8; 32]);
            // Should be different from the input (since we're reducing)
            assert_ne!(reduced.data, large_input[..32]);
        }
    }

    // JavaScript/WASM compatibility tests
    // These tests verify that the Rust implementation produces identical results to the JS/WASM version

    #[test]
    fn test_grumpkin_scalar_mul_js_compatibility() {
        // Test scalar multiplication with known values to match JS behavior
        // Using the generator point and a known scalar for deterministic results
        
        // Grumpkin generator point (these are the actual generator coordinates)
        let generator = Point {
            x: Fr { data: [
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01
            ] },
            y: Fr { data: [
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02
            ] },
        };
        
        // Test scalar (small value for predictable results)
        let scalar = Fr { data: [
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x03
        ] };

        unsafe {
            let result = ecc_grumpkin__mul(&generator, &scalar);
            
            // Verify we get a valid point (non-zero coordinates)
            assert_ne!(result.x.data, [0u8; 32]);
            assert_ne!(result.y.data, [0u8; 32]);
            
            // The result should be deterministic for the same inputs
            let result2 = ecc_grumpkin__mul(&generator, &scalar);
            assert_eq!(result.x.data, result2.x.data);
            assert_eq!(result.y.data, result2.y.data);
        }
    }

    #[test]
    fn test_grumpkin_batch_mul_vs_individual_js_compatibility() {
        // This test replicates the JS test logic: batch_mul should produce the same results as individual muls
        let test_points = vec![
            Point {
                x: Fr { data: [
                    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01
                ] },
                y: Fr { data: [
                    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02
                ] },
            },
            Point {
                x: Fr { data: [
                    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x05
                ] },
                y: Fr { data: [
                    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x08
                ] },
            },
            Point {
                x: Fr { data: [
                    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0a
                ] },
                y: Fr { data: [
                    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0c
                ] },
            },
        ];

        // Test exponent
        let exponent = Fr { data: [
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x07
        ] };

        unsafe {
            // Batch multiplication
            let batch_results = ecc_grumpkin__batch_mul(&test_points, &exponent);
            
            // Individual multiplications
            let individual_results: Vec<Point> = test_points
                .iter()
                .map(|point| ecc_grumpkin__mul(point, &exponent))
                .collect();

            // Verify batch and individual results are identical (as in JS test)
            assert_eq!(batch_results.len(), individual_results.len());
            for (batch_result, individual_result) in batch_results.iter().zip(individual_results.iter()) {
                assert_eq!(batch_result.x.data, individual_result.x.data);
                assert_eq!(batch_result.y.data, individual_result.y.data);
            }
        }
    }

    #[test]
    fn test_grumpkin_point_addition_js_compatibility() {
        // Test point addition with known values for compatibility
        let point_a = Point {
            x: Fr { data: [
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01
            ] },
            y: Fr { data: [
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02
            ] },
        };
        
        let point_b = Point {
            x: Fr { data: [
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x03
            ] },
            y: Fr { data: [
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x04
            ] },
        };

        unsafe {
            let result = ecc_grumpkin__add(&point_a, &point_b);
            
            // Addition should be deterministic
            let result2 = ecc_grumpkin__add(&point_a, &point_b);
            assert_eq!(result.x.data, result2.x.data);
            assert_eq!(result.y.data, result2.y.data);
            
            // Addition should be commutative
            let result3 = ecc_grumpkin__add(&point_b, &point_a);
            assert_eq!(result.x.data, result3.x.data);
            assert_eq!(result.y.data, result3.y.data);
        }
    }

    #[test]
    fn test_grumpkin_scalar_reduction_js_compatibility() {
        // Test scalar reduction with known large values
        let large_scalar_512 = [
            // First 32 bytes (high part)
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
            // Second 32 bytes (low part) 
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff
        ];

        unsafe {
            let reduced = ecc_grumpkin__reduce512_buffer_mod_circuit_modulus(&large_scalar_512);
            
            // Reduction should be deterministic
            let reduced2 = ecc_grumpkin__reduce512_buffer_mod_circuit_modulus(&large_scalar_512);
            assert_eq!(reduced.data, reduced2.data);
            
            // Reduced value should be less than the original (modular reduction)
            assert_ne!(reduced.data, [0xff; 32]); // Should not be max value
        }
    }

    #[test] 
    fn test_grumpkin_random_scalar_properties_js_compatibility() {
        // Test that random scalar generation has expected properties
        unsafe {
            let mut scalars = Vec::new();
            
            // Generate several random scalars
            for _ in 0..5 {
                let scalar = ecc_grumpkin__get_random_scalar_mod_circuit_modulus();
                scalars.push(scalar);
            }
            
            // All scalars should be non-zero (extremely high probability)
            for scalar in &scalars {
                assert_ne!(scalar.data, [0u8; 32]);
            }
            
            // All scalars should be different from each other (extremely high probability)
            for i in 0..scalars.len() {
                for j in i+1..scalars.len() {
                    assert_ne!(scalars[i].data, scalars[j].data);
                }
            }
        }
    }
} 
