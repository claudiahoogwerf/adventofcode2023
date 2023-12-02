use std::fs::File;
use std::io::{BufReader,BufRead};
use std::error::Error;
use std::path::Path;
use std::collections::HashMap;
use itertools::Itertools;

fn main() {
    // Create a path to the desired file
    let path = Path::new("../input.txt");
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
    let cleansed_string = clean_string(string_org);
    let digit_string = word_to_digits(cleansed_string);
    let digits_only: String = digit_string.chars().filter(|c| c.is_digit(10)).collect();
    let first_digit = digits_only.chars().nth(0).unwrap();
    let last_digit =  digits_only.chars().last().unwrap();
    let result = format!("{}{}", first_digit, last_digit);
    result.parse::<i32>().unwrap()
}

fn clean_string(string_org: String)-> String{
    /*
    This functions cleans the string so that for example oneight becomes oneeight
    Also automatically strips all other characters that we don't need
    */
    let possible_digits = vec![
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    "1", "2", "3", "4", "5", "6", "7", "8", "9", "10"
    ];

    // Collect all positions with the found substring in a hashmap
    let mut positions: HashMap<i32, String> = HashMap::new();
    for digit in possible_digits {
        let positions_for_digit: Vec<_> = string_org.match_indices(digit).collect();
        for p in positions_for_digit {
            positions.insert(p.0 as i32, p.1.to_string());
        }
    }

    // Sort that hashmap on position and print in a string again
    let mut string: String = "".to_string();
    for key in positions.keys().sorted() {
        string = format!("{}{}", string, positions[key]);
        println!("{:?} has {:?}", key, positions[key]);
    }


    string
}
