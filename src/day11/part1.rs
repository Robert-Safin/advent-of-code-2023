use std::collections::HashSet;

pub fn solution(input: String) -> i32 {
    let input: Vec<Vec<char>> = parse_to_matrix(input);

    let mut matrix = expand_matrix_row(input);
    matrix = expand_matrix_columns(matrix);

    let cords = collect_galaxy_cords(&matrix);

    let sum = loop_and_sum(cords);
    println!("{sum}");
    sum
}

fn parse_to_matrix(input: String) -> Vec<Vec<char>> {
    input
        .split_whitespace()
        .map(|line| line.trim().chars().collect::<Vec<char>>())
        .collect()
}

fn expand_matrix_row(input: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut matrix: Vec<Vec<char>> = Vec::new();

    for line in input.iter() {
        matrix.push(line.clone());

        if line.iter().all(|&e| e == '.') {
            matrix.push(vec!['.'; line.len()]);
        }
    }
    matrix
}

fn expand_matrix_columns(input: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut columns_to_expand = HashSet::new();
    let row_length = input[0].len();

    for col in 0..row_length {
        if input.iter().all(|row| row[col] == '.') {
            columns_to_expand.insert(col);
        }
    }

    input
        .into_iter()
        .map(|row| {
            let mut expanded_row = Vec::new();
            for (i, char) in row.iter().enumerate() {
                expanded_row.push(*char);
                if columns_to_expand.contains(&i) {
                    expanded_row.push('.');
                }
            }
            expanded_row
        })
        .collect()
}

fn collect_galaxy_cords(input: &Vec<Vec<char>>) -> Vec<(i32, i32)> {
    let mut cords: Vec<(i32, i32)> = Vec::new();

    input.iter().enumerate().for_each(|(line_i, line)| {
        line.iter().enumerate().for_each(|(char_i, char)| {
            if char == &'#' {
                cords.push((char_i as i32, line_i as i32))
            }
        });
    });

    cords
}

fn loop_and_sum(cords: Vec<(i32, i32)>) -> i32 {
    let mut sum = 0;
    let cords_len = cords.len();
    for i in 0..cords_len {
        for j in i + 1..cords_len {
            let x_diff = (cords[i].0 - cords[j].0).abs();
            let y_diff = (cords[i].1 - cords[j].1).abs();
            sum += x_diff + y_diff;
        }
    }
    sum
}

// #[cfg(test)]
// mod test {
//     use super::*;
//     #[test]
//     fn test_brief() {
//         let input = "
//         ...#......
//         .......#..
//         #.........
//         ..........
//         ......#...
//         .#........
//         .........#
//         ..........
//         .......#..
//         #...#....."
//             .to_string();
//         assert_eq!(solution(input), 374);
//     }
// }
