pub fn get_diamond(c: char) -> Vec<String> {
    let n = (c as u8 - b'A' + 1) as usize;
    let mut diamond: Vec<String> = vec![];
    
    for i in 0..n {
        let spaces = n - i - 1;
        let letters = (b'A' + i as u8) as char;
        let mut row = " ".repeat(spaces);
        
        if i == 0 {
            row.push(letters);
        } else {
            row.push(letters);
            row.push_str(&" ".repeat(i * 2 - 1));
            row.push(letters);
        }
        // Add trailing spaces to make the line a square
        row.push_str(&" ".repeat(spaces));
        
        diamond.push(row);
    }
    
    let mut bottom_half = diamond.clone();
    bottom_half.pop(); // Remove the last row (already in the top half)
    bottom_half.reverse();
    diamond.extend(bottom_half);

    diamond
}
