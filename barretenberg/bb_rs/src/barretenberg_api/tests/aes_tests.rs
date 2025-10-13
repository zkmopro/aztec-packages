#[cfg(test)]
mod tests {
    use crate::barretenberg_api::aes::{aes_encrypt_buffer_cbc, aes_decrypt_buffer_cbc};
    use crate::barretenberg_api::bindgen;

    // Initialize the slab allocator before running AES tests
    fn init_allocator() {
        unsafe {
            let circuit_size = 1024u32;
            bindgen::common_init_slab_allocator(&circuit_size);
        }
    }

    /// Apply PKCS#7 padding to input data (for testing purposes)
    fn apply_pkcs7_padding(input: &[u8]) -> Vec<u8> {
        let block_size = 16;
        let padding_len = block_size - (input.len() % block_size);
        let mut padded = input.to_vec();
        padded.extend(vec![padding_len as u8; padding_len]);
        padded
    }

    /// Remove PKCS#7 padding from decrypted data (for testing purposes)
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

    #[test]
    fn test_aes_encrypt_decrypt_roundtrip() {
        init_allocator();
        
        let plaintext = b"Hello, AES world! This is a test message for encryption.";
        let key = [0x2b, 0x7e, 0x15, 0x16, 0x28, 0xae, 0xd2, 0xa6, 
                   0xab, 0xf7, 0x15, 0x88, 0x09, 0xcf, 0x4f, 0x3c]; // 128-bit key
        let iv = [0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07,
                  0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f]; // 128-bit IV

        unsafe {
            // Apply padding before encryption (simulating TypeScript layer behavior)
            let padded_plaintext = apply_pkcs7_padding(plaintext);
            let encrypted_buffer = aes_encrypt_buffer_cbc(&padded_plaintext, &iv, &key);
            let ciphertext = encrypted_buffer.as_slice();
            assert_ne!(ciphertext, plaintext);
            assert!(!ciphertext.is_empty());

            let decrypted_buffer = aes_decrypt_buffer_cbc(ciphertext, &iv, &key);
            let decrypted_with_padding = decrypted_buffer.as_slice();
            
            // Remove padding after decryption (simulating TypeScript layer behavior)
            let decrypted = remove_pkcs7_padding(decrypted_with_padding)
                .expect("Failed to remove padding");
            
            println!("=== DEBUG INFO ===");
            println!("Plaintext: {:?}", plaintext);
            println!("Padded plaintext len: {}", padded_plaintext.len());
            println!("Ciphertext len: {}, data: {:?}", ciphertext.len(), &ciphertext[..std::cmp::min(ciphertext.len(), 20)]);
            println!("Decrypted len: {}, data: {:?}", decrypted.len(), &decrypted[..std::cmp::min(decrypted.len(), 20)]);
            
            // The decrypted data should match the original plaintext exactly
            assert_eq!(decrypted, plaintext, "Decrypted data doesn't match plaintext");
        }
    }

    #[test]
    fn test_aes_buffer_encrypt_decrypt() {
        let plaintext = b"AES buffer test message";
        let key = [0x2b, 0x7e, 0x15, 0x16, 0x28, 0xae, 0xd2, 0xa6, 
                   0xab, 0xf7, 0x15, 0x88, 0x09, 0xcf, 0x4f, 0x3c];
        let iv = [0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07,
                  0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f];

        unsafe {
            // Apply padding before encryption
            let padded_plaintext = apply_pkcs7_padding(plaintext);
            let encrypted_buffer = aes_encrypt_buffer_cbc(&padded_plaintext, &iv, &key);
            assert!(!encrypted_buffer.as_slice().is_empty());

            let decrypted_buffer = aes_decrypt_buffer_cbc(encrypted_buffer.as_slice(), &iv, &key);
            let decrypted_with_padding = decrypted_buffer.as_slice();
            
            // Remove padding after decryption
            let decrypted = remove_pkcs7_padding(decrypted_with_padding)
                .expect("Failed to remove padding");
            
            assert_eq!(decrypted, plaintext, "Decrypted data doesn't match plaintext");
        }
    }

    #[test]
    fn test_aes_different_keys_produce_different_outputs() {
        let plaintext = b"Test message for key difference";
        let key1 = [0x2b, 0x7e, 0x15, 0x16, 0x28, 0xae, 0xd2, 0xa6, 
                    0xab, 0xf7, 0x15, 0x88, 0x09, 0xcf, 0x4f, 0x3c];
        let key2 = [0x3c, 0x4f, 0xcf, 0x09, 0x88, 0x15, 0xf7, 0xab, 
                    0xa6, 0xd2, 0xae, 0x28, 0x16, 0x15, 0x7e, 0x2b];
        let iv = [0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07,
                  0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f];

        unsafe {
            // Apply padding before encryption
            let padded_plaintext = apply_pkcs7_padding(plaintext);
            let encrypted_buffer1 = aes_encrypt_buffer_cbc(&padded_plaintext, &iv, &key1);
            let encrypted_buffer2 = aes_encrypt_buffer_cbc(&padded_plaintext, &iv, &key2);
            let ciphertext1 = encrypted_buffer1.as_slice();
            let ciphertext2 = encrypted_buffer2.as_slice();
            
            // Different keys should produce different ciphertexts
            assert_ne!(ciphertext1, ciphertext2);
        }
    }

    #[test]
    fn test_aes_empty_input() {
        let plaintext = b"";
        let key = [0x2b, 0x7e, 0x15, 0x16, 0x28, 0xae, 0xd2, 0xa6, 
                   0xab, 0xf7, 0x15, 0x88, 0x09, 0xcf, 0x4f, 0x3c];
        let iv = [0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07,
                  0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f];

        unsafe {
            // Apply padding before encryption (empty input becomes 16 bytes of padding)
            let padded_plaintext = apply_pkcs7_padding(plaintext);
            assert_eq!(padded_plaintext.len(), 16); // Should be exactly one block of padding
            
            let encrypted_buffer = aes_encrypt_buffer_cbc(&padded_plaintext, &iv, &key);
            let ciphertext = encrypted_buffer.as_slice();
            // Should produce exactly one block of encrypted data
            assert_eq!(ciphertext.len(), 16);
            
            let decrypted_buffer = aes_decrypt_buffer_cbc(ciphertext, &iv, &key);
            let decrypted_with_padding = decrypted_buffer.as_slice();
            
            // Remove padding after decryption
            let decrypted = remove_pkcs7_padding(decrypted_with_padding)
                .expect("Failed to remove padding");
            
            // Should decrypt back to empty
            assert_eq!(decrypted, plaintext);
        }
    }

    #[test] 
    fn test_aes_padding_validation() {
        let key = [0x2b, 0x7e, 0x15, 0x16, 0x28, 0xae, 0xd2, 0xa6, 
                   0xab, 0xf7, 0x15, 0x88, 0x09, 0xcf, 0x4f, 0x3c];
        let iv = [0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07,
                  0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f];

        // Test that unpadded input (not multiple of 16) panics
        let unpadded_input = b"This is not padded properly"; // 27 bytes, not multiple of 16
        
        let result = std::panic::catch_unwind(|| {
            unsafe {
                aes_encrypt_buffer_cbc(unpadded_input, &iv, &key);
            }
        });
        
        assert!(result.is_err(), "Function should panic with unpadded input");
    }
}
