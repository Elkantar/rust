pub fn transform_and_save_on_heap(s: String) -> Box<Vec<u32>> {
    let mut result = Vec::new();

    for part in s.split_whitespace() {
        if let Ok(mut value) = part.to_lowercase().replace("k", "").parse::<f64>() {
            if part.contains('k') {
                value *= 1000.0;
            }
            result.push(value as u32);
        }
    }

    Box::new(result)
}

pub fn take_value_ownership(a: Box<Vec<u32>>) -> Vec<u32> {
    *a
}