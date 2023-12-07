use std::collections::HashMap;
#[derive(Debug)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

pub fn solution(input: String) -> i32 {
    let mut sum = 0;
    let tuples = input
        .lines()
        .map(|line| {
            let mut parts = line.split_whitespace();
            let str_part = parts.next().unwrap().to_string();
            let int_part = parts.next().unwrap().parse::<i32>().unwrap();
            (str_part, int_part)
        })
        .collect::<Vec<_>>();

    let mut fives: Vec<(String, i32)> = Vec::new();
    let mut fours: Vec<(String, i32)> = Vec::new();
    let mut full_houses: Vec<(String, i32)> = Vec::new();
    let mut thees: Vec<(String, i32)> = Vec::new();
    let mut twos: Vec<(String, i32)> = Vec::new();
    let mut one: Vec<(String, i32)> = Vec::new();
    let mut highs: Vec<(String, i32)> = Vec::new();

    for tuple in tuples {
        let hand_type: HandType = determine_type(&tuple);
        match hand_type {
            HandType::FiveOfAKind => fives.push(tuple),
            HandType::FourOfAKind => fours.push(tuple),
            HandType::FullHouse => full_houses.push(tuple),
            HandType::ThreeOfAKind => thees.push(tuple),
            HandType::TwoPair => twos.push(tuple),
            HandType::OnePair => one.push(tuple),
            HandType::HighCard => highs.push(tuple),
        }
    }

    let sorted_fives = sort_type_group(fives);
    let sorted_fours = sort_type_group(fours);
    let sorted_full_houses = sort_type_group(full_houses);
    let sorted_thees = sort_type_group(thees);
    let sorted_twos = sort_type_group(twos);
    let sorted_one = sort_type_group(one);
    let sorted_highs = sort_type_group(highs);

    let mut joined: Vec<(String, i32)> = Vec::new();
    joined.extend(sorted_fives);
    joined.extend(sorted_fours);
    joined.extend(sorted_full_houses);
    joined.extend(sorted_thees);
    joined.extend(sorted_twos);
    joined.extend(sorted_one);
    joined.extend(sorted_highs);

    joined.reverse();

    joined.iter().enumerate().for_each(|(tuple_i, tuple)| {
        let bid_value = tuple.1;
        let score = bid_value * (tuple_i + 1) as i32;
        sum += score;
    });
    println!("sum: {}", sum);

    sum
}

fn determine_type(tuple: &(String, i32)) -> HandType {
    let chars: Vec<char> = tuple.0.chars().collect::<Vec<_>>();

    let mut map: HashMap<char, i32> = HashMap::new();

    chars.iter().for_each(|char| {
        let count: &mut i32 = map.entry(*char).or_insert(0);
        *count += 1;
    });

    let mut values: Vec<&i32> = map.values().collect::<Vec<_>>();
    values.sort();
    values.reverse();

    let mut hand_type = HandType::HighCard;
    match values[0] {
        5 => hand_type = HandType::FiveOfAKind,
        4 => hand_type = HandType::FourOfAKind,
        3 => {
            if values[1] == &2 {
                hand_type = HandType::FullHouse
            } else {
                hand_type = HandType::ThreeOfAKind
            }
        }
        2 => {
            if values[1] == &2 {
                hand_type = HandType::TwoPair
            } else {
                hand_type = HandType::OnePair
            }
        }
        _ => hand_type = HandType::HighCard,
    }
    hand_type
}

fn sort_type_group(mut group: Vec<(String, i32)>) -> Vec<(String, i32)> {
    group.sort_by(|a, b| {
        for (char_a, char_b) in a.0.chars().zip(b.0.chars()) {
            let prio_a = char_priority(char_a);
            let prio_b = char_priority(char_b);
            if prio_a != prio_b {
                return prio_a.cmp(&prio_b);
            }
        }
        a.0.len().cmp(&b.0.len())
    });

    group
}

fn char_priority(c: char) -> i32 {
    match c {
        'A' => 1,
        'K' => 2,
        'Q' => 3,
        'J' => 4,
        'T' => 5,
        '9' => 6,
        '8' => 7,
        '7' => 8,
        '6' => 9,
        '5' => 10,
        '4' => 11,
        '3' => 12,
        '2' => 13,
        _ => 14,
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_brief() {
        let input = "32T3K 765
      T55J5 684
      KK677 28
      KTJJT 220
      QQQJA 483"
            .to_string();
        assert_eq!(solution(input), 6440);
    }
}
