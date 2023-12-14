#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CipherError {
    pub validation: bool,
    pub expected: String,
}
impl CipherError {
    pub fn new(validation: bool, expected: String) -> CipherError {
        CipherError { validation, expected }
    }
}
pub fn cipher(original: &str, ciphered: &str) -> Option<Result<bool, CipherError>> {
    if original.is_empty() || ciphered.is_empty() {
        return None;
    }
    
    let atbash_ciphered = original.chars().map(|c| {
        match c {
            'a'..='z' => ('z' as u8 - c as u8 + 'a' as u8) as char,
            'A'..='Z' => ('Z' as u8 - c as u8 + 'A' as u8) as char,
            _ => c,
        }
    }).collect::<String>();
    
    if atbash_ciphered == ciphered {
        Some(Ok(true))
    } else {
        Some(Err(CipherError::new(false, atbash_ciphered)))
    }
}
