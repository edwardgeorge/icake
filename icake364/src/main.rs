use rand::Rng;

fn rand25<F>(rand5: F) -> i32
where
    F: Fn() -> i32,
{
    let x = rand5() - 1;
    let y = rand5();
    (x * 5) + y
}

fn rand7<F>(rand5: F) -> i32
where
    F: Fn() -> i32,
{
    loop {
        let r = rand25(&rand5);
        if (1..=21).contains(&r) {
            return (r % 7) + 1;
        }
    }
}

fn main() {
    println!(
        "random number between 1 and 7: {}",
        rand7(|| rand::thread_rng().gen_range(1..=5))
    );
}
