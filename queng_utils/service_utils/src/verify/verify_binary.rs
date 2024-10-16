use std::process::Command;

use common_env::prelude::EnvironmentType;

use crate::error::verify_binary_error::VerifyBinaryError;
use crate::fields::{BINARIES, PATH};

/// Verifies all binaries in the `BINARIES` array.
///
/// # Arguments
///
/// * `dbg` - A boolean indicating whether to print debug information.
/// * `env` - The environment to check against.
///
/// # Returns
///
/// Returns `Ok(())` if all binaries are verified successfully, or an
/// `Err(VerifyBinaryError)` if an error occurs.
///
pub(crate) fn verify_all_binaries(
    dbg: bool,
    env: EnvironmentType,
) -> Result<(), VerifyBinaryError> {
    for bin in BINARIES {
        match verify_binary(bin, dbg, env) {
            Ok(_) => {}
            Err(e) => return Err(e),
        }
    }

    Ok(())
}

/// Verifies the given binary.
///
/// # Arguments
///
/// * `binary` - The name of the binary to verify.
/// * `dbg` - A boolean indicating whether to print debug information.
/// * `env` - The environment to check against.
///
/// # Returns
///
/// Returns `Ok(())` if the binary is verified successfully, or an
/// `Err(VerifyBinaryError)` if an error occurs.
///
pub(crate) fn verify_binary(
    binary: &str,
    dbg: bool,
    env: EnvironmentType,
) -> Result<(), VerifyBinaryError> {
    if dbg {
        println!("Check if binary exists: {}", binary)
    }
    match check_if_binary_exists(binary, dbg) {
        Ok(_) => {}
        Err(e) => return Err(e),
    }

    if dbg {
        println!("Check if binary architecture: {}", binary)
    }
    match check_binary_architecture(binary, dbg, env) {
        Ok(_) => {}
        Err(e) => return Err(e),
    }

    Ok(())
}

/// Checks if the binary exists.
///
/// # Arguments
///
/// * `binary` - The name of the binary to check.
/// * `dbg` - A boolean indicating whether to print debug information.
///
/// # Returns
///
/// Returns `Ok(())` if the binary exists, or an `Err(VerifyBinaryError)` if
/// the binary does not.
///
fn check_if_binary_exists(binary: &str, dbg: bool) -> Result<(), VerifyBinaryError> {
    // test -f /path/to/binary
    // https://sentry.io/answers/determine-whether-a-file-exists-or-not-in-bash/
    let binary = format!("{}/{}", PATH, binary);
    let mut cmd = Command::new("test");
    cmd.arg("-f").arg(binary.clone());

    match cmd.output() {
        Ok(out) => {
            if dbg {
                println!(
                    "[check_if_binary_exists]: {} \n
                    success: {} \n
                    Output: {}",
                    binary,
                    out.status.success(),
                    String::from_utf8_lossy(out.stdout.as_slice()),
                );
            }

            if out.status.success() {
                Ok(())
            } else {
                Err(VerifyBinaryError::BinaryNotFound(binary.to_string()))
            }
        }
        Err(e) => Err(VerifyBinaryError::BinaryNotFound(e.to_string())),
    }
}

/// Checks if the binary has the correct architecture for the given environment.
///
/// # Arguments
///
/// * `binary` - The name of the binary to check.
/// * `dbg` - A boolean indicating whether to print debug information.
/// * `env` - The environment to check against.
///
/// # Returns
///
/// Returns `Ok(())` if the binary has the correct architecture, or an
/// `Err(VerifyBinaryError)` if the binary does not.
///
fn check_binary_architecture(
    binary: &str,
    dbg: bool,
    env: EnvironmentType,
) -> Result<(), VerifyBinaryError> {
    let binary = format!("{}/{}", PATH, binary);

    let mut cmd = Command::new("file");
    cmd.arg("-L").arg(binary.clone());

    let msg = match cmd.output() {
        Ok(out) => {
            if dbg {
                println!(
                    "[check_binary_architecture]: {} \n
                    success: {} \n
                    Output: {}",
                    binary,
                    out.status.success(),
                    String::from_utf8_lossy(out.stdout.as_slice()),
                );
            }

            if out.status.success() {
                String::from_utf8_lossy(out.stdout.as_slice()).to_string()
            } else {
                return Err(VerifyBinaryError::BinaryNotFound(binary.to_string()));
            }
        }
        Err(e) => return Err(VerifyBinaryError::BinaryNotFound(e.to_string())),
    };

    match env {
        EnvironmentType::UNKNOWN => {
            panic!("Unknown environment type")
        }

        EnvironmentType::LOCAL => {
            // Local runs on MacOS, ARM64
            // Mach-O 64-bit executable arm64
            if !msg.contains("Mach-O 64-bit executable arm64") {
                return Err(VerifyBinaryError::BinaryWrongPlatform(
                    "Mach-O 64-bit executable arm64".to_string(),
                    msg,
                ));
            };
        }
        EnvironmentType::CLUSTER => {
            // We ASSUME that the cluster runs on Linux, X86_64, STATICALLY linked
            if !msg.contains("ELF 64-bit LSB pie executable, x86-64") {
                return Err(VerifyBinaryError::BinaryWrongPlatform(
                    "ELF 64-bit LSB pie executable, x86-64".to_string(),
                    msg,
                ));
            };
        }
        EnvironmentType::CI => {
            // CI server runs on Linux, X86_64, regular dynamically linked
            if !msg.contains("ELF 64-bit LSB shared object, x86-64") {
                return Err(VerifyBinaryError::BinaryWrongPlatform(
                    "ELF 64-bit LSB shared object, x86-64".to_string(),
                    msg,
                ));
            };
        }
    }

    Ok(())
}
