pub fn factorial(num: u64) -> u64 {
    let mut i = 1;
    let mut result = 1;

    if num == 0 {
        return 1
    }
    if num == 1 {
        return 1
    }
    loop {
        result *= i;
            if i > num-1 {
               return result;                
            }
        i += 1;
    }
}
