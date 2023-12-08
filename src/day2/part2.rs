pub fn solution(input: String) -> i32 {
    let mut sum = 0;
    input.lines().for_each(|line| {
        let parts: Vec<&str> = line.splitn(2, ':').collect::<Vec<&str>>();
        let sets = parts[1].trim();

        let mut max_red: i32 = 0;
        let mut max_green: i32 = 0;
        let mut max_blue: i32 = 0;
        sets.split(";").for_each(|set| {
            set.trim().split(",").for_each(|pull| {
                let pair: Vec<&str> = pull.trim().split_whitespace().collect::<Vec<&str>>();
                let number_of_cubes = pair[0].parse::<i32>().unwrap();
                let color = pair[1];

                match color {
                    "red" if number_of_cubes > max_red => max_red = number_of_cubes,
                    "green" if number_of_cubes > max_green => max_green = number_of_cubes,
                    "blue" if number_of_cubes > max_blue => max_blue = number_of_cubes,
                    _ => (),
                }
            });
        });

        let power = max_red * max_green * max_blue;
        sum += power;
    });
    println!("Sum: {}", sum);
    sum
}

// #[cfg(test)]
// mod test {
//     use super::*;
//     #[test]
//     fn test_brief() {
//         let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
//       Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
//       Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
//       Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
//       Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
//             .to_string();
//         assert_eq!(solution(input), 2286);
//     }
// }
