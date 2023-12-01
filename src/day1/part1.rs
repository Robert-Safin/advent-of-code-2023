pub fn solution(input: String) -> i32 {
    let mut sum: i32 = 0;
    input.split("\n").into_iter().for_each(|line| {
        let mut numbers_in_line: Vec<i32> = Vec::new();
        for char in line.chars() {
            if char.is_numeric() {
                numbers_in_line.push(char.to_digit(10).unwrap() as i32);
            }
        }
        let calibration_value: i32 = format!(
            "{}{}",
            numbers_in_line.first().unwrap_or(&0),
            numbers_in_line.last().unwrap_or(&0),
        )
        .parse::<i32>()
        .unwrap();
        sum += calibration_value;
    });
    println!("{}", sum);
    sum
}

#[cfg(test)]
mod test {
    use std::fs;

    use super::*;
    #[test]
    fn test() {
        let input = fs::read_to_string("src/inputs/day1.txt").unwrap();

        assert_eq!(solution(input), 54916)
    }
}
