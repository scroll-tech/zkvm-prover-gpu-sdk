use std::ffi::c_char;

pub unsafe extern "C" fn init(config: *const c_char) { 
    todo!()
}

pub unsafe extern "C" fn generate_chunk_proof(
    proof: *mut c_char,
    input: *const c_char,
    fork_name: *const c_char,
) {
    todo!()
}

pub unsafe extern "C" fn generate_batch_proof(
    proof: *mut c_char,
    input: *const c_char,
    fork_name: *const c_char,
) {
    todo!()
}

pub unsafe extern "C" fn generate_bundle_proof(
    proof: *mut c_char,
    input: *const c_char,
    fork_name: *const c_char,
) {
    todo!()
}
