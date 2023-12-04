pub fn solution(input: String) -> i32 {
    let mut card_count: i32 = 0;
    let lines: Vec<&str> = input.lines().collect();
    let inputs: Vec<&str> = lines
        .iter()
        .map(|x| x.split(":").collect::<Vec<&str>>()[1].trim())
        .collect();
    let parts: Vec<Vec<&str>> = inputs
        .iter()
        .map(|x| x.split("|").collect::<Vec<&str>>())
        .collect();

    let mut cards_todo: Vec<i32> = (0..parts.len() as i32).collect();

    while let Some(current_card_index) = cards_todo.pop() {
        card_count += 1;

        let current_card = parts[current_card_index as usize]
            .iter()
            .map(|x| x.trim())
            .collect::<Vec<&str>>();
        let current_card_score = calc_score(&current_card);

        for i in 1..=current_card_score {
            let next_card_index = current_card_index + i;
            if next_card_index < parts.len() as i32 && !cards_todo.contains(&next_card_index) {
                cards_todo.push(next_card_index);
            }
        }
    }
    card_count
}

fn calc_score(card: &Vec<&str>) -> i32 {
    let mut number_matches = 0;
    let winning_numbers = card[0]
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let scratch_numbers = card[1]
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    for win_num in &winning_numbers {
        for scratch_num in &scratch_numbers {
            if win_num == scratch_num {
                number_matches += 1;
            }
        }
    }
    number_matches
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
        assert_eq!(solution(input), 30);
    }
}
