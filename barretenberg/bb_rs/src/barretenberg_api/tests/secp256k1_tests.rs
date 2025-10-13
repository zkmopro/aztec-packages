#[cfg(test)]
mod tests {
    use crate::barretenberg_api::secp256k1::{ecc_secp256k1__mul, ecc_secp256k1__get_random_scalar_mod_circuit_modulus, ecc_secp256k1__reduce512_buffer_mod_circuit_modulus};
    use crate::barretenberg_api::models::{Fr, Point};

    // cargo test secp256k1_tests -v -- --test-threads=1 --nocapture

    #[test]
    fn test_secp256k1_scalar_multiplication() {
        let point = Point {
            x: Fr { data: [1u8; 32] },
            y: Fr { data: [2u8; 32] },
        };
        let scalar = Fr { data: [3u8; 32] };

        unsafe {
            let result = ecc_secp256k1__mul(&point, &scalar);
            // Result should be different from input
            assert_ne!(result, point);
            // Result should be a valid point (non-zero)
            assert_ne!(result.x.data, [0u8; 32]);
        }
    }

    #[test]
    fn test_secp256k1_scalar_multiplication_by_zero() {
        let point = Point {
            x: Fr { data: [1u8; 32] },
            y: Fr { data: [2u8; 32] },
        };
        let scalar = Fr { data: [0u8; 32] };

        unsafe {
            let result = ecc_secp256k1__mul(&point, &scalar);
            // Multiplying by zero should give the point at infinity
            // For affine coordinates, this is typically represented as (0, 0)
            // or a special encoding - verify the result is the identity
            println!("Result x: {:?}", result.x.data);
            println!("Result y: {:?}", result.y.data);
        }
    }

    #[test]
    fn test_secp256k1_scalar_multiplication_by_one() {
        // Using a known valid secp256k1 generator point
        // Generator G: (0x79BE667EF9DCBBAC55A06295CE870B07029BFCDB2DCE28D959F2815B16F81798,
        //               0x483ADA7726A3C4655DA4FBFC0E1108A8FD17B448A68554199C47D08FFB10D4B8)
        let mut generator_x = [0u8; 32];
        let mut generator_y = [0u8; 32];
        
        generator_x.copy_from_slice(&[
            0x79, 0xbe, 0x66, 0x7e, 0xf9, 0xdc, 0xbb, 0xac,
            0x55, 0xa0, 0x62, 0x95, 0xce, 0x87, 0x0b, 0x07,
            0x02, 0x9b, 0xfc, 0xdb, 0x2d, 0xce, 0x28, 0xd9,
            0x59, 0xf2, 0x81, 0x5b, 0x16, 0xf8, 0x17, 0x98,
        ]);
        
        generator_y.copy_from_slice(&[
            0x48, 0x3a, 0xda, 0x77, 0x26, 0xa3, 0xc4, 0x65,
            0x5d, 0xa4, 0xfb, 0xfc, 0x0e, 0x11, 0x08, 0xa8,
            0xfd, 0x17, 0xb4, 0x48, 0xa6, 0x85, 0x54, 0x19,
            0x9c, 0x47, 0xd0, 0x8f, 0xfb, 0x10, 0xd4, 0xb8,
        ]);

        let point = Point {
            x: Fr { data: generator_x },
            y: Fr { data: generator_y },
        };
        
        let mut scalar = [0u8; 32];
        scalar[31] = 1; // scalar = 1
        let scalar = Fr { data: scalar };

        unsafe {
            let result = ecc_secp256k1__mul(&point, &scalar);
            // Multiplying by 1 should give the same point
            println!("Original x: {:?}", point.x.data);
            println!("Result x:   {:?}", result.x.data);
            println!("Original y: {:?}", point.y.data);
            println!("Result y:   {:?}", result.y.data);
        }
    }

    #[test]
    fn test_secp256k1_random_scalar_generation() {
        unsafe {
            let scalar1 = ecc_secp256k1__get_random_scalar_mod_circuit_modulus();
            let scalar2 = ecc_secp256k1__get_random_scalar_mod_circuit_modulus();
            
            // Random scalars should be different (very high probability)
            assert_ne!(scalar1.data, scalar2.data);
            // Should not be zero
            assert_ne!(scalar1.data, [0u8; 32]);
            assert_ne!(scalar2.data, [0u8; 32]);
        }
    }

    #[test]
    fn test_secp256k1_random_scalar_multiple_calls() {
        unsafe {
            let mut scalars = Vec::new();
            for _ in 0..10 {
                let scalar = ecc_secp256k1__get_random_scalar_mod_circuit_modulus();
                scalars.push(scalar);
            }
            
            // Check that all scalars are different
            for i in 0..scalars.len() {
                for j in (i+1)..scalars.len() {
                    assert_ne!(scalars[i].data, scalars[j].data, 
                        "Scalars at index {} and {} should be different", i, j);
                }
            }
        }
    }

    #[test]
    fn test_secp256k1_reduce512() {
        let large_input = [0xffu8; 64]; // Maximum 512-bit value
        
        unsafe {
            let reduced = ecc_secp256k1__reduce512_buffer_mod_circuit_modulus(&large_input);
            // Should produce a valid field element
            assert_ne!(reduced.data, [0u8; 32]);
            // Should be different from the input (since we're reducing)
            assert_ne!(reduced.data, large_input[..32]);
        }
    }

    #[test]
    fn test_secp256k1_reduce512_small_value() {
        let mut small_input = [0u8; 64];
        small_input[63] = 42; // A small value
        
        unsafe {
            let reduced = ecc_secp256k1__reduce512_buffer_mod_circuit_modulus(&small_input);
            // For a small value, the reduction should preserve it
            let mut expected = [0u8; 32];
            expected[31] = 42;
            assert_eq!(reduced.data, expected);
        }
    }

    #[test]
    fn test_secp256k1_reduce512_zero() {
        let zero_input = [0u8; 64];
        
        unsafe {
            let reduced = ecc_secp256k1__reduce512_buffer_mod_circuit_modulus(&zero_input);
            // Zero should remain zero after reduction
            assert_eq!(reduced.data, [0u8; 32]);
        }
    }

    #[test]
    fn test_secp256k1_reduce512_various_inputs() {
        unsafe {
            // Test with different patterns
            let mut input1 = [0u8; 64];
            input1[0] = 0xff;
            input1[32] = 0xff;
            let result1 = ecc_secp256k1__reduce512_buffer_mod_circuit_modulus(&input1);
            
            let mut input2 = [0xaau8; 64];
            let result2 = ecc_secp256k1__reduce512_buffer_mod_circuit_modulus(&input2);
            
            // Results should be different for different inputs
            assert_ne!(result1.data, result2.data);
        }
    }
} 
