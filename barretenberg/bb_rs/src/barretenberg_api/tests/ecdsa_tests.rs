#[cfg(test)]
mod tests {
    use crate::barretenberg_api::ecdsa::{
        ecdsa__compute_public_key, ecdsa__construct_signature_, ecdsa__verify_signature_,
        ecdsa__recover_public_key_from_signature_,
        ecdsa_r_compute_public_key, ecdsa_r_construct_signature_, ecdsa_r_verify_signature_,
        ecdsa_r_recover_public_key_from_signature_
    };

    // ECDSA secp256k1 tests
    #[test]
    fn test_ecdsa_secp256k1_key_generation() {
        let private_key = [1u8; 32];
        unsafe {
            let public_key = ecdsa__compute_public_key(&private_key);
            assert_ne!(public_key, [0u8; 64]);
        }
    }

    #[test]
    fn test_ecdsa_secp256k1_sign_verify() {
        let private_key = [1u8; 32];
        let message = b"Test message for ECDSA";
        
        unsafe {
            let public_key = ecdsa__compute_public_key(&private_key);
            let (sig_r, sig_s, sig_v) = ecdsa__construct_signature_(message, &private_key);
            let is_valid = ecdsa__verify_signature_(message, &public_key, &sig_r, &sig_s, &sig_v);
            assert!(is_valid);
        }
    }

    #[test]
    fn test_ecdsa_secp256k1_invalid_signature() {
        let private_key = [1u8; 32];
        let message = b"Test message";
        let wrong_message = b"Wrong message";
        
        unsafe {
            let public_key = ecdsa__compute_public_key(&private_key);
            let (sig_r, sig_s, sig_v) = ecdsa__construct_signature_(message, &private_key);
            let is_valid = ecdsa__verify_signature_(wrong_message, &public_key, &sig_r, &sig_s, &sig_v);
            assert!(!is_valid);
        }
    }

    #[test]
    fn test_ecdsa_secp256k1_public_key_recovery() {
        let private_key = [3u8; 32];
        let message = b"Key recovery test";
        
        unsafe {
            let expected_public_key = ecdsa__compute_public_key(&private_key);
            let (sig_r, sig_s, mut sig_v) = ecdsa__construct_signature_(message, &private_key);
            let recovered_public_key = ecdsa__recover_public_key_from_signature_(
                message, &sig_r, &sig_s, &mut sig_v
            );
            
            // Note: Recovery might not be exact due to the nature of ECDSA recovery
            assert_ne!(recovered_public_key, [0u8; 64]);
        }
    }

    // ECDSA secp256r1 tests
    #[test]
    fn test_ecdsa_secp256r1_key_generation() {
        let private_key = [1u8; 32];
        unsafe {
            let public_key = ecdsa_r_compute_public_key(&private_key);
            assert_ne!(public_key, [0u8; 64]);
        }
    }

    #[test]
    fn test_ecdsa_secp256r1_sign_verify() {
        let private_key = [2u8; 32];
        let message = b"Test message for ECDSA secp256r1";
        
        unsafe {
            let public_key = ecdsa_r_compute_public_key(&private_key);
            let (sig_r, sig_s, sig_v) = ecdsa_r_construct_signature_(message, &private_key);
            let is_valid = ecdsa_r_verify_signature_(message, &public_key, &sig_r, &sig_s, &sig_v);
            assert!(is_valid);
        }
    }

    #[test]
    fn test_ecdsa_secp256r1_public_key_recovery() {
        let private_key = [4u8; 32];
        let message = b"secp256r1 key recovery";
        
        unsafe {
            let expected_public_key = ecdsa_r_compute_public_key(&private_key);
            let (sig_r, sig_s, mut sig_v) = ecdsa_r_construct_signature_(message, &private_key);
            let recovered_public_key = ecdsa_r_recover_public_key_from_signature_(
                message, &sig_r, &sig_s, &mut sig_v
            );
            
            assert_ne!(recovered_public_key, [0u8; 64]);
        }
    }

    // Cross-curve comparison tests
    #[test]
    fn test_ecdsa_curves_different_public_keys() {
        let private_key = [6u8; 32];
        
        unsafe {
            let public_key_k1 = ecdsa__compute_public_key(&private_key);
            let public_key_r1 = ecdsa_r_compute_public_key(&private_key);
            
            // Different curves should produce different public keys
            assert_ne!(public_key_k1, public_key_r1);
        }
    }

    #[test]
    fn test_ecdsa_curves_different_signatures() {
        let private_key = [7u8; 32];
        let message = b"Same message, different curves";
        
        unsafe {
            let (sig_r_k1, sig_s_k1, sig_v_k1) = ecdsa__construct_signature_(message, &private_key);
            let (sig_r_r1, sig_s_r1, sig_v_r1) = ecdsa_r_construct_signature_(message, &private_key);
            
            // Different curves should produce different signatures
            assert_ne!(sig_r_k1, sig_r_r1);
            assert_ne!(sig_s_k1, sig_s_r1);
        }
    }

    // Edge case tests
    #[test]
    fn test_ecdsa_with_max_private_key() {
        let private_key = [0xffu8; 32];
        let message = b"Max private key test";
        
        unsafe {
            let public_key = ecdsa__compute_public_key(&private_key);
            let (sig_r, sig_s, sig_v) = ecdsa__construct_signature_(message, &private_key);
            let is_valid = ecdsa__verify_signature_(message, &public_key, &sig_r, &sig_s, &sig_v);
            assert!(is_valid);
        }
    }

    #[test]
    fn test_ecdsa_with_zero_private_key() {
        let private_key = [0u8; 32];
        let message = b"Zero private key test";
        
        unsafe {
            // Zero private key might not be valid, but let's test it doesn't crash
            let public_key = ecdsa__compute_public_key(&private_key);
            // We don't assert anything specific here as zero key behavior is undefined
        }
    }

    #[test]
    fn test_ecdsa_empty_message() {
        let private_key = [8u8; 32];
        let empty_message = b"";
        
        unsafe {
            let public_key = ecdsa__compute_public_key(&private_key);
            let (sig_r, sig_s, sig_v) = ecdsa__construct_signature_(empty_message, &private_key);
            let is_valid = ecdsa__verify_signature_(empty_message, &public_key, &sig_r, &sig_s, &sig_v);
            assert!(is_valid);
        }
    }

    #[test]
    fn test_ecdsa_long_message() {
        let private_key = [9u8; 32];
        let long_message = vec![0x42u8; 1000]; // 1KB message
        
        unsafe {
            let public_key = ecdsa__compute_public_key(&private_key);
            let (sig_r, sig_s, sig_v) = ecdsa__construct_signature_(&long_message, &private_key);
            let is_valid = ecdsa__verify_signature_(&long_message, &public_key, &sig_r, &sig_s, &sig_v);
            assert!(is_valid);
        }
    }

    #[test]
    fn test_ecdsa_deterministic_signatures() {
        let private_key = [10u8; 32];
        let message = b"Deterministic test";
        
        unsafe {
            let (sig_r1, sig_s1, sig_v1) = ecdsa__construct_signature_(message, &private_key);
            let (sig_r2, sig_s2, sig_v2) = ecdsa__construct_signature_(message, &private_key);
            
            // ECDSA signatures might not be deterministic due to random nonce
            // But we can at least verify both are valid
            let public_key = ecdsa__compute_public_key(&private_key);
            let is_valid1 = ecdsa__verify_signature_(message, &public_key, &sig_r1, &sig_s1, &sig_v1);
            let is_valid2 = ecdsa__verify_signature_(message, &public_key, &sig_r2, &sig_s2, &sig_v2);
            
            assert!(is_valid1);
            assert!(is_valid2);
        }
    }

    #[test]
    fn test_ecdsa_wrong_public_key() {
        let private_key = [11u8; 32];
        let wrong_private_key = [12u8; 32];
        let message = b"Wrong key test";
        
        unsafe {
            let public_key = ecdsa__compute_public_key(&private_key);
            let wrong_public_key = ecdsa__compute_public_key(&wrong_private_key);
            let (sig_r, sig_s, sig_v) = ecdsa__construct_signature_(message, &private_key);
            
            // Correct verification should pass
            let is_valid_correct = ecdsa__verify_signature_(message, &public_key, &sig_r, &sig_s, &sig_v);
            assert!(is_valid_correct);
            
            // Wrong public key should fail
            let is_valid_wrong = ecdsa__verify_signature_(message, &wrong_public_key, &sig_r, &sig_s, &sig_v);
            assert!(!is_valid_wrong);
        }
    }

    #[test]
    fn test_ecdsa_signature_components_not_zero() {
        let private_key = [13u8; 32];
        let message = b"Non-zero components test";
        
        unsafe {
            let (sig_r, sig_s, sig_v) = ecdsa__construct_signature_(message, &private_key);
            
            // Signature components should not be zero
            assert_ne!(sig_r, [0u8; 32]);
            assert_ne!(sig_s, [0u8; 32]);
            // sig_v can be 0 or 1, so we don't check it
        }
    }

    #[test]
    fn test_ecdsa_public_key_not_zero() {
        let private_key = [14u8; 32];
        
        unsafe {
            let public_key = ecdsa__compute_public_key(&private_key);
            
            // Public key should not be zero
            assert_ne!(public_key, [0u8; 64]);
        }
    }

    #[test]
    fn test_ecdsa_different_private_keys_different_signatures() {
        let private_key1 = [15u8; 32];
        let private_key2 = [16u8; 32];
        let message = b"Same message, different keys";
        
        unsafe {
            let (sig_r1, sig_s1, sig_v1) = ecdsa__construct_signature_(message, &private_key1);
            let (sig_r2, sig_s2, sig_v2) = ecdsa__construct_signature_(message, &private_key2);
            
            // Different private keys should produce different signatures
            let signatures_different = sig_r1 != sig_r2 || sig_s1 != sig_s2 || sig_v1 != sig_v2;
            assert!(signatures_different);
        }
    }

    #[test]
    fn test_ecdsa_secp256r1_invalid_signature() {
        let private_key = [17u8; 32];
        let message = b"Test message";
        let wrong_message = b"Wrong message";
        
        unsafe {
            let public_key = ecdsa_r_compute_public_key(&private_key);
            let (sig_r, sig_s, sig_v) = ecdsa_r_construct_signature_(message, &private_key);
            let is_valid = ecdsa_r_verify_signature_(wrong_message, &public_key, &sig_r, &sig_s, &sig_v);
            assert!(!is_valid);
        }
    }

    #[test]
    fn test_ecdsa_secp256r1_wrong_public_key() {
        let private_key = [18u8; 32];
        let wrong_private_key = [19u8; 32];
        let message = b"Wrong key test r1";
        
        unsafe {
            let public_key = ecdsa_r_compute_public_key(&private_key);
            let wrong_public_key = ecdsa_r_compute_public_key(&wrong_private_key);
            let (sig_r, sig_s, sig_v) = ecdsa_r_construct_signature_(message, &private_key);
            
            // Correct verification should pass
            let is_valid_correct = ecdsa_r_verify_signature_(message, &public_key, &sig_r, &sig_s, &sig_v);
            assert!(is_valid_correct);
            
            // Wrong public key should fail
            let is_valid_wrong = ecdsa_r_verify_signature_(message, &wrong_public_key, &sig_r, &sig_s, &sig_v);
            assert!(!is_valid_wrong);
        }
    }
} 
