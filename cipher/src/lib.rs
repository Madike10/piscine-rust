#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CipherError {
    // expected public fields
   pub boolean: bool,
   pub error: String, 
}
impl CipherError {
    pub fn new(validation: bool, expected: String) -> CipherError {
        CipherError{
            boolean : validation,
            error: expected,
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
       return Some(Err(CipherError::new(false, res)));
    }


    fn main() {
        println!("{:?}", cipher("1Hello 2world!", "1Svool 2dliow!"));
        println!("{:?}", cipher("1Hello 2world!", "svool"));
        println!("{:?}", cipher("", "svool"));
    }
    