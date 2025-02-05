```
impl Solution {
	pub fn are_almost_equal(s1: String, s2: String) -> bool {
		if s1 == s2 {
			return true;
		}

		let mut s1_mismatch: Vec<char> = Vec::new();
		let mut s2_mismatch: Vec<char> = Vec::new();

		s1.chars().zip(s2.chars()).filter(|(a,b)| a != b).for_each(|(a, b)| {

			s1_mismatch.push(a);

			s2_mismatch.push(b);

		});

		if s1_mismatch.len() != 2 || s2_mismatch.len() != 2 {
			return false;
		}

		s1_mismatch[0] == s2_mismatch[1] && s1_mismatch[1] == s2_mismatch[0]
	}
}
```

