use super::{
    bindgen,
    traits::SerializeBuffer,
};

// ECDSA secp256k1 curve functions

pub unsafe fn ecdsa__compute_public_key(private_key: &[u8; 32]) -> [u8; 64] {
    let mut public_key = [0; 64];
    bindgen::ecdsa__compute_public_key(private_key.as_ptr(), public_key.as_mut_ptr());
    public_key
}

pub unsafe fn ecdsa__construct_signature_(
    message_buf: &[u8],
    private_key: &[u8; 32],
) -> ([u8; 32], [u8; 32], u8) {
    let mut sig_r = [0; 32];
    let mut sig_s = [0; 32];
    let mut sig_v = 0u8;
    bindgen::ecdsa__construct_signature_(
        message_buf.to_buffer().as_slice().as_ptr(),
        private_key.as_ptr(),
        sig_r.as_mut_ptr(),
        sig_s.as_mut_ptr(),
        &mut sig_v,
    );
    (sig_r, sig_s, sig_v)
}

pub unsafe fn ecdsa__recover_public_key_from_signature_(
    message_buf: &[u8],
    sig_r: &[u8; 32],
    sig_s: &[u8; 32],
    sig_v: &mut u8,
) -> [u8; 64] {
    let mut output_pub_key = [0; 64];
    bindgen::ecdsa__recover_public_key_from_signature_(
        message_buf.to_buffer().as_slice().as_ptr(),
        sig_r.as_ptr(),
        sig_s.as_ptr(),
        sig_v,
        output_pub_key.as_mut_ptr(),
    );
    output_pub_key
}

pub unsafe fn ecdsa__verify_signature_(
    message_buf: &[u8],
    pub_key: &[u8; 64],
    sig_r: &[u8; 32],
    sig_s: &[u8; 32],
    sig_v: &u8,
) -> bool {
    let mut result = false;
    bindgen::ecdsa__verify_signature_(
        message_buf.to_buffer().as_slice().as_ptr(),
        pub_key.as_ptr(),
        sig_r.as_ptr(),
        sig_s.as_ptr(),
        sig_v,
        &mut result,
    );
    result
}

// ECDSA secp256r1 curve functions

pub unsafe fn ecdsa_r_compute_public_key(private_key: &[u8; 32]) -> [u8; 64] {
    let mut public_key = [0; 64];
    bindgen::ecdsa_r_compute_public_key(private_key.as_ptr(), public_key.as_mut_ptr());
    public_key
}

pub unsafe fn ecdsa_r_construct_signature_(
    message_buf: &[u8],
    private_key: &[u8; 32],
) -> ([u8; 32], [u8; 32], u8) {
    let mut sig_r = [0; 32];
    let mut sig_s = [0; 32];
    let mut sig_v = 0u8;
    bindgen::ecdsa_r_construct_signature_(
        message_buf.to_buffer().as_slice().as_ptr(),
        private_key.as_ptr(),
        sig_r.as_mut_ptr(),
        sig_s.as_mut_ptr(),
        &mut sig_v,
    );
    (sig_r, sig_s, sig_v)
}

pub unsafe fn ecdsa_r_recover_public_key_from_signature_(
    message_buf: &[u8],
    sig_r: &[u8; 32],
    sig_s: &[u8; 32],
    sig_v: &mut u8,
) -> [u8; 64] {
    let mut output_pub_key = [0; 64];
    bindgen::ecdsa_r_recover_public_key_from_signature_(
        message_buf.to_buffer().as_slice().as_ptr(),
        sig_r.as_ptr(),
        sig_s.as_ptr(),
        sig_v,
        output_pub_key.as_mut_ptr(),
    );
    output_pub_key
}

pub unsafe fn ecdsa_r_verify_signature_(
    message_buf: &[u8],
    pub_key: &[u8; 64],
    sig_r: &[u8; 32],
    sig_s: &[u8; 32],
    sig_v: &u8,
) -> bool {
    let mut result = false;
    bindgen::ecdsa_r_verify_signature_(
        message_buf.to_buffer().as_slice().as_ptr(),
        pub_key.as_ptr(),
        sig_r.as_ptr(),
        sig_s.as_ptr(),
        sig_v,
        &mut result,
    );
    result
} 
