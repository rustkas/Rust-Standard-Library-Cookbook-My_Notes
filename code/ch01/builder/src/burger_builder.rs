// mod burger;

use crate::burger::Burger;


pub struct BurgerBuilder {
  patty_count: i32,
  vegetarian: bool,
  cheese: bool,
  bacon: bool,
  salad: bool,
}
impl BurgerBuilder {
  // in the constructor, we can specify
  // the standard values
  pub fn new() -> Self {
      BurgerBuilder {
          patty_count: 1,
          vegetarian: false,
          cheese: false,
          bacon: false,
          salad: true,
      }
  }

  // Now we have to define a method for every
  // configurable value
  pub fn patty_count(mut self, val: i32) -> Self {
      self.patty_count = val;
      self
  }

  pub fn vegetarian(mut self, val: bool) -> Self {
      self.vegetarian = val;
      self
  }
  pub fn cheese(mut self, val: bool) -> Self {
      self.cheese = val;
      self
  }
  pub fn bacon(mut self, val: bool) -> Self {
      self.bacon = val;
      self
  }
  pub fn salad(mut self, val: bool) -> Self {
      self.salad = val;
      self
  }

  // The final method actually constructs our object
  pub fn build(&self) -> Result<Burger, String> {
      let burger = Burger {
          patty_count: self.patty_count,
          vegetarian: self.vegetarian,
          cheese: self.cheese,
          bacon: self.bacon,
          salad: self.salad,
      };
      // Check for invalid configuration
      if burger.vegetarian && burger.bacon {
          Err("Sorry, but we don't server vegetarian bacon yet".to_string())
      } else {
          Ok(burger)
      }
  }
}
