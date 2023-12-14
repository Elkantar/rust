pub fn insert(vec: &mut Vec<String>, val: String) {
    vec.push(val)
}

pub fn at_index(vec: &Vec<String>, index: usize) -> String {
    if let Some(value) = vec.get(index) {
        value.clone()
    } else {
        String::new()
    }
}