#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CipherError {
    // expected public fields
   pub validation: bool,
   pub expected: String, 
}
impl CipherError {
    pub fn new(validation: bool, expected: String) -> CipherError {
        CipherError{
            validation,
            expected,
        }
    }
}
pub fn cipher(original: &str, ciphered: &str) -> Option<Result<bool, CipherError>> {
    let mut res = String::new();
    if original.is_empty() || ciphered.is_empty(){
        return None;
    }
    for i in original.chars(){
        let val = i as u8;
            if i.is_ascii_uppercase(){
                let cip = (122 - (val-97)) as char;
                res.push( cip);
            } else if i.is_ascii_lowercase(){
                let cip = (90 - (val-65)) as char;
                res.push( cip );
            }else{
                res.push(i);
            }
        }
        
        if res == ciphered {
            return Some(Ok(true));
        }
    Some(Err(CipherError::new(false, res)))
    }


   
    