// -- Day 1: Secret Entrance ---

// You arrive at the secret entrance to the North Pole base ready to start decorating.
// Unfortunately, the password seems to have been changed, so you can't get in.
// A document taped to the wall helpfully explains:

// "Due to new security protocols, the password is locked in the safe below. Please see
// the attached document for the new combination."

// The safe has a dial with only an arrow on it; around the dial are the numbers 0 through 99 in order.
// As you turn the dial, it makes a small click noise as it reaches each number.

// The attached document (your puzzle input) contains a sequence of rotations, one per line,
// which tell you how to open the safe. A rotation starts with an L or R which indicates whether
// the rotation should be to the left (toward lower numbers) or to the right (toward higher numbers).
// Then, the rotation has a distance value which indicates how many clicks the dial should be rotated
//in that direction.

// So, if the dial were pointing at 11, a rotation of R8 would cause the dial to point at 19.
// After that, a rotation of L19 would cause it to point at 0.
// Because the dial is a circle, turning the dial left from 0 one click makes it point at 99.
// Similarly, turning the dial right from 99 one click makes it point at 0.

// So, if the dial were pointing at 5, a rotation of L10 would cause it to point at 95.
//After that, a rotation of R5 could cause it to point at 0.

// The dial starts by pointing at 50.

// You could follow the instructions, but your recent required official North Pole secret entrance security training
// seminar taught you that the safe is actually a decoy. The actual password is the number of times the dial is left
// pointing at 0 after any rotation in the sequence.

// For example, suppose the attached document contained the following rotations:

// L68
// L30
// R48
// L5
// R60
// L55
// L1
// L99
// R14
// L82
// Following these rotations would cause the dial to move as follows:

// The dial starts by pointing at 50.
// The dial is rotated L68 to point at 82.
// The dial is rotated L30 to point at 52.
// The dial is rotated R48 to point at 0.
// The dial is rotated L5 to point at 95.
// The dial is rotated R60 to point at 55.
// The dial is rotated L55 to point at 0.
// The dial is rotated L1 to point at 99.
// The dial is rotated L99 to point at 0.
// The dial is rotated R14 to point at 14.
// The dial is rotated L82 to point at 32.
// Because the dial points at 0 a total of three times during this process, the password in this example is 3.

// Analyze the rotations in your attached document. What's the actual password to open the door?

use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn day_one() {
    part_one();
    // part_two();
}

pub fn part_one() {
    let file_path = "src/day_1_input.txt";
    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);
    let rotations = reader
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>();

    println!("Rotations length: {}", rotations.len());

    let mut current_position = 50;
    let mut times_zero = 0;

    for rotation in rotations {
        let (direction, turn_amount) = rotation.split_at(1);
        let turn_amount = turn_amount.parse::<i32>().unwrap();

        match direction {
            // rem_euclid works but I wanted to use my own fn
            // "L" => current_num = (current_num - turn_amount).rem_euclid(100),
            // "R" => current_num = (current_num + turn_amount).rem_euclid(100),
            "L" => current_position = turn_left(current_position, turn_amount),
            "R" => current_position = turn_right(current_position, turn_amount),
            _ => panic!("Invalid direction: {}", direction),
        }

        if current_position == 0 {
            times_zero += 1;
        }
    }

    println!("The password is: {}", times_zero);
}

fn turn_left(current_position: i32, distance: i32) -> i32 {
    let mut difference = current_position - distance;

    while difference < 0 {
        difference += 100;
    }

    difference
}

fn turn_right(current_position: i32, distance: i32) -> i32 {
    let mut difference: i32 = current_position + distance;

    while difference > 99 {
        difference -= 100;
    }

    difference
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_turn_left() {
        assert_eq!(turn_left(50, 10), 40);
        assert_eq!(turn_left(0, 10), 90);

        assert_eq!(turn_left(50, 68), 82);
        assert_eq!(turn_left(82, 30), 52);
        assert_eq!(turn_left(0, 5), 95);
        assert_eq!(turn_left(55, 55), 0);
        assert_eq!(turn_left(0, 1), 99);
        assert_eq!(turn_left(99, 99), 0);
        assert_eq!(turn_left(14, 82), 32);

        // looping multiple times
        assert_eq!(turn_left(14, 200), 14);
    }

    #[test]
    fn test_turn_right() {
        assert_eq!(turn_right(50, 10), 60);
        assert_eq!(turn_right(99, 10), 9);
        assert_eq!(turn_right(50, 68), 18);
        assert_eq!(turn_right(82, 30), 12);
        assert_eq!(turn_right(0, 5), 5);
        assert_eq!(turn_right(55, 55), 10);
        assert_eq!(turn_right(0, 1), 1);
        assert_eq!(turn_right(99, 99), 98);
        assert_eq!(turn_right(14, 82), 96);
        assert_eq!(turn_right(52, 48), 0);
        assert_eq!(turn_right(95, 60), 55);
        assert_eq!(turn_right(0, 14), 14);

        // looping multiple times]
        assert_eq!(turn_right(14, 200), 14);
    }
}
