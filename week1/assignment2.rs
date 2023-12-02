use std::fs::File;
use std::io::{BufReader,BufRead};
use std::error::Error;
use std::path::Path;
use std::collections::HashMap;

fn main() {
    // Create a path to the desired file
    let path = Path::new("input.txt");
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let file = match File::open(&path) {
        // The `description` method of `io::Error` returns a string that describes the error
        Err(why) => panic!("couldn't open {}: {}", display, <dyn Error>::to_string(&why)),
        Ok(file) => file,
    };

    let mut digits = Vec::new();
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line_string = line.unwrap();
        let digit: i32 = get_digit(line_string.clone());
        digits.push(digit);
        println!("Line {} has digit: {}", line_string, digit);
    }

    let sum: i32 = digits.iter().sum();
    println!("the total sum is: {}", sum);

}

fn word_to_digits(string_org: String)-> String{
    let numbers = HashMap::from([
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9")
    ]);
    let mut string: String = string_org;
    for (key, value) in numbers {
        string = string.replace(key, value);
    }
    string
}

fn get_digit(string_org: String) -> i32{
    let string = word_to_digits(string_org);
    let digits_only: String = string.chars().filter(|c| c.is_digit(10)).collect();
    let first_digit = digits_only.chars().nth(0).unwrap();
    let last_digit =  digits_only.chars().last().unwrap();
    let result = format!("{}{}", first_digit, last_digit);
    result.parse::<i32>().unwrap()
}
