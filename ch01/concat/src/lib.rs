use lazy_static::lazy_static;

// Импортируем структуру Mutex для использования мьютекса
use std::sync::Mutex;

use std::ops::Deref;

// Импортируем макрос lazy_static
lazy_static! {
    // Объявляем статическую переменную HELLO типа Mutex<String>
    static ref HELLO: Mutex<String> = Mutex::new(String::from("Hello, world!"));
}
#[allow(dead_code)]
const WORLD: &str = "world!";

#[test]
fn by_moving() {


  let hello = HELLO.lock().unwrap();
  let hello = hello.deref().clone();
  
  let world = WORLD;

  // Moving hello into a new variable
  let hello_world = hello + world;
  assert_eq!("Hello, world!", hello_world);
  
}

#[test]
fn by_cloning() {
  let hello = HELLO.lock().unwrap();
  let hello = hello.deref().clone();
  let world = WORLD;

  // Creating a copy of hello and moving it into a new variable
  let hello_world = hello.clone() + world;
  assert_eq!("Hello, world!", hello_world);
}

#[test]
fn by_mutating() {
  let mut hello_world = "hello ".to_string();
  let world = "world!";

  // hello gets modified in place
  hello_world.push_str(world);
  assert_eq!("Hello, world!", hello_world);
}

#[test]
fn by_using_macros() {
  let  hello = "Hello ".to_string();
  let world = "world!";


  // hello gets modified in place
  let hello_world = format!("{hello}{world}");
  assert_eq!("Hello, world!", hello_world);
}
