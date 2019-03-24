use std::collections::HashSet;

struct LongestSubstringCalc {
	set: HashSet<char>,
	substring: String,
}

impl LongestSubstringCalc {
	fn new() -> LongestSubstringCalc {
		LongestSubstringCalc{set: HashSet::new(), substring: String::new()}
	}

	fn push(&mut self, chr: char) -> i32 {
		if self.set.contains(&chr) {
			if let Some(index) = self.substring.find(chr) {
				if let Some(removed_str) = self.substring.get(0..=index) {
					for removed_char in removed_str.chars() {
						assert!(self.set.remove(&removed_char));
					}
					self.substring.replace_range(0..=index, "");
				} else {
					unreachable!();
				}
			} else {
				unreachable!();
			}
		}
		self.set.insert(chr);
		self.substring.push(chr);
		self.set.len() as i32
	}
}

pub fn length_of_longest_substring(s: String) -> i32 {
	let mut calc = LongestSubstringCalc::new();
	let mut max_length = 0i32;
	for chr in s.chars() {
		let new_length = calc.push(chr);
		max_length = max_length.max(new_length);
	}
	return max_length;
}

fn main() {
    assert_eq!(length_of_longest_substring("dvdf".to_string()), 3);
    assert_eq!(length_of_longest_substring("aab".to_string()), 2);
    assert_eq!(length_of_longest_substring("abcabcbb".to_string()), 3);
    assert_eq!(length_of_longest_substring("bbbbb".to_string()), 1);
    assert_eq!(length_of_longest_substring("pwwkew".to_string()), 3);
}
