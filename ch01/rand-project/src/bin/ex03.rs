use rand::Rng;

fn main() {
    // We can use a reusable generator
    let mut rng = rand::thread_rng();
    // This is equivalent to rand::random()
    if rng.gen() {
        println!("This message has a 50-50 chance of being printed");
    }
    // A generator enables us to use ranges
    // random_num3 will be between 0 and 9
    let random_num3 = rng.gen_range(0..10);
    println!("random_num3: {}", random_num3);
}
