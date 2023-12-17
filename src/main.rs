use advent_of_code_2023::day17;

fn main() {
    let input: String = std::fs::read_to_string("src/inputs/day17.txt").unwrap();
    day17::part2::solution(&input);
}
