pub fn solution(input: String) -> i32 {
    let mut sum: i32 = 0;

    for line in input.split("\n") {
        let mut line_tuple: (i32, i32) = (0, 0);

        'outer: for (i, c) in line.chars().enumerate() {
            if c.is_numeric() {
                line_tuple.0 = c.to_digit(10).unwrap() as i32;
                break 'outer;
            }
            for &(word, value) in &NUM_WORDS {
                if line[i..].starts_with(word) {
                    line_tuple.0 = value;
                    break 'outer;
                }
            }
        }

        let reversed_line: String = line.chars().rev().collect();
        'outer_rev: for (i, c) in reversed_line.chars().enumerate() {
            if c.is_numeric() {
                line_tuple.1 = c.to_digit(10).unwrap() as i32;
                break 'outer_rev;
            }
            for &(word, value) in &NUM_WORDS {
                let rev_word: String = word.chars().rev().collect();
                if reversed_line[i..].starts_with(&rev_word) {
                    line_tuple.1 = value;
                    break 'outer_rev;
                }
            }
        }

        let calibration_value: i32 = format!("{}{}", line_tuple.0, line_tuple.1)
            .parse::<i32>()
            .unwrap();
        sum += calibration_value;
    }

    sum
}

const NUM_WORDS: [(&str, i32); 9] = [
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
];

#[cfg(test)]
mod test {
    use std::fs;

    use super::*;
    #[test]
    fn test() {
        let input = fs::read_to_string("src/inputs/day1.txt").unwrap();

        assert_eq!(solution(input), 54728)
    }
}
