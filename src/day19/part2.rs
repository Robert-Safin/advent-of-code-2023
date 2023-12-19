use std::collections::HashMap;

use nom::bytes::complete::{is_not, tag, take_till};
use nom::character::complete::{digit1, one_of};
use nom::multi::separated_list1;
use nom::IResult;

pub fn solution(input: String) -> i64 {
    let out = process_part_2(&input);
    println!("{:?}", out);
    out as i64
}

fn process_part_2(input: &str) -> usize {
    let (operations, _) = input.split_once("\n\n").unwrap();
    let (_, operations) = parse_operations(operations).unwrap();

    let mut accepted_part_combination = 0;
    let mut accepted_paths = vec![];
    let mut queue = vec![(OperationType::Destination("in".to_string()), vec![])];
    while let Some((next, constraints)) = queue.pop() {
        if next == OperationType::Destination("A".to_string()) {
            accepted_paths.push(constraints);
            continue;
        }
        if next == OperationType::Destination("R".to_string()) {
            continue;
        }
        match next {
            OperationType::Destination(n) => {
                let mut inverted_constraints = constraints.clone();
                for o in operations.get(&n).unwrap() {
                    match o {
                        OperationType::Destination(_) => {
                            queue.push((o.clone(), inverted_constraints.clone()));
                        }
                        OperationType::Operation(oper) => {
                            queue.push((o.clone(), inverted_constraints.clone()));
                            let mut oper = oper.clone();
                            oper.operator = if oper.operator == Operator::LessThan {
                                oper.count -= 1;
                                Operator::GreaterThan
                            } else {
                                oper.count += 1;
                                Operator::LessThan
                            };
                            inverted_constraints.push(oper.clone());
                        }
                    }
                }
            }
            OperationType::Operation(o) => {
                let mut constraints = constraints.clone();
                constraints.push(o.clone());
                let dest = OperationType::Destination(o.destination);
                queue.push((dest, constraints.clone()));
            }
        }
    }

    for path in accepted_paths {
        let mut min_x = 1;
        let mut max_x = 4000;
        let mut min_m = 1;
        let mut max_m = 4000;
        let mut min_a = 1;
        let mut max_a = 4000;
        let mut min_s = 1;
        let mut max_s = 4000;

        for ele in path {
            let count = ele.count;
            match ele.part {
                'x' => {
                    if ele.operator == Operator::LessThan {
                        max_x = max_x.min(count - 1);
                    } else {
                        min_x = min_x.max(count + 1);
                    }
                }
                'm' => {
                    if ele.operator == Operator::LessThan {
                        max_m = max_m.min(count - 1);
                    } else {
                        min_m = min_m.max(count + 1);
                    }
                }
                'a' => {
                    if ele.operator == Operator::LessThan {
                        max_a = max_a.min(count - 1);
                    } else {
                        min_a = min_a.max(count + 1);
                    }
                }
                's' => {
                    if ele.operator == Operator::LessThan {
                        max_s = max_s.min(count - 1);
                    } else {
                        min_s = min_s.max(count + 1);
                    }
                }
                _ => panic!(),
            }
        }
        if min_x > max_x || min_m > max_m || min_a > max_a || min_s > max_s {
            continue;
        }
        let t =
            (max_x - min_x + 1) * (max_m - min_m + 1) * (max_a - min_a + 1) * (max_s - min_s + 1);
        accepted_part_combination += t;
    }
    accepted_part_combination
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum OperationType {
    Operation(Operation),
    Destination(String),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Operation {
    part: char,
    operator: Operator,
    count: usize,
    destination: String,
}

#[derive(Clone, Debug, Copy, PartialEq, Eq, Hash)]
enum Operator {
    LessThan,
    GreaterThan,
}

fn parse_operations(input: &str) -> IResult<&str, HashMap<String, Vec<OperationType>>> {
    let mut out = HashMap::new();
    let (input, operations) = separated_list1(tag("\n"), parse_operation)(input)?;
    for (name, o) in operations {
        out.insert(name.to_string(), o);
    }
    Ok((input, out))
}

fn parse_operation(input: &str) -> IResult<&str, (String, Vec<OperationType>)> {
    let (input, name) = take_till(|c| c == '{')(input)?;
    let (input, _) = tag("{")(input)?;
    let (input, operations) = take_till(|c| c == '}')(input)?;
    let (_, operations) = separated_list1(tag(","), parse_operation_type)(operations)?;
    let (input, _) = tag("}")(input)?;
    Ok((input, (name.to_string(), operations)))
}

fn parse_operation_type(input: &str) -> IResult<&str, OperationType> {
    if !input.contains(":") {
        return Ok(("", OperationType::Destination(input.to_string())));
    }
    let (input, part) = one_of("xmas")(input)?;
    let (input, operator) = one_of("<>")(input)?;
    let operator = if operator == '>' {
        Operator::GreaterThan
    } else {
        Operator::LessThan
    };
    let (input, count) = digit1(input)?;
    let (input, _) = tag(":")(input)?;
    let (input, destination) = is_not(",}")(input)?;
    let out = Operation {
        part,
        count: count.parse().unwrap(),
        operator,
        destination: destination.to_string(),
    };
    Ok((input, OperationType::Operation(out)))
}

// #[cfg(test)]
// mod test {
//     use super::*;

//     #[test]
//     fn test_solution() {
//         let input = "
//         px{a<2006:qkq,m>2090:A,rfg}
//         pv{a>1716:R,A}
//         lnx{m>1548:A,A}
//         rfg{s<537:gd,x>2440:R,A}
//         qs{s>3448:A,lnx}
//         qkq{x<1416:A,crn}
//         crn{x>2662:A,R}
//         in{s<1351:px,qqz}
//         qqz{s>2770:qs,m<1801:hdj,R}
//         gd{a>3333:R,R}
//         hdj{m>838:A,pv}

//         {x=787,m=2655,a=1222,s=2876}
//         {x=1679,m=44,a=2067,s=496}
//         {x=2036,m=264,a=79,s=2244}
//         {x=2461,m=1339,a=466,s=291}
//         {x=2127,m=1623,a=2188,s=1013}"
//             .to_string();

//         assert_eq!(solution(input), 167409079868000);
//     }
// }
