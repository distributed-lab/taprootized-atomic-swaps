mod verifier;

use libc::{c_char, c_ulong, c_void};
use std::ffi::CStr;

pub use verifier::{groth16_verifier, FromJson, PublicInputs, VerificationKey, ZkProof};

#[derive(Debug, thiserror::Error)]
#[error("failed to generate groth16 proof: {0}")]
pub struct Error(String);

const ERROR_MSG_SIZE: usize = 4096;

pub fn groth16_prover(zkey: &[u8], witness: &[u8]) -> Result<(String, String), Error> {
    let zkey_size = zkey.len() as c_ulong;
    let wtns_size = witness.len() as c_ulong;

    let mut proof_size = 0;
    let mut public_size = 0;

    // Initial dummy call to get buffer sizes
    unsafe {
        rapidsnark_sys::groth16_prover(
            zkey.as_ptr() as *const c_void,
            zkey_size,
            witness.as_ptr() as *const c_void,
            wtns_size,
            std::ptr::null_mut(),
            &mut proof_size,
            std::ptr::null_mut(),
            &mut public_size,
            std::ptr::null_mut(),
            0,
        );
    }

    // Allocate buffers based on the sizes obtained
    let mut proof_buffer = vec![0 as c_char; proof_size as usize];
    let mut public_buffer = vec![0 as c_char; public_size as usize];
    let mut error_message = vec![0 as c_char; ERROR_MSG_SIZE];

    let result = unsafe {
        rapidsnark_sys::groth16_prover(
            zkey.as_ptr() as *const c_void,
            zkey_size,
            witness.as_ptr() as *const c_void,
            wtns_size,
            proof_buffer.as_mut_ptr(),
            &mut proof_size,
            public_buffer.as_mut_ptr(),
            &mut public_size,
            error_message.as_mut_ptr(),
            ERROR_MSG_SIZE as c_ulong,
        )
    };

    if result != 0 {
        // If there was an error, convert the error message to a Rust String and return it
        let error_str = unsafe {
            CStr::from_ptr(error_message.as_ptr())
                .to_string_lossy()
                .into_owned()
        };

        return Err(Error(error_str));
    }

    // Convert the output buffers to Rust Strings
    let proof = unsafe {
        CStr::from_ptr(proof_buffer.as_ptr())
            .to_string_lossy()
            .into_owned()
    };
    let public_inputs = unsafe {
        CStr::from_ptr(public_buffer.as_ptr())
            .to_string_lossy()
            .into_owned()
    };

    Ok((proof, public_inputs))
}
