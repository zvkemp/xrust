use std::collections::HashMap;

pub fn count(nucleotide: char, string: &str) -> uint {
    string.chars().filter(|&c| c == nucleotide).count()
}

pub fn nucleotide_counts(string: &str) -> HashMap<char, uint> {
    let mut map: HashMap<char, uint> = HashMap::new();
    for c in "ATCG".chars() { map.insert(c, 0); }
    for c in string.chars() { map.insert_or_update_with(c, 1, |_, count| *count += 1); }

    map
}
