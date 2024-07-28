use crate::error::verify_binary_error::VerifyBinaryError;

const BINARIES: [&str; 1] = ["dbgw"];

pub(crate) fn verify_all_binaries() -> Result<(), VerifyBinaryError> {
    for bin in BINARIES {
        match verify_binary(bin) {
            Ok(_) => {}
            Err(e) => return Err(e),
        }
    }

    Ok(())
}

fn verify_binary(binary: &str) -> Result<(), VerifyBinaryError> {
    match check_if_binary_exists(binary) {
        Ok(_) => {}
        Err(e) => return Err(e),
    }

    Ok(())
}

fn check_if_binary_exists(binary: &str) -> Result<(), VerifyBinaryError> {
    Err(VerifyBinaryError::BinaryNotFound(
        "Not implemented".to_string(),
    ))
}
