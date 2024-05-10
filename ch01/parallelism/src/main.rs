use std::thread;

use parallelism::parallel_sum;

fn main() {
    // Spawning a thread lets it execute a lambda
    let child = thread::spawn(|| println!("Hello from a new thread!"));
    println!("Hello from the main thread!");
    // Joining a child thread with the main thread means
    // that the main thread waits until the child has
    // finished it's work
    child.join().expect("Failed to join the child thread");

    let sum = parallel_sum(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    println!("The sum of the numbers 1 to 10 is {}", sum);
}
