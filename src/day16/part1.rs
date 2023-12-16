use std::collections::HashSet;

pub fn solution(input: String) -> i32 {
    let matrix: Vec<Vec<char>> = parse_input(input);
    let mut visited: HashSet<Coords> = HashSet::new();
    let origin: Coords = Coords {
        y: 0,
        x: -1,
        direction: Direction::Right,
    };
    let energized: Vec<Coords> = launch_beam(&matrix, Direction::Right, origin, &mut visited);

    let marked_matrix: Vec<Vec<char>> = mark_energized_coords(&matrix, &energized);

    for row in &marked_matrix {
        println!("{:?}", row);
    }

    let count = count_x(&marked_matrix);
    println!("count: {}", count);
    count
}
fn count_x(matrix: &Vec<Vec<char>>) -> i32 {
    let mut count = 0;
    for row in matrix {
        for cell in row {
            if *cell == 'X' {
                count += 1;
            }
        }
    }
    count
}

fn mark_energized_coords(
    matrix: &Vec<Vec<char>>,
    energized_coords: &Vec<Coords>,
) -> Vec<Vec<char>> {
    let mut marked_matrix: Vec<Vec<char>> = matrix.clone();

    for coord in energized_coords {
        if (coord.y as usize) < matrix.len() && (coord.x as usize) < matrix[0].len() {
            marked_matrix[coord.y as usize][coord.x as usize] = 'X';
        }
    }

    marked_matrix
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
struct Coords {
    y: i32,
    x: i32,
    direction: Direction,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn launch_beam(
    matrix: &Vec<Vec<char>>,
    direction: Direction,
    origin: Coords,
    visited: &mut HashSet<Coords>,
) -> Vec<Coords> {
    let mut energized_coords: Vec<Coords> = Vec::new();

    let next_cell = match direction {
        Direction::Down => Coords {
            y: origin.y + 1,
            x: origin.x,
            direction: Direction::Down,
        },
        Direction::Up => Coords {
            y: origin.y - 1,
            x: origin.x,
            direction: Direction::Up,
        },
        Direction::Left => Coords {
            y: origin.y,
            x: origin.x - 1,
            direction: Direction::Left,
        },
        Direction::Right => Coords {
            y: origin.y,
            x: origin.x + 1,
            direction: Direction::Right,
        },
    };

    if !is_in_bounds(next_cell.x, next_cell.y, matrix[0].len(), matrix.len())
        || visited.contains(&next_cell)
    {
        return energized_coords;
    }

    visited.insert(next_cell);
    energized_coords.push(next_cell);


    let encountered_space_type: char = matrix[next_cell.y as usize][next_cell.x as usize];

    match encountered_space_type {
        '.' => {
            let energized_cells: Vec<Coords> = launch_beam(matrix, direction, next_cell, visited);
            energized_coords.extend(energized_cells);
        }

        '\\' => {
            match direction {
                Direction::Down => {
                    let energized_cells = launch_beam(matrix, Direction::Right, next_cell, visited);
                    energized_coords.extend(energized_cells);
                }
                Direction::Up => {
                    let energized_cells = launch_beam(matrix, Direction::Left, next_cell, visited);
                    energized_coords.extend(energized_cells);
                }
                Direction::Left => {
                    let energized_cells = launch_beam(matrix, Direction::Up, next_cell, visited);
                    energized_coords.extend(energized_cells);
                }
                Direction::Right => {
                    let energized_cells = launch_beam(matrix, Direction::Down, next_cell, visited);
                    energized_coords.extend(energized_cells);
                }
            };
        }

        '/' => {
            match direction {
                Direction::Down => {
                    let energized_cells = launch_beam(matrix, Direction::Left, next_cell, visited);
                    energized_coords.extend(energized_cells);
                }
                Direction::Up => {
                    let energized_cells = launch_beam(matrix, Direction::Right, next_cell, visited);
                    energized_coords.extend(energized_cells);
                }
                Direction::Left => {
                    let energized_cells = launch_beam(matrix, Direction::Down, next_cell, visited);
                    energized_coords.extend(energized_cells);
                }
                Direction::Right => {
                    let energized_cells = launch_beam(matrix, Direction::Up, next_cell, visited);
                    energized_coords.extend(energized_cells);
                }
            };
        }

        '|' => {
            match direction {
                Direction::Down => {
                    let energized_cells = launch_beam(matrix, Direction::Down, next_cell, visited);
                    energized_coords.extend(energized_cells);
                }
                Direction::Up => {
                    let energized_cells = launch_beam(matrix, Direction::Up, next_cell, visited);
                    energized_coords.extend(energized_cells);
                }
                Direction::Left => {
                    let energized_cells = launch_beam(matrix, Direction::Up, next_cell, visited);
                    energized_coords.extend(energized_cells);

                    let energized_cells = launch_beam(matrix, Direction::Down, next_cell, visited);
                    energized_coords.extend(energized_cells);
                }
                Direction::Right => {
                    let energized_cells = launch_beam(matrix, Direction::Up, next_cell, visited);
                    energized_coords.extend(energized_cells);

                    let energized_cells = launch_beam(matrix, Direction::Down, next_cell, visited);
                    energized_coords.extend(energized_cells);
                }
            };
        }
        '-' => {
            match direction {
                Direction::Down => {
                    let energized_cells = launch_beam(matrix, Direction::Left, next_cell, visited);
                    energized_coords.extend(energized_cells);

                    let energized_cells = launch_beam(matrix, Direction::Right, next_cell, visited);
                    energized_coords.extend(energized_cells);
                }
                Direction::Up => {
                    let energized_cells = launch_beam(matrix, Direction::Left, next_cell, visited);
                    energized_coords.extend(energized_cells);

                    let energized_cells = launch_beam(matrix, Direction::Right, next_cell, visited);
                    energized_coords.extend(energized_cells);
                }
                Direction::Left => {
                    let energized_cells = launch_beam(matrix, Direction::Left, next_cell, visited);
                    energized_coords.extend(energized_cells);
                }
                Direction::Right => {
                    let energized_cells = launch_beam(matrix, Direction::Right, next_cell, visited);
                    energized_coords.extend(energized_cells);
                }
            };
        }
        _ => (),
    };

    energized_coords
}

fn is_in_bounds(x: i32, y: i32, grid_width: usize, grid_height: usize) -> bool {
    x >= 0 && y >= 0 && (x as usize) < grid_width && (y as usize) < grid_height
}

fn parse_input(input: String) -> Vec<Vec<char>> {
    let out: Vec<String> = input
        .lines()
        .map(|l| l.trim().to_string())
        .filter(|l| !l.is_empty())
        .collect();
    let out: Vec<Vec<char>> = out.iter().map(|l| l.chars().collect()).collect();
    out
}

// #[cfg(test)]
// mod test {
//     use super::*;
//     #[test]
//     fn test_brief() {
//         let input: String = r#"
//         .|...\....
//         |.-.\.....
//         .....|-...
//         ........|.
//         ..........
//         .........\
//         ..../.\\..
//         .-.-/..|..
//         .|....-|.\
//         ..//.|....
//         "#
//         .to_string();

//         assert_eq!(solution(input), 46);
//     }
// }
