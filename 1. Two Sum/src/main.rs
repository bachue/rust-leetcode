use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut nums_map: HashMap<i32, i32> = HashMap::with_capacity(nums.len());
    for i in 0..nums.len() {
        if let Some(&get_index) = nums_map.get(&(target - nums[i])) {
            if get_index != i as i32 {
                return vec!(get_index, i as i32);
            }
        } else {
            nums_map.insert(nums[i], i as i32);
        }
    }
    nums
}

fn main() {
    assert_eq!(two_sum(vec!(2, 7, 11, 15), 9), vec!(0, 1));
}
