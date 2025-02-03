```
impl Solution {
    pub fn longest_monotonic_subarray(nums: Vec<i32>) -> i32 {
        let mut subarrays: Vec<usize> = Vec::new();
        let mut start = 0;
        let mut end = 1;

        while end < nums.len() {
            subarrays.push(end - start);
            if nums[end - 1] <= nums[end] {
                start = end;
            }
            end += 1;
        }
        subarrays.push(end - start);

        start = 0;
        end = 1;

        while end < nums.len() {
            subarrays.push(end - start);
            if nums[end - 1] >= nums[end] {
                start = end;
            }
            end += 1;
        }
        subarrays.push(end - start);

        subarrays.into_iter().max().unwrap() as i32
    }
}
```

