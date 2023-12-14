pub fn stars(n: u32) -> String {
    let mut star = String::new();
    let v = "*";

    for i in n..=n {
        star.push_str(&v.repeat(2u32.pow(i) as usize));
    }

    star
}
