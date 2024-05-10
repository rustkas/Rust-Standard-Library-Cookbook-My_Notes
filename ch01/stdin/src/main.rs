use stdin::{print_single_line, read_line_buffer, read_line_iter, read_number};

fn main() {
    print_single_line("Please enter your forename: ");
    let forename = read_line_iter();

    print_single_line("Please enter your surname: ");
    let surname = read_line_buffer();

    print_single_line("Please enter your age: ");
    let age = read_number();

    println!(
        "Hello, {} year old human named {} {}!",
        age, forename, surname
    );
}