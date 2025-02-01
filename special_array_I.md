```
impl Solution {
	pub fn is_array_special(nums: Vec<i32>) -> bool {
		for i in nums.windows(2) {
			if i[0] % 2 == i[1] % 2 {
			return false;
		}
	}
	return true;
	}
}
```
