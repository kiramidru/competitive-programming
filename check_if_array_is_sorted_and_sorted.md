```
impl Solution {
	pub fn check(nums: Vec<i32>) -> bool {
		let mut changes = 0;
		for i in 0..nums.len() {
			if nums[i] > nums[(i + 1) % nums.len()] {
				changes += 1;
			}
		}
		changes <= 1
	}
}
```
