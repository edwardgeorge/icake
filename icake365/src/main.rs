use std::collections::HashSet;

#[allow(dead_code)]
fn has_palindrome_permutation(word: &str) -> bool {
    let mut seen = HashSet::new();
    for c in word.chars() {
        if !seen.insert(c) {
            seen.remove(&c);
        }
    }
    seen.len() <= 1
}

#[cfg(test)]
mod tests {
    use super::has_palindrome_permutation;

    #[test]
    fn test_examples() {
        assert_eq!(has_palindrome_permutation("civic"), true);
        assert_eq!(has_palindrome_permutation("ivicc"), true);
        assert_eq!(has_palindrome_permutation("civil"), false);
        assert_eq!(has_palindrome_permutation("livci"), false);
    }
}

fn main() {}
