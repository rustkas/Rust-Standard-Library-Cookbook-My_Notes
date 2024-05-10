pub struct Burger {
  pub patty_count: i32,
  pub vegetarian: bool,
  pub cheese: bool,
  pub bacon: bool,
  pub salad: bool,
}
impl Burger {
  // This method is just here for illustrative purposes
  pub fn print(&self) {
      let pretty_patties = if self.patty_count == 1 {
          "patty"
      } else {
          "patties"
      };
      let pretty_bool = |val| if val { "" } else { "no " };
      let pretty_vegetarian = if self.vegetarian { "vegetarian " } else { "" };
      println!(
          "This is a {}burger with {} {}, {}cheese, {}bacon and {}salad",
          pretty_vegetarian,
          self.patty_count,
          pretty_patties,
          pretty_bool(self.cheese),
          pretty_bool(self.bacon),
          pretty_bool(self.salad)
      )
  }
}