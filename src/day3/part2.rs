pub fn solution(input: String) -> i32 {
    let mut sum = 0;
    let lines: Vec<&str> = input.lines().map(|line| line.trim()).collect();

    lines.iter().enumerate().for_each(|(line_index, line)| {
        line.chars().enumerate().for_each(|(char_index, char)| {
            let mut numbers: Vec<i32> = Vec::new();

            if char == '*' {
                if has_num_left(&lines, line_index, char_index) {
                    let mut num = String::new();
                    let mut offset = 1;
                    while char_index >= offset
                        && line
                            .chars()
                            .nth(char_index - offset)
                            .unwrap_or(' ')
                            .is_digit(10)
                    {
                        num.insert(0, line.chars().nth(char_index - offset).unwrap());
                        offset += 1;
                    }
                    numbers.push(num.parse::<i32>().unwrap_or(0));
                }

                if has_num_right(&lines, line_index, char_index) {
                    let mut num = String::new();
                    let mut offset = 1;
                    while char_index + offset < line.len()
                        && line
                            .chars()
                            .nth(char_index + offset)
                            .unwrap_or(' ')
                            .is_digit(10)
                    {
                        num.push(line.chars().nth(char_index + offset).unwrap());
                        offset += 1;
                    }
                    numbers.push(num.parse::<i32>().unwrap_or(0));
                }

                if has_num_above(&lines, line_index, char_index)
                    && !has_num_top_left(&lines, line_index, char_index)
                    && !has_num_top_right(&lines, line_index, char_index)
                {
                    if line_index != 0 {
                        let mut num = String::new();
                        let num_above = lines
                            .get(line_index - 1)
                            .unwrap()
                            .chars()
                            .nth(char_index)
                            .unwrap_or(' ');
                        num.push(num_above);
                        let num = num.replace(".", "");
                        numbers.push(num.parse::<i32>().unwrap_or(0));
                    }
                }

                if has_num_below(&lines, line_index, char_index)
                    && !has_num_bottom_left(&lines, line_index, char_index)
                    && !has_num_bottom_right(&lines, line_index, char_index)
                {
                    if line_index + 1 < lines.len() {
                        let mut num = String::new();
                        let num_below = lines
                            .get(line_index + 1)
                            .unwrap()
                            .chars()
                            .nth(char_index)
                            .unwrap_or(' ');
                        num.push(num_below);
                        let num = num.replace(".", "");
                        numbers.push(num.parse::<i32>().unwrap_or(0));
                    }
                }

                if has_num_above(&lines, line_index, char_index)
                    && has_num_top_left(&lines, line_index, char_index)
                    && has_num_top_right(&lines, line_index, char_index)
                {
                    if line_index != 0 {
                        let mut num = String::new();
                        let top_digit = lines
                            .get(line_index - 1)
                            .unwrap()
                            .chars()
                            .nth(char_index)
                            .unwrap_or(' ');
                        let top_left_digit = lines
                            .get(line_index - 1)
                            .unwrap()
                            .chars()
                            .nth(char_index - 1)
                            .unwrap_or(' ');
                        let top_right_digit = lines
                            .get(line_index - 1)
                            .unwrap()
                            .chars()
                            .nth(char_index + 1)
                            .unwrap_or(' ');
                        num.push(top_left_digit);
                        num.push(top_digit);
                        num.push(top_right_digit);
                        let num = num.replace(".", "");
                        numbers.push(num.parse::<i32>().unwrap_or(0));
                    }
                }

                if has_num_below(&lines, line_index, char_index)
                    && has_num_bottom_left(&lines, line_index, char_index)
                    && has_num_bottom_right(&lines, line_index, char_index)
                {
                    if line_index + 1 < lines.len() {
                        let mut num = String::new();
                        let bottom_digit = lines
                            .get(line_index + 1)
                            .unwrap()
                            .chars()
                            .nth(char_index)
                            .unwrap_or(' ');
                        let bottom_left_digit = lines
                            .get(line_index + 1)
                            .unwrap()
                            .chars()
                            .nth(char_index - 1)
                            .unwrap_or(' ');
                        let bottom_right_digit = lines
                            .get(line_index + 1)
                            .unwrap()
                            .chars()
                            .nth(char_index + 1)
                            .unwrap_or(' ');
                        num.push(bottom_left_digit);
                        num.push(bottom_digit);
                        num.push(bottom_right_digit);
                        let num = num.replace(".", "");
                        numbers.push(num.parse::<i32>().unwrap_or(0));
                    }
                }

                if has_num_top_right(&lines, line_index, char_index)
                    && !has_num_above(&lines, line_index, char_index)
                {
                    if line_index != 0 {
                        let mut num = String::new();
                        let top_right_digit = lines
                            .get(line_index - 1)
                            .unwrap()
                            .chars()
                            .nth(char_index + 1)
                            .unwrap_or(' ');
                        let top_right_right_digit = lines
                            .get(line_index - 1)
                            .unwrap()
                            .chars()
                            .nth(char_index + 2)
                            .unwrap_or(' ');
                        let top_right_right_right_digit = lines
                            .get(line_index - 1)
                            .unwrap()
                            .chars()
                            .nth(char_index + 3)
                            .unwrap_or(' ');
                        num.push(top_right_digit);
                        num.push(top_right_right_digit);
                        num.push(top_right_right_right_digit);
                        let num = num.replace(".", "");
                        numbers.push(num.parse::<i32>().unwrap_or(0));
                    }
                }

                if has_num_top_left(&lines, line_index, char_index)
                    && !has_num_above(&lines, line_index, char_index)
                {
                    if line_index != 0 {
                        let mut num = String::new();
                        let top_left_digit = lines
                            .get(line_index - 1)
                            .unwrap()
                            .chars()
                            .nth(char_index - 1)
                            .unwrap_or(' ');
                        let top_left_left_digit = lines
                            .get(line_index - 1)
                            .unwrap()
                            .chars()
                            .nth(char_index - 2)
                            .unwrap_or(' ');
                        let top_left_left_left_digit = lines
                            .get(line_index - 1)
                            .unwrap()
                            .chars()
                            .nth(char_index - 3)
                            .unwrap_or(' ');
                        num.push(top_left_left_left_digit);
                        num.push(top_left_left_digit);
                        num.push(top_left_digit);
                        let num = num.replace(".", "");
                        numbers.push(num.parse::<i32>().unwrap_or(0));
                    }
                }

                if has_num_bottom_right(&lines, line_index, char_index)
                    && !has_num_below(&lines, line_index, char_index)
                {
                    if line_index + 1 < lines.len() {
                        let mut num = String::new();
                        let bottom_right_digit = lines
                            .get(line_index + 1)
                            .unwrap()
                            .chars()
                            .nth(char_index + 1)
                            .unwrap_or(' ');
                        let bottom_right_right_digit = lines
                            .get(line_index + 1)
                            .unwrap()
                            .chars()
                            .nth(char_index + 2)
                            .unwrap_or(' ');
                        let bottom_right_right_right_digit = lines
                            .get(line_index + 1)
                            .unwrap()
                            .chars()
                            .nth(char_index + 3)
                            .unwrap_or(' ');
                        num.push(bottom_right_digit);
                        num.push(bottom_right_right_digit);
                        num.push(bottom_right_right_right_digit);
                        let num = num.replace(".", "");
                        numbers.push(num.parse::<i32>().unwrap_or(0));
                    }
                }

                if has_num_bottom_left(&lines, line_index, char_index)
                    && !has_num_below(&lines, line_index, char_index)
                {
                    if line_index + 1 < lines.len() {
                        let mut num = String::new();
                        let bottom_left_digit = lines
                            .get(line_index + 1)
                            .unwrap()
                            .chars()
                            .nth(char_index - 1)
                            .unwrap_or(' ');
                        let bottom_left_left_digit = lines
                            .get(line_index + 1)
                            .unwrap()
                            .chars()
                            .nth(char_index - 2)
                            .unwrap_or(' ');
                        let bottom_left_left_left_digit = lines
                            .get(line_index + 1)
                            .unwrap()
                            .chars()
                            .nth(char_index - 3)
                            .unwrap_or(' ');
                        num.push(bottom_left_left_left_digit);
                        num.push(bottom_left_left_digit);
                        num.push(bottom_left_digit);
                        let num = num.replace(".", "");
                        numbers.push(num.parse::<i32>().unwrap_or(0));
                    }
                }

                if has_num_above(&lines, line_index, char_index)
                    && has_num_top_right(&lines, line_index, char_index)
                    && !has_num_top_left(&lines, line_index, char_index)
                {
                    if line_index != 0 {
                        let mut num = String::new();
                        let top_digit = lines
                            .get(line_index - 1)
                            .unwrap()
                            .chars()
                            .nth(char_index)
                            .unwrap_or(' ');
                        let top_right_digit = lines
                            .get(line_index - 1)
                            .unwrap()
                            .chars()
                            .nth(char_index + 1)
                            .unwrap_or(' ');
                        let top_right_right_digit = lines
                            .get(line_index - 1)
                            .unwrap()
                            .chars()
                            .nth(char_index + 2)
                            .unwrap_or(' ');
                        num.push(top_digit);
                        num.push(top_right_digit);
                        num.push(top_right_right_digit);
                        let num = num.replace(".", "").replace(" ", "");
                        numbers.push(num.parse::<i32>().unwrap_or(0));
                    }
                }

                if has_num_above(&lines, line_index, char_index)
                    && has_num_top_left(&lines, line_index, char_index)
                    && !has_num_top_right(&lines, line_index, char_index)
                {
                    if line_index != 0 {
                        let mut num = String::new();
                        let top_digit = lines
                            .get(line_index - 1)
                            .unwrap()
                            .chars()
                            .nth(char_index)
                            .unwrap_or(' ');
                        let top_left_digit = lines
                            .get(line_index - 1)
                            .unwrap()
                            .chars()
                            .nth(char_index - 1)
                            .unwrap_or(' ');
                        let top_left_left_digit = lines
                            .get(line_index - 1)
                            .unwrap()
                            .chars()
                            .nth(char_index - 2)
                            .unwrap_or(' ');
                        num.push(top_left_left_digit);
                        num.push(top_left_digit);
                        num.push(top_digit);
                        let num = num.replace(".", "").replace(" ", "");
                        numbers.push(num.parse::<i32>().unwrap_or(0));
                    }
                }

                if has_num_below(&lines, line_index, char_index)
                    && has_num_bottom_right(&lines, line_index, char_index)
                    && !has_num_bottom_left(&lines, line_index, char_index)
                {
                    if line_index + 1 < lines.len() {
                        let mut num = String::new();
                        let bottom_digit = lines
                            .get(line_index + 1)
                            .unwrap()
                            .chars()
                            .nth(char_index)
                            .unwrap_or(' ');
                        let bottom_right_digit = lines
                            .get(line_index + 1)
                            .unwrap()
                            .chars()
                            .nth(char_index + 1)
                            .unwrap_or(' ');
                        let bottom_right_right_digit = lines
                            .get(line_index + 1)
                            .unwrap()
                            .chars()
                            .nth(char_index + 2)
                            .unwrap_or(' ');
                        num.push(bottom_digit);
                        num.push(bottom_right_digit);
                        num.push(bottom_right_right_digit);
                        let num = num.replace(".", "").replace(" ", "");
                        numbers.push(num.parse::<i32>().unwrap_or(0));
                    }
                }

                if has_num_below(&lines, line_index, char_index)
                    && has_num_bottom_left(&lines, line_index, char_index)
                    && !has_num_bottom_right(&lines, line_index, char_index)
                {
                    if line_index + 1 < lines.len() {
                        let mut num = String::new();
                        let bottom_digit = lines
                            .get(line_index + 1)
                            .unwrap()
                            .chars()
                            .nth(char_index)
                            .unwrap_or(' ');
                        let bottom_left_digit = lines
                            .get(line_index + 1)
                            .unwrap()
                            .chars()
                            .nth(char_index - 1)
                            .unwrap_or(' ');
                        let bottom_left_left_digit = lines
                            .get(line_index + 1)
                            .unwrap()
                            .chars()
                            .nth(char_index - 2)
                            .unwrap_or(' ');
                        num.push(bottom_left_left_digit);
                        num.push(bottom_left_digit);
                        num.push(bottom_digit);
                        let num = num.replace(".", "").replace(" ", "");
                        numbers.push(num.parse::<i32>().unwrap_or(0));
                    }
                }
            }

            if numbers.len() == 2 {
                sum += numbers[0] * numbers[1];
            }
        })
    });
    println!("sum: {}", sum);
    sum
}

fn has_num_above(lines: &Vec<&str>, line_index: usize, char_index: usize) -> bool {
    let mut has_num_above = false;
    if line_index > 0 {
        let line_above = lines.get(line_index - 1).unwrap().to_string();
        let char_above = line_above.chars().nth(char_index).unwrap_or('.');
        if char_above.is_numeric() {
            has_num_above = true;
        }
    }
    has_num_above
}

fn has_num_below(lines: &Vec<&str>, line_index: usize, char_index: usize) -> bool {
    let mut has_num_below = false;
    if line_index + 1 < lines.len() {
        let line_below = lines.get(line_index + 1).unwrap().to_string();
        let char_below = line_below.chars().nth(char_index).unwrap_or('.');
        if char_below.is_numeric() {
            has_num_below = true;
        }
    }
    has_num_below
}

fn has_num_left(lines: &Vec<&str>, line_index: usize, char_index: usize) -> bool {
    let mut has_num_left = false;
    if char_index > 0 {
        let line = lines.get(line_index).unwrap().to_string();
        let char_left = line.chars().nth(char_index - 1).unwrap_or('.');
        if char_left.is_numeric() {
            has_num_left = true;
        }
    }
    has_num_left
}

fn has_num_right(lines: &Vec<&str>, line_index: usize, char_index: usize) -> bool {
    let mut has_num_right = false;
    if char_index + 1 < lines.get(line_index).unwrap().len() {
        let line = lines.get(line_index).unwrap().to_string();
        let char_right = line.chars().nth(char_index + 1).unwrap_or('.');
        if char_right.is_numeric() {
            has_num_right = true;
        }
    }
    has_num_right
}

fn has_num_top_left(lines: &Vec<&str>, line_index: usize, char_index: usize) -> bool {
    let mut has_num_top_left = false;
    if line_index > 0 && char_index > 0 {
        let line_above = lines.get(line_index - 1).unwrap().to_string();
        let char_above = line_above.chars().nth(char_index - 1).unwrap_or('.');
        if char_above.is_numeric() {
            has_num_top_left = true;
        }
    }
    has_num_top_left
}

fn has_num_top_right(lines: &Vec<&str>, line_index: usize, char_index: usize) -> bool {
    let mut has_num_top_right = false;
    if line_index > 0 && char_index + 1 < lines.get(line_index).unwrap().len() {
        let line_above = lines.get(line_index - 1).unwrap().to_string();
        let char_above = line_above.chars().nth(char_index + 1).unwrap_or('.');
        if char_above.is_numeric() {
            has_num_top_right = true;
        }
    }
    has_num_top_right
}

fn has_num_bottom_left(lines: &Vec<&str>, line_index: usize, char_index: usize) -> bool {
    let mut has_num_bottom_left = false;
    if line_index + 1 < lines.len() && char_index > 0 {
        let line_below = lines.get(line_index + 1).unwrap().to_string();
        let char_below = line_below.chars().nth(char_index - 1).unwrap_or('.');
        if char_below.is_numeric() {
            has_num_bottom_left = true;
        }
    }
    has_num_bottom_left
}

fn has_num_bottom_right(lines: &Vec<&str>, line_index: usize, char_index: usize) -> bool {
    let mut has_num_bottom_right = false;
    if line_index + 1 < lines.len() && char_index + 1 < lines.get(line_index).unwrap().len() {
        let line_below = lines.get(line_index + 1).unwrap().to_string();
        let char_below = line_below.chars().nth(char_index + 1).unwrap_or('.');
        if char_below.is_numeric() {
            has_num_bottom_right = true;
        }
    }
    has_num_bottom_right
}

// #[cfg(test)]
// mod test {
//     use super::*;
//     #[test]
//     fn test_brief() {
//         let input = "467..114..
// ...*......
// ..35..633.
// ......#...
// 617*......
// .....+.58.
// ..592.....
// ......755.
// ...$.*....
// .664.598.."
//             .to_string();
//         //467835
//         //565922
//         //626540
//         //351249

//         let input2 = "
//           ..747.....415.....*.....94..680..738.
//           ...*..316*........199.....&....*.....
//           .146............................479.."
//             .to_string();

//         let input3 = "
//         938..69.......92...521.....*390.....=237....287.......182
//         ...*...........*........713..................*.........*.
//         .341...........122.730.............890......20..570....64
//         "
//         .to_string();
//         let input4 = "
//         ....573.613...
//         .......*......
//         ...........328"
//             .to_string();
//         assert_eq!(solution(input4), 351249);
//     }
// }
