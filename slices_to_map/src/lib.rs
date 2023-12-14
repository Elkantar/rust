use std::collections::HashMap;
use std::hash::Hash;

pub fn slices_to_map<'a, T, U>(keys: &'a [T], values: &'a [U]) -> HashMap<&'a T, &'a U>
where
    T: Eq + Hash,
{
    let mut hashmap: HashMap<&T, &U> = HashMap::new();

    let size = std::cmp::min(keys.len(), values.len());

    for i in 0..size {
        hashmap.insert(&keys[i], &values[i]);
    }

    hashmap
}