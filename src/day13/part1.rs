pub fn solution(input: String) -> i32 {
    let mut ver_count = 0;
    let mut hor_count = 0;
    let parsed_matrices = parse_input_to_matrices(input);

    for (matrix_i, matrix) in parsed_matrices.iter().enumerate() {
        let ver_mirror: Option<(usize, usize)> = find_largest_vertical_mirror(&matrix);
        let hor_mirror: Option<(usize, usize)> = find_largest_horizontal_mirror(&matrix);

        if get_mirror_size(ver_mirror) > get_mirror_size(hor_mirror) {
            if ver_mirror.is_some() {
                let (start, end) = ver_mirror.unwrap();
                let middle = (start + end) / 2;
                let len = matrix[0].len();
                let cols_left = len - middle;
                ver_count += cols_left;
            }
        } else {
            if hor_mirror.is_some() {
                let (start, end) = hor_mirror.unwrap();
                let middle = (start + end) / 2;
                let len = matrix.len();
                let rows_left = len - middle;
                hor_count += rows_left;
            }
        }
    }
    let result = ver_count + (hor_count * 100);
    println!("{}", result);
    result as i32
}

fn get_mirror_size(mirror: Option<(usize, usize)>) -> i32 {
    match mirror {
        Some((start, end)) => {
            if start <= end {
                (end - start) as i32
            } else {
                0
            }
        }
        None => 0,
    }
}

fn find_largest_horizontal_mirror(matrix: &Vec<Vec<char>>) -> Option<(usize, usize)> {
    let mut largest_mirror = None;
    let mut max_length = 0;

    for start in 0..matrix.len() {
        for end in (start + 1)..matrix.len() {
            if matrix[start] == matrix[end] {
                let mut current_start = start;
                let mut current_end = end;
                while current_start > 0 && current_end < matrix.len() - 1 {
                    if matrix[current_start - 1] != matrix[current_end + 1] {
                        break;
                    }
                    current_start -= 1;
                    current_end += 1;
                }
                let length = current_end - current_start + 1;
                if length > max_length {
                    largest_mirror = Some((current_start, current_end));
                    max_length = length;
                }
            }
        }
    }

    largest_mirror
}

fn find_largest_vertical_mirror(matrix: &Vec<Vec<char>>) -> Option<(usize, usize)> {
    let mut largest_mirror = None;
    let mut max_length = 0;

    for mid_col in 1..matrix[0].len() - 1 {
        let mut left = mid_col - 1;
        let mut right = mid_col;
        while left > 0 && right < matrix[0].len() - 1 && is_column_identical(matrix, left, right) {
            left -= 1;
            right += 1;
        }
        let length = right - left + 1;
        if length > max_length {
            largest_mirror = Some((left + 1, right - 1));
            max_length = length;
        }
    }

    largest_mirror
}

fn is_column_identical(matrix: &Vec<Vec<char>>, col1: usize, col2: usize) -> bool {
    for row in 0..matrix.len() {
        if matrix[row][col1] != matrix[row][col2] {
            return false;
        }
    }
    true
}

fn parse_input_to_matrices(input: String) -> Vec<Vec<Vec<char>>> {
    let matrices = input.split("\n\n").collect::<Vec<&str>>();
    let mut parsed_matrices = Vec::new();
    for matrix in matrices {
        let rows = matrix
            .lines()
            .map(|line| line.trim().chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();
        parsed_matrices.push(rows);
    }
    parsed_matrices
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_brief() {
        let input = "#.##..##.
        ..#.##.#.
        ##......#
        ##......#
        ..#.##.#.
        ..##..##.
        #.#.##.#.

        #...##..#
        #....#..#
        ..##..###
        #####.##.
        #####.##.
        ..##..###
        #....#..#"
            .to_string();
        assert_eq!(solution(input), 405);
    }
}
// not 34282 low
// not 31757 high
// not 34783
// not 54294
