```
use std::collections::HashSet;

impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        let mut k = k as usize;
        let mut hash: HashSet<i32> = HashSet::new();

        for (i, &v) in nums.iter().enumerate() {
            if !hash.insert(v) {
                return true;
            }
            if i + 1 > k {
                hash.remove(&nums[i - k]);
            }
        }
        return false;
    }
}
```

