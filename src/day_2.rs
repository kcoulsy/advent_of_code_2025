use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn day_two() {
    part_one();
    part_two();
}

// You get inside and take the elevator to its only other stop: the gift shop.
// "Thank you for visiting the North Pole!" gleefully exclaims a nearby sign.
// You aren't sure who is even allowed to visit the North Pole, but you know
// you can access the lobby through here, and from there you can access the rest of the North Pole base.

// As you make your way through the surprisingly extensive selection, one of
// the clerks recognizes you and asks for your help.

// As it turns out, one of the younger Elves was playing on a gift shop computer
// and managed to add a whole bunch of invalid product IDs to their gift shop database!
// Surely, it would be no trouble for you to identify the invalid product IDs for them, right?

// They've even checked most of the product ID ranges already; they only have a
// few product ID ranges (your puzzle input) that you'll need to check. For example:

// 11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
// 1698522-1698528,446443-446449,38593856-38593862,565653-565659,
// 824824821-824824827,2121212118-2121212124
// (The ID ranges are wrapped here for legibility; in your input, they appear
//   on a single long line.)

// The ranges are separated by commas (,); each range gives its first ID and
// last ID separated by a dash (-).

// Since the young Elf was just doing silly patterns, you can find the invalid
// IDs by looking for any ID which is made only of some sequence of digits repeated twice.
//So, 55 (5 twice), 6464 (64 twice), and 123123 (123 twice) would all be invalid IDs.

// None of the numbers have leading zeroes; 0101 isn't an ID at all.
// (101 is a valid ID that you would ignore.)

// Your job is to find all of the invalid IDs that appear in the given ranges. In the above example:

// 11-22 has two invalid IDs, 11 and 22.
// 95-115 has one invalid ID, 99.
// 998-1012 has one invalid ID, 1010.
// 1188511880-1188511890 has one invalid ID, 1188511885.
// 222220-222224 has one invalid ID, 222222.
// 1698522-1698528 contains no invalid IDs.
// 446443-446449 has one invalid ID, 446446.
// 38593856-38593862 has one invalid ID, 38593859.
// The rest of the ranges contain no invalid IDs.
// Adding up all the invalid IDs in this example produces 1227775554.

pub fn part_one() {
    let ranges = get_input_data();

    let sum = get_sum_invalid_ids(&ranges);

    println!("The sum of the invalid IDs is: {}", sum);
}

pub fn get_input_data() -> Vec<Vec<i64>> {
    let file_path = "src/day_2_input.txt";
    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);

    let data = reader
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>()
        .get(0)
        .unwrap()
        .split(',')
        .map(|range| {
            range
                .split('-')
                .map(|num| num.parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
        })
        .collect::<Vec<Vec<i64>>>();

    data
}

// Since the young Elf was just doing silly patterns, you can find the invalid IDs by looking for any ID which is made only of some sequence of digits repeated twice. So, 55 (5 twice), 6464 (64 twice), and 123123 (123 twice) would all be invalid IDs.
pub fn is_valid_id(id: i64) -> bool {
    let id_str = id.to_string();

    // strip out the leading 0
    let id_str = id_str.trim_start_matches('0');

    // if odd length then can't have 2 equal sequences
    if id_str.len() % 2 != 0 {
        return true;
    }

    // split into 2 strings
    let first_half = id_str[..id_str.len() / 2].to_string();
    let second_half = id_str[id_str.len() / 2..].to_string();

    // check if the first half is equal to the second half
    if first_half == second_half {
        return false;
    }

    return true;
}

pub fn get_count_invalid_ids(range: &Vec<i64>) -> i64 {
    let mut count = 0;

    assert!(range.len() == 2, "Range must have 2 numbers");

    let start = range[0];
    let end = range[1];

    for id in start..=end {
        if !is_valid_id(id) {
            count += 1;
        }
    }

    return count;
}

pub fn get_invalid_ids(range: &Vec<i64>) -> Vec<i64> {
    let mut invalid_ids = Vec::new();

    assert!(range.len() == 2, "Range must have 2 numbers");

    let start = range[0];
    let end = range[1];

    for id in start..=end {
        if !is_valid_id(id) {
            invalid_ids.push(id);
        }
    }

    return invalid_ids;
}

pub fn get_sum_invalid_ids(ranges: &Vec<Vec<i64>>) -> i64 {
    let mut sum = 0;

    for range in ranges {
        let invalid_ids = get_invalid_ids(range);
        for id in invalid_ids {
            sum += id;
        }
    }

    return sum;
}

pub fn part_two() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid_id() {
        assert_eq!(is_valid_id(11), false, "11 is an invalid ID");
        assert_eq!(is_valid_id(22), false, "22 is an invalid ID");
        assert_eq!(is_valid_id(99), false, "99 is an invalid ID");
        assert_eq!(is_valid_id(101), true, "101 is a valid ID");
        assert_eq!(is_valid_id(123123), false, "123123 is an invalid ID");
        assert_eq!(is_valid_id(12345), true, "12345 is a valid ID");
        assert_eq!(is_valid_id(6464), false, "6464 is an invalid ID");
    }

    #[test]
    fn test_get_count_invalid_ids() {
        assert_eq!(
            get_count_invalid_ids(&vec![11, 22]),
            2,
            "11 and 22 are invalid IDs"
        );
        assert_eq!(
            get_count_invalid_ids(&vec![95, 115]),
            1,
            "99 is an invalid ID"
        );
        assert_eq!(
            get_count_invalid_ids(&vec![998, 1012]),
            1,
            "1010 is an invalid ID"
        );
        assert_eq!(
            get_count_invalid_ids(&vec![1188511880, 1188511890]),
            1,
            "1188511885 is an invalid ID"
        );
        assert_eq!(
            get_count_invalid_ids(&vec![222220, 222224]),
            1,
            "222222 is an invalid ID"
        );
        assert_eq!(
            get_count_invalid_ids(&vec![1698522, 1698528]),
            0,
            "No invalid IDs"
        );
    }

    fn test_get_invalid_ids() {
        assert_eq!(
            get_invalid_ids(&vec![11, 22]),
            vec![11, 22],
            "11 and 22 are invalid IDs"
        );
        assert_eq!(
            get_invalid_ids(&vec![95, 115]),
            vec![99],
            "99 is an invalid ID"
        );
        assert_eq!(
            get_invalid_ids(&vec![998, 1012]),
            vec![1010],
            "1010 is an invalid ID"
        );
        assert_eq!(
            get_invalid_ids(&vec![1188511880, 1188511890]),
            vec![1188511885],
            "1188511885 is an invalid ID"
        );
        assert_eq!(
            get_invalid_ids(&vec![222220, 222224]),
            vec![222222],
            "222222 is an invalid ID"
        );
        assert_eq!(
            get_invalid_ids(&vec![1698522, 1698528]),
            vec![],
            "No invalid IDs"
        );
        assert_eq!(
            get_invalid_ids(&vec![446443, 446449]),
            vec![446446],
            "446446 is an invalid ID"
        );
        assert_eq!(
            get_invalid_ids(&vec![38593856, 38593862]),
            vec![38593859],
            "38593859 is an invalid ID"
        );
        assert_eq!(
            get_invalid_ids(&vec![565653, 565659]),
            vec![],
            "No invalid IDs"
        );
        assert_eq!(
            get_invalid_ids(&vec![824824821, 824824827]),
            vec![],
            "No invalid IDs"
        );
        assert_eq!(
            get_invalid_ids(&vec![2121212118, 2121212124]),
            vec![],
            "No invalid IDs"
        );
    }

    #[test]
    fn test_get_sum_invalid_ids() {
        assert_eq!(
            get_sum_invalid_ids(&vec![
                vec![11, 22],
                vec![95, 115],
                vec![998, 1012],
                vec![1188511880, 1188511890],
                vec![222220, 222224],
                vec![1698522, 1698528],
                vec![446443, 446449],
                vec![38593856, 38593862],
                vec![565653, 565659],
                vec![824824821, 824824827],
                vec![2121212118, 2121212124]
            ]),
            1227775554,
            "The sum of the invalid IDs is 1227775554"
        );
    }
}
