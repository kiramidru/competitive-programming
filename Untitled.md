
```
impl Solution {

pub fn clear_digits(s: String) -> String {

let mut stack: Vec<char> = Vec::with_capacity(100);

  

for i in s.chars() {

if i.is_ascii_digit() {

match stack.pop() {

_ => {}

}

} else {

stack.push(i)

}

}

stack.iter().collect()

}

}
```
`