use constructor::name_length::NameLength;

fn main() {
  // We don't need to care about
  // the internal structure of NameLength
  // Instead, we can just call it's constructor
  let name_length = NameLength::new("John");

  // Prints "The name 'John' is '4' characters long"
  name_length.print();
}
