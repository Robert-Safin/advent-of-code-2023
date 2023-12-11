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
        let prediction = extrapolate_previous_value(line);
        sum += prediction;
    }

    println!("{}", sum);

    sum
}
fn extrapolate_previous_value(input: Vec<i32>) -> i32 {
    let mut matrix: Vec<Vec<i32>> = vec![input];

    while matrix.last().unwrap().len() > 1 {
        let last_row: &Vec<i32> = matrix.last().unwrap();
        let new_row: Vec<i32> = last_row.windows(2).map(|w| w[1] - w[0]).collect();
        matrix.push(new_row);
    }

    for i in (1..matrix.len()).rev() {
        matrix[i].insert(0, 0);
        let new_value = matrix[i - 1][0] - matrix[i][1];
        matrix[i - 1].insert(0, new_value);
    }

    matrix[0][0]
}
// #[cfg(test)]
// mod test {
//     use super::*;
//     #[test]
//     fn test_brief() {
//         let input = "10  13  16  21  30  45".to_string();
//         assert_eq!(solution(input), 5);
//     }
// }
