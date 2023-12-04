pub fn solution(input: String) -> i32 {
    let mut sum: i32 = 0;
    let lines: Vec<&str> = input.lines().collect::<Vec<&str>>();
    let inputs: Vec<&str> = lines
        .iter()
        .map(|x| x.split(":").collect::<Vec<&str>>()[1].trim())
        .collect();
    let parts: Vec<Vec<&str>> = inputs
        .iter()
        .map(|x| x.split("|").collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>();

    for (line_index, line) in parts.iter().enumerate() {
        let mut number_matches = 0;
        let winning_numbers = line[0]
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        let scratch_numbers = line[1]
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        winning_numbers.iter().for_each(|win_num| {
            scratch_numbers.iter().for_each(|scratch_num| {
                if win_num == scratch_num {
                    number_matches += 1;
                }
            })
        });
        let mut sub_sum = 0;
        for _ in 0..number_matches {
            if sub_sum == 0 {
                sub_sum += 1;
            } else {
                sub_sum *= 2;
            }
        }
        sum += sub_sum;
    }
    println!("{}", sum);
    sum
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_brief() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
      Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
      Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
      Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
      Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
      Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"
            .to_string();
        assert_eq!(solution(input), 13);
    }
}
