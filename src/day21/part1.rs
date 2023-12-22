pub fn solution(input: String, max_steps: i32) -> i32 {
    let mut matrix = parse_input_to_matrix(input);
    let starting_location = get_starting_location(&matrix);
    recursive_update(
        &mut matrix,
        &starting_location,
        0,
        max_steps,
        starting_location.clone(),
    );

    remove_non_ending_steps(&mut matrix, max_steps);

    for line in &matrix {
        println!("{:?}", line.join(""));
    }

    let plots = count_plots(&mut matrix);
    println!("plots: {}", plots);
    plots
}

// fn replace_tiles_from_origin(matrix: &mut Vec<Vec<String>>, starting_location: Tile, radius: i32) {
//   let rows = matrix.len() as i32;
//   let cols = matrix[0].len() as i32;

//   // Define the bounds of the square to replace tiles within
//   let y_start = (starting_location.y as i32 - radius).max(0);
//   let y_end = (starting_location.y as i32 + radius).min(rows - 1);
//   let x_start = (starting_location.x as i32 - radius).max(0);
//   let x_end = (starting_location.x as i32 + radius).min(cols - 1);

//   for y in y_start..=y_end {
//       for x in x_start..=x_end {
//           // Replace tiles within a Manhattan distance of `radius` from the starting location
//           if (x - starting_location.x as i32).abs() + (y - starting_location.y as i32).abs() <= radius {
//               matrix[y as usize][x as usize] = "#".to_string();
//           }
//       }
//   }
// }


fn recursive_update(
    matrix: &mut Vec<Vec<String>>,
    tile: &Tile,
    current_step: i32,
    max_steps: i32,
    starting_location: Tile,
) {
    if current_step > max_steps {
        return;
    }



    // if current_step % 5 == 0 {
    //     let radius = current_step - 14;
    //     replace_tiles_from_origin(matrix, starting_location, radius);
    // }

    update_tile(matrix, *tile, current_step);

    let adjacent_tiles = get_adjacent_tiles(&matrix, *tile);
    let filtered_tiles = filter_tiles(&matrix, adjacent_tiles);

    for adj_tile in filtered_tiles {
        recursive_update(
            matrix,
            &adj_tile,
            current_step + 1,
            max_steps,
            starting_location,
        );
    }
}

fn count_plots(matrix: &mut Vec<Vec<String>>) -> i32 {
    let mut count = 0;
    for line in matrix {
        for tile in line {
            if *tile != "#".to_string() && *tile != ".".to_string() {
                count += 1;
            }
        }
    }
    count
}

fn remove_non_ending_steps(matrix: &mut Vec<Vec<String>>, max_steps: i32) {
    let max_step_str = max_steps.to_string();

    for row in matrix.iter_mut() {
        for tile in row.iter_mut() {
            if *tile != "#".to_string() && *tile != max_step_str {
                *tile = ".".to_string();
            }
        }
    }
}

fn update_tile(matrix: &mut Vec<Vec<String>>, tile: Tile, current_step: i32) {
    if matrix[tile.y][tile.x] != "#".to_string() {
        matrix[tile.y][tile.x] = current_step.to_string();
    }
}

fn filter_tiles(matrix: &Vec<Vec<String>>, adjacent_tiles: Vec<Tile>) -> Vec<Tile> {
    let mut filtered_tiles = Vec::new();
    for adj_tile in adjacent_tiles.iter() {
        if matrix[adj_tile.y][adj_tile.x] != "#" {
            filtered_tiles.push(*adj_tile);
        }
    }
    filtered_tiles
}

fn get_adjacent_tiles(matrix: &Vec<Vec<String>>, tile: Tile) -> Vec<Tile> {
    let mut adjecent_tiles = Vec::new();
    let x = tile.x;
    let y = tile.y;
    if x > 0 {
        adjecent_tiles.push(Tile { x: x - 1, y });
    }
    if x < matrix[0].len() - 1 {
        adjecent_tiles.push(Tile { x: x + 1, y });
    }
    if y > 0 {
        adjecent_tiles.push(Tile { x, y: y - 1 });
    }
    if y < matrix.len() - 1 {
        adjecent_tiles.push(Tile { x, y: y + 1 });
    }
    adjecent_tiles
}

fn get_starting_location(matrix: &Vec<Vec<String>>) -> Tile {
    for (y, row) in matrix.iter().enumerate() {
        for (x, tile) in row.iter().enumerate() {
            if *tile == "S" {
                return Tile { y, x };
            }
        }
    }
    panic!("at the disco");
}

#[derive(Debug, Clone, Copy)]
struct Tile {
    x: usize,
    y: usize,
}

fn parse_input_to_matrix(input: String) -> Vec<Vec<String>> {
    input
        .lines()
        .map(|line| line.trim().chars().map(|c| c.to_string()).collect())
        .collect()
}
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solution() {
        let input = "...........
        .....###.#.
        .###.##..#.
        ..#.#...#..
        ....#.#....
        .##..S####.
        .##..#...#.
        .......##..
        .##.#.####.
        .##..##.##.
        ..........."
            .to_string();

        assert_eq!(solution(input, 6), 16);
    }
}
