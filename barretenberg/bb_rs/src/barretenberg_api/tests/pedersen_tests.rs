#[cfg(test)]
mod tests {
    use crate::barretenberg_api::pedersen::{pedersen_commit, pedersen_hash, pedersen_hashes, pedersen_hash_buffer};
    use crate::barretenberg_api::models::Fr;

    // cargo test pedersen_tests -v -- --test-threads=1 --nocapture

    #[test]
    fn test_pedersen_commit() {
        let inputs = vec![Fr { data: [1u8; 32] }, Fr { data: [2u8; 32] }];
        let hash_index = 0u32;
        
        let result = unsafe { pedersen_commit(&inputs, hash_index) };
        
        // Result should be a valid point (non-zero coordinates)
        assert_ne!(result.x.data, [0u8; 32]);
        assert_ne!(result.y.data, [0u8; 32]);
    }

    #[test]
    fn test_pedersen_commit_different_inputs() {
        let inputs1 = vec![Fr { data: [1u8; 32] }, Fr { data: [2u8; 32] }];
        let inputs2 = vec![Fr { data: [3u8; 32] }, Fr { data: [4u8; 32] }];
        let hash_index = 0u32;
        
        let result1 = unsafe { pedersen_commit(&inputs1, hash_index) };
        let result2 = unsafe { pedersen_commit(&inputs2, hash_index) };
        
        // Different inputs should produce different commitments
        assert_ne!(result1.x.data, result2.x.data);
        assert_ne!(result1.y.data, result2.y.data);
    }

    #[test]
    fn test_pedersen_commit_different_hash_index() {
        let inputs = vec![Fr { data: [1u8; 32] }, Fr { data: [2u8; 32] }];
        
        let result1 = unsafe { pedersen_commit(&inputs, 0) };
        let result2 = unsafe { pedersen_commit(&inputs, 1) };
        
        // Different hash indices should produce different commitments
        assert_ne!(result1.x.data, result2.x.data);
        assert_ne!(result1.y.data, result2.y.data);
    }

    #[test]
    fn test_pedersen_hash() {
        let inputs = vec![Fr { data: [1u8; 32] }, Fr { data: [2u8; 32] }];
        let hash_index = 0u32;
        
        let result = unsafe { pedersen_hash(&inputs, hash_index) };
        
        // Result should be non-zero
        assert_ne!(result.data, [0u8; 32]);
    }

    #[test]
    fn test_pedersen_hash_deterministic() {
        let inputs = vec![Fr { data: [42u8; 32] }];
        let hash_index = 0u32;
        
        let result1 = unsafe { pedersen_hash(&inputs, hash_index) };
        let result2 = unsafe { pedersen_hash(&inputs, hash_index) };
        
        // Same input should produce same hash
        assert_eq!(result1.data, result2.data);
    }

    #[test]
    fn test_pedersen_hash_different_inputs() {
        let inputs1 = vec![Fr { data: [1u8; 32] }];
        let inputs2 = vec![Fr { data: [2u8; 32] }];
        let hash_index = 0u32;
        
        let result1 = unsafe { pedersen_hash(&inputs1, hash_index) };
        let result2 = unsafe { pedersen_hash(&inputs2, hash_index) };
        
        // Different inputs should produce different hashes
        assert_ne!(result1.data, result2.data);
    }

    #[test]
    fn test_pedersen_hashes_multiple() {
        // Each inner vector represents a pair of Fr elements to hash
        let inputs = vec![
            vec![Fr { data: [1u8; 32] }, Fr { data: [2u8; 32] }],
            vec![Fr { data: [3u8; 32] }, Fr { data: [4u8; 32] }],
            vec![Fr { data: [5u8; 32] }, Fr { data: [6u8; 32] }],
        ];
        let hash_index = 0u32;
        
        let results = unsafe { pedersen_hashes(&inputs, hash_index) };
        
        // Should return same number of results as input pairs
        assert_eq!(results.len(), inputs.len());
        
        // Each result should be non-zero
        for result in &results {
            assert_ne!(result.data, [0u8; 32]);
        }
        
        // All results should be different
        for i in 0..results.len() {
            for j in i+1..results.len() {
                assert_ne!(results[i].data, results[j].data);
            }
        }
    }

    #[test]
    fn test_pedersen_hash_buffer() {
        let buffer = vec![1u8, 2u8, 3u8, 4u8, 5u8];
        let hash_index = 0u32;
        
        let result = unsafe { pedersen_hash_buffer(&buffer, hash_index) };
        
        // Result should be non-zero
        assert_ne!(result.data, [0u8; 32]);
    }

    #[test]
    fn test_pedersen_hash_buffer_different_data() {
        let buffer1 = vec![1u8, 2u8, 3u8];
        let buffer2 = vec![4u8, 5u8, 6u8];
        let hash_index = 0u32;
        
        let result1 = unsafe { pedersen_hash_buffer(&buffer1, hash_index) };
        let result2 = unsafe { pedersen_hash_buffer(&buffer2, hash_index) };
        
        // Different buffers should produce different hashes
        assert_ne!(result1.data, result2.data);
    }

    #[test]
    fn test_pedersen_hash_buffer_empty() {
        let buffer: Vec<u8> = vec![];
        let hash_index = 0u32;
        
        let result = unsafe { pedersen_hash_buffer(&buffer, hash_index) };
        
        // Even empty buffer should produce valid hash
        assert_ne!(result.data, [0u8; 32]);
    }

    #[test]
    fn test_pedersen_commit_single_input() {
        let inputs = vec![Fr { data: [42u8; 32] }];
        let hash_index = 0u32;
        
        let result = unsafe { pedersen_commit(&inputs, hash_index) };
        
        // Single input should produce valid commitment
        assert_ne!(result.x.data, [0u8; 32]);
        assert_ne!(result.y.data, [0u8; 32]);
    }

    #[test]
    fn test_pedersen_hash_multiple_inputs() {
        let inputs = vec![
            Fr { data: [1u8; 32] },
            Fr { data: [2u8; 32] },
            Fr { data: [3u8; 32] },
            Fr { data: [4u8; 32] },
        ];
        let hash_index = 0u32;
        
        let result = unsafe { pedersen_hash(&inputs, hash_index) };
        
        // Multiple inputs should produce valid hash
        assert_ne!(result.data, [0u8; 32]);
    }

    #[test]
    fn test_pedersen_hashes_vs_individual() {
        let inputs = vec![
            vec![Fr { data: [10u8; 32] }, Fr { data: [11u8; 32] }],
            vec![Fr { data: [20u8; 32] }, Fr { data: [21u8; 32] }],
        ];
        let hash_index = 0u32;
        
        // Batch hash
        let batch_results = unsafe { pedersen_hashes(&inputs, hash_index) };
        
        // Individual hashes
        let individual_results: Vec<Fr> = inputs
            .iter()
            .map(|input| unsafe { pedersen_hash(input, hash_index) })
            .collect();
        
        // Results should be the same
        assert_eq!(batch_results.len(), individual_results.len());
        for (batch, individual) in batch_results.iter().zip(individual_results.iter()) {
            assert_eq!(batch.data, individual.data);
        }
    }

    // JavaScript/WASM compatibility tests
    // These tests verify that the Rust implementation produces identical results to the JS/WASM version

    #[test]
    fn test_pedersen_commit_js_compatibility() {
        // JS test: pedersenCommit([toBufferBE(1n, 32), toBufferBE(1n, 32)])
        // Expected: [
        //   Buffer.from('2f7a8f9a6c96926682205fb73ee43215bf13523c19d7afe36f12760266cdfe15', 'hex'),
        //   Buffer.from('01916b316adbbf0e10e39b18c1d24b33ec84b46daddf72f43878bcc92b6057e6', 'hex'),
        // ]
        let mut input1 = Fr { data: [0; 32] };
        let mut input2 = Fr { data: [0; 32] };
        // Set the last byte to 1 for big-endian representation of 1n
        input1.data[31] = 1;
        input2.data[31] = 1;
        let inputs = vec![input1, input2];
        
        let result = unsafe { pedersen_commit(&inputs, 0) };
        
        // Expected x coordinate
        let expected_x = [
            0x2f, 0x7a, 0x8f, 0x9a, 0x6c, 0x96, 0x92, 0x66, 0x82, 0x20, 0x5f, 0xb7, 0x3e, 0xe4, 0x32, 0x15,
            0xbf, 0x13, 0x52, 0x3c, 0x19, 0xd7, 0xaf, 0xe3, 0x6f, 0x12, 0x76, 0x02, 0x66, 0xcd, 0xfe, 0x15
        ];
        
        // Expected y coordinate  
        let expected_y = [
            0x01, 0x91, 0x6b, 0x31, 0x6a, 0xdb, 0xbf, 0x0e, 0x10, 0xe3, 0x9b, 0x18, 0xc1, 0xd2, 0x4b, 0x33,
            0xec, 0x84, 0xb4, 0x6d, 0xad, 0xdf, 0x72, 0xf4, 0x38, 0x78, 0xbc, 0xc9, 0x2b, 0x60, 0x57, 0xe6
        ];
        
        assert_eq!(result.x.data, expected_x);
        assert_eq!(result.y.data, expected_y);
    }

    #[test]
    fn test_pedersen_commit_with_zero_js_compatibility() {
        // JS test: pedersenCommit([toBufferBE(0n, 32), toBufferBE(1n, 32)])
        // Expected: [
        //   Buffer.from('054aa86a73cb8a34525e5bbed6e43ba1198e860f5f3950268f71df4591bde402', 'hex'),
        //   Buffer.from('209dcfbf2cfb57f9f6046f44d71ac6faf87254afc7407c04eb621a6287cac126', 'hex'),
        // ]
        let input1 = Fr { data: [0; 32] }; // toBufferBE(0n, 32) - all zeros
        let mut input2 = Fr { data: [0; 32] }; // toBufferBE(1n, 32) - big endian 1
        input2.data[31] = 1;
        let inputs = vec![input1, input2];
        
        let result = unsafe { pedersen_commit(&inputs, 0) };
        
        // Expected x coordinate
        let expected_x = [
            0x05, 0x4a, 0xa8, 0x6a, 0x73, 0xcb, 0x8a, 0x34, 0x52, 0x5e, 0x5b, 0xbe, 0xd6, 0xe4, 0x3b, 0xa1,
            0x19, 0x8e, 0x86, 0x0f, 0x5f, 0x39, 0x50, 0x26, 0x8f, 0x71, 0xdf, 0x45, 0x91, 0xbd, 0xe4, 0x02
        ];
        
        // Expected y coordinate
        let expected_y = [
            0x20, 0x9d, 0xcf, 0xbf, 0x2c, 0xfb, 0x57, 0xf9, 0xf6, 0x04, 0x6f, 0x44, 0xd7, 0x1a, 0xc6, 0xfa,
            0xf8, 0x72, 0x54, 0xaf, 0xc7, 0x40, 0x7c, 0x04, 0xeb, 0x62, 0x1a, 0x62, 0x87, 0xca, 0xc1, 0x26
        ];
        
        assert_eq!(result.x.data, expected_x);
        assert_eq!(result.y.data, expected_y);
    }

    #[test]
    fn test_pedersen_hash_js_compatibility() {
        // JS test: pedersenHash([toBufferBE(1n, 32), toBufferBE(1n, 32)])
        // Expected: '0x07ebfbf4df29888c6cd6dca13d4bb9d1a923013ddbbcbdc3378ab8845463297b'
        let mut input1 = Fr { data: [0; 32] };
        let mut input2 = Fr { data: [0; 32] };
        input1.data[31] = 1;
        input2.data[31] = 1;
        let inputs = vec![input1, input2];
        
        let result = unsafe { pedersen_hash(&inputs, 0) };
        
        let expected = [
            0x07, 0xeb, 0xfb, 0xf4, 0xdf, 0x29, 0x88, 0x8c, 0x6c, 0xd6, 0xdc, 0xa1, 0x3d, 0x4b, 0xb9, 0xd1,
            0xa9, 0x23, 0x01, 0x3d, 0xdb, 0xbc, 0xbd, 0xc3, 0x37, 0x8a, 0xb8, 0x84, 0x54, 0x63, 0x29, 0x7b
        ];
        
        assert_eq!(result.data, expected);
    }

    #[test]
    fn test_pedersen_hash_with_index_js_compatibility() {
        // JS test: pedersenHash([toBufferBE(1n, 32), toBufferBE(1n, 32)], 5)
        // Expected: '0x1c446df60816b897cda124524e6b03f36df0cec333fad87617aab70d7861daa6'
        let mut input1 = Fr { data: [0; 32] };
        let mut input2 = Fr { data: [0; 32] };
        input1.data[31] = 1;
        input2.data[31] = 1;
        let inputs = vec![input1, input2];
        
        let result = unsafe { pedersen_hash(&inputs, 5) };
        
        let expected = [
            0x1c, 0x44, 0x6d, 0xf6, 0x08, 0x16, 0xb8, 0x97, 0xcd, 0xa1, 0x24, 0x52, 0x4e, 0x6b, 0x03, 0xf3,
            0x6d, 0xf0, 0xce, 0xc3, 0x33, 0xfa, 0xd8, 0x76, 0x17, 0xaa, 0xb7, 0x0d, 0x78, 0x61, 0xda, 0xa6
        ];
        
        assert_eq!(result.data, expected);
    }

    #[test]
    fn test_pedersen_hash_buffer_js_compatibility() {
        // JS test: Buffer.alloc(123) with writeUint32BE(321, 0) and writeUint32BE(456, 119)
        // This creates a 123-byte buffer with 321 at position 0 and 456 at position 119
        let mut buffer = vec![0u8; 123];
        // writeUint32BE(321, 0) - write 321 as big-endian uint32 at position 0
        buffer[0] = 0x00;
        buffer[1] = 0x00;
        buffer[2] = 0x01;
        buffer[3] = 0x41; // 321 in big-endian
        
        // writeUint32BE(456, 119) - write 456 as big-endian uint32 at position 119
        buffer[119] = 0x00;
        buffer[120] = 0x00;
        buffer[121] = 0x01;
        buffer[122] = 0xc8; // 456 in big-endian
        
        let result = unsafe { pedersen_hash_buffer(&buffer, 0) };
        
        // The exact expected value would come from running the JS test
        // For now, we verify it produces a non-zero result
        assert_ne!(result.data, [0u8; 32]);
        
        // TODO: Run JS test to get exact expected value and update this assertion
        // This test verifies the structure works correctly
    }
} 
