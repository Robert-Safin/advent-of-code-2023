pub fn solution(input: String) -> i32 {
    let mut sum = 0;
    let lines: Vec<&str> = input.lines().map(|line| line.trim()).collect();

    for (line_index, line) in lines.iter().enumerate() {
        let mut full_number = String::new();
        let mut has_adjacent_symbols = false;

        for (char_index, char) in line.chars().enumerate() {
            if char.is_numeric() {
                full_number.push(char);
                if char_has_adjacent_symbol(lines.clone(), line_index, line, char_index) {
                    has_adjacent_symbols = true;
                }
            }

            if (!char.is_numeric() || char_index == line.len() - 1) && !full_number.is_empty() {
                if has_adjacent_symbols {
                    sum += full_number.parse::<i32>().unwrap_or(0);
                }
                full_number.clear();
                has_adjacent_symbols = false;
            }
        }
    }
    println!("sum: {}", sum);
    sum
}
fn char_has_adjacent_symbol(
    lines: Vec<&str>,
    line_index: usize,
    line: &&str,
    char_index: usize,
) -> bool {
    let mut has_adjacent_symbols = false;

    fn is_symbol(char: char) -> bool {
        !char.is_numeric() && char != '.'
    }

    // Check left
    if char_index > 0 && is_symbol(line.chars().nth(char_index - 1).unwrap_or('.')) {
        has_adjacent_symbols = true;
    }

    // Check right
    if char_index + 1 < line.len() && is_symbol(line.chars().nth(char_index + 1).unwrap_or('.')) {
        has_adjacent_symbols = true;
    }

    // Check top
    if line_index > 0
        && is_symbol(
            lines
                .get(line_index - 1)
                .unwrap()
                .chars()
                .nth(char_index)
                .unwrap_or('.'),
        )
    {
        has_adjacent_symbols = true;
    }

    // Check bottom
    if line_index + 1 < lines.len()
        && is_symbol(
            lines
                .get(line_index + 1)
                .unwrap()
                .chars()
                .nth(char_index)
                .unwrap_or('.'),
        )
    {
        has_adjacent_symbols = true;
    }

    // Check diagonal top left
    if line_index > 0
        && char_index > 0
        && is_symbol(
            lines
                .get(line_index - 1)
                .unwrap()
                .chars()
                .nth(char_index - 1)
                .unwrap_or('.'),
        )
    {
        has_adjacent_symbols = true;
    }

    // Check diagonal top right
    if line_index > 0
        && char_index + 1 < line.len()
        && is_symbol(
            lines
                .get(line_index - 1)
                .unwrap()
                .chars()
                .nth(char_index + 1)
                .unwrap_or('.'),
        )
    {
        has_adjacent_symbols = true;
    }

    // Check diagonal bottom left
    if line_index + 1 < lines.len()
        && char_index > 0
        && is_symbol(
            lines
                .get(line_index + 1)
                .unwrap()
                .chars()
                .nth(char_index - 1)
                .unwrap_or('.'),
        )
    {
        has_adjacent_symbols = true;
    }

    // Check diagonal bottom right
    if line_index + 1 < lines.len()
        && char_index + 1 < line.len()
        && is_symbol(
            lines
                .get(line_index + 1)
                .unwrap()
                .chars()
                .nth(char_index + 1)
                .unwrap_or('.'),
        )
    {
        has_adjacent_symbols = true;
    }
    has_adjacent_symbols
}

// #[cfg(test)]
// mod test {
//     use super::*;
//     #[test]
//     fn test_brief() {
//         let input = "467..114..
//       ...*......
//       ..35..633.
//       ......#...
//       617*......
//       .....+.58.
//       ..592.....
//       ......755.
//       ...$.*....
//       .664.598.."
//             .to_string();
//         assert_eq!(solution(input), 4361);
//     }
// }
