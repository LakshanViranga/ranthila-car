use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use std::fs;
use std::path::Path;
use uuid::Uuid;

pub fn save_image(image: Vec<u8>, directory: &Path) -> Result<String, String> {
    let file_name = format!("{}.jpg", Uuid::new_v4());

    let file_path = directory.join(&file_name);

    fs::write(&file_path, image).map_err(|e| format!("Failed to save image: {}", e))?;

    Ok(file_name)
}

pub fn get_image(image: String, directory: &Path) -> Result<Vec<u8>, String> {
    let full_path = directory.join(&image);
    let data = fs::read(&full_path).map_err(|e| format!("Failed to read image: {}", e));

    Ok(data?)
}

pub fn hash_password(password: &str) -> Result<String, String> {
    let salt = SaltString::generate(rand::thread_rng());
    let argon2 = Argon2::default();
    argon2
        .hash_password(password.as_bytes(), &salt)
        .map(|h| h.to_string())
        .map_err(|e| format!("Password hashing failed: {}", e))
}

// Verify password
pub fn verify_password(password: &str, hash: &str) -> Result<bool, String> {
    let parsed_hash =
        PasswordHash::new(hash).map_err(|e| format!("Invalid password hash: {}", e))?;
    let argon2 = Argon2::default();
    Ok(argon2
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok())
}
