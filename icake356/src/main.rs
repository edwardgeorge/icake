const EXAMPLE1: &str = "cake pound steal";

fn reverse_words(input: &mut [char]) {
    input.reverse();
    let len = input.len();
    let mut i = 0;
    for j in 1..len {
        if input[j] == ' ' && j > i {
            input[i..j].reverse();
            i = j + 1;
        }
    }
    if i < len {
        input[i..len].reverse();
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_example() {
        let mut ch: Vec<_> = EXAMPLE1.to_string().chars().collect();
        reverse_words(&mut ch);
        assert_eq!(
            ch.iter().collect::<String>(),
            "steal pound cake".to_string()
        );
    }
}

fn main() {
    let mut ch: Vec<_> = EXAMPLE1.to_string().chars().collect();
    reverse_words(&mut ch);
    println!("{}", ch.iter().collect::<String>());
}
