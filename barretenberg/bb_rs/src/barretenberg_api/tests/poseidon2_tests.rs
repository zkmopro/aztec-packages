#[cfg(test)]
mod tests {
    use crate::barretenberg_api::poseidon2::{poseidon2_hash, poseidon2_hashes, poseidon2_permutation};
    use crate::barretenberg_api::models::Fr;

    // cargo test poseidon2_tests -v -- --test-threads=1 --nocapture

    #[test]
    fn test_poseidon2_hash_single() {
        let input = Fr { data: [1u8; 32] };
        
        let result = unsafe { poseidon2_hash(&[input]) };
        
        // Result should be different from input
        assert_ne!(result.data, input.data);
        // Result should not be zero
        assert_ne!(result.data, [0u8; 32]);
    }

    #[test]
    fn test_poseidon2_hash_deterministic() {
        let input = Fr { data: [42u8; 32] };
        
        let result1 = unsafe { poseidon2_hash(&[input]) };
        let result2 = unsafe { poseidon2_hash(&[input]) };
        
        // Same input should produce same output
        assert_eq!(result1.data, result2.data);
    }

    #[test]
    fn test_poseidon2_hash_different_inputs() {
        let input1 = Fr { data: [1u8; 32] };
        let input2 = Fr { data: [2u8; 32] };
        
        let result1 = unsafe { poseidon2_hash(&[input1]) };
        let result2 = unsafe { poseidon2_hash(&[input2]) };
        
        // Different inputs should produce different outputs
        assert_ne!(result1.data, result2.data);
    }

    #[test]
    fn test_poseidon2_hashes_multiple() {
        // poseidon2_hashes processes inputs in pairs, so we need even number of inputs
        let inputs = vec![
            Fr { data: [1u8; 32] },
            Fr { data: [2u8; 32] },
            Fr { data: [3u8; 32] },
            Fr { data: [4u8; 32] },
        ];
        
        let results = unsafe { poseidon2_hashes(&inputs) };
        
        // Should return half as many results as inputs (pairs)
        assert_eq!(results.len(), inputs.len() / 2);
        
        // Each result should be non-zero
        for result in &results {
            assert_ne!(result.data, [0u8; 32]);
        }
        
        // All results should be different from each other
        for i in 0..results.len() {
            for j in i+1..results.len() {
                assert_ne!(results[i].data, results[j].data);
            }
        }
    }

    #[test]
    fn test_poseidon2_hashes_vs_individual() {
        // Test that poseidon2_hashes processes pairs correctly
        let inputs = vec![
            Fr { data: [10u8; 32] },
            Fr { data: [20u8; 32] },
            Fr { data: [30u8; 32] },
            Fr { data: [40u8; 32] },
        ];
        
        // Batch hash (processes pairs)
        let batch_results = unsafe { poseidon2_hashes(&inputs) };
        
        // Individual hashes of pairs
        let individual_results: Vec<Fr> = vec![
            unsafe { poseidon2_hash(&[inputs[0], inputs[1]]) },
            unsafe { poseidon2_hash(&[inputs[2], inputs[3]]) },
        ];
        
        // Results should be the same
        assert_eq!(batch_results.len(), individual_results.len());
        for (batch, individual) in batch_results.iter().zip(individual_results.iter()) {
            assert_eq!(batch.data, individual.data);
        }
    }

    #[test]
    fn test_poseidon2_hash_zero_input() {
        let input = Fr { data: [0u8; 32] };
        
        let result = unsafe { poseidon2_hash(&[input]) };
        
        // Even zero input should produce non-zero output
        assert_ne!(result.data, [0u8; 32]);
        assert_ne!(result.data, input.data);
    }

    #[test]
    fn test_poseidon2_hash_max_input() {
        let input = Fr { data: [0xffu8; 32] };
        
        let result = unsafe { poseidon2_hash(&[input]) };
        
        // Max input should produce valid output
        assert_ne!(result.data, input.data);
        assert_ne!(result.data, [0u8; 32]);
    }

    #[test]
    fn test_poseidon2_hashes_empty_input() {
        let inputs: Vec<Fr> = vec![];
        
        let results = unsafe { poseidon2_hashes(&inputs) };
        
        // Empty input should produce empty output
        assert_eq!(results.len(), 0);
    }

    #[test]
    fn test_poseidon2_hash_multiple_elements() {
        let inputs = vec![
            Fr { data: [1u8; 32] },
            Fr { data: [2u8; 32] },
            Fr { data: [3u8; 32] },
        ];
        
        let result = unsafe { poseidon2_hash(&inputs) };
        
        // Multiple inputs should produce valid hash
        assert_ne!(result.data, [0u8; 32]);
    }

    #[test]
    fn test_poseidon2_hashes_single_pair() {
        let inputs = vec![
            Fr { data: [100u8; 32] },
            Fr { data: [200u8; 32] },
        ];
        
        // Batch hash of single pair
        let batch_results = unsafe { poseidon2_hashes(&inputs) };
        
        // Individual hash of the pair
        let individual_result = unsafe { poseidon2_hash(&inputs) };
        
        // Single pair batch hash and individual hash should be the same
        assert_eq!(batch_results.len(), 1);
        assert_eq!(individual_result.data, batch_results[0].data);
    }

    // JavaScript/WASM compatibility tests
    // These tests verify that the Rust implementation produces identical results to the JS/WASM version

    #[test]
    fn test_poseidon2_permutation_js_compatibility_cpp() {
        // JS test: poseidon2Permutation([0, 1, 2, 3])
        // Expected: [
        //   new Fr(0x01bd538c2ee014ed5141b29e9ae240bf8db3fe5b9a38629a9647cf8d76c01737n),
        //   new Fr(0x239b62e7db98aa3a2a8f6a0d2fa1709e7a35959aa6c7034814d9daa90cbac662n),
        //   new Fr(0x04cbb44c61d928ed06808456bf758cbf0c18d1e15a7b6dbc8245fa7515d5e3cbn),
        //   new Fr(0x2e11c5cff2a22c64d01304b778d78f6998eff1ab73163a35603f54794c30847an),
        // ]
        let mut inputs = vec![
            Fr { data: [0u8; 32] },  // 0
            Fr { data: [0u8; 32] },  // 1
            Fr { data: [0u8; 32] },  // 2
            Fr { data: [0u8; 32] },  // 3
        ];
        // Set the values: 0, 1, 2, 3 in big-endian
        // inputs[0] stays 0
        inputs[1].data[31] = 1;
        inputs[2].data[31] = 2;
        inputs[3].data[31] = 3;
        
        let results = unsafe { poseidon2_permutation(&inputs) };
        
        assert_eq!(results.len(), 4);
        
        // Expected results from the JS test
        let expected_0 = [
            0x01, 0xbd, 0x53, 0x8c, 0x2e, 0xe0, 0x14, 0xed, 0x51, 0x41, 0xb2, 0x9e, 0x9a, 0xe2, 0x40, 0xbf,
            0x8d, 0xb3, 0xfe, 0x5b, 0x9a, 0x38, 0x62, 0x9a, 0x96, 0x47, 0xcf, 0x8d, 0x76, 0xc0, 0x17, 0x37
        ];
        let expected_1 = [
            0x23, 0x9b, 0x62, 0xe7, 0xdb, 0x98, 0xaa, 0x3a, 0x2a, 0x8f, 0x6a, 0x0d, 0x2f, 0xa1, 0x70, 0x9e,
            0x7a, 0x35, 0x95, 0x9a, 0xa6, 0xc7, 0x03, 0x48, 0x14, 0xd9, 0xda, 0xa9, 0x0c, 0xba, 0xc6, 0x62
        ];
        let expected_2 = [
            0x04, 0xcb, 0xb4, 0x4c, 0x61, 0xd9, 0x28, 0xed, 0x06, 0x80, 0x84, 0x56, 0xbf, 0x75, 0x8c, 0xbf,
            0x0c, 0x18, 0xd1, 0xe1, 0x5a, 0x7b, 0x6d, 0xbc, 0x82, 0x45, 0xfa, 0x75, 0x15, 0xd5, 0xe3, 0xcb
        ];
        let expected_3 = [
            0x2e, 0x11, 0xc5, 0xcf, 0xf2, 0xa2, 0x2c, 0x64, 0xd0, 0x13, 0x04, 0xb7, 0x78, 0xd7, 0x8f, 0x69,
            0x98, 0xef, 0xf1, 0xab, 0x73, 0x16, 0x3a, 0x35, 0x60, 0x3f, 0x54, 0x79, 0x4c, 0x30, 0x84, 0x7a
        ];
        
        assert_eq!(results[0].data, expected_0);
        assert_eq!(results[1].data, expected_1);
        assert_eq!(results[2].data, expected_2);
        assert_eq!(results[3].data, expected_3);
    }

    #[test]
    fn test_poseidon2_permutation_js_compatibility_noir() {
        // JS test: poseidon2Permutation([1n, 2n, 3n, 0x0a0000000000000000n])
        // Expected: [
        //   new Fr(0x0369007aa630f5dfa386641b15416ecb16fb1a6f45b1acb903cb986b221a891cn),
        //   new Fr(0x1919fd474b4e2e0f8e0cf8ca98ef285675781cbd31aa4807435385d28e4c02a5n),
        //   new Fr(0x0810e7e9a1c236aae4ebff7d3751d9f7346dc443d1de863977d2b81fe8c557f4n),
        //   new Fr(0x1f4a188575e29985b6f8ad03afc1f0759488f8835aafb6e19e06160fb64d3d4an),
        // ]
        let mut inputs = vec![
            Fr { data: [0u8; 32] },  // 1n
            Fr { data: [0u8; 32] },  // 2n
            Fr { data: [0u8; 32] },  // 3n
            Fr { data: [0u8; 32] },  // 0x0a0000000000000000n
        ];
        
        // Set the values in big-endian
        inputs[0].data[31] = 1;  // 1n
        inputs[1].data[31] = 2;  // 2n
        inputs[2].data[31] = 3;  // 3n
        // 0x0a0000000000000000n = 720575940379279360
        inputs[3].data[23] = 0x0a;  // Set the appropriate bytes for this large number
        
        let results = unsafe { poseidon2_permutation(&inputs) };
        
        assert_eq!(results.len(), 4);
        
        // Expected results from the JS test
        let expected_0 = [
            0x03, 0x69, 0x00, 0x7a, 0xa6, 0x30, 0xf5, 0xdf, 0xa3, 0x86, 0x64, 0x1b, 0x15, 0x41, 0x6e, 0xcb,
            0x16, 0xfb, 0x1a, 0x6f, 0x45, 0xb1, 0xac, 0xb9, 0x03, 0xcb, 0x98, 0x6b, 0x22, 0x1a, 0x89, 0x1c
        ];
        let expected_1 = [
            0x19, 0x19, 0xfd, 0x47, 0x4b, 0x4e, 0x2e, 0x0f, 0x8e, 0x0c, 0xf8, 0xca, 0x98, 0xef, 0x28, 0x56,
            0x75, 0x78, 0x1c, 0xbd, 0x31, 0xaa, 0x48, 0x07, 0x43, 0x53, 0x85, 0xd2, 0x8e, 0x4c, 0x02, 0xa5
        ];
        let expected_2 = [
            0x08, 0x10, 0xe7, 0xe9, 0xa1, 0xc2, 0x36, 0xaa, 0xe4, 0xeb, 0xff, 0x7d, 0x37, 0x51, 0xd9, 0xf7,
            0x34, 0x6d, 0xc4, 0x43, 0xd1, 0xde, 0x86, 0x39, 0x77, 0xd2, 0xb8, 0x1f, 0xe8, 0xc5, 0x57, 0xf4
        ];
        let expected_3 = [
            0x1f, 0x4a, 0x18, 0x85, 0x75, 0xe2, 0x99, 0x85, 0xb6, 0xf8, 0xad, 0x03, 0xaf, 0xc1, 0xf0, 0x75,
            0x94, 0x88, 0xf8, 0x83, 0x5a, 0xaf, 0xb6, 0xe1, 0x9e, 0x06, 0x16, 0x0f, 0xb6, 0x4d, 0x3d, 0x4a
        ];
        
        assert_eq!(results[0].data, expected_0);
        assert_eq!(results[1].data, expected_1);
        assert_eq!(results[2].data, expected_2);
        assert_eq!(results[3].data, expected_3);
    }
} 
