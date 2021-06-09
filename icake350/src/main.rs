use rand::distributions::{Distribution, Uniform};

fn get_random(floor: usize, high: usize) -> usize {
    let between = Uniform::new_inclusive(floor, high);
    let mut rng = rand::thread_rng();
    return between.sample(&mut rng);
}

fn shuffle<A>(inp: &mut Vec<A>) {
    let max = inp.len() - 1;
    for i in 0..inp.len() {
        if i != max {
            let swpix = get_random(i, max);
            if swpix != i {
                inp.swap(i, swpix)
            }
        }
    }
}

fn main() {
    let mut inp: Vec<u16> = (0..10).collect();
    shuffle(&mut inp);
    println!("{:?}", inp);
}
