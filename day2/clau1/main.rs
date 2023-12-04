use std::fs::File;
use std::io::{BufReader,BufRead};
use std::error::Error;
use std::path::Path;
use std::collections::HashMap;
use itertools::Itertools;

struct Game {
    id: i32,
    results: Vec<MarbleCollection>,
}

struct MarbleCollection {
    red_count: i32,
    green_count: i32,
    blue_count: i32,
}

fn main() {
    let marble_bag = MarbleCollection{red_count: 12, blue_count: 14, green_count: 13};

    // Create a path to the desired file
    let path = Path::new("../input.txt");
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let file = match File::open(&path) {
        // The `description` method of `io::Error` returns a string that describes the error
        Err(why) => panic!("couldn't open {}: {}", display, <dyn Error>::to_string(&why)),
        Ok(file) => file,
    };

    let mut game_ids = Vec::new();
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line_string = line.unwrap();
        let game: Game =parse_line_as_game(line_string.clone());

        let mut result_possible_from_bag = true;
        for result in game.results {
            if result.red_count > marble_bag.red_count || result.blue_count > marble_bag.blue_count || result.green_count > marble_bag.green_count {
                result_possible_from_bag = false;
                break;
            }
        }

        if result_possible_from_bag {
            game_ids.push(game.id);
        }
    }

    let sum: i32 = game_ids.iter().sum();
    println!("the total sum is: {}", sum);

}

fn parse_line_as_game(string: String)->Game {
    let mut grabs = Vec::new();
    let (game_text, game_results) = split_string_to_tuple(string, ':');
    let (_, game_id_str) = split_string_to_tuple(game_text, ' ');
    let game_id = game_id_str.parse::<i32>().unwrap();

    let results_collection = split_string_to_vector(game_results, ';');

    for result in results_collection {
        let marble_data = split_string_to_vector(result.clone(), ',');
        let mut red: i32 = 0;
        let mut green: i32 = 0;
        let mut blue: i32 = 0;
        for marble_info in marble_data {
            let (number_string, color) = split_string_to_tuple(marble_info, ' ');
            let number = number_string.parse::<i32>().unwrap();
                match color.as_str(){
                    "red"=>red = red + number,
                    "green"=>green = green + number,
                    "blue"=>blue = blue + number,
                    _=>println!("Unknown color found. Skipping it!"),
                }
        }
        let grab = MarbleCollection{red_count: red, blue_count: blue, green_count: green};
        grabs.push(grab)
    }

    let game = Game{id:game_id, results:grabs};
    println!("Parsed game: {}", game_id);
    game
}

fn split_string_to_vector(string: String, splitter: char) -> Vec<String> {
    let mut parts_vector = Vec::new();
    let parts = string.split(splitter);
    for part in parts {
        parts_vector.push(part.trim().to_string());
    }
    parts_vector
}

fn split_string_to_tuple(string: String, splitter: char) -> (String, String) {
    let parts_collection = split_string_to_vector(string, splitter);
    let (part1, part2) = parts_collection.iter().collect_tuple().unwrap();
    (part1.to_string(), part2.to_string())
}
