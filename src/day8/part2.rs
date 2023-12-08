use std::collections::HashMap;

pub fn solution(input: String) -> i64 {
    let inputs: Vec<_> = input.split("\n\n").collect();
    let instructions: Vec<_> = inputs[0].chars().collect();
    let map: HashMap<_, _> = inputs[1]
        .split("\n")
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|line| {
            let split = line.split('=').collect::<Vec<_>>();
            let origin = split[0].trim().to_owned();
            let directions = split[1]
                .trim()
                .replace(",", "")
                .replace("(", "")
                .replace(")", "");
            let directions_split: Vec<_> = directions.split_whitespace().collect();
            let left_direction = directions_split[0].to_owned();
            let right_direction = directions_split[1].to_owned();

            (origin, (left_direction, right_direction))
        })
        .collect();

    let starting_nodes: Vec<_> = map
        .keys()
        .filter(|&origin| origin.ends_with('A'))
        .cloned()
        .collect();

    let mut cycles: Vec<i64> = Vec::new();

    for node in starting_nodes {
        let mut next_origin = node;
        let mut step_count = 0;

        for instruction in instructions.iter().cycle() {
            if next_origin.ends_with('Z') {
                break;
            }
            step_count += 1;
            let tuple = map.iter().find(|tuple| tuple.0 == &next_origin).unwrap();
            next_origin = if instruction == &'L' {
                tuple.1 .0.clone()
            } else {
                tuple.1 .1.clone()
            };
        }
        cycles.push(step_count)
    }

    let result: i64 = lcm_of_vec(&cycles);
    println!("{result}");
    result
}

fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a.abs()
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: i64, b: i64) -> i64 {
    a.abs() / gcd(a, b) * b.abs()
}

fn lcm_of_vec(numbers: &[i64]) -> i64 {
    numbers.iter().fold(1, |l, &n| lcm(l, n))
}

// #[cfg(test)]
// mod test {
//     use super::*;
//     #[test]
//     fn test_brief() {
//         let input = "LR

//     11A = (11B, XXX)
//     11B = (XXX, 11Z)
//     11Z = (11B, XXX)
//     22A = (22B, XXX)
//     22B = (22C, 22C)
//     22C = (22Z, 22Z)
//     22Z = (22B, 22B)
//     XXX = (XXX, XXX)"
//             .to_string();
//         assert_eq!(solution(input), 6);
//     }
// }
