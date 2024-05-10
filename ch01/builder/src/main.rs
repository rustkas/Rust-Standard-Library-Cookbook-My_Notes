use builder::burger_builder::BurgerBuilder;



fn main() {
  // We can easily create different configurations
  let normal_burger = BurgerBuilder::new().build();
  let cheese_burger = BurgerBuilder::new().cheese(true).salad(false).build();
  let veggie_bigmac = BurgerBuilder::new().vegetarian(true).patty_count(2).build();

  if let Ok(normal_burger) = normal_burger {
      normal_burger.print();
  }
  if let Ok(cheese_burger) = cheese_burger {
      cheese_burger.print();
  }
  if let Ok(veggie_bigmac) = veggie_bigmac {
      veggie_bigmac.print();
  }

  // Our builder can perform a check for
  // invalid configurations
  let invalid_burger = BurgerBuilder::new().vegetarian(true).bacon(true).build();
  if let Err(error) = invalid_burger {
      println!("Failed to print burger: {}", error);
  }

  // If we omit the last step, we can reuse our builder
  let cheese_burger_builder = BurgerBuilder::new().cheese(true);
  for i in 1..10 {
      let cheese_burger = cheese_burger_builder.build();
      if let Ok(cheese_burger) = cheese_burger {
          println!("cheese burger number {} is ready!", i);
          cheese_burger.print();
      }
  }
}