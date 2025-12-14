use std::{collections::HashMap, fs::File, io::BufRead, io::BufReader};

pub fn day_five() {
    part_one();
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

fn get_input_data() -> Vec<String> {
    let file_path = "src/day_5_input.txt";
    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);
    let lines = reader.lines().map(|line| line.unwrap()).collect::<Vec<String>>();
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
            for id in  &available_ids {
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

    return (fresh_reversed_back, available_ids_reversed_back, fresh_ids_reversed_back);
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
}
