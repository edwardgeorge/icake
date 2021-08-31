use rand::Rng;

fn rand5<F>(rand7: F) -> u8
where
    F: Fn() -> u8,
{
    loop {
        let r = rand7();
        if (1..=5).contains(&r) {
            return r;
        }
    }
}

fn main() {
    println!(
        "picking a random number between 1 and 5: {}",
        rand5(|| rand::thread_rng().gen_range(1..=7))
    );
}
