use std::collections::HashMap;

pub fn mean(list: &Vec<i32>) -> f64 {
    if list.is_empty() {
        return 0.0;
    }
    
    let sum: i32 = list.iter().sum();
    sum as f64 / list.len() as f64
}

pub fn median(list: &Vec<i32>) -> i32 {
    if list.is_empty() {
        return 0;
    }

    let mut sorted_list = list.clone();
    sorted_list.sort();

    let middle_index = sorted_list.len() / 2;
    
    if sorted_list.len() % 2 == 1 {
        sorted_list[middle_index]
    } else {
        let middle1 = sorted_list[middle_index - 1];
        let middle2 = sorted_list[middle_index];
        (middle1 + middle2) / 2
    }
}

pub fn mode(list: &Vec<i32>) -> i32 {
    if list.is_empty() {
        return 0;
    }

    let mut counts = HashMap::new();

    for &value in list {
        *counts.entry(value).or_insert(0) += 1;
    }

    let mut mode_value = list[0];
    let mut max_count = 0;

    for (&value, &count) in &counts {
        if count > max_count {
            mode_value = value;
            max_count = count;
        }
    }

    mode_value
}

