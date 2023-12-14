pub fn fibonacci(n: u32) -> u32 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    }

    let mut prev = 0;
    let mut current = 1;

    for _ in 2..=n {
        let next = prev + current;
        prev = current;
        current = next;
    }

    return current;
}

