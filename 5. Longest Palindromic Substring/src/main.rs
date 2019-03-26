pub fn longest_palindrome(s: String) -> String {
	if s.len() > 0 {
		let bytes = s.as_bytes();
		let iter1 = (1..bytes.len() - 1).map(|center_idx| {
			longest_palindrome_in_odd_substring_expanded_from_center(bytes, center_idx)
		});
		let iter2 = (0..bytes.len() - 1).map(|center_idx| {
			longest_palindrome_in_even_substring_expanded_from_center(bytes, center_idx)
		});
		let longest_palindrome = iter1
			.chain(iter2)
			.max_by_key(|palindrome| palindrome.len())
			.or_else(|| s.get(0..1).map(|ss| ss.as_bytes()))
			.unwrap();
		unsafe { String::from_utf8_unchecked(longest_palindrome.to_vec()) }
	} else {
		s
	}
}

fn longest_palindrome_in_odd_substring_expanded_from_center(s: &[u8], center_idx: usize) -> &[u8] {
	let mut substring = s.get(center_idx..(center_idx + 1)).unwrap();
	for expand_length in 1..=center_idx {
		let left_bound = center_idx - expand_length;
		let right_bound = center_idx + expand_length;
		let left = s.get(left_bound);
		let right = s.get(right_bound);

		match (left, right) {
			(Some(leftval), Some(rightval)) => {
				if leftval == rightval {
					substring = s.get(left_bound..=right_bound).unwrap();
				} else {
					break;
				}
			}
			_ => {
				break;
			}
		}
	}
	substring
}

fn longest_palindrome_in_even_substring_expanded_from_center(
	s: &[u8],
	center_idx_1: usize,
) -> &[u8] {
	let center_idx_2 = center_idx_1 + 1;
	let mut substring = s.get(center_idx_1..center_idx_2).unwrap();
	for expand_length in 0..=center_idx_1 {
		let left_bound = center_idx_1 - expand_length;
		let right_bound = center_idx_2 + expand_length;
		let left = s.get(left_bound);
		let right = s.get(right_bound);
		match (left, right) {
			(Some(leftval), Some(rightval)) => {
				if leftval == rightval {
					substring = s.get(left_bound..=right_bound).unwrap();
				} else {
					break;
				}
			}
			_ => {
				break;
			}
		}
	}
	substring
}

fn main() {
	assert_eq!(longest_palindrome("babad".to_string()), "aba");
	assert_eq!(longest_palindrome("cbbd".to_string()), "bb");
	assert_eq!(longest_palindrome("bb".to_string()), "bb");
	assert_eq!(longest_palindrome("b".to_string()), "b");
	assert_eq!(longest_palindrome("".to_string()), "");
}
