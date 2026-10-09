use magic_crypt::{new_magic_crypt, MagicCryptTrait};
use std::fs;
use std::path::PathBuf;

fn get_magic_crypt() -> magic_crypt::MagicCrypt256 {
    // A hardcoded secret key for local obfuscation to prevent manual editing.
    new_magic_crypt!("VisualClient_SuperSecretKey_123!@#", 256)
}

pub fn read_encrypted_file(file: &PathBuf) -> Result<String, String> {
    if !file.exists() {
        return Ok(String::new());
    }
    let content = fs::read_to_string(file).map_err(|e| e.to_string())?;
    
    // Support migration from older plain text
    if content.starts_with('[') || content.starts_with('{') || content.is_empty() {
        Ok(content)
    } else {
        let mc = get_magic_crypt();
        mc.decrypt_base64_to_string(&content).map_err(|e| e.to_string())
    }
}

pub fn write_encrypted_file(file: &PathBuf, data: &str) -> Result<(), String> {
    if let Some(parent) = file.parent() {
        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    let mc = get_magic_crypt();
    let encrypted = mc.encrypt_str_to_base64(data);
    fs::write(file, encrypted).map_err(|e| e.to_string())
}
