use std::collections::HashMap;

pub fn length_of_longest_substring(s: String) -> i32 {
    let mut hash = HashMap::with_capacity(s.len());
    let mut max = 0;
    let mut start = 0;
    let mut end = 0;

    for (i, item) in s.chars().enumerate() {
        if let Some(j) = hash.get(&item) {
            if *j >= start {
	            max = max.max(end - start);
                start = *j + 1;
            }
        }
        end += 1;
        hash.insert(item, i);
    }
    max.max(end - start) as i32
}

fn main() {
    assert_eq!(length_of_longest_substring("dvdf".to_string()), 3);
    assert_eq!(length_of_longest_substring("aab".to_string()), 2);
    assert_eq!(length_of_longest_substring("abcabcbb".to_string()), 3);
    assert_eq!(length_of_longest_substring("bbbbb".to_string()), 1);
    assert_eq!(length_of_longest_substring("pwwkew".to_string()), 3);
    assert_eq!(length_of_longest_substring("abcdecfgbijklm".to_string()), 11);
}
