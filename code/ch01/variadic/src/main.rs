use variadic::multiply;


fn main() {
  let val = multiply!(2, 4, 8);
  println!("2*4*8 = {}", val)
}
