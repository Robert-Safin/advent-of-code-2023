pub fn solution(input: String) -> i64 {
    let mut sum: i64 = 0;

    let input: Vec<Vec<String>> = parse_input(input);

    for line in input.iter() {
        let cogs_to_match: i64 = cogs_left(&line);
        let string: &String = &line[0];
        let possibilities: Vec<String> = generate_possibilities(&string, cogs_to_match);

        let mut valid_count: i64 = 0;
        for possibility in possibilities.iter() {
            if is_valid_config(&possibility, &line[1]) {
                valid_count += 1;
            }
        }
        sum += valid_count;
    }
    println!("{sum}");
    sum
}

fn generate_possibilities(string: &str, cog_count: i64) -> Vec<String> {
    let mut possibilities: Vec<String> = Vec::new();
    let question_marks: i64 = string.chars().filter(|&c| c == '?').count() as i64;
    let dot_count: i64 = question_marks - cog_count;

    generate_possibilities_helper(
        string,
        cog_count,
        dot_count,
        0,
        &mut String::new(),
        &mut possibilities,
    );
    possibilities
}

fn generate_possibilities_helper(
    string: &str,
    cog_count: i64,
    dot_count: i64,
    index: usize,
    current: &mut String,
    possibilities: &mut Vec<String>,
) {
    if index == string.len() {
        possibilities.push(current.clone());
        return;
    }

    match string.chars().nth(index).unwrap() {
        '?' => {
            if cog_count > 0 {
                current.push('#');
                generate_possibilities_helper(
                    string,
                    cog_count - 1,
                    dot_count,
                    index + 1,
                    current,
                    possibilities,
                );
                current.pop();
            }
            if dot_count > 0 {
                current.push('.');
                generate_possibilities_helper(
                    string,
                    cog_count,
                    dot_count - 1,
                    index + 1,
                    current,
                    possibilities,
                );
                current.pop();
            }
        }
        other => {
            current.push(other);
            generate_possibilities_helper(
                string,
                cog_count,
                dot_count,
                index + 1,
                current,
                possibilities,
            );
            current.pop();
        }
    }
}

fn is_valid_config(possibility: &String, config_str: &str) -> bool {
    let groups: Vec<&str> = possibility.split('.').filter(|&s| !s.is_empty()).collect();
    let config: Vec<usize> = config_str
        .split(',')
        .filter_map(|s| s.parse::<usize>().ok())
        .collect();

    if groups.len() != config.len() {
        return false;
    }

    for (group, &size) in groups.iter().zip(config.iter()) {
        if group.len() != size {
            return false;
        }
    }

    true
}

fn cogs_left(configuration: &Vec<String>) -> i64 {
    let cog_string: &String = &configuration[0];
    let config_string: &String = &configuration[1];

    let config_count: i64 = config_string
        .split(',')
        .filter_map(|s| s.parse::<i64>().ok())
        .sum();

    let broken_cog_count: i64 = cog_string.chars().filter(|&c| c == '#').count() as i64;

    let y = config_count - broken_cog_count;

    y as i64
}

fn parse_input(input: String) -> Vec<Vec<String>> {
    let lines: Vec<_> = input.lines().map(|line| line.trim()).collect();

    let lines: Vec<Vec<String>> = lines
        .iter()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|s| s.to_string())
                .collect::<Vec<String>>()
        })
        .collect();

    lines
}

// #[cfg(test)]
// mod test {
//     use super::*;
//     #[test]
//     fn test_brief() {
//         let input =  "?###??????????###??????????###??????????###???????? 3,2,1,3,2,1,3,2,1"
//             .to_string();
//         assert_eq!(solution(input), 2250);
//     }
// }
