use std::collections::HashSet;

pub fn solution(input: String) -> i32 {
    let matrix: Vec<Vec<char>> = parse_input(input);
    let mut scores = Vec::new();

    let launch_points = generate_launch_points(matrix[0].len(), matrix.len());
    for point in launch_points {
        let score = launch_point(&matrix, point);
        scores.push(score);
    }

    let max_score = scores.iter().max().unwrap();
    println!("max_score: {}", max_score);
    max_score.clone()
}

fn launch_point(matrix: &Vec<Vec<char>>, point: Coords) -> i32 {
    let mut visited: HashSet<Coords> = HashSet::new();
    let energized = launch_beam(&matrix, point.direction, point, &mut visited);
    let marked_matrix = mark_energized_coords(&matrix, &energized);
    let count = count_x(&marked_matrix);
    count
}

fn generate_launch_points(grid_width: usize, grid_height: usize) -> Vec<Coords> {
    let mut launch_points = Vec::new();

    for x in 0..grid_width {
        launch_points.push(Coords {
            y: -1,
            x: x as i32,
            direction: Direction::Down,
        });
    }

    for x in 0..grid_width {
        launch_points.push(Coords {
            y: grid_height as i32,
            x: x as i32,
            direction: Direction::Up,
        });
    }

    for y in 0..grid_height {
        launch_points.push(Coords {
            y: y as i32,
            x: -1,
            direction: Direction::Right,
        });
    }

    for y in 0..grid_height {
        launch_points.push(Coords {
            y: y as i32,
            x: grid_width as i32,
            direction: Direction::Left,
        });
    }

    launch_points
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
    let mut marked_matrix = matrix.clone();

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

    if !is_in_bounds(next_cell.x, next_cell.y, matrix[0].len(), matrix.len()) {
        return energized_coords;
    }

    let encountered_space_type = matrix[next_cell.y as usize][next_cell.x as usize];

    match encountered_space_type {
        '.' => {
            let energized_cells = launch_beam(matrix, direction, next_cell, visited);
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

//         assert_eq!(solution(input), 51);
//     }
// }
