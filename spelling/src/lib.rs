pub fn spell(n: u64) -> String {
    if n == 0 {
        return "zero".to_string();
    }

    let ones = vec![
        "", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten",
        "eleven", "twelve", "thirteen", "fourteen", "fifteen", "sixteen", "seventeen", "eighteen", "nineteen",
    ];

    let tens = vec![
        "", "", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety",
    ];

    fn spell_less_than_thousand(num: u64, ones: &Vec<&str>, tens: &Vec<&str>) -> String {
        if num < 20 {
            ones[num as usize].to_string()
        } else if num < 100 {
            let ten = tens[(num / 10) as usize];
            let one = ones[(num % 10) as usize];
            if one.is_empty() {
                ten.to_string()
            } else {
                format!("{}-{}", ten, one)
            }
        } else {
            let hundred = ones[(num / 100) as usize];
            let rest = num % 100;
            if rest == 0 {
                format!("{} hundred", hundred)
            } else {
                format!("{} hundred {}", hundred, spell_less_than_thousand(rest, ones, tens))
            }
        }
    }

    let units = ["", "thousand", "million", "billion"];

    let mut num = n;
    let mut unit_idx = 0;
    let mut result = String::new();

    while num > 0 {
        let part = num % 1000;
        if part > 0 {
            let part_str = spell_less_than_thousand(part, &ones, &tens);
            if !result.is_empty() {
                result = format!("{} {} {}", part_str, units[unit_idx], result);
            } else {
                result = format!("{} {}", part_str, units[unit_idx]);
            }
        }
        num /= 1000;
        unit_idx += 1;
    }

    result.trim().to_string()
}