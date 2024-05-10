use rand::Rng;

fn main() {
  let mut rng = rand::thread_rng();
    // random_float will be between 0.0 and 0.999999999999...
    let random_float = rng.gen_range(0.0..1.0);
    println!("random_float: {}", random_float);

}
