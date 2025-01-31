impl Solution {

pub fn is_valid(word: String) -> bool {

word.len() >= 3 &&

word.chars().any(|x|{"aeiou".contains(x.to_ascii_lowercase())}) &&

word.chars().any(|x|{x.is_ascii_alphabetic() && !"aeiou".contains(x.to_ascii_lowercase())}) &&

word.chars().all(|x|{x.is_ascii_alphanumeric()})

}

}