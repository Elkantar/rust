use std::collections::HashMap;
use std::num::ParseFloatError;

// Define the Flag struct
pub struct Flag {
    pub short_hand: String,
    pub long_hand: String,
    pub desc: String,
}

impl Flag {
    pub fn opt_flag(l_h: &str, d: &str) -> Flag {
        Flag {
            short_hand: format!("-{}", l_h.chars().next().unwrap()), // get the first character for shorthand
            long_hand: format!("--{}", l_h),
            desc: d.to_string(),
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
        if let Some(func) = self.flags.get(&flag) {
            match func(argv[0], argv[1]) {
                Ok(res) => res,
                Err(_) => "invalid float literal".to_string(),
            }
        } else {
            "flag not found".to_string()
        }
    }
}

pub fn div(a: &str, b: &str) -> Result<String, ParseFloatError> {
    let x: f32 = a.parse()?;
    let y: f32 = b.parse()?;
    
    if y == 0.0 {
        return Ok("inf".to_string());
    }
    
    Ok((x / y).to_string())
}



pub fn rem(a: &str, b: &str) -> Result<String, ParseFloatError> {
    let x: f32 = a.parse()?;
    let y: f32 = b.parse()?;
    
    if y == 0.0 {
        return Ok("NaN".to_string());
    }
    
    Ok((x % y).to_string())
}