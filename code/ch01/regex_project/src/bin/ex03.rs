use regex::Regex;


fn main() {
  let text_with_dates = "Alan Turing was born on 23.06.1912 and died on 07.06.1954. \
  A movie about his life called 'The Imitation Game' came out on 14.11.2017";
 // Replacing groups is easier when we name them
    // ?P<somename> gives a capture group a name
    let date_regex = Regex::new(r"(?P<day>\d{2}).(?P<month>\d{2}).(?P<year>\d{4})")
        .expect("Failed to create regex");
    let text_with_american_dates = date_regex.replace_all(text_with_dates, "$month/$day/$year");
    println!("In american format:\t{}", text_with_american_dates);
    let rust_regex = Regex::new(r"(?i)rust").expect("Failed to create regex");
    println!("Do we match RuSt? {}", rust_regex.is_match("RuSt"));
    use regex::RegexBuilder;
    let rust_regex = RegexBuilder::new(r"rust")
        .case_insensitive(true)
        .build()
        .expect("Failed to create regex");
    println!("Do we still match RuSt? {}", rust_regex.is_match("RuSt"));
}
