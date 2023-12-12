use std::collections::HashMap;
pub fn solution(input: String) -> i64 {
  let mut total: i64 = 0;
  let input_str = input.as_str();
  for line in input_str.lines() {
      let trimmed = line.trim();
      if trimmed.is_empty() {
          continue;
      }
      let parts = trimmed.split_whitespace().collect::<Vec<_>>();
      let mut _masked = parts[0].chars().collect::<Vec<_>>();
      _masked.push('?');
      let mut masked = _masked
          .iter()
          .cycle()
          .take(_masked.len() * 5)
          .copied()
          .collect::<Vec<_>>();
      masked.pop();
      let _nums = parts[1]
          .split(',')
          .map(|n| n.parse::<usize>().unwrap())
          .collect::<Vec<_>>();
      let nums = _nums
          .iter()
          .cycle()
          .take(_nums.len() * 5)
          .copied()
          .collect::<Vec<_>>();
      let sum: usize = nums.iter().sum();
      let space = masked.len() + 1 - sum - nums.len();
      total += rec(&masked, 0, &nums, vec![], space, HashMap::new()).0;
  }
  println!("{}", total);
  total
}

fn match_prefix(masked: &Vec<char>, val: &Vec<char>) -> bool {
    let mut all = true;
    for i in 0..val.len() {
        if !(masked[i] == '?' || masked[i] == val[i]) {
            all = false;
        }
    }
    all
}

fn clean(mut curr: Vec<char>, len: usize) -> Vec<char> {
    while curr.len() > len {
        curr.pop();
    }
    curr
}

fn pushn(mut curr: Vec<char>, c: char, n: usize) -> Vec<char> {
    for _ in 0..n {
        curr.push(c);
    }
    curr
}

fn rec(
    masked: &Vec<char>,
    i: usize,
    nums: &Vec<usize>,
    mut curr: Vec<char>,
    rem: usize,
    mut memo: HashMap<(String, usize), i64>,
) -> (i64, Vec<char>, HashMap<(String, usize), i64>) {
    let tail = masked
        .iter()
        .rev()
        .take(masked.len() - curr.len())
        .cloned()
        .collect::<String>();
    if let Some(&val) = memo.get(&(tail.clone(), i)) {
        return (val, curr, memo);
    }
    let min = if i == 0 || i == nums.len() { 0 } else { 1 };
    let starting = curr.len();
    if i < nums.len() {
        let mut total = 0;
        for n in 0..rem + 1 {
            curr = pushn(curr, '.', min + n);
            curr = pushn(curr, '#', nums[i]);
            if !match_prefix(masked, &curr) {
                curr = clean(curr, starting);
                continue;
            }
            let (matches, curr2, memo2) = rec(masked, i + 1, nums, curr, rem - n, memo);
            total += matches;
            memo = memo2;
            curr = clean(curr2, starting)
        }
        memo.insert((tail, i), total);
        (total, curr, memo)
    } else {
        assert!(masked.len() - curr.len() == rem);
        curr = pushn(curr, '.', rem);
        let m = if match_prefix(masked, &curr) { 1 } else { 0 };
        memo.insert((tail, i), m);
        (m, clean(curr, starting), memo)
    }
}


#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_brief() {
        let input =  "#.#.### 1,1,3
        .#...#....###. 1,1,3
        .#.###.#.###### 1,3,1,6
        ####.#...#... 4,1,1
        #....######..#####. 1,6,5
        .###.##....# 3,2,1"
            .to_string();
        assert_eq!(solution(input), 525152);
    }
}
