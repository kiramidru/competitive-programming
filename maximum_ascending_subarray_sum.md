```
impl Solution {
	pub fn max_ascending_sum(nums: Vec<i32>) -> i32 {
		let mut start: usize = 0;
		let mut end: usize = 1;
		let mut max: i32 = nums[0];

		while end < nums.len() {
			if nums[end - 1] >= nums[end] {
				start = end;
			}
			end += 1;
			max = i32::max(nums[start..end].iter().sum::<i32>(), max);
		}
		return max;
	}
}
```
