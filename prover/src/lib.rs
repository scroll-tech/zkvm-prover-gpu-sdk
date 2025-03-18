mod prover;
mod utils;

use prover::ProofType;
use std::{
    ffi::{c_char, CString},
    ptr,
};
use utils::c_char_to_str;

#[no_mangle]
pub unsafe extern "C" fn init(config: *const c_char) {
    let config_str = c_char_to_str(config);
    prover::init(config_str.to_string());
}

fn generate_proof(input: *const c_char, proof_type: ProofType) -> *mut c_char {
    let prover = prover::get_prover().unwrap();
    let input_str = c_char_to_str(input).to_string();

    match prover
        .as_ref()
        .get_proof_data(proof_type.clone(), input_str)
    {
        Err(e) => {
            log::error!(
                "failed to generate proof for type = {:?}, error = {}",
                proof_type,
                e
            );
            ptr::null::<c_char>() as *mut c_char
        }
        Ok(proof_data) => {
            if let Ok(proof_data) =
                CString::new(proof_data).and_then(|proof_data| Ok(proof_data.into_raw()))
            {
                proof_data as *mut c_char
            } else {
                log::error!("failed to copy proof data to output buffer");
                ptr::null::<c_char>() as *mut c_char
            }
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn generate_chunk_proof(
    input: *const c_char,
    _fork_name: *const c_char,
) -> *mut c_char {
    generate_proof(input, ProofType::Chunk)
}

#[no_mangle]
pub unsafe extern "C" fn generate_batch_proof(
    input: *const c_char,
    _fork_name: *const c_char,
) -> *mut c_char {
    generate_proof(input, ProofType::Batch)
}

#[no_mangle]
pub unsafe extern "C" fn generate_bundle_proof(
    input: *const c_char,
    _fork_name: *const c_char,
) -> *mut c_char {
    generate_proof(input, ProofType::Bundle)
}

#[no_mangle]
pub extern "C" fn free_proof(proof: *mut c_char) {
    if proof.is_null() {
        return;
    }
    // convert the pointer to a CString and then drop it
    unsafe {
        drop(CString::from_raw(proof));
    }
}
