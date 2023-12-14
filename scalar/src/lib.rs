pub fn sum(a: u8, b: u8) -> u8 {
    // Ensure that both values are within the range [0, 255]
    let a = a.min(255);
    let b = b.min(255);
    a + b
}

pub fn diff(a: i16, b: i16) -> i16 {
    // Ensure that both values are within the range [-32768, 32767]
    let a = a.min(32767).max(-32768);
    let b = b.min(32767).max(-32768);
    a - b
}

pub fn pro(a: i8, b: i8) -> i8 {
    // Ensure that both values are within the range [-128, 127]
    let a = a.min(127).max(-128);
    let b = b.min(127).max(-128);
    a * b
}

pub fn quo(a: f32, b: f32) -> f32 {
    // No need to check for range in i32
    a / b
}

pub fn rem(a: f32, b: f32) -> f32 {
    // No need to check for range in i32
    a % b
}