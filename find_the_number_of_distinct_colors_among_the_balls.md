```
use std::collections::HashMap;

impl Solution {
	pub fn query_results(limit: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
		let mut color_map: HashMap<i32, i32> = HashMap::new();
		let mut ball_map: HashMap<i32, i32> = HashMap::new();
		let mut result: Vec<i32> = Vec::new();

		for i in 0..queries.len() {
			let ball = queries[i][0];
			let color = queries[i][1];
			if ball_map.contains_key(&ball) {
				let prev_color = ball_map[&ball];
				color_map.entry(prev_color).and_modify(|x| *x -= 1);
				if color_map[&prev_color] == 0 {
					color_map.remove(&prev_color);
				}
			}
			ball_map.insert(ball, color);
			if color_map.contains_key(&color) {
				color_map.entry(color).and_modify(|x| *x += 1);
			} else {
				color_map.insert(color, 1);
			}
			result.push(color_map.len() as i32);
		}
		result
	}
}
```