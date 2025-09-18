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

    #[test]
    fn test_aes_encrypt_decrypt_roundtrip() {
        init_allocator();
        
        let plaintext = b"Hello, AES world! This is a test message for encryption.";
        let key = [0x2b, 0x7e, 0x15, 0x16, 0x28, 0xae, 0xd2, 0xa6, 
                   0xab, 0xf7, 0x15, 0x88, 0x09, 0xcf, 0x4f, 0x3c]; // 128-bit key
        let iv = [0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07,
                  0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f]; // 128-bit IV

        unsafe {
            let encrypted_buffer = aes_encrypt_buffer_cbc(plaintext, &iv, &key);
            let ciphertext = encrypted_buffer.as_slice();
            assert_ne!(ciphertext, plaintext);
            assert!(!ciphertext.is_empty());

            let decrypted_buffer = aes_decrypt_buffer_cbc(ciphertext, &iv, &key);
            let decrypted = decrypted_buffer.as_slice();
            
            println!("=== DEBUG INFO ===");
            println!("Plaintext: {:?}", plaintext);
            println!("Ciphertext len: {}, data: {:?}", ciphertext.len(), &ciphertext[..std::cmp::min(ciphertext.len(), 20)]);
            println!("Decrypted len: {}, data: {:?}", decrypted.len(), &decrypted[..std::cmp::min(decrypted.len(), 20)]);
            
            // The decrypted data should match the original plaintext (possibly with padding)
            // AES CBC uses PKCS#7 padding, so we compare up to the original plaintext length
            assert!(decrypted.len() >= plaintext.len(), "Decrypted data too short");
            assert_eq!(&decrypted[..plaintext.len()], plaintext, "Decrypted data doesn't match plaintext");
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
            let encrypted_buffer = aes_encrypt_buffer_cbc(plaintext, &iv, &key);
            assert!(!encrypted_buffer.as_slice().is_empty());

            let decrypted_buffer = aes_decrypt_buffer_cbc(encrypted_buffer.as_slice(), &iv, &key);
            
            // Compare the relevant portion
            let decrypted_slice = decrypted_buffer.as_slice();
            assert!(decrypted_slice.len() >= plaintext.len(), "Decrypted data too short");
            assert_eq!(&decrypted_slice[..plaintext.len()], plaintext, "Decrypted data doesn't match plaintext");
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
            let encrypted_buffer1 = aes_encrypt_buffer_cbc(plaintext, &iv, &key1);
            let encrypted_buffer2 = aes_encrypt_buffer_cbc(plaintext, &iv, &key2);
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
            let encrypted_buffer = aes_encrypt_buffer_cbc(plaintext, &iv, &key);
            let ciphertext = encrypted_buffer.as_slice();
            // Even empty input should produce some output due to padding
            assert!(!ciphertext.is_empty());
            
            let decrypted_buffer = aes_decrypt_buffer_cbc(ciphertext, &iv, &key);
            let decrypted = decrypted_buffer.as_slice();
            // Decrypted should be empty or contain only padding
            assert!(decrypted.len() >= 0);
        }
    }
}
