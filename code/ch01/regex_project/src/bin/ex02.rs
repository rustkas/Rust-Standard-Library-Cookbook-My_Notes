use regex::Regex;

fn main() {
    // Let's use capture groups now
    let date_regex = Regex::new(r"(\d{2}).(\d{2}).(\d{4})").expect("Failed to create regex");
    let text_with_dates = "Alan Turing was born on 23.06.1912 and died on 07.06.1954. \
A movie about his life called 'The Imitation Game' came out on 14.11.2017";
    // Iterate over the matches
    for cap in date_regex.captures_iter(text_with_dates) {
        println!("Found date {}", &cap[0]);
        println!("Year: {} Month: {} Day: {}", &cap[3], &cap[2], &cap[1]);
    }
    // Replace the date format
    println!("Original text:\t\t{}", text_with_dates);
    let text_with_indian_dates = date_regex.replace_all(text_with_dates, "$1-$2-$3");
    println!("In indian format:\t{}", text_with_indian_dates);
}
