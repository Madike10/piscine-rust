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
            let mut _cipher : u8 ;
            if i.is_ascii_lowercase(){
                _cipher = 122 - ((i as u8)-97);
                res.push( _cipher as char);
            } else if i.is_ascii_uppercase(){
                _cipher = 90 - ((i as u8)-65);
                res.push( _cipher as char);
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
    