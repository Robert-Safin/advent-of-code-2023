use std::time::Instant;

pub fn solution(input: String) -> i64 {
  let start = Instant::now();
    let input: Vec<_> = input
        .lines()
        .map(|line| {
            let split: Vec<_> = line.split(":").collect();
            let num = split[1].trim().replace(" ", "");
            num.parse::<i64>().unwrap()
        })
        .collect();

    let race_time = input[0];
    let race_record = input[1];

    let mut possibilities: Vec<i64> = Vec::new();
    for i in 0..=race_time as i64 {
        let posibillity = (race_time - i) * i;
        possibilities.push(posibillity);
    }
    let mut count = 0;
    possibilities.iter().for_each(|val| {
        if val > &race_record {
            count += 1;
        }
    });
    let duration = start.elapsed(); // End timing

    println!("Time elapsed in some_function() is: {:?}", duration);
    println!("count: {}", count);
    count
}

// #[cfg(test)]
// mod test {
//     use super::*;
//     #[test]
//     fn test_brief() {
//         let input = "Time:      7  15   30
//     Distance:  9  40  200"
//             .to_string();
//         assert_eq!(solution(input), 71503);
//     }
// }
