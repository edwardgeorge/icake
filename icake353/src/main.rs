#[inline]
fn inverse_for(c: char) -> Option<char> {
    match c {
        ']' => Some('['),
        '}' => Some('{'),
        ')' => Some('('),
        _ => None,
    }
}

fn match_brackets(input: &str) -> bool {
    let mut stack: Vec<char> = Vec::new();
    for i in input.chars() {
        if matches!(i, '[' | '{' | '(') {
            stack.push(i);
        } else if let Some(expected) = inverse_for(i) {
            if stack.is_empty() || stack.pop().unwrap() != expected {
                return false;
            }
        }
    }
    true
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn example1() {
        assert!(match_brackets("{ [ ] ( ) }"))
    }
    #[test]
    fn example2() {
        assert!(!match_brackets("{ [ ( ] ) }"))
    }
    #[test]
    fn example3() {
        assert!(!match_brackets("{ [ }"))
    }
}

fn main() {
    let example = "{ [ ] ( ) }";
    println!("is {:?} valid? {:?}", example, match_brackets(example));
}
