use std::collections::HashMap;

pub fn solution(input: String) -> i32 {
    let mut sum: i32 = 0;

    let mut box_hash: HashMap<i32, Vec<String>> = HashMap::new();

    let input: Vec<String> = parse_input(input);

    for seq in input {
        let box_info: SequenceInfo = get_sequence_info(seq);
        let entry: &mut Vec<String> = box_hash.entry(box_info.box_number).or_insert_with(Vec::new);

        if box_info.operator == '-' {
            entry.retain(|v: &String| v.split(" ").next() != Some(&box_info.label));
        }

        if box_info.operator == '=' {
            let mut found: bool = false;
            for lens in entry.iter_mut() {
                if lens.split(" ").next() == Some(&box_info.label) {
                    *lens = format!("{} {}", box_info.label, box_info.focal_strength.unwrap());
                    found = true;
                    break;
                }
            }
            if !found {
                let fmt: String =
                    format!("{} {}", box_info.label, box_info.focal_strength.unwrap());
                entry.push(fmt);
            }
        }
    }

    for (k, v) in box_hash {
        for (lens_i, lens) in v.iter().enumerate() {
            let order_score: i32 = (lens_i as i32) + 1;
            let box_score: i32 = k + 1;

            let split: Vec<&str> = lens.split(" ").collect();
            let focal_score: i32 = split[1].parse::<i32>().unwrap();

            let multiply: i32 = order_score * box_score * focal_score;

            sum += multiply;
        }
    }

    println!("sum: {}", sum);
    sum
}

#[derive(Debug)]
struct SequenceInfo {
    box_number: i32,
    operator: char,
    focal_strength: Option<i32>,
    label: String,
}

fn get_sequence_info(seq: String) -> SequenceInfo {
    if seq.contains("=") {
        let split: Vec<_> = seq.split("=").collect();
        let map = split[0];

        let mut current_value: i32 = 0;
        for c in map.chars() {
            let ascii_value = c as u8;
            current_value += ascii_value as i32;
            current_value *= 17;
            current_value %= 256;
        }

        SequenceInfo {
            box_number: current_value,
            operator: '=',
            focal_strength: Some(split[1].to_string().parse::<i32>().unwrap()),
            label: map.to_string(),
        }
    } else {
        let split: Vec<_> = seq.split("-").collect();
        let map = split[0];
        let mut current_value: i32 = 0;
        for c in map.chars() {
            let ascii_value = c as u8;
            current_value += ascii_value as i32;
            current_value *= 17;
            current_value %= 256;
        }

        SequenceInfo {
            box_number: current_value,
            operator: '-',
            focal_strength: None,
            label: map.to_string(),
        }
    }
}

fn parse_input(input: String) -> Vec<String> {
    let out: Vec<String> = input.split(",").map(|v| v.trim().to_string()).collect();
    out
}

// #[cfg(test)]
// mod test {
//     use super::*;
//     #[test]
//     fn test_brief() {
//         let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7".to_string();
//         assert_eq!(solution(input), 145);
//     }
// }
