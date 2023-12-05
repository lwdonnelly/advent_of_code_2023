use std::fs;
use std::collections::HashMap;

fn main() {
    day1();
    day2();
    day3();
}

fn day1() {
    let input = fs::read_to_string("day1_input.txt").expect("Reading input failed");
    let digit_map: HashMap<&str, u32> = HashMap::from([
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
        let mut first: (Option<usize>, u32) = (None, 0);
        let mut last: (Option<usize>, u32) = (None, 0);

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
    println!("Day 1 - {}", total);
}

fn day2() {
    let input = fs::read_to_string("day2_input.txt").expect("Reading input failed");

    let mut total = 0;
    for line in input.split("\n") {
        let colon_index = line.find(":");
        let colon_index = match colon_index {
            Some(_i) => colon_index.unwrap(),
            None => break
        };

        let line = &line[colon_index + 1..];
        let mut max_map: HashMap<&str, i32> = HashMap::from([
            ("blue", 0),
            ("red", 0),
            ("green", 0)
        ]);
        for set in line.split(";") {
            for round in set.split(",") {
                let round = round.trim();
                let space_index = round.find(" ").expect("could not find space");
                let num_cubes = round[..space_index].parse::<i32>().unwrap();
                let color = &round[space_index + 1..];
                if num_cubes > *max_map.get(color).expect("Color not recognized") {
                    max_map.insert(color, num_cubes);
                }
            }
        }
        total += max_map.get("red").unwrap_or(&1) * max_map.get("green").unwrap_or(&1) * max_map.get("blue").unwrap_or(&1);
    }
    println!("Day 2 - {}", total)
}

fn day3() {
    let original_input = fs::read_to_string("day3_input.txt").expect("Reading input failed").to_owned();
    let mut input = original_input.clone();


    let mut numbers_with_indices: Vec<_> = original_input.match_indices(|c: char| c.is_digit(10)).collect();
    for i in (1..numbers_with_indices.len()).rev() {
        if numbers_with_indices[i].0 == numbers_with_indices[i-1].0 + 1 {
            numbers_with_indices[i].1 = &original_input[numbers_with_indices[i-1].0..numbers_with_indices[i].0 + numbers_with_indices[i].1.len()];
            numbers_with_indices[i].0 -= 1;
            numbers_with_indices.remove(i-1);
        }
    }

    input.retain(|c| !c.is_ascii_whitespace());
    let mut total = 0;
    for i in 0..numbers_with_indices.len() {
        let index = numbers_with_indices[i].0 - (numbers_with_indices[i].0 / 141);//account for deleted new line chars
        let number = numbers_with_indices[i].1;
        let mut adjacent_chars = "".to_owned();
        let left_edge = index % 140 == 0;
        let right_edge = (index + number.len() - 1) % 140 == 139;
        if index >= 140 {
            if !left_edge {
                adjacent_chars =  adjacent_chars + &input[index - 140 - 1..index - 140];
            }
            adjacent_chars = adjacent_chars + &input[index - 140..index - 140 + number.len()];
            if !right_edge {
                adjacent_chars = adjacent_chars + &input[index - 140 + number.len()..index - 140 + number.len() + 1];
            }
        }
        if !left_edge {
            adjacent_chars = adjacent_chars + &input[index - 1..index];
        }
        if !right_edge {
            adjacent_chars = adjacent_chars + &input[index + number.len()..index + number.len() + 1];
        }
        if !(input.len() - 140..input.len()).contains(&index)   {
            if !left_edge {
                adjacent_chars = adjacent_chars + &input[index + 140 - 1..index + 140];
            }
            adjacent_chars = adjacent_chars + &input[index + 140..index + 140 + number.len()];
            if !right_edge {
                adjacent_chars = adjacent_chars + &input[index + 140 + number.len()..index + 140 + number.len() + 1];
            }
        }

        println!("{} - {} - {}", index, number, adjacent_chars);
        if !adjacent_chars.chars().filter(|c: &char| !c.is_digit(10)).filter(|c: &char| *c != '.').collect::<Vec<_>>().is_empty() {
            total += number.parse::<i32>().unwrap();
        }
    }

    println!("Day 3 - {}", total)
}
