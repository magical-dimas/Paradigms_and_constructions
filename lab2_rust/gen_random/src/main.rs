use rand::Rng;

fn gen_random(count: usize, min: i32, max: i32) -> impl Iterator<Item = i32> {
    let mut rng = rand::thread_rng();
    (0..count).map(move |_x| rng.gen_range(min..=max))
}

fn main() {
    let count: usize = 5;
    let min: i32 = -2;
    let max: i32 = 5;

    let numbers: Vec<i32> = gen_random(count, min, max).collect();
    for num in numbers {
        println!("{}", num);
    }
}