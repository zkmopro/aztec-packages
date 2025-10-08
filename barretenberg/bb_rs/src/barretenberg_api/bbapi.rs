use super::bindgen;
use std::ptr;
use num_bigint::BigUint;
use serde::{Deserialize, Serialize};
use rmp_serde::{encode, decode};

// This is not used for now, but may replace the acir functions later

#[derive(Debug, Serialize, Deserialize)]
pub struct CircuitInputNoVK {
    pub name: String,
    #[serde(with = "serde_bytes")]
    pub bytecode: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CircuitInput {
    pub name: String,
    #[serde(with = "serde_bytes")]
    pub bytecode: Vec<u8>,
    #[serde(with = "serde_bytes")]
    pub verification_key: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProofSystemSettings {
    #[serde(default)]
    pub ipa_accumulation: bool,
    #[serde(default = "default_oracle_hash_type")]
    pub oracle_hash_type: String,
    #[serde(default)]
    pub disable_zk: bool,
    #[serde(default)]
    pub optimized_solidity_verifier: bool,
}

fn default_oracle_hash_type() -> String {
    "poseidon2".to_string()
}

impl Default for ProofSystemSettings {
    fn default() -> Self {
        Self {
            ipa_accumulation: false,
            oracle_hash_type: "poseidon2".to_string(),
            disable_zk: false,
            optimized_solidity_verifier: false,
        }
    }
}

// Command structs
#[derive(Debug, Serialize, Deserialize)]
pub struct CircuitProve {
    pub circuit: CircuitInput,
    #[serde(with = "serde_bytes")]
    pub witness: Vec<u8>,
    pub settings: ProofSystemSettings,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CircuitProveResponse {
    pub public_inputs: Vec<[u8; 32]>,
    pub proof: Vec<[u8; 32]>,
    pub vk: CircuitComputeVkResponse,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CircuitVerify {
    #[serde(with = "serde_bytes")]
    pub verification_key: Vec<u8>,
    pub public_inputs: Vec<[u8; 32]>,
    pub proof: Vec<[u8; 32]>,
    pub settings: ProofSystemSettings,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CircuitVerifyResponse {
    pub verified: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CircuitComputeVk {
    pub circuit: CircuitInputNoVK,
    pub settings: ProofSystemSettings,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CircuitComputeVkResponse {
    #[serde(with = "serde_bytes")]
    pub bytes: Vec<u8>,
    pub fields: Vec<[u8; 32]>,
    #[serde(with = "serde_bytes")]
    pub hash: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VkAsFields {
    #[serde(with = "serde_bytes")]
    pub verification_key: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VkAsFieldsResponse {
    pub fields: Vec<[u8; 32]>,
}

// Error handling
#[derive(Debug, thiserror::Error)]
pub enum BbApiError {
    #[error("Msgpack encode error: {0}")]
    EncodeError(#[from] encode::Error),
    #[error("Msgpack decode error: {0}")]
    DecodeError(#[from] decode::Error),
    #[error("Invalid response: expected {expected}, got {actual}")]
    InvalidResponse { expected: String, actual: String },
    #[error("API error: {0}")]
    ApiError(String),
}

impl From<BbApiError> for String {
    fn from(error: BbApiError) -> Self {
        match error {
            BbApiError::EncodeError(e) => e.to_string(),
            BbApiError::DecodeError(e) => e.to_string(),
            BbApiError::InvalidResponse { expected, actual } => format!("Invalid response: expected {}, got {}", expected, actual),
            BbApiError::ApiError(e) => e,
        }
    }
}

impl std::str::FromStr for BbApiError {
    type Err = BbApiError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Err(BbApiError::ApiError(s.to_string()))
    }
}

// Main bbapi function that handles msgpack encoding/decoding
pub unsafe fn bbapi_command<T, R>(command_name: &str, command_data: &T) -> Result<R, BbApiError>
where
    T: Serialize,
    R: for<'de> Deserialize<'de>,
{
    // Encode command as msgpack array [command_name, command_data]
    let command = (command_name, command_data);
    let encoded_command = encode::to_vec(&command)?;
    
    // Call the C++ bbapi function
    let response_bytes = bbapi(&encoded_command);
    
    // Decode response as msgpack array [response_name, response_data]
    let (response_name, response_data): (String, R) = decode::from_slice(&response_bytes)?;
    
    // Verify response type matches what we expect
    let expected_response = format!("{}Response", command_name);
    if response_name != expected_response {
        return Err(BbApiError::InvalidResponse {
            expected: expected_response,
            actual: response_name,
        });
    }
    
    Ok(response_data)
}

// Low-level bbapi function that directly calls the C++ binding
pub unsafe fn bbapi(command: &[u8]) -> Vec<u8> {
    let mut out_ptr: *mut u8 = ptr::null_mut();
    let mut out_len: usize = 0;
    
    // Call the C++ bbapi function with all 4 required parameters
    bindgen::bbapi(
        command.as_ptr(),              // input buffer
        command.len(),                 // input length
        &mut out_ptr,                  // output buffer pointer
        &mut out_len,                  // output length pointer
    );
    
    // Convert the output to a Vec<u8>
    if out_ptr.is_null() {
        Vec::new()
    } else {
        let result = std::slice::from_raw_parts(out_ptr, out_len).to_vec();
        // Note: The C++ code is responsible for memory management of out_ptr
        result
    }
}

// Helper functions to convert between different representations
pub fn pack_proof_into_fields(vec_u8: &[u8]) -> Vec<[u8; 32]> {
    vec_u8.chunks(32).map(|chunk| {
        let mut array = [0u8; 32];
        array.copy_from_slice(chunk);
        array
    }).collect()
}

pub fn fields_to_bytes(fields: &[[u8; 32]]) -> Vec<u8> {
    fields.iter().flat_map(|field| field.iter().copied()).collect()
}

pub fn pack_proof_into_biguints(vec_u8: &[u8]) -> Vec<BigUint> {
    vec_u8.chunks(32).map(|chunk| BigUint::from_bytes_be(chunk)).collect()
}

pub fn from_biguints_to_hex_strings(biguints: &[BigUint]) -> Vec<String> {
    biguints.iter().map(|biguint| format!("0x{:064x}", biguint)).collect()
}

// High-level API functions using the new command-based approach

/// Generate a proof using the bbapi command system
pub unsafe fn prove_ultra_honk(
    constraint_system_buf: &[u8],
    witness_buf: &[u8],
    vkey_buf: &[u8],
) -> Result<Vec<u8>, BbApiError> {
    let settings = ProofSystemSettings {
        ipa_accumulation: false,
        oracle_hash_type: "poseidon2".to_string(),
        disable_zk: false,
        optimized_solidity_verifier: false,
    };

    let command = CircuitProve {
        circuit: CircuitInput {
            name: "circuit".to_string(),
            bytecode: constraint_system_buf.to_vec(),
            verification_key: vkey_buf.to_vec(),
        },
        witness: witness_buf.to_vec(),
        settings,
    };

    let response = bbapi_command::<CircuitProve, CircuitProveResponse>("CircuitProve", &command)?;
    Ok(fields_to_bytes(&response.proof))
}

/// Generate a proof using Keccak for EVM verification
pub unsafe fn prove_ultra_keccak_honk(
    constraint_system_buf: &[u8],
    witness_buf: &[u8],
    vkey_buf: &[u8],
) -> Result<Vec<u8>, BbApiError> {
    let settings = ProofSystemSettings {
        ipa_accumulation: false,
        oracle_hash_type: "keccak".to_string(),
        disable_zk: true,
        optimized_solidity_verifier: false,
    };

    let command = CircuitProve {
        circuit: CircuitInput {
            name: "circuit".to_string(),
            bytecode: constraint_system_buf.to_vec(),
            verification_key: vkey_buf.to_vec(),
        },
        witness: witness_buf.to_vec(),
        settings,
    };

    let response = bbapi_command::<CircuitProve, CircuitProveResponse>("CircuitProve", &command)?;
    Ok(fields_to_bytes(&response.proof))
}

/// Generate a proof using Keccak with ZK enabled
pub unsafe fn prove_ultra_keccak_zk_honk(
    constraint_system_buf: &[u8],
    witness_buf: &[u8],
    vkey_buf: &[u8],
) -> Result<Vec<u8>, BbApiError> {
    let settings = ProofSystemSettings {
        ipa_accumulation: false,
        oracle_hash_type: "keccak".to_string(),
        disable_zk: false,
        optimized_solidity_verifier: false,
    };

    let command = CircuitProve {
        circuit: CircuitInput {
            name: "circuit".to_string(),
            bytecode: constraint_system_buf.to_vec(),
            verification_key: vkey_buf.to_vec(),
        },
        witness: witness_buf.to_vec(),
        settings,
    };

    let response = bbapi_command::<CircuitProve, CircuitProveResponse>("CircuitProve", &command)?;
    Ok(fields_to_bytes(&response.proof))
}

/// Compute verification key
pub unsafe fn get_ultra_honk_verification_key(constraint_system_buf: &[u8]) -> Result<Vec<u8>, BbApiError> {
    let settings = ProofSystemSettings {
        ipa_accumulation: false,
        oracle_hash_type: "poseidon2".to_string(),
        disable_zk: false,
        optimized_solidity_verifier: false,
    };

    let command = CircuitComputeVk {
        circuit: CircuitInputNoVK {
            name: "circuit".to_string(),
            bytecode: constraint_system_buf.to_vec(),
        },
        settings,
    };

    let response = bbapi_command::<CircuitComputeVk, CircuitComputeVkResponse>("CircuitComputeVk", &command)?;
    Ok(response.bytes)
}

/// Compute verification key for Keccak
pub unsafe fn get_ultra_honk_keccak_verification_key(constraint_system_buf: &[u8]) -> Result<Vec<u8>, BbApiError> {
    let settings = ProofSystemSettings {
        ipa_accumulation: false,
        oracle_hash_type: "keccak".to_string(),
        disable_zk: true,
        optimized_solidity_verifier: false,
    };

    let command = CircuitComputeVk {
        circuit: CircuitInputNoVK {
            name: "circuit".to_string(),
            bytecode: constraint_system_buf.to_vec(),
        },
        settings,
    };

    let response = bbapi_command::<CircuitComputeVk, CircuitComputeVkResponse>("CircuitComputeVk", &command)?;
    Ok(response.bytes)
}

/// Compute verification key for Keccak with ZK
pub unsafe fn get_ultra_honk_keccak_zk_verification_key(constraint_system_buf: &[u8]) -> Result<Vec<u8>, BbApiError> {
    let settings = ProofSystemSettings {
        ipa_accumulation: false,
        oracle_hash_type: "keccak".to_string(),
        disable_zk: false,
        optimized_solidity_verifier: false,
    };

    let command = CircuitComputeVk {
        circuit: CircuitInputNoVK {
            name: "circuit".to_string(),
            bytecode: constraint_system_buf.to_vec(),
        },
        settings,
    };

    let response = bbapi_command::<CircuitComputeVk, CircuitComputeVkResponse>("CircuitComputeVk", &command)?;
    Ok(response.bytes)
}

/// Verify a proof
pub unsafe fn verify_ultra_honk(proof_buf: &[u8], vkey_buf: &[u8]) -> Result<bool, BbApiError> {
    let settings = ProofSystemSettings {
        ipa_accumulation: false,
        oracle_hash_type: "poseidon2".to_string(),
        disable_zk: false,
        optimized_solidity_verifier: false,
    };

    let command = CircuitVerify {
        verification_key: vkey_buf.to_vec(),
        public_inputs: vec![], // Public inputs are embedded in the proof for Honk
        proof: pack_proof_into_fields(proof_buf),
        settings,
    };

    let response = bbapi_command::<CircuitVerify, CircuitVerifyResponse>("CircuitVerify", &command)?;
    Ok(response.verified)
}

/// Verify a Keccak proof
pub unsafe fn verify_ultra_keccak_honk(proof_buf: &[u8], vkey_buf: &[u8]) -> Result<bool, BbApiError> {
    let settings = ProofSystemSettings {
        ipa_accumulation: false,
        oracle_hash_type: "keccak".to_string(),
        disable_zk: true,
        optimized_solidity_verifier: false,
    };

    let command = CircuitVerify {
        verification_key: vkey_buf.to_vec(),
        public_inputs: vec![], // Public inputs are embedded in the proof for Honk
        proof: pack_proof_into_fields(proof_buf),
        settings,
    };

    let response = bbapi_command::<CircuitVerify, CircuitVerifyResponse>("CircuitVerify", &command)?;
    Ok(response.verified)
}

/// Verify a Keccak ZK proof
pub unsafe fn verify_ultra_keccak_zk_honk(proof_buf: &[u8], vkey_buf: &[u8]) -> Result<bool, BbApiError> {
    let settings = ProofSystemSettings {
        ipa_accumulation: false,
        oracle_hash_type: "keccak".to_string(),
        disable_zk: false,
        optimized_solidity_verifier: false,
    };

    let command = CircuitVerify {
        verification_key: vkey_buf.to_vec(),
        public_inputs: vec![], // Public inputs are embedded in the proof for Honk
        proof: pack_proof_into_fields(proof_buf),
        settings,
    };

    let response = bbapi_command::<CircuitVerify, CircuitVerifyResponse>("CircuitVerify", &command)?;
    Ok(response.verified)
}

/// Convert VK to field elements
pub unsafe fn vk_as_fields_ultra_honk(vk_buf: &[u8]) -> Result<Vec<u8>, BbApiError> {
    let command = VkAsFields {
        verification_key: vk_buf.to_vec(),
    };

    let response = bbapi_command::<VkAsFields, VkAsFieldsResponse>("VkAsFields", &command)?;
    Ok(fields_to_bytes(&response.fields))
}

/// Convert proof to hex strings
pub unsafe fn proof_as_fields_ultra_honk(proof_buf: &[u8]) -> Vec<String> {
    from_biguints_to_hex_strings(&pack_proof_into_biguints(&proof_buf))
}
