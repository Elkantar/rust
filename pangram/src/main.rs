use pangram::*;

fn main() {
    println!(
        "{}",
        is_pangram("the quick brown fox jumps over the lazy dog!")
    );
    println!("{}", is_pangram("this is not a pangram!"));
    println!("{}",is_pangram("\\\"Five quacking Zephyrs jolt my wax bed.\\\"\""))
}
