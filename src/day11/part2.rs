pub fn solution(input: String) -> i64 {
    let input: Vec<Vec<char>> = parse_to_matrix(input);

    let empty_rows: Vec<bool> = count_empty_rows(&input);
    let empty_columns: Vec<bool> = count_empty_columns(&input);

    let cords: Vec<(i32, i32)> = collect_galaxy_cords(&input);

    let sum = loop_and_sum(cords, &empty_rows, &empty_columns);
    println!("{sum}");
    sum
}

fn count_empty_rows(input: &Vec<Vec<char>>) -> Vec<bool> {
    input
        .iter()
        .map(|line| line.iter().all(|&e| e == '.'))
        .collect()
}

fn count_empty_columns(input: &Vec<Vec<char>>) -> Vec<bool> {
    let row_length = input[0].len();
    (0..row_length)
        .map(|col| input.iter().all(|row| row[col] == '.'))
        .collect()
}

fn parse_to_matrix(input: String) -> Vec<Vec<char>> {
    input
        .split_whitespace()
        .map(|line| line.trim().chars().collect::<Vec<char>>())
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

fn loop_and_sum(cords: Vec<(i32, i32)>, empty_rows: &[bool], empty_columns: &[bool]) -> i64 {
    let mut sum: i64 = 0;
    let cords_len = cords.len();
    for i in 0..cords_len {
        for j in i + 1..cords_len {
            let x_diff = (cords[i].0 - cords[j].0).abs() as i64;
            let y_diff = (cords[i].1 - cords[j].1).abs() as i64;

            let empty_row_count = count_intervals(cords[i].1, cords[j].1, empty_rows);
            let empty_col_count = count_intervals(cords[i].0, cords[j].0, empty_columns);

            sum += x_diff
                + (empty_col_count as i64 * 999_999)
                + y_diff
                + (empty_row_count as i64 * 999_999);
        }
    }
    sum
}

fn count_intervals(start: i32, end: i32, empty: &[bool]) -> i32 {
    let (low, high) = (start.min(end) as usize, start.max(end) as usize);
    empty[low..=high]
        .iter()
        .filter(|&&is_empty| is_empty)
        .count() as i32
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
//         assert_eq!(solution(input), 1030);
//     }
// }
