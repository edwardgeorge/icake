use rand::Rng;

fn rand25<F>(mut rand5: F) -> i32
where
    F: FnMut() -> i32,
{
    let x = rand5() - 1;
    let y = rand5();
    (x * 5) + y
}

fn rand7<F>(mut rand5: F) -> i32
where
    F: FnMut() -> i32,
{
    loop {
        let r = rand25(|| rand5());
        if (1..=21).contains(&r) {
            return (r % 7) + 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::rand7;
    use std::panic::catch_unwind;

    #[test]
    fn test_distribution() {
        let mut res = [0; 7];
        let mut panic_count = 0;
        (1..=5)
            .flat_map(|i| (1..=5).map(move |j| (i, j)))
            .for_each(|(a, b)| {
                let r = catch_unwind(|| {
                    let rand_results = [a, b];
                    let mut i = rand_results.iter();
                    rand7(|| *i.next().unwrap())
                });
                match r {
                    Ok(v) => {
                        res[v as usize - 1] = res[v as usize - 1] + 1;
                    }
                    Err(_) => {
                        panic_count += 1;
                    }
                }
            });
        assert_eq!(panic_count, 4);
        assert_eq!(res, [3, 3, 3, 3, 3, 3, 3]);
    }
}

fn main() {
    println!(
        "random number between 1 and 7: {}",
        rand7(|| rand::thread_rng().gen_range(1..=5))
    );
}
