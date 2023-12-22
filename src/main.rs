use advent_of_code_2023::day21;

fn main() {
    let input: String = std::fs::read_to_string("src/inputs/day21.txt").unwrap();
    day21::part1::solution(input, 64);
}
