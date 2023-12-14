use std::collections::HashMap;

pub fn bigger(h: HashMap<&str, i32>) -> i32 {
    let mut max_value = i32::min_value(); 

    for &value in h.values() {
        if value > max_value {
            max_value = value;
        }
    }

    max_value
}