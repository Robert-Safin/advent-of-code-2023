pub fn solution(input: String) -> i32 {
    let mut sum: i32 = 0;
    let input: Vec<_> = input
        .lines()
        .map(|l| {
            l.trim()
                .split_whitespace()
                .map(|n| n.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect();

    for line in input {
        let prediction = extrapolate_next_value(line);
        sum += prediction;
    }

    println!("{}", sum);

    sum
}
fn extrapolate_next_value(input: Vec<i32>) -> i32 {
    let mut matrix: Vec<Vec<i32>> = vec![input];

    while matrix.last().unwrap().len() > 1 {
        let last_row: &Vec<i32> = matrix.last().unwrap();
        let new_row: Vec<i32> = last_row.windows(2).map(|w| w[1] - w[0]).collect();
        matrix.push(new_row);
    }

    for i in (1..matrix.len()).rev() {
        let last_value: i32 = *matrix[i].last().unwrap();
        let new_value: i32 = matrix[i - 1].last().unwrap() + last_value;
        matrix[i - 1].push(new_value);
    }
    println!("{:#?}", matrix);
    *matrix[0].last().unwrap()
}
// #[cfg(test)]
// mod test {
//     use super::*;
//     #[test]
//     fn test_brief() {
//         let input = "0 3 6 9 12 15
//         1 3 6 10 15 21
//         10 13 16 21 30 45"
//             .to_string();
//         assert_eq!(solution(input), 114);
//     }
// }
