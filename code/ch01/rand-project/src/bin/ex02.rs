fn main() {
  // Every primitive data type can be randomized
  let random_char = rand::random::<char>();
  // Altough random_char will probably not be
  // representable on most operating systems
  println!("random_char: {}", random_char);

}