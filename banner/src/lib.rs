use std::collections::HashMap;
use std::num::ParseFloatError;
#[derive(Debug, PartialEq, Eq)]
struct Flag {
    short_hand: String,
    long_hand: String,
    desc: String,
}
#[warn(dead_code)]
impl Flag {
    pub fn opt_flag(l_h : &str, d :&str) -> Flag {
        Flag{
            short_hand: format!("-{}", l_h.chars().next().unwrap()),
            long_hand: format!("--{}", l_h),
            desc : d.to_string(),
        } 

    }
}

pub type Callback = fn(&str, &str) -> Result<String, ParseFloatError>;

pub struct FlagsHandler {
    pub flags: HashMap<(String, String), Callback>,
}

impl FlagsHandler {
    pub fn add_flag(&mut self, flag: (String, String), func: Callback) {
        self.flags.insert(flag, func);
    }
    pub fn exec_func(&mut self, flag: (String, String), argv: &[&str]) -> String {
        match self.flags.get(&flag) {
            Some(callback) => {
                if let Ok(result) = callback(argv[0], argv[1]) {
                    return result;
                } else {
                    return "invalid float literal".to_string();
                }
            }
            None => return "no Flag".to_string(),
        }
    }
}

pub fn div(a: &str, b: &str) -> Result<String, ParseFloatError> {
    // let a = a.parse::<f32>().unwrap();
    let a : f32 = a.parse()?;
    let b : f32 = b.parse()?;
    Ok((a / b).to_string())
}
pub fn rem(a: &str, b: &str) -> Result<String, ParseFloatError> {
    let a : f32 = a.parse()?;
    let b : f32 = b.parse()?;
    Ok((a % b).to_string())
}


// fn main() {
//     let mut handler = FlagsHandler { flags: HashMap::new() };

//     let d = Flag::opt_flag("division", "divides the values, formula (a / b)");
//     let r = Flag::opt_flag(
//         "remainder",
//         "remainder of the division between two values, formula (a % b)",
//     );

//     handler.add_flag((d.short_hand, d.long_hand), div);
//     handler.add_flag((r.short_hand, r.long_hand), rem);

//     println!("{:?}", handler.exec_func(("-d".to_string(), "--division".to_string()), &["1.0", "2.0"]));

//     println!("{:?}",handler.exec_func(("-r".to_string(), "--remainder".to_string()), &["2.0", "2.0"]));

//     println!("{:?}",handler.exec_func(("-d".to_string(), "--division".to_string()), &["a", "2.0"]));

//     println!("{:?}",handler.exec_func(("-r".to_string(), "--remainder".to_string()), &["2.0", "fd"]));
// }
