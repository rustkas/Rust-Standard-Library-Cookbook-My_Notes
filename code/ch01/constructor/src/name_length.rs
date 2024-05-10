pub struct NameLength {
  name: String,
  length: usize,
}

impl NameLength {
  // The user doesn't need to setup length
  // We do it for him!
  pub fn new(name: &str) -> Self {
      NameLength {
          length: name.len(),
          name: name.to_string(),
      }
  }

  pub fn print(&self) {
      println!(
          "The name '{}' is '{}' characters long",
          self.name,
          self.length
      );
  }
}
