```
use std::collections::HashSet;


impl Solution {
	pub fn find_error_nums(nums: Vec<i32>) -> Vec<i32> {
		let mut repeat = 0;
		let mut set: HashSet<i32> = HashSet::new();

		for i in nums.iter() {
			if set.contains(i) {
			repeat = *i;
			}
			set.insert(*i);
		}
		for i in 1..=nums.len() {
			if !set.contains(&(i as i32)) {
				return vec![repeat, i as i32];
			}
		}
		return vec![];
	}
}
```
