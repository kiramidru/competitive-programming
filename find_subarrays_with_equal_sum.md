```
use std::collections::HashMap;

impl Solution {
	pub fn find_subarrays(nums: Vec<i32>) -> bool {
		let mut hash: HashMap<i32, i32> = HashMap::new();

		for i in 0..nums.len() - 1 {
			*hash.entry(nums[i] + nums[i + 1]).or_insert(0) += 1;
		}
		hash.values().any(|x| x > &1)
	}
}
```
