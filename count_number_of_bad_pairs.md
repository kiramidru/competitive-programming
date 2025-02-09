```
use std::collections::HashMap;

impl Solution {
	pub fn count_bad_pairs(nums: Vec<i32>) -> i64 {
		let mut hash: HashMap<i32, i32> = HashMap::new();
		let mut count: i64 = 0;
	
		for (i,v) in nums.iter().enumerate() {
			if let Some(&num) = hash.get(&(v - i as i32)) {
				count += num as i64;
			}
			*hash.entry(v - i as i32).or_insert(0) += 1;
		}
		(nums.len() as i64 * (nums.len() as i64 - 1) / 2) - count as i64
	}
}
```
