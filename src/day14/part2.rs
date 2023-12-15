use std::collections::HashMap;

pub fn solution(input: String) -> i32 {
    let mut matrix: Vec<Vec<char>> = parse_input_to_matrix(input);
    let mut hash: HashMap<String, i32> = HashMap::new();
    let mut sum: i32 = 0;
    let mut cycle_start_index: usize = 0;
    let mut found_loop_start: bool = false;
    let mut found_loop_end: bool = false;

    let mut cycle_outcomes: Vec<i32> = Vec::new();

    while !found_loop_end {
        bubble_up(&mut matrix);
        bubble_left(&mut matrix);
        bubble_down(&mut matrix);
        bubble_right(&mut matrix);

        let mut sum: usize = 0;
        for (line_i, line) in matrix.iter().enumerate() {
            let max: usize = matrix.len();
            for &char in line {
                if char == 'O' {
                    sum += max - line_i;
                }
            }
        }
        cycle_outcomes.push(sum as i32);

        let matrix_str: String = matrix_to_string(&matrix);
        let count: &mut i32 = hash.entry(matrix_str.clone()).or_insert(0);
        *count += 1;

        if *count == 2 && !found_loop_start {
            found_loop_start = true;
            cycle_start_index = cycle_outcomes.len() - 1;
        }
        if *count == 3 {
            found_loop_end = true;
        }
    }

    if found_loop_start {
        let cycle_length: usize = cycle_outcomes.len() - cycle_start_index - 1;
        let pre_cycle_len: usize = cycle_start_index - cycle_length;

        let cycle_index = (1_000_000_000 - pre_cycle_len) % cycle_length;

        cycle_outcomes.pop();
        let cycle_slice: &[i32] = &cycle_outcomes[cycle_start_index..cycle_outcomes.len()];
        println!("{}", (1_000_000_000 - pre_cycle_len) / cycle_length);
        sum = cycle_slice[cycle_index - 1]
    }

    //println!("{sum}");
    sum
}
fn matrix_to_string(matrix: &[Vec<char>]) -> String {
    matrix
        .iter()
        .map(|row| row.iter().collect::<String>())
        .collect::<Vec<_>>()
        .join("\n")
}

fn bubble_up(matrix: &mut Vec<Vec<char>>) {
    let rows: usize = matrix.len();
    let cols: usize = matrix[0].len();
    let mut changes_made: bool;

    loop {
        let mut next_matrix: Vec<Vec<char>> = matrix.clone();
        changes_made = false;

        for line_i in 1..rows {
            for char_i in 0..cols {
                let current_char: char = matrix[line_i][char_i];
                let char_above: char = matrix[line_i - 1][char_i];

                if char_above == '.' && current_char == 'O' {
                    next_matrix[line_i][char_i] = '.';
                    next_matrix[line_i - 1][char_i] = 'O';
                    changes_made = true;
                }
            }
        }

        if !changes_made {
            break;
        }

        *matrix = next_matrix;
    }
}

fn bubble_right(matrix: &mut Vec<Vec<char>>) {
    let rows: usize = matrix.len();
    let cols: usize = matrix[0].len();
    let mut changes_made: bool;

    loop {
        let mut next_matrix: Vec<Vec<char>> = matrix.clone();
        changes_made = false;

        for row in 0..rows {
            for col in (0..cols - 1).rev() {
                let current_char: char = matrix[row][col];
                let char_right: char = matrix[row][col + 1];

                if char_right == '.' && current_char == 'O' {
                    next_matrix[row][col] = '.';
                    next_matrix[row][col + 1] = 'O';
                    changes_made = true;
                }
            }
        }

        if !changes_made {
            break;
        }

        *matrix = next_matrix;
    }
}

fn bubble_left(matrix: &mut Vec<Vec<char>>) {
    let rows: usize = matrix.len();
    let cols: usize = matrix[0].len();
    let mut changes_made: bool;

    loop {
        let mut next_matrix: Vec<Vec<char>> = matrix.clone();
        changes_made = false;

        for row in 0..rows {
            for col in 1..cols {
                let current_char: char = matrix[row][col];
                let char_left: char = matrix[row][col - 1];

                if char_left == '.' && current_char == 'O' {
                    next_matrix[row][col] = '.';
                    next_matrix[row][col - 1] = 'O';
                    changes_made = true;
                }
            }
        }

        if !changes_made {
            break;
        }

        *matrix = next_matrix;
    }
}

fn bubble_down(matrix: &mut Vec<Vec<char>>) {
    let rows: usize = matrix.len();
    let cols: usize = matrix[0].len();
    let mut changes_made: bool;

    loop {
        let mut next_matrix: Vec<Vec<char>> = matrix.clone();
        changes_made = false;

        for row in (0..rows - 1).rev() {
            for col in 0..cols {
                let current_char: char = matrix[row][col];
                let char_below: char = matrix[row + 1][col];

                if char_below == '.' && current_char == 'O' {
                    next_matrix[row][col] = '.';
                    next_matrix[row + 1][col] = 'O';
                    changes_made = true;
                }
            }
        }

        if !changes_made {
            break;
        }

        *matrix = next_matrix;
    }
}

fn parse_input_to_matrix(input: String) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|line| line.trim().chars().collect())
        .collect()
}

// #[cfg(test)]
// mod test {
//     use super::*;
//     #[test]
//     fn test_brief() {
//         let input = "O....#....
//       O.OO#....#
//       .....##...
//       OO.#O....O
//       .O.....O#.
//       O.#..O.#.#
//       ..O..#O..O
//       .......O..
//       #....###..
//       #OO..#...."
//             .to_string();
//         assert_eq!(solution(input), 64);
//     }
// }
