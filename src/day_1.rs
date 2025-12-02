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
    part_two();
}

pub fn part_one() {
    let file_path = "src/day_1_input.txt";
    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);
    let rotations = reader
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>();

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

    println!("The part 1 password is: {}", times_zero);
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

// You're sure that's the right password, but the door won't open. You knock, but nobody answers.
// You build a snowman while you think. As you're rolling the snowballs for your snowman,
// you find another security document that must have fallen into the snow:

// "Due to newer security protocols, please use password method 0x434C49434B until further notice."

// You remember from the training seminar that "method 0x434C49434B" means you're actually
// supposed to count the number of times any click causes the dial to point at 0, regardless of
// whether it happens during a rotation or at the end of one.

// Following the same rotations as in the above example, the dial points at zero a few extra times during its rotations:

// The dial starts by pointing at 50.
// The dial is rotated L68 to point at 82; during this rotation, it points at 0 once.
// The dial is rotated L30 to point at 52.
// The dial is rotated R48 to point at 0.
// The dial is rotated L5 to point at 95.
// The dial is rotated R60 to point at 55; during this rotation, it points at 0 once.
// The dial is rotated L55 to point at 0.
// The dial is rotated L1 to point at 99.
// The dial is rotated L99 to point at 0.
// The dial is rotated R14 to point at 14.
// The dial is rotated L82 to point at 32; during this rotation, it points at 0 once.
// In this example, the dial points at 0 three times at the end of a rotation, plus three more times during a rotation. So, in this example, the new password would be 6.

// Be careful: if the dial were pointing at 50, a single rotation like R1000 would cause the dial to point at 0 ten times before returning back to 50!

// Using password method 0x434C49434B, what is the password to open the door?

pub fn part_two() {
    let file_path = "src/day_1_input.txt";
    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);
    let rotations = reader
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>();

    let mut current_position = 50;
    let mut times_zero = 0;

    for rotation in rotations {
        let (direction, turn_amount) = rotation.split_at(1);
        let turn_amount = turn_amount.parse::<i32>().unwrap();

        match direction {
            // rem_euclid works but I wanted to use my own fn
            // "L" => current_num = (current_num - turn_amount).rem_euclid(100),
            // "R" => current_num = (current_num + turn_amount).rem_euclid(100),
            "L" => {
                let (new_position, zero_clicks) =
                    turn_left_with_clicks(current_position, turn_amount);
                current_position = new_position;
                times_zero += zero_clicks;
            }
            "R" => {
                let (new_position, zero_clicks) =
                    turn_right_with_clicks(current_position, turn_amount);
                current_position = new_position;
                times_zero += zero_clicks;
            }
            _ => panic!("Invalid direction: {}", direction),
        }
    }

    println!("The part 2 password is: {}", times_zero);
}

fn turn_left_with_clicks(current_position: i32, distance: i32) -> (i32, i32) {
    let mut zero_clicks = 0;
    let mut difference = current_position - distance;

    if current_position == 0 {
        zero_clicks = distance / 100;
    } else {
        let first_zero = current_position;
        if first_zero < distance {
            zero_clicks += 1;

            let remaining_after = distance - first_zero;
            zero_clicks += remaining_after / 100;
        }
    }

    // wrap around
    while difference < 0 {
        difference += 100;
    }

    if difference == 0 {
        if current_position == 0 {
            if distance % 100 == 0 && distance <= 100 {
                // count the starting position for smaller multiples of 100
                zero_clicks += 1;
            } else if distance % 100 != 0 {
                zero_clicks += 1; // landed on 0 but wasn't counted
            }
        } else {
            let first_zero = current_position;
            if distance == first_zero {
                zero_clicks += 1;
            } else if distance > first_zero && (distance - first_zero) % 100 == 0 {
                // already counted
            } else if difference == 0 {
                // landed on 0 but wasn't counted
                zero_clicks += 1;
            }
        }
    }

    (difference, zero_clicks)
}

fn turn_right_with_clicks(current_position: i32, distance: i32) -> (i32, i32) {
    let mut zero_clicks = 0;
    let mut difference = current_position + distance;
    let mut wrapped = false;

    while difference > 99 {
        difference -= 100;
        wrapped = true;
        zero_clicks += 1;
    }

    // final pos is 0
    if difference == 0 && !wrapped {
        zero_clicks += 1;
    }

    (difference, zero_clicks)
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

    #[test]
    fn test_turn_left_with_clicks() {
        assert_eq!(turn_left_with_clicks(50, 10), (40, 0));
        // starting on 0
        assert_eq!(turn_left_with_clicks(0, 10), (90, 0), "starting on 0");
        // landing on 0
        assert_eq!(turn_left_with_clicks(1, 1), (0, 1), "landing on 0");
        // landing on 0 after wrapping
        assert_eq!(
            turn_left_with_clicks(0, 100),
            (0, 2),
            "landing on 0 after wrapping"
        );
        // wrapping around multiple times
        assert_eq!(
            turn_left_with_clicks(10, 1000),
            (10, 10),
            "wrapping around multiple times"
        );
    }

    #[test]
    fn test_turn_right_with_clicks() {
        assert_eq!(turn_right_with_clicks(50, 10), (60, 0));
        // looping around once
        assert_eq!(
            turn_right_with_clicks(99, 10),
            (9, 1),
            "looping around once"
        );
        // landing on 0
        assert_eq!(turn_right_with_clicks(99, 1), (0, 1), "landing on 0");
        // landing on 0 after wrapping
        assert_eq!(
            turn_right_with_clicks(99, 101),
            (0, 2),
            "landing on 0 after wrapping"
        );
        // wrapping around multiple times
        assert_eq!(
            turn_right_with_clicks(10, 1000),
            (10, 10),
            "wrapping around multiple times"
        );
        // starting on 0
        assert_eq!(turn_right_with_clicks(0, 10), (10, 0), "starting on 0");
        // starting on 0 after wrapping
        assert_eq!(
            turn_right_with_clicks(0, 101),
            (1, 1),
            "starting on 0 after wrapping"
        );
        // wrapping around multiple times
        assert_eq!(
            turn_right_with_clicks(0, 1000),
            (0, 10),
            "wrapping around multiple times"
        );
    }

    #[test]
    fn test_day_two() {
        // The dial starts by pointing at 50.
        // The dial is rotated L68 to point at 82; during this rotation, it points at 0 once.
        // The dial is rotated L30 to point at 52.
        // The dial is rotated R48 to point at 0.
        // The dial is rotated L5 to point at 95.
        // The dial is rotated R60 to point at 55; during this rotation, it points at 0 once.
        // The dial is rotated L55 to point at 0.
        // The dial is rotated L1 to point at 99.
        // The dial is rotated L99 to point at 0.
        // The dial is rotated R14 to point at 14.
        // The dial is rotated L82 to point at 32; during this rotation, it points at 0 once.
        assert_eq!(
            turn_left_with_clicks(50, 68),
            (82, 1),
            "The dial is rotated L68 to point at 82; during this rotation, it points at 0 once."
        );
        assert_eq!(
            turn_left_with_clicks(82, 30),
            (52, 0),
            "The dial is rotated L30 to point at 52."
        );
        assert_eq!(
            turn_right_with_clicks(52, 48),
            (0, 1),
            "The dial is rotated R48 to point at 0."
        );
        assert_eq!(
            turn_left_with_clicks(0, 5),
            (95, 0),
            "The dial is rotated L5 to point at 95."
        );
        assert_eq!(
            turn_right_with_clicks(95, 60),
            (55, 1),
            "The dial is rotated R60 to point at 55; during this rotation, it points at 0 once."
        );
        assert_eq!(
            turn_left_with_clicks(55, 55),
            (0, 1),
            "The dial is rotated L55 to point at 0."
        );
        assert_eq!(
            turn_left_with_clicks(0, 1),
            (99, 0),
            "The dial is rotated L1 to point at 99."
        );
        assert_eq!(
            turn_left_with_clicks(99, 99),
            (0, 1),
            "The dial is rotated L99 to point at 0."
        );
        assert_eq!(
            turn_right_with_clicks(0, 14),
            (14, 0),
            "The dial is rotated R14 to point at 14."
        );
        assert_eq!(
            turn_left_with_clicks(14, 82),
            (32, 1),
            "The dial is rotated L82 to point at 32; during this rotation, it points at 0 once."
        );

        // checking wrapping logic
        assert_eq!(
            turn_left_with_clicks(50, 1000),
            (50, 10),
            "The dial is rotated L1000 to point at 50."
        );
        assert_eq!(
            turn_left_with_clicks(0, 1000),
            (0, 10),
            "The dial is rotated L1000 to point at 50."
        );
    }
}
