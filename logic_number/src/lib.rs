pub fn number_logic(num: u32) -> bool {
    let num_str = num.to_string();
    let num_digits = num_str.len();

    let sum_of_powers: u32 = num_str
        .chars()
        .map(|c| c.to_digit(10).unwrap().pow(num_digits as u32))
        .sum();

    return sum_of_powers == num;
}
