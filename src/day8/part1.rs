pub fn solution(input: String) -> i32 {
    let mut sum = 0;

    let inputs: Vec<_> = input.split("\n\n").collect();
    let instructions: Vec<_> = inputs[0].chars().collect();
    let map: Vec<_> = inputs[1]
        .split("\n")
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|line| {
            let split = line.split("=").collect::<Vec<_>>();
            let origin = split[0].trim().to_string();
            let directions = split[1]
                .trim()
                .replace(",", "")
                .replace("(", "")
                .replace(")", "");
            let directions_split: Vec<_> = directions.split_whitespace().collect();
            let left_direction = directions_split[0].to_string();
            let right_direction = directions_split[1].to_string();

            (origin, (left_direction, right_direction))
        })
        .collect();

    let mut next_origin = "AAA";

    for instruction in instructions.iter().cycle() {
        sum += 1;
        let tuple = map.iter().find(|tuple| tuple.0 == next_origin).unwrap();
        next_origin = if instruction == &'L' {
            &tuple.1 .0
        } else {
            &tuple.1 .1
        };

        if next_origin == "ZZZ" {
            break;
        }
    }
    println!("{sum}");
    sum
}

// #[cfg(test)]
// mod test {
//     use super::*;
//     #[test]
//     fn test_brief() {
//         let input = "LLR

//       AAA = (BBB, BBB)
//       BBB = (AAA, ZZZ)
//       ZZZ = (ZZZ, ZZZ)"
//             .to_string();
//         assert_eq!(solution(input), 6);
//     }
// }
