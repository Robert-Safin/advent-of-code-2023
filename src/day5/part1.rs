pub fn solution(input: String) -> i64 {
    let blocks: Vec<_> = input.split("\n\n").collect();

    let seeds: Vec<i64> = match parse_seeds(&blocks[0]) {
        Some(seeds) => seeds,
        None => {
            return -1;
        }
    };

    let mut maps: Vec<Vec<((i64, i64), i64)>> = Vec::new();

    for block in &blocks[1..=7] {
        let map: Vec<((i64, i64), i64)> = parse_map_to_ranges(block);
        maps.push(map);
    }

    let smallest: i64 = seeds
        .iter()
        .map(|&seed| transform_through_stages_with_ranges(seed, &maps))
        .min()
        .unwrap_or(-1);

    println!("Smallest: {}", smallest);
    smallest
}

fn parse_seeds(input: &str) -> Option<Vec<i64>> {
    input
        .split(": ")
        .nth(1)?
        .split_whitespace()
        .map(|s| s.parse().ok())
        .collect()
}

fn parse_map_to_ranges(input: &str) -> Vec<((i64, i64), i64)> {
    input
        .split(":\n")
        .skip(1)
        .next()
        .unwrap()
        .split('\n')
        .filter_map(|line| {
            let parts: Vec<i64> = line
                .split_whitespace()
                .filter_map(|num| num.parse().ok())
                .collect();
            if parts.len() == 3 {
                let (start_dest, start_source, length) = (parts[0], parts[1], parts[2]);
                Some(((start_source, start_source + length), start_dest))
            } else {
                None
            }
        })
        .collect()
}

fn transform_through_stages_with_ranges(seed: i64, maps: &[Vec<((i64, i64), i64)>]) -> i64 {
    maps.iter()
        .fold(seed, |current, map| transform_with_range_map(current, map))
}

fn transform_with_range_map(seed: i64, map: &[((i64, i64), i64)]) -> i64 {
    map.iter()
        .find(|&&((start, end), _)| seed >= start && seed < end)
        .map_or(seed, |&((start, _), dest_start)| {
            dest_start + (seed - start)
        })
}

// #[cfg(test)]
// mod test {
//     use super::*;
//     #[test]
//     fn test_brief() {
//         let input = "seeds: 79 14 55 13

//       seed-to-soil map:
//       50 98 2
//       52 50 48

//       soil-to-fertilizer map:
//       0 15 37
//       37 52 2
//       39 0 15

//       fertilizer-to-water map:
//       49 53 8
//       0 11 42
//       42 0 7
//       57 7 4

//       water-to-light map:
//       88 18 7
//       18 25 70

//       light-to-temperature map:
//       45 77 23
//       81 45 19
//       68 64 13

//       temperature-to-humidity map:
//       0 69 1
//       1 0 69

//       humidity-to-location map:
//       60 56 37
//       56 93 4"
//             .to_string();
//         assert_eq!(solution(input), 35);
//     }
// }
