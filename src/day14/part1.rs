pub fn solution(input: String) -> i32 {
    let mut matrix = parse_input_to_matrix(input);
    bubble_up(&mut matrix);

    let mut sum: usize = 0;
    for (line_i, line) in matrix.iter().enumerate() {
        let max: usize = matrix.len();
        for &char in line {
            if char == 'O' {
                sum += max - line_i;
            }
        }
    }
    println!("{sum}");
    sum as i32
}

fn bubble_up(matrix: &mut Vec<Vec<char>>) {
    let rows: usize = matrix.len();
    let cols: usize = matrix[0].len();
    let mut changes_made;

    loop {
        let mut next_matrix = matrix.clone();
        changes_made = false;

        for line_i in 1..rows {
            for char_i in 0..cols {
                let current_char = matrix[line_i][char_i];
                let char_above = matrix[line_i - 1][char_i];

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
//         O.OO#....#
//         .....##...
//         OO.#O....O
//         .O.....O#.
//         O.#..O.#.#
//         ..O..#O..O
//         .......O..
//         #....###..
//         #OO..#...."
//             .to_string();
//         assert_eq!(solution(input), 136);
//     }
// }
