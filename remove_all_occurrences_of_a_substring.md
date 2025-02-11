```
impl Solution {
	pub fn remove_occurrences(s: String, part: String) -> String {
		let mut stack: Vec<char> = Vec::new();
		let mut s_vec: Vec<char> = s.chars().collect();
		let mut part_vec: Vec<char> = part.chars().collect();
		let mut pos = 0;
		let mut found = true;

		while found || pos < s.len() {
			found = false;
			if pos < s.len() {
				stack.push(s_vec[pos]);
				pos += 1;
			}

			if stack.len() >= part.len() {
				if stack[(stack.len() - part.len())..] == part_vec {
					found = true;
					stack = stack[..stack.len() - part.len()].to_vec();
				}
			}
		}
		return stack.iter().collect::<String>();
	}
}
```