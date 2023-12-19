pub fn solution(input: String) -> i32 {
    let split: Vec<String> = input
        .split("\n\n")
        .map(|line| line.to_string())
        .collect::<Vec<_>>();

    let workflows: Vec<Workflow> = parse_workflows(&split[0]);
    let mut parts: Vec<Part> = parse_parts(&split[1]);

    for part in parts.iter_mut() {
        while part.current_location != "A" && part.current_location != "R" {
            send_part(part, &workflows);
        }
    }

    let sum: i32 = count_approved_sums(&parts);
    println!("{sum}");
    sum
}

fn count_approved_sums(parts: &Vec<Part>) -> i32 {
    let mut sum: i32 = 0;
    for part in parts.iter() {
        if part.current_location == "A" {
            sum += part.x + part.m + part.a + part.s;
        }
    }
    sum
}

fn send_part(part: &mut Part, workflows: &Vec<Workflow>) {
    let current_location: &String = &part.current_location;
    let current_workflow: &Workflow = workflows
        .iter()
        .find(|w| w.name == *current_location)
        .unwrap();

    let mut condition_met: bool = false;
    for condition in current_workflow.conditions.iter() {
        match condition.property {
            'x' => {
                if condition.operator == '<' && part.x < condition.value {
                    part.current_location = condition.destination.clone();
                    condition_met = true;
                    break;
                } else if condition.operator == '>' && part.x > condition.value {
                    part.current_location = condition.destination.clone();
                    condition_met = true;
                    break;
                }
            }
            'm' => {
                if condition.operator == '<' && part.m < condition.value {
                    part.current_location = condition.destination.clone();
                    condition_met = true;
                    break;
                } else if condition.operator == '>' && part.m > condition.value {
                    part.current_location = condition.destination.clone();
                    condition_met = true;
                    break;
                }
            }
            'a' => {
                if condition.operator == '<' && part.a < condition.value {
                    part.current_location = condition.destination.clone();
                    condition_met = true;
                    break;
                } else if condition.operator == '>' && part.a > condition.value {
                    part.current_location = condition.destination.clone();
                    condition_met = true;
                    break;
                }
            }

            's' => {
                if condition.operator == '<' && part.s < condition.value {
                    part.current_location = condition.destination.clone();
                    condition_met = true;
                    break;
                } else if condition.operator == '>' && part.s > condition.value {
                    part.current_location = condition.destination.clone();
                    condition_met = true;
                    break;
                }
            }
            _ => panic!("Unknown property"),
        };
    }

    if condition_met == false {
        part.current_location = current_workflow.else_clause.clone();
    }
}

#[derive(Debug)]
struct Workflow {
    name: String,
    conditions: Vec<Condition>,
    else_clause: String,
}
#[derive(Debug)]
struct Condition {
    property: char,
    operator: char,
    value: i32,
    destination: String,
}
#[derive(Debug, Clone)]
struct Part {
    x: i32,
    m: i32,
    a: i32,
    s: i32,
    current_location: String,
}

fn parse_workflows(workflow_split: &String) -> Vec<Workflow> {
    let workflows: Vec<String> = workflow_split
        .lines()
        .map(|l| l.trim().to_string())
        .collect();

    let mut workflows_parsed: Vec<Workflow> = vec![];

    for flow in workflows {
        let flow_name: String = flow.split("{").nth(0).unwrap().to_string();
        let flow_conditions: String = flow.split("{").nth(1).unwrap().to_string().replace("}", "");
        let mut flow_conditions: Vec<_> = flow_conditions.split(",").collect();
        let else_clause: String = flow_conditions.pop().unwrap().to_string();
        let mut workflow: Workflow = Workflow {
            name: flow_name,
            conditions: vec![],
            else_clause: else_clause,
        };
        flow_conditions.iter().for_each(|cond| {
            let property: char = cond.chars().nth(0).unwrap();
            let operator: char = cond.chars().nth(1).unwrap();
            let num_and_dest_string: String = cond.chars().skip(2).collect::<String>();
            let num_and_dest_split: Vec<&str> = num_and_dest_string.split(":").collect::<Vec<_>>();
            let value: &str = num_and_dest_split[0];
            let destination: &str = num_and_dest_split[1];
            let condition: Condition = Condition {
                property: property,
                operator: operator,
                value: value.parse::<i32>().unwrap(),
                destination: destination.to_string(),
            };

            workflow.conditions.push(condition);
        });
        workflows_parsed.push(workflow);
    }
    workflows_parsed
}

fn parse_parts(parts_split: &String) -> Vec<Part> {
    let mut parts: Vec<Part> = vec![];

    for line in parts_split.lines() {
        let line: String = line.trim().to_string().replace("{", "").replace("}", "");
        let line_split: Vec<_> = line.split(",").collect();
        let mut part: Part = Part {
            x: 0,
            m: 0,
            a: 0,
            s: 0,
            current_location: "in".to_string(),
        };
        for prop in line_split {
            let prop_split: Vec<_> = prop.split("=").collect();
            let property: &str = prop_split[0];
            let value: &str = prop_split[1];

            match property {
                "x" => part.x = value.parse::<i32>().unwrap().clone(),
                "m" => part.m = value.parse::<i32>().unwrap().clone(),
                "a" => part.a = value.parse::<i32>().unwrap().clone(),
                "s" => part.s = value.parse::<i32>().unwrap().clone(),
                _ => panic!("Unknown property"),
            }
        }
        parts.push(part);
    }
    parts
}

// #[cfg(test)]
// mod test {
//     use super::*;

//     #[test]
//     fn test_solution() {
//         let input = "px{a<2006:qkq,m>2090:A,rfg}
//       pv{a>1716:R,A}
//       lnx{m>1548:A,A}
//       rfg{s<537:gd,x>2440:R,A}
//       qs{s>3448:A,lnx}
//       qkq{x<1416:A,crn}
//       crn{x>2662:A,R}
//       in{s<1351:px,qqz}
//       qqz{s>2770:qs,m<1801:hdj,R}
//       gd{a>3333:R,R}
//       hdj{m>838:A,pv}

//       {x=787,m=2655,a=1222,s=2876}
//       {x=1679,m=44,a=2067,s=496}
//       {x=2036,m=264,a=79,s=2244}
//       {x=2461,m=1339,a=466,s=291}
//       {x=2127,m=1623,a=2188,s=1013}"
//             .to_string();

//         assert_eq!(solution(input), 19114);
//     }
// }
