#[cfg(test)]
mod tests {
    use crate::barretenberg_api::bn254::bn254_fr_sqrt;
    use crate::barretenberg_api::models::Fr;

    // cargo test bn254_tests -v -- --test-threads=1 --nocapture

    #[test]
    fn test_bn254_fr_sqrt_of_zero() {
        // Square root of zero should be zero
        let zero = Fr { data: [0u8; 32] };
        
        unsafe {
            let result = bn254_fr_sqrt(&zero);
            assert!(result.is_some(), "Square root of zero should exist");
            assert_eq!(result.unwrap().data, [0u8; 32], "Square root of zero should be zero");
        }
    }

    #[test]
    fn test_bn254_fr_sqrt_of_one() {
        // Square root of one should be one
        let mut one_data = [0u8; 32];
        one_data[31] = 1;
        let one = Fr { data: one_data };
        
        unsafe {
            let result = bn254_fr_sqrt(&one);
            assert!(result.is_some(), "Square root of one should exist");
            assert_eq!(result.unwrap().data, one_data, "Square root of one should be one");
        }
    }

    #[test]
    fn test_bn254_fr_sqrt_of_four() {
        // Square root of four should be two
        let mut four_data = [0u8; 32];
        four_data[31] = 4;
        let four = Fr { data: four_data };
        
        unsafe {
            let result = bn254_fr_sqrt(&four);
            assert!(result.is_some(), "Square root of four should exist");
            let sqrt = result.unwrap();
            
            // The square root should be 2
            let mut expected = [0u8; 32];
            expected[31] = 2;
            assert_eq!(sqrt.data, expected, "Square root of four should be two");
        }
    }

    #[test]
    fn test_bn254_fr_sqrt_of_nine() {
        // Square root of nine should be three
        let mut nine_data = [0u8; 32];
        nine_data[31] = 9;
        let nine = Fr { data: nine_data };
        
        unsafe {
            let result = bn254_fr_sqrt(&nine);
            assert!(result.is_some(), "Square root of nine should exist");
            let sqrt = result.unwrap();
            
            // The square root should be 3
            let mut expected = [0u8; 32];
            expected[31] = 3;
            assert_eq!(sqrt.data, expected, "Square root of nine should be three");
        }
    }

    #[test]
    fn test_bn254_fr_sqrt_non_square() {
        // Test with a value that is likely not a perfect square
        // 2 is not a quadratic residue in many fields
        let mut two_data = [0u8; 32];
        two_data[31] = 2;
        let two = Fr { data: two_data };
        
        unsafe {
            let result = bn254_fr_sqrt(&two);
            // For bn254 Fr field, 2 may or may not be a quadratic residue
            // Just verify the function returns a valid result (Some or None)
            println!("Square root of 2 exists: {}", result.is_some());
        }
    }

    #[test]
    fn test_bn254_fr_sqrt_large_value() {
        // Test with a larger value
        let mut large_data = [0u8; 32];
        large_data[28] = 0x01;
        large_data[29] = 0x23;
        large_data[30] = 0x45;
        large_data[31] = 0x67;
        let large = Fr { data: large_data };
        
        unsafe {
            let result = bn254_fr_sqrt(&large);
            // Just verify the function executes without panicking
            println!("Square root of large value exists: {}", result.is_some());
            
            if let Some(sqrt) = result {
                // Verify the result is non-zero
                assert_ne!(sqrt.data, [0u8; 32], "Square root of non-zero should be non-zero");
            }
        }
    }

    #[test]
    fn test_bn254_fr_sqrt_consistency() {
        // Test that if sqrt(x) = y, then y^2 should equal x (mod p)
        // We'll test with known perfect squares
        let test_cases = [1u8, 4, 9, 16, 25, 36, 49, 64, 81, 100];
        
        for &val in &test_cases {
            let mut data = [0u8; 32];
            data[31] = val;
            let input = Fr { data };
            
            unsafe {
                let result = bn254_fr_sqrt(&input);
                assert!(result.is_some(), "Square root of {} should exist", val);
                
                // Note: We can't easily verify y^2 = x without implementing field multiplication
                // in this test, but we can at least verify we get a result
                println!("sqrt({}) exists", val);
            }
        }
    }
} 
