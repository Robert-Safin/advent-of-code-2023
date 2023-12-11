#[derive(Debug)]
struct Coords {
    x: i32,
    y: i32,
    last_move_from: String,
}

pub fn solution(input: String) -> i32 {
    let mut steps: i32 = 0;
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

    loop {
        current_location = match_pipes_and_move_coordinates(&current_location, &input);
        steps += 1;

        if current_location.x == starting_location.x && current_location.y == starting_location.y {
            break;
        }
    }

    println!("steps: {}", steps / 2);
    steps
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

// #[cfg(test)]
// mod test {
//     use super::*;
//     #[test]
//     fn test_brief() {
//         let input = "..F7.
//         .FJ|.
//         SJ.L7
//         |F--J
//         LJ..."
//             .to_string();
//         assert_eq!(solution(input), 8);
//     }
// }
