pub fn solution(input: String) -> i32 {
    let mut winning_ways: Vec<i32> = Vec::new();

    let input: Vec<_> = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .filter_map(|word| word.parse::<i32>().ok())
                .collect::<Vec<_>>()
        })
        .collect();

    let race_durations = &input[0];
    let race_records = &input[1];

    race_durations
        .into_iter()
        .enumerate()
        .for_each(|(race_i, race)| {
            let mut race_distances: Vec<i32> = Vec::new();

            for i in 0..=*race as i32 {
                let distance = (race - i) * i;
                race_distances.push(distance);
            }

            let mut count = 0;
            race_distances.iter().for_each(|val| {
                if val > &race_records[race_i] {
                    count += 1;
                }
            });
            winning_ways.push(count)
        });

    let factor = winning_ways.iter().fold(1, |acc, x| acc * x);
    println!("Factor: {}", factor);
    factor
}

// #[cfg(test)]
// mod test {
//     use super::*;
//     #[test]
//     fn test_brief() {
//         let input = "Time:      7  15   30
//       Distance:  9  40  200"
//             .to_string();
//         assert_eq!(solution(input), 288);
//     }
// }
