use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn day_four() {
    part_one();
}

// If you can optimize the work the forklifts are doing, maybe they would have
// time to spare to break through the wall.

// The rolls of paper (@) are arranged on a large grid; the Elves even have a
// helpful diagram (your puzzle input) indicating where everything is located.

// For example:

// ..@@.@@@@.
// @@@.@.@.@@
// @@@@@.@.@@
// @.@@@@..@.
// @@.@@@@.@@
// .@@@@@@@.@
// .@.@.@.@@@
// @.@@@.@@@@
// .@@@@@@@@.
// @.@.@@@.@.
// The forklifts can only access a roll of paper if there are fewer than four
// rolls of paper in the eight adjacent positions. If you can figure out which
// rolls of paper the forklifts can access, they'll spend less time looking and
// more time breaking down the wall to the cafeteria.

// In this example, there are 13 rolls of paper that can be accessed by a forklift (marked with x):

// ..xx.xx@x.
// x@@.@.@.@@
// @@@@@.x.@@
// @.@@@@..@.
// x@.@@@@.@x
// .@@@@@@@.@
// .@.@.@.@@@
// x.@@@.@@@@
// .@@@@@@@@.
// x.x.@@@.x.
// Consider your complete diagram of the paper roll locations.
// How many rolls of paper can be accessed by a forklift?

fn get_input_data() -> Vec<String> {
    let file_path = "src/day_4_input.txt";
    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);
    let lines = reader
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>();
    lines
}

fn part_one() {
    let grid = get_input_data();
    let (_accessible_rolls, count_accessible_rolls) = get_accessible_rolls(&grid);
    println!(
        "The number of accessible rolls is: {}",
        count_accessible_rolls
    );
}

fn get_accessible_rolls(grid: &Vec<String>) -> (Vec<String>, u32) {
    let mut accessible_rolls = Vec::new();
    let mut count_accessible_rolls = 0;
    for (row_index, row) in grid.iter().enumerate() {
        let mut string_to_add = String::new();
        for (i, cell) in row.chars().enumerate() {
            let is_roll = cell == '@';
            if is_roll {
                let count = get_adjacent_cells_count(grid, row_index, i);

                if count < 4 {
                    count_accessible_rolls += 1;
                    string_to_add.push('x');
                } else {
                    string_to_add.push(cell);
                }
            } else {
                string_to_add.push('.');
            }
        }
        accessible_rolls.push(string_to_add);
    }

    return (accessible_rolls, count_accessible_rolls);
}

fn get_adjacent_cells_count(grid: &Vec<String>, row: usize, column: usize) -> u32 {
    let mut count = 0;
    let is_first_row = row == 0;
    let is_last_row = row == grid.len() - 1;
    let is_first_column = column == 0;
    let is_last_column = column == grid[row].len() - 1;

    let get_char = |x: usize, y: usize| -> char { grid[x].chars().nth(y).unwrap() };

    let is_roll = |x: usize, y: usize| -> bool { get_char(x, y) == '@' };

    if !is_first_row && !is_first_column && is_roll(row - 1, column - 1) {
        count += 1;
    }

    if !is_first_row && !is_last_column && is_roll(row - 1, column + 1) {
        count += 1;
    }

    if !is_first_row && is_roll(row - 1, column) {
        count += 1;
    }

    if !is_first_column && is_roll(row, column - 1) {
        count += 1;
    }

    if !is_last_column && is_roll(row, column + 1) {
        count += 1;
    }

    if !is_last_row && !is_first_column && is_roll(row + 1, column - 1) {
        count += 1;
    }

    if !is_last_row && !is_last_column && is_roll(row + 1, column + 1) {
        count += 1;
    }

    if !is_last_row && is_roll(row + 1, column) {
        count += 1;
    }

    return count;
}

fn get_accessible_rolls_count(grid: &Vec<&str>) -> u32 {
    let mut count = 0;
    for (row_index, row) in grid.iter().enumerate() {
        for (i, cell) in row.chars().enumerate() {
            if cell == 'x' {
                count += 1;
            }
        }
    }
    return count;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_accessible_rolls() {
        let input = vec![
            "..@@.@@@@.",
            "@@@.@.@.@@",
            "@@@@@.@.@@",
            "@.@@@@..@.",
            "@@.@@@@.@@",
            ".@@@@@@@.@",
            ".@.@.@.@@@",
            "@.@@@.@@@@",
            ".@@@@@@@@.",
            "@.@.@@@.@.",
        ];
        let input = input.iter().map(|s| s.to_string()).collect::<Vec<String>>();
        let output = vec![
            "..xx.xx@x.",
            "x@@.@.@.@@",
            "@@@@@.x.@@",
            "@.@@@@..@.",
            "x@.@@@@.@x",
            ".@@@@@@@.@",
            ".@.@.@.@@@",
            "x.@@@.@@@@",
            ".@@@@@@@@.",
            "x.x.@@@.x.",
        ];
        let output = output
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        let (accessible_rolls, _count) = get_accessible_rolls(&input);

        for roll in &accessible_rolls {
            println!("roll: {}", roll);
        }
        println!("output:");
        for roll in &output {
            println!("roll: {}", roll);
        }
        assert_eq!(accessible_rolls, output);
    }
}
