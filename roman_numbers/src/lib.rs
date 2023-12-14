pub use crate::RomanDigit::*;
pub use std::convert::From;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum RomanDigit {
    Nulla,
    I,
    V,
    X,
    L,
    C,
    D,
    M,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RomanNumber(pub Vec<RomanDigit>);

impl From<u32> for RomanDigit {
    fn from(num: u32) -> Self {
        match num {
            0 => RomanDigit::Nulla,
            1 => RomanDigit::I,
            5 => RomanDigit::V,
            10 => RomanDigit::X,
            50 => RomanDigit::L,
            100 => RomanDigit::C,
            500 => RomanDigit::D,
            1000 => RomanDigit::M,
            _ => panic!("Invalid Roman digit: {}", num),
        }
    }
}

impl From<u32> for RomanNumber {
    fn from(mut num: u32) -> Self {
        let mut roman_number = Vec::new();

        // Process thousands (M)
        while num >= 1000 {
            roman_number.push(RomanDigit::M);
            num -= 1000;
        }

        // Process hundreds (C, D, CM)
        match num / 100 {
            0 => {}
            1..=3 => {
                for _ in 1..=num / 100 {
                    roman_number.push(RomanDigit::C);
                }
            }
            4 => {
                roman_number.push(RomanDigit::C);
                roman_number.push(RomanDigit::D);
            }
            5..=8 => {
                roman_number.push(RomanDigit::D);
                for _ in 6..=num / 100 {
                    roman_number.push(RomanDigit::C);
                }
            }
            9 => {
                roman_number.push(RomanDigit::C);
                roman_number.push(RomanDigit::M);
            }
            _ => panic!("Invalid Roman number: {}", num),
        }
        num %= 100;

        // Process tens (X, L, XC)
        match num / 10 {
            0 => {}
            1..=3 => {
                for _ in 1..=num / 10 {
                    roman_number.push(RomanDigit::X);
                }
            }
            4 => {
                roman_number.push(RomanDigit::X);
                roman_number.push(RomanDigit::L);
            }
            5..=8 => {
                roman_number.push(RomanDigit::L);
                for _ in 6..=num / 10 {
                    roman_number.push(RomanDigit::X);
                }
            }
            9 => {
                roman_number.push(RomanDigit::X);
                roman_number.push(RomanDigit::C);
            }
            _ => panic!("Invalid Roman number: {}", num),
        }

        num %= 10;

        // Process ones (I, V, IX)
        match num {
            0 => {
                if roman_number.is_empty() {
                    roman_number.push(RomanDigit::Nulla);
                }
            }
            1..=3 => {
                for _ in 1..=num {
                    roman_number.push(RomanDigit::I);
                }
            }
            4 => {
                roman_number.push(RomanDigit::I);
                roman_number.push(RomanDigit::V);
            }
            5..=8 => {
                roman_number.push(RomanDigit::V);
                for _ in 6..=num {
                    roman_number.push(RomanDigit::I);
                }
            }
            9 => {
                roman_number.push(RomanDigit::I);
                roman_number.push(RomanDigit::X);
            }
            _ => panic!("Invalid Roman number: {}", num),
        }

        RomanNumber(roman_number)
    }
}