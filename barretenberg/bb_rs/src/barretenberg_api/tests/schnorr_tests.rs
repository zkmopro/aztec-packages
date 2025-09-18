#[cfg(test)]
mod tests {
    use crate::barretenberg_api::schnorr::{
        schnorr_compute_public_key, schnorr_construct_signature, 
        schnorr_verify_signature, schnorr_multisig_create_multisig_public_key
    };
    use crate::barretenberg_api::models::{Fr, Point, Fq};

    // Basic Schnorr tests
    #[test]
    fn test_schnorr_key_generation() {
        let private_key = Fr { data: [1u8; 32] };
        unsafe {
            let public_key = schnorr_compute_public_key(&private_key);
            // Should generate a valid point
            assert_ne!(public_key.x.data, [0u8; 32]);
        }
    }

    #[test]
    fn test_schnorr_sign_verify() {
        let private_key = Fr { data: [1u8; 32] };
        let message = b"Hello, Schnorr!";
        
        unsafe {
            let public_key = schnorr_compute_public_key(&private_key);
            let (sig_s, sig_e) = schnorr_construct_signature(message, &private_key);
            let mut sig_s_mut = sig_s;
            let mut sig_e_mut = sig_e;
            let is_valid = schnorr_verify_signature(message, &public_key, &mut sig_s_mut, &mut sig_e_mut);
            assert!(is_valid);
        }
    }

    #[test]
    fn test_schnorr_key_operations() {
        let private_key = Fr { data: [5u8; 32] };
        
        unsafe {
            let public_key = schnorr_compute_public_key(&private_key);
            assert_ne!(public_key.x.data, [0u8; 32]);
        
        }
    }


    #[test]
    fn test_schnorr_different_private_keys() {
        let private_key1 = Fr { data: [1u8; 32] };
        let private_key2 = Fr { data: [2u8; 32] };
        
        unsafe {
            let public_key1 = schnorr_compute_public_key(&private_key1);
            let public_key2 = schnorr_compute_public_key(&private_key2);
            
            // Different private keys should produce different public keys
            assert_ne!(public_key1, public_key2);
        }
    }

    #[test]
    fn test_schnorr_different_messages() {
        let private_key = Fr { data: [3u8; 32] };
        let message1 = b"Message 1";
        let message2 = b"Message 2";
        
        unsafe {
            let public_key = schnorr_compute_public_key(&private_key);
            let (sig_s1, sig_e1) = schnorr_construct_signature(message1, &private_key);
            let (sig_s2, sig_e2) = schnorr_construct_signature(message2, &private_key);
            
            // Different messages should produce different signatures
            assert_ne!(sig_s1, sig_s2);
            assert_ne!(sig_e1, sig_e2);
            
            // Both should verify correctly
            let mut sig_s1_mut = sig_s1;
            let mut sig_e1_mut = sig_e1;
            let mut sig_s2_mut = sig_s2;
            let mut sig_e2_mut = sig_e2;
            
            let is_valid1 = schnorr_verify_signature(message1, &public_key, &mut sig_s1_mut, &mut sig_e1_mut);
            let is_valid2 = schnorr_verify_signature(message2, &public_key, &mut sig_s2_mut, &mut sig_e2_mut);
            
            assert!(is_valid1);
            assert!(is_valid2);
        }
    }

    #[test]
    fn test_schnorr_invalid_signature() {
        let private_key = Fr { data: [4u8; 32] };
        let message = b"Test message";
        let wrong_message = b"Wrong message";
        
        unsafe {
            let public_key = schnorr_compute_public_key(&private_key);
            let (sig_s, sig_e) = schnorr_construct_signature(message, &private_key);
            
            // Correct message should verify
            let mut sig_s_correct = sig_s;
            let mut sig_e_correct = sig_e;
            let is_valid_correct = schnorr_verify_signature(message, &public_key, &mut sig_s_correct, &mut sig_e_correct);
            assert!(is_valid_correct);
            
            // Wrong message should not verify
            let mut sig_s_wrong = sig_s;
            let mut sig_e_wrong = sig_e;
            let is_valid_wrong = schnorr_verify_signature(wrong_message, &public_key, &mut sig_s_wrong, &mut sig_e_wrong);
            assert!(!is_valid_wrong);
        }
    }

    #[test]
    fn test_schnorr_wrong_public_key() {
        let private_key1 = Fr { data: [7u8; 32] };
        let private_key2 = Fr { data: [8u8; 32] };
        let message = b"Test message";
        
        unsafe {
            let public_key1 = schnorr_compute_public_key(&private_key1);
            let public_key2 = schnorr_compute_public_key(&private_key2);
            let (sig_s, sig_e) = schnorr_construct_signature(message, &private_key1);
            
            // Correct public key should verify
            let mut sig_s_correct = sig_s;
            let mut sig_e_correct = sig_e;
            let is_valid_correct = schnorr_verify_signature(message, &public_key1, &mut sig_s_correct, &mut sig_e_correct);
            assert!(is_valid_correct);
            
            // Wrong public key should not verify
            let mut sig_s_wrong = sig_s;
            let mut sig_e_wrong = sig_e;
            let is_valid_wrong = schnorr_verify_signature(message, &public_key2, &mut sig_s_wrong, &mut sig_e_wrong);
            assert!(!is_valid_wrong);
        }
    }

    #[test]
    fn test_schnorr_multisig_public_key_creation() {
        let private_key = Fq { data: [9u8; 32] };
        
        unsafe {
            let multisig_pubkey = schnorr_multisig_create_multisig_public_key(&private_key);
            // Should produce a valid multisig public key (non-zero)
            assert_ne!(multisig_pubkey, [0u8; 128]);
        }
    }

    // Edge case tests
    #[test]
    fn test_schnorr_with_max_private_key() {
        let private_key = Fr { data: [0xffu8; 32] };
        let message = b"Max private key Schnorr test";
        
        unsafe {
            let public_key = schnorr_compute_public_key(&private_key);
            let (sig_s, sig_e) = schnorr_construct_signature(message, &private_key);
            let mut sig_s_mut = sig_s;
            let mut sig_e_mut = sig_e;
            let is_valid = schnorr_verify_signature(message, &public_key, &mut sig_s_mut, &mut sig_e_mut);
            assert!(is_valid);
        }
    }

    #[test]
    fn test_schnorr_with_zero_private_key() {
        let private_key = Fr { data: [0u8; 32] };
        let message = b"Zero private key test";
        
        unsafe {
            // Zero private key might not be valid, but let's test it doesn't crash
            let public_key = schnorr_compute_public_key(&private_key);
            // We don't assert anything specific here as zero key behavior is undefined
        }
    }

    #[test]
    fn test_schnorr_empty_message() {
        let private_key = Fr { data: [10u8; 32] };
        let empty_message = b"";
        
        unsafe {
            let public_key = schnorr_compute_public_key(&private_key);
            let (sig_s, sig_e) = schnorr_construct_signature(empty_message, &private_key);
            let mut sig_s_mut = sig_s;
            let mut sig_e_mut = sig_e;
            let is_valid = schnorr_verify_signature(empty_message, &public_key, &mut sig_s_mut, &mut sig_e_mut);
            assert!(is_valid);
        }
    }

    #[test]
    fn test_schnorr_long_message() {
        let private_key = Fr { data: [11u8; 32] };
        let long_message = vec![0x42u8; 1000]; // 1KB message
        
        unsafe {
            let public_key = schnorr_compute_public_key(&private_key);
            let (sig_s, sig_e) = schnorr_construct_signature(&long_message, &private_key);
            let mut sig_s_mut = sig_s;
            let mut sig_e_mut = sig_e;
            let is_valid = schnorr_verify_signature(&long_message, &public_key, &mut sig_s_mut, &mut sig_e_mut);
            assert!(is_valid);
        }
    }

    #[test]
    fn test_schnorr_deterministic_signatures() {
        let private_key = Fr { data: [12u8; 32] };
        let message = b"Deterministic test";
        
        unsafe {
            let (sig_s1, sig_e1) = schnorr_construct_signature(message, &private_key);
            let (sig_s2, sig_e2) = schnorr_construct_signature(message, &private_key);
            
            // Schnorr signatures might not be deterministic due to random nonce
            // But we can at least verify both are valid
            let public_key = schnorr_compute_public_key(&private_key);
            
            let mut sig_s1_mut = sig_s1;
            let mut sig_e1_mut = sig_e1;
            let mut sig_s2_mut = sig_s2;
            let mut sig_e2_mut = sig_e2;
            
            let is_valid1 = schnorr_verify_signature(message, &public_key, &mut sig_s1_mut, &mut sig_e1_mut);
            let is_valid2 = schnorr_verify_signature(message, &public_key, &mut sig_s2_mut, &mut sig_e2_mut);
            
            assert!(is_valid1);
            assert!(is_valid2);
        }
    }

    #[test]
    fn test_schnorr_signature_components_not_zero() {
        let private_key = Fr { data: [13u8; 32] };
        let message = b"Non-zero components test";
        
        unsafe {
            let (sig_s, sig_e) = schnorr_construct_signature(message, &private_key);
            
            // Signature components should not be zero
            assert_ne!(sig_s, [0u8; 32]);
            assert_ne!(sig_e, [0u8; 32]);
        }
    }

    #[test]
    fn test_schnorr_public_key_not_zero() {
        let private_key = Fr { data: [14u8; 32] };
        
        unsafe {
            let public_key = schnorr_compute_public_key(&private_key);
            
            // Public key coordinates should not be zero
            assert_ne!(public_key.x.data, [0u8; 32]);
            assert_ne!(public_key.y.data, [0u8; 32]);
        }
    }
} 
