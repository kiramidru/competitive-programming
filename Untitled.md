```
use std::collections::HashMap;

impl Solution {
	pub fn maximum_sum(nums: Vec<i32>) -> i32 {
		let mut vec = vec![0; 82];
		let mut max = -1;

		for i in nums.iter() {
			let mut temp = i.clone();
			let mut sum = 0;

			while temp != 0 {
				sum += temp % 10;
				temp /= 10;
			}

			if vec[sum as usize] != 0 {
				max = i32::max(max, vec[sum as usize] + i);
			}
			
			vec[sum as usize] = i32::max(vec[sum as usize], *i);
		}
		return max;
	}
}
```
