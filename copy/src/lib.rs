use std::f64;

pub fn nbr_function(c: i32) -> (i32, f64, f64) {
    let original_value = c;
    let exp_value = (c as f64).exp();
    let ln_abs_value = (c.abs() as f64).ln();
    
    (original_value, exp_value, ln_abs_value)
}

pub fn str_function(a: String) -> (String, String) {
    let exp_values: Vec<String> = a
        .split_whitespace()
        .map(|s| {
            let value = s.parse::<f64>().unwrap_or(0.0);
            f64::exp(value).to_string()
        })
        .collect();
    (a, exp_values.join(" "))
}

pub fn vec_function(b: Vec<i32>) -> (Vec<i32>, Vec<f64>) {
    let original_value = b.clone();
    let ln_abs_values: Vec<f64> = b.iter().map(|&x| (x.abs() as f64).ln()).collect();
    (original_value, ln_abs_values)
}
