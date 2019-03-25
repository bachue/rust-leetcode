struct MergeSorter {
	nums1: Vec<i32>,
	nums2: Vec<i32>,
	idx1: usize,
	idx2: usize,
}

impl MergeSorter {
	fn new(nums1: Vec<i32>, nums2: Vec<i32>) -> MergeSorter {
		MergeSorter{nums1: nums1, nums2: nums2, idx1: 0, idx2: 0}
	}

	fn get(&mut self) -> i32 {
		let val1 = self.nums1.get(self.idx1);
		let val2 = self.nums2.get(self.idx2);
		let result = match (val1, val2) {
			(Some(&v1), Some(&v2)) => {
				if v1 < v2 {
					self.idx1 += 1;
					v1
				} else {
					self.idx2 += 1;
					v2
				}
			},
			(Some(&v1), None) => {
				self.idx1 += 1;
				v1
			},
			(None, Some(&v2)) => {
				self.idx2 += 1;
				v2
			},
			(None, None) => {
				unreachable!()
			}
		};
		result
	}
}

pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
	let length = nums1.len() + nums2.len();
	let start = match (length >> 1).checked_sub(1) {
		Some(v) => v as i32,
		None => -1 as i32,
	};
	let mut sorter = MergeSorter::new(nums1, nums2);
	if length & 0x01 == 0 { // even
		for _ in 0..start {
			sorter.get();
		}
		(sorter.get() + sorter.get()) as f64 / 2.0_f64
	} else { // odd
		for _ in 0..=start {
			sorter.get();
		}
		sorter.get() as f64
	}
}

fn main() {
    assert_eq!(find_median_sorted_arrays(vec!(1, 3), vec!(2)), 2.0_f64);
    assert_eq!(find_median_sorted_arrays(vec!(1), vec!()), 1.0_f64);
    assert_eq!(find_median_sorted_arrays(vec!(), vec!(2)), 2.0_f64);
    assert_eq!(find_median_sorted_arrays(vec!(), vec!(2, 3)), 2.5_f64);
    assert_eq!(find_median_sorted_arrays(vec!(1, 2), vec!(3, 4)), 2.5_f64);
    assert_eq!(find_median_sorted_arrays(vec!(1, 2, 3, 4, 5), vec!(6, 7, 8, 9, 10)), 5.5_f64);
    assert_eq!(find_median_sorted_arrays(vec!(1, 2, 3, 4, 5), vec!(6, 7, 8, 9, 10, 11)), 6.0_f64);
}
