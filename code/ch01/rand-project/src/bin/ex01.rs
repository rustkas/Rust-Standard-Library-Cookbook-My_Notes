fn main() {
    // random_num1 will be any integer between
    // std::i32::MIN and std::i32::MAX
    let random_num1 = rand::random::<i32>();
    println!("random_num1: {}", random_num1);
    let random_num2: i32 = rand::random();
    println!("random_num2: {}", random_num2);
    // The initialization of random_num1 and random_num2
    // is equivalent.
}
