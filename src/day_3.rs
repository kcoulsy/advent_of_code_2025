// --- Day 3: Lobby ---
// You descend a short staircase, enter the surprisingly vast lobby, and are quickly
// leared by the security checkpoint. When you get to the main elevators, however,
// you discover that each one has a red light above it: they're all offline.

// "Sorry about that," an Elf apologizes as she tinkers with a nearby control panel.
// "Some kind of electrical surge seems to have fried them. I'll try to get them online soon."

// You explain your need to get further underground. "Well, you could at least take
// the escalator down to the printing department, not that you'd get much further than
// that without the elevators working. That is, you could if the escalator weren't also offline."

// "But, don't worry! It's not fried; it just needs power. Maybe you can get it
// running while I keep working on the elevators."

// There are batteries nearby that can supply emergency power to the escalator
// for just such an occasion. The batteries are each labeled with their joltage
// rating, a value from 1 to 9. You make a note of their joltage ratings
// (your puzzle input). For example:

// 987654321111111
// 811111111111119
// 234234234234278
// 818181911112111

// The batteries are arranged into banks; each line of digits in your input corresponds
// to a single bank of batteries. Within each bank, you need to turn on exactly two batteries;
// the joltage that the bank produces is equal to the number formed by the digits on the batteries
// you've turned on. For example, if you have a bank like 12345 and you turn on batteries 2 and 4,
// the bank would produce 24 jolts. (You cannot rearrange batteries.)

// You'll need to find the largest possible joltage each bank can produce. In the above example:

// In 987654321111111, you can make the largest joltage possible, 98, by turning on the first two batteries.
// In 811111111111119, you can make the largest joltage possible by turning on the batteries labeled 8 and 9, producing 89 jolts.
// In 234234234234278, you can make 78 by turning on the last two batteries (marked 7 and 8).
// In 818181911112111, the largest joltage you can produce is 92.

// The total output joltage is the sum of the maximum joltage from each bank, so in this example,
// the total output joltage is 98 + 89 + 78 + 92 = 357.

// There are many batteries in front of you. Find the maximum joltage possible from each bank; what is the total output joltage?

use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn day_three() {
    part_one();
    part_two();
}
pub fn part_one() {
    let banks = get_input_data();
    let joltage = get_joltage_from_banks(banks);
    println!("Part 1: The total output joltage is: {}", joltage);
}

// --- Part Two ---
// The escalator doesn't move. The Elf explains that it probably needs
// more joltage to overcome the static friction of the system and hits
// the big red "joltage limit safety override" button. You lose count
// of the number of times she needs to confirm "yes, I'm sure" and
// decorate the lobby a bit while you wait.

// Now, you need to make the largest joltage by turning on exactly twelve
// batteries within each bank.

// The joltage output for the bank is still the number formed by the digits
// of the batteries you've turned on; the only difference is that now there
// will be 12 digits in each bank's joltage output instead of two.

// Consider again the example from before:

// 987654321111111
// 811111111111119
// 234234234234278
// 818181911112111
// Now, the joltages are much larger:

// In 987654321111111, the largest joltage can be found by turning on everything
// except some 1s at the end to produce 987654321111.
// In the digit sequence 811111111111119, the largest joltage can be found by
// turning on everything except some 1s, producing 811111111119.
// In 234234234234278, the largest joltage can be found by turning on everything
// except a 2 battery, a 3 battery, and another 2 battery near the start to produce 434234234278.
// In 818181911112111, the joltage 888911112111 is produced by turning on
// everything except some 1s near the front.
// The total output joltage is now much larger: 987654321111 + 811111111119
// + 434234234278 + 888911112111 = 3121910778619.

pub fn part_two() {
    let banks = get_input_data();
    let joltage = get_joltage_from_banks_part_two(banks);
    println!("Part 2: The total output joltage is: {}", joltage);
}

pub fn get_input_data() -> Vec<String> {
    let file_path = "src/day_3_input.txt";
    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);

    let data = reader
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>();

    return data;
}

pub fn get_joltage_from_bank(bank: &str) -> i32 {
    let mut first_height_digit = 0;
    let mut second_height_digit = 0;

    for (i, digit) in bank.chars().enumerate() {
        let digit = digit.to_digit(10).unwrap();
        let is_last_digit = i == bank.len() - 1;

        if digit > first_height_digit && !is_last_digit {
            first_height_digit = digit;
            let next_digit = bank.chars().nth(i + 1).unwrap().to_digit(10).unwrap();
            // resetting the 2nd digit to be the next one
            second_height_digit = next_digit;
            continue;
        }

        if digit > second_height_digit {
            second_height_digit = digit;
        }
    }

    first_height_digit as i32 * 10 + second_height_digit as i32
}

pub fn get_joltage_from_banks(banks: Vec<String>) -> i32 {
    let mut joltage = 0;
    for bank in banks {
        joltage += get_joltage_from_bank(&bank.as_str());
    }
    return joltage;
}

pub fn get_joltage_from_bank_part_two(bank: &str) -> i64 {
    let length = bank.len();
    if length <= 12 {
        return bank.parse::<i64>().unwrap();
    }

    let mut result = String::new();
    let mut start_idx = 0;

    // the 12 digits we need to find
    for position in 0..12 {
        let remaining_digits = 12 - position - 1;
        let max_to_choose_from = length - remaining_digits;

        let mut max_digit = 0;
        let mut max_idx = start_idx;

        for i in start_idx..max_to_choose_from {
            let digit = bank.chars().nth(i).unwrap().to_digit(10).unwrap();
            if digit > max_digit {
                max_digit = digit;
                max_idx = i;
            }
        }

        result.push(bank.chars().nth(max_idx).unwrap());

        start_idx = max_idx + 1;
    }

    return result.parse::<i64>().unwrap();
}

pub fn get_joltage_from_banks_part_two(banks: Vec<String>) -> i64 {
    let mut joltage = 0;
    for bank in banks {
        joltage += get_joltage_from_bank_part_two(&bank.as_str());
    }
    return joltage;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_joltage_from_bank() {
        assert_eq!(get_joltage_from_bank("987654321111111"), 98);
        assert_eq!(get_joltage_from_bank("811111111111119"), 89);
        assert_eq!(get_joltage_from_bank("234234234234278"), 78);
        assert_eq!(get_joltage_from_bank("818181911112111"), 92);
    }

    #[test]
    fn test_get_joltage_from_banks() {
        assert_eq!(
            get_joltage_from_banks(vec![
                String::from("987654321111111"),
                String::from("811111111111119"),
                String::from("234234234234278"),
                String::from("818181911112111")
            ]),
            357
        );
    }

    #[test]
    fn test_get_joltage_from_bank_part_two() {
        // test less than 12
        assert_eq!(get_joltage_from_bank_part_two("1234"), 1234);

        // test = 12
        assert_eq!(get_joltage_from_bank_part_two("123456789012"), 123456789012);

        assert_eq!(
            get_joltage_from_bank_part_two("1223456789012"),
            223456789012
        );
        assert_eq!(
            get_joltage_from_bank_part_two("13223456789012"),
            323456789012
        );
        assert_eq!(
            get_joltage_from_bank_part_two("987654321111111"),
            987654321111
        );
        assert_eq!(
            get_joltage_from_bank_part_two("811111111111119"),
            811111111119
        );
        assert_eq!(
            get_joltage_from_bank_part_two("234234234234278"),
            434234234278
        );
        assert_eq!(
            get_joltage_from_bank_part_two("818181911112111"),
            888911112111
        );
    }

    #[test]
    fn test_get_joltage_from_banks_part_two() {
        assert_eq!(
            get_joltage_from_banks_part_two(vec![
                String::from("987654321111111"),
                String::from("811111111111119"),
                String::from("234234234234278"),
                String::from("818181911112111")
            ]),
            3121910778619
        );
    }
}
