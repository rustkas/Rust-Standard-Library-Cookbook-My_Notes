use crate::crust_type::CrustType;

#[derive(Default)]
pub struct PizzaConfig {
    pub wants_cheese: bool,
    pub number_of_olives: i32,
    pub special_message: String,
    pub crust_type: CrustType,
}