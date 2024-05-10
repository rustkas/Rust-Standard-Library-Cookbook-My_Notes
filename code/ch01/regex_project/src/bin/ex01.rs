use regex::Regex;


fn main() {
// Beginning a string with 'r' makes it a raw string,
    // in which you don't need to escape any symbols
    let date_regex = Regex::new(r"^\d{2}.\d{2}.\d{4}$").expect("Failed to create regex");
    let date = "15.10.2017";
    let is_date = date_regex.is_match(date);
    println!("Is '{}' a date? {}", date, is_date);
}
