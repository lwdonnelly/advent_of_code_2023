use std::fs;
use std::collections::HashMap;

fn main() {
    day1();
}

fn day1() {
    let input = fs::read_to_string("day1_input.txt").expect("Reading input failed");
    let digit_map:HashMap<&str, u32> = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9)
    ]);
    let mut total = 0;
    for word in input.split("\n") {
        let mut first:(Option<usize>, u32) = (None, 0);
        let mut last:(Option<usize>, u32) = (None, 0);

        for (key, value) in &digit_map {
            let cur_first_index = word.find(key);
            let cur_last_index = word.rfind(key);

            if cur_first_index.is_some() {
                if cur_first_index.unwrap().le(&first.0.unwrap_or(word.len())) {
                    first.0 = cur_first_index;
                    first.1 = *value;
                }
                if cur_last_index.unwrap().ge(&last.0.unwrap_or(0)) {
                   last.0 = cur_last_index;
                   last.1 = *value;
                }
            }
        }

       total += (first.1 * 10) + last.1;
    }
    println!("{}", total);
}
