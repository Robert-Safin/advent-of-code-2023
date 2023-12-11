use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
struct Coords {
    x: i32,
    y: i32,
    last_move_from: String,
}

pub fn solution(input: String) -> i32 {
    let mut enclosed_count: i32 = 0;
    let input: Vec<_> = input
        .lines()
        .map(|line| line.trim().chars().collect::<Vec<char>>())
        .collect();

    let mut current_location = Coords {
        x: 0,
        y: 0,
        last_move_from: "".to_string(),
    };

    let mut starting_location = Coords {
        x: 0,
        y: 0,
        last_move_from: "".to_string(),
    };

    for (line_index, line) in input.iter().enumerate() {
        line.iter().enumerate().for_each(|(char_index, &char)| {
            if char == 'S' {
                current_location = Coords {
                    x: char_index as i32,
                    y: line_index as i32,
                    last_move_from: "".to_string(),
                };

                starting_location = Coords {
                    x: char_index as i32,
                    y: line_index as i32,
                    last_move_from: "".to_string(),
                };
            }
        });
    }

    let mut main_loop = Vec::new();

    loop {
        main_loop.push((current_location.x, current_location.y));
        current_location = match_pipes_and_move_coordinates(&current_location, &input);

        if current_location.x == starting_location.x && current_location.y == starting_location.y {
            break;
        }
    }

    main_loop.push((current_location.x, current_location.y));
    // find char 'S' in in put and replace with '7'
    let mut clean_input = input.clone();
    for (y, row) in clean_input.iter_mut().enumerate() {
        for (x, tile) in row.iter_mut().enumerate() {
            if *tile == 'S' {
                *tile = 'F';
            }
        }
    }

    let non_junk: Vec<Vec<char>> = generate_clean_input(&clean_input, &main_loop);

    for (line_i, line) in non_junk.iter().enumerate() {
        println!("--{:?}--", line_i);
        for (char_index, &char) in line.iter().enumerate() {
            if char == '.' {
                let char_counts_left = count_chars_to_left_of_dot(line, char_index);
                let char_counts_right = count_chars_to_the_right_of_dot(line, char_index);
                let join_hash = char_counts_left
                    .iter()
                    .chain(char_counts_right.iter())
                    .map(|(k, v)| (*k, *v))
                    .collect::<HashMap<char, i32>>();
                let pipe_count = join_hash.get(&'|').unwrap_or(&0);
                let f_count = join_hash.get(&'F').unwrap_or(&0);
                let j_count = join_hash.get(&'J').unwrap_or(&0);

                let sum_of_counts = pipe_count + f_count + j_count;
                if sum_of_counts % 2 != 0 {
                    enclosed_count += 1;
                }
            }
        }
    }

    enclosed_count
}
fn count_chars_to_left_of_dot(line: &Vec<char>, char_index: usize) -> HashMap<char, i32> {
    let mut counts = HashMap::new();

    for &c in &line[..char_index] {
        if c == '|' || c == 'F' || c == 'J' {
            *counts.entry(c).or_insert(0) += 1;
        }
    }

    counts
}

fn count_chars_to_the_right_of_dot(line: &Vec<char>, char_index: usize) -> HashMap<char, i32> {
    let mut counts = HashMap::new();

    for &c in &line[char_index + 1..] {
        if c == '|' || c == 'F' || c == 'J' {
            *counts.entry(c).or_insert(0) += 1;
        }
    }

    counts
}

fn generate_clean_input(input: &Vec<Vec<char>>, main_loop: &Vec<(i32, i32)>) -> Vec<Vec<char>> {
    let main_loop_set: HashSet<(i32, i32)> = main_loop.iter().cloned().collect();
    let mut clean_input = input.clone();

    for (y, row) in clean_input.iter_mut().enumerate() {
        for (x, tile) in row.iter_mut().enumerate() {
            if !main_loop_set.contains(&(x as i32, y as i32)) {
                *tile = '.';
            }
        }
    }

    clean_input
}
fn match_pipes_and_move_coordinates(current_location: &Coords, input: &Vec<Vec<char>>) -> Coords {
    let mut new_location = Coords {
        x: current_location.x,
        y: current_location.y,
        last_move_from: current_location.last_move_from.clone(),
    };

    let current_pipe = get_char_or_default(
        input,
        current_location.x as isize,
        current_location.y as isize,
        '.',
    );

    if current_pipe == 'S' {
        new_location.last_move_from = "top".to_string();
        new_location.y = current_location.y + 1;
    }

    match current_pipe {
        '|' if current_location.last_move_from == "top" => {
            new_location.last_move_from = "top".to_string();
            new_location.y = current_location.y + 1;
        }

        '|' if current_location.last_move_from == "bottom" => {
            new_location.last_move_from = "bottom".to_string();
            new_location.y = current_location.y - 1;
        }

        '-' if current_location.last_move_from == "left" => {
            new_location.last_move_from = "left".to_string();
            new_location.x = current_location.x + 1;
        }

        '-' if current_location.last_move_from == "right" => {
            new_location.last_move_from = "right".to_string();
            new_location.x = current_location.x - 1;
        }

        'L' if current_location.last_move_from == "top" => {
            new_location.last_move_from = "left".to_string();
            new_location.x = current_location.x + 1;
        }

        'L' if current_location.last_move_from == "right" => {
            new_location.last_move_from = "bottom".to_string();
            new_location.y = current_location.y - 1;
        }

        'J' if current_location.last_move_from == "top" => {
            new_location.last_move_from = "right".to_string();
            new_location.x = current_location.x - 1;
        }

        'J' if current_location.last_move_from == "left" => {
            new_location.last_move_from = "bottom".to_string();
            new_location.y = current_location.y - 1;
        }

        'F' if current_location.last_move_from == "bottom" => {
            new_location.last_move_from = "left".to_string();
            new_location.x = current_location.x + 1;
        }

        'F' if current_location.last_move_from == "right" => {
            new_location.last_move_from = "top".to_string();
            new_location.y = current_location.y + 1;
        }

        '7' if current_location.last_move_from == "bottom" => {
            new_location.last_move_from = "right".to_string();
            new_location.x = current_location.x - 1;
        }

        '7' if current_location.last_move_from == "left" => {
            new_location.last_move_from = "top".to_string();
            new_location.y = current_location.y + 1;
        }

        _ => (),
    }

    new_location
}

fn get_char_or_default(input: &Vec<Vec<char>>, x: isize, y: isize, default: char) -> char {
    if y < 0 || y >= input.len() as isize {
        return default;
    }

    let row = &input[y as usize];
    if x < 0 || x >= row.len() as isize {
        return default;
    }

    row[x as usize]
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_brief() {
        let input = "..........
        .S------7.
        .|F----7|.
        .||OOOO||.
        .||OOOO||.
        .|L-7F-J|.
        .|II||II|.
        .L--JL--J.
        .........."
            .to_string();
        assert_eq!(solution(input), 10);
    }
}
