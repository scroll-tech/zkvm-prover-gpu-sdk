pub mod prover;
pub mod zk_circuits_handler;
pub mod utils;

use prover::ProofType;
use std::{
    ffi::{c_char, CString},
    ptr,
};
use utils::c_char_to_str;

#[no_mangle]
pub unsafe extern "C" fn init(config: *const c_char) {
    let _ = env_logger::try_init();
    let config_str = c_char_to_str(config);
    prover::init(config_str.to_string());
}

fn generate_proof(input: *const c_char, proof_type: ProofType, fork_name: *const c_char,) -> *mut c_char {
    let input_str = c_char_to_str(input).to_string();
    let fork_name_str = c_char_to_str(fork_name);
    prover::set_active_handler(fork_name_str);

    match ACTIVE_HANDLER
        .1
        .as_ref()
        .get_proof_data(proof_type.clone(), input_str, fork_name_str)
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
pub unsafe extern "C" fn generate_chunk_proof(input: *const c_char, fork_name: *const c_char) -> *mut c_char {
    generate_proof(input, ProofType::Chunk, fork_name)
}

#[no_mangle]
pub unsafe extern "C" fn generate_batch_proof(input: *const c_char, fork_name: *const c_char) -> *mut c_char {
    generate_proof(input, ProofType::Batch, fork_name)
}

#[no_mangle]
pub unsafe extern "C" fn generate_bundle_proof(input: *const c_char, fork_name: *const c_char) -> *mut c_char {
    generate_proof(input, ProofType::Bundle, fork_name)
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

fn get_vk(circuit_type: ProofType, fork_name: *const c_char) -> *mut c_char {
    let fork_name_str = c_char_to_str(fork_name);
    prover::set_active_handler(fork_name_str);

    match ACTIVE_HANDLER.1.as_ref().get_vk(circuit_type.clone()) {
        Some(vk) => {
            if let Ok(vk) = CString::new(base64::encode(vk)).and_then(|vk| Ok(vk.into_raw())) {
                vk as *mut c_char
            } else {
                log::error!("failed to copy vk to output buffer");
                ptr::null::<c_char>() as *mut c_char
            }
        }
        None => {
            log::error!("failed to get vk for circuit type = {:?}", circuit_type);
            ptr::null::<c_char>() as *mut c_char
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn get_chunk_vk(fork_name: *const c_char) -> *mut c_char {
    get_vk(ProofType::Chunk, fork_name)
}

#[no_mangle]
pub unsafe extern "C" fn get_batch_vk(fork_name: *const c_char) -> *mut c_char {
    get_vk(ProofType::Batch, fork_name)
}

#[no_mangle]
pub unsafe extern "C" fn get_bundle_vk(fork_name: *const c_char) -> *mut c_char {
    get_vk(ProofType::Bundle, fork_name)
}

#[no_mangle]
pub unsafe extern "C" fn free_vk(vk: *mut c_char) {
    if vk.is_null() {
        return;
    }
    // convert the pointer to a CString and then drop it
    unsafe {
        drop(CString::from_raw(vk));
    }
}
