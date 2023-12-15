pub fn solution(input: String) -> i32 {
    let mut sum = 0;

    let input = parse_input(input);

    for seq in input {
        let mut current_value: i32 = 0;
        for c in seq.chars() {
            if c != '\n' {
                let ascii_value = c as u8;
                current_value += ascii_value as i32;
                current_value *= 17;
                current_value %= 256;
            }
        }
        sum += current_value;
    }
    println!("{sum}");
    sum
}

fn parse_input(input: String) -> Vec<String> {
    let out: Vec<String> = input.split(",").map(|v| v.to_string()).collect();
    out
}

// #[cfg(test)]
// mod test {
//     use super::*;
//     #[test]
//     fn test_brief() {
//         let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7".to_string();
//         assert_eq!(solution(input), 1320);
//     }
// }
