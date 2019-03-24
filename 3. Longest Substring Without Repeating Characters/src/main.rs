use std::collections::HashMap;
use std::ops::Range;

pub fn length_of_longest_substring(s: String) -> i32 {
	let mut map: HashMap<char, usize> = HashMap::new();
	let mut substring_range: Range<usize> = Range {start: 0, end: 0};
	let mut max_length = 0usize;

	for (idx, chr) in s.chars().enumerate() {
		if let Some(&repeated_idx) = map.get(&chr) {
			if let Some(removed_str) = s.get(substring_range.start..repeated_idx) {
				for removed_char in removed_str.chars() {
					map.remove(&removed_char);
				}
				substring_range.start = repeated_idx + 1;
			} else {
				unreachable!();
			}
		}
		map.insert(chr, idx);
		substring_range.end = idx;
		max_length = max_length.max(map.len());
	}
	return max_length as i32;
}

fn main() {
    assert_eq!(length_of_longest_substring("dvdf".to_string()), 3);
    assert_eq!(length_of_longest_substring("aab".to_string()), 2);
    assert_eq!(length_of_longest_substring("abcabcbb".to_string()), 3);
    assert_eq!(length_of_longest_substring("bbbbb".to_string()), 1);
    assert_eq!(length_of_longest_substring("pwwkew".to_string()), 3);
}
