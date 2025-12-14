use std::{cmp, collections::HashMap, fs::File, io::BufRead, io::BufReader};

pub fn day_five() {
    part_one();
    part_two();
}

// The database operates on ingredient IDs. It consists
// of a list of fresh ingredient ID ranges, a blank line,
// and a list of available ingredient IDs. For example:

// 3-5
// 10-14
// 16-20
// 12-18

// 1
// 5
// 8
// 11
// 17
// 32

// The fresh ID ranges are inclusive: the range 3-5 means that ingredient IDs 3, 4, and 5 are all fresh.
// The ranges can also overlap; an ingredient ID is fresh if it is in any range.

fn part_one() {
    let input = get_input_data();
    let (_fresh_ranges, _available_ids, fresh_ids) = parse_input(input);
    println!("The number of fresh IDs is: {}", fresh_ids.len());
}

// So that they can stop bugging you when they get new inventory, the
// Elves would like to know all of the IDs that the fresh ingredient ID
// ranges consider to be fresh. An ingredient ID is still considered fresh
// if it is in any range.

// Now, the second section of the database (the available ingredient IDs)
// is irrelevant. Here are the fresh ingredient ID ranges from the above example:

// 3-5
// 10-14
// 16-20
// 12-18
// The ingredient IDs that these ranges consider to be fresh are
// 3, 4, 5, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, and 20.
// So, in this example, the fresh ingredient ID ranges consider a
// total of 14 ingredient IDs to be fresh.

fn part_two() {
    let input = get_input_data();
    let (fresh_ranges, _available_ids, _fresh_ids) = parse_input(input);
    let (count_fresh_ids, _merged_ranges) = parse_input_part_two(fresh_ranges);
    println!("The number of fresh IDs is: {}", count_fresh_ids);
}   

fn get_input_data() -> Vec<String> {
    let file_path = "src/day_5_input.txt";
    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);
    let lines = reader
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>();
    lines
}

fn parse_input(input: Vec<String>) -> (Vec<(i64, i64)>, Vec<i64>, Vec<i64>) {
    // reversing first to put the ids in the hashmap
    let reversed_input = input.into_iter().rev().collect::<Vec<String>>();
    let mut fresh_ranges = Vec::new();
    let mut parsing_ids = true;
    let mut available_ids = Vec::new();
    let mut hashmap_ids = HashMap::new();
    let mut fresh_ids = Vec::new();

    for line in reversed_input {
        if line.is_empty() {
            parsing_ids = false;
            continue;
        }

        if parsing_ids {
            let id = line.parse::<i64>().unwrap();
            hashmap_ids.insert(id, false); // assuming false initially
            available_ids.push(id); // keeping a list of ids to loop through later
        }

        if !parsing_ids {
            let range = line.split('-').collect::<Vec<&str>>();
            let start = range[0].parse::<i64>().unwrap();
            let end = range[1].parse::<i64>().unwrap();
            for id in &available_ids {
                let id = *id;
                if id >= start && id <= end && !hashmap_ids.get(&id).unwrap() {
                    hashmap_ids.insert(id, true);
                    fresh_ids.push(id);
                }
            }
            fresh_ranges.push((start, end));
        }
    }

    // probably no need to reverse back but I just wanted to in case part 2 needs it.
    let fresh_reversed_back = fresh_ranges.into_iter().rev().collect::<Vec<(i64, i64)>>();
    let available_ids_reversed_back = available_ids.into_iter().rev().collect::<Vec<i64>>();
    let fresh_ids_reversed_back = fresh_ids.into_iter().rev().collect::<Vec<i64>>();

    return (
        fresh_reversed_back,
        available_ids_reversed_back,
        fresh_ids_reversed_back,
    );
}

// in an ideal world I would copy and modify parse_input so we don't need to loop twice but I'm lazy
fn parse_input_part_two(fresh_ranges: Vec<(i64, i64)>) -> (i64, Vec<(i64, i64)>) {
    let mut merged_ranges = Vec::new();
    let mut count_fresh_ids = 0;
    let mut processed_ranges = vec![false; fresh_ranges.len()];

    // go through the fresh ranges and check for overlapping ranges
    for (i, range_a) in fresh_ranges.iter().enumerate() {
        if processed_ranges[i] {
            continue;
        }

        processed_ranges[i] = true;
        let mut current_range = *range_a;

        loop {
            let mut found_overlapping_range = false;
            for (j, range_b) in fresh_ranges.iter().enumerate() {
                if processed_ranges[j] {
                    continue;
                }

                if current_range.0 <= range_b.1 && current_range.1 >= range_b.0 {
                    found_overlapping_range = true;
                    processed_ranges[j] = true;
                    current_range = (
                        cmp::min(current_range.0, range_b.0),
                        cmp::max(current_range.1, range_b.1),
                    );
                    break;
                }
            }

            if !found_overlapping_range {
                break;
            }
        }

        merged_ranges.push(current_range);
    }

    for range in &merged_ranges {
        count_fresh_ids += range.1 - range.0 + 1;
    }

    return (count_fresh_ids, merged_ranges);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input = vec![
            String::from("3-5"),
            String::from("10-14"),
            String::from("16-20"),
            String::from("12-18"),
            String::from(""),
            String::from("1"),
            String::from("5"),
            String::from("8"),
            String::from("11"),
            String::from("17"),
            String::from("32"),
        ];
        let (fresh_ranges, available_ids, fresh_ids) = parse_input(input);
        assert_eq!(fresh_ranges, vec![(3, 5), (10, 14), (16, 20), (12, 18)]);
        assert_eq!(available_ids, vec![1, 5, 8, 11, 17, 32]);
        assert_eq!(fresh_ids, vec![5, 11, 17]);
        assert_eq!(fresh_ids.len(), 3);
    }

    #[test]
    fn test_parse_input_part_two() {
        let fresh_ranges = vec![(3, 5), (10, 14), (16, 20), (12, 18)];
        let (count_fresh_ids, merged_ranges) = parse_input_part_two(fresh_ranges);
        assert_eq!(count_fresh_ids, 14);
        assert_eq!(merged_ranges, vec![(3, 5), (10, 20)]);
    }
}
