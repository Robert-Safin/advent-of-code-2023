use std::num::NonZeroUsize;

pub struct Grid {
    data: Box<[u8]>,
    offset: usize,
}
impl Grid {
    fn from_str(s: &str) -> Self {
        let mut lines = s.lines().peekable();
        let line_len = lines.peek().map_or(0, |l| l.len());
        Self {
            data: lines.flat_map(str::as_bytes).map(|&c| c - b'0').collect::<Box<_>>(),
            offset: line_len,
        }
    }
    const fn next_pos(&self, p: usize, dir: u8) -> Option<usize> {
        // 0 = up, 1 = right, 2 = down, 3 = left
        Some(match dir {
            0 if p > self.offset => p - self.offset,
            1 if (p + 1) % self.offset != 0 => p + 1,
            2 if p < self.data.len() - self.offset => p + self.offset,
            3 if p % self.offset != 0 => p - 1,
            _ => { return None }
        })
    }
    fn run(&self, dmin: usize, dmax: usize) -> Option<NonZeroUsize> {
        use std::cmp::Reverse as Rev;
        let lp = self.data.len() - 1;
        let mut visit = vec![0u8; self.data.len()];
        // Cache for vertical and horizontal directions
        let mut ccache = vec![usize::MAX; 2 * self.data.len()];
        let mut q = std::collections::BinaryHeap::new();
        q.push((Rev(0), 0, 0));
        q.push((Rev(0), 0, 1));
        while let Some((Rev(cost), p, dir)) = q.pop() {
            if p == lp {
                return NonZeroUsize::new(cost);
            }
            if visit[p] & (1u8 << dir) != 0 {
                continue
            }
            visit[p] |= 1u8 << dir;
            let odir = dir ^ 1;
            for nd in [odir, odir ^ 2] {
                let mut costsum = 0;
                let mut np = p;
                for dist in 1..=dmax {
                    if let Some(op) = self.next_pos(np, nd) {
                        costsum += self.data[op] as usize;
                        if dist >= dmin {
                            let ncost = cost + costsum;
                            let cache_idx = (op << 1) | odir as usize;
                            if ccache[cache_idx] > ncost {
                                ccache[cache_idx] = ncost;
                                q.push((Rev(ncost), op, odir));
                            }
                        }
                        np = op;
                    }
                }
            }
        }
        None
    }
}

pub fn solve(input: &str, dmin: usize, dmax: usize) -> Option<NonZeroUsize> {
    Grid::from_str(input).run(dmin, dmax)
}


pub fn solution(input: &str) -> Option<NonZeroUsize> {
    let out = solve(input, 4, 10);
    println!("part2: {:?}", out);
    out
}

// #[cfg(test)]
// mod test {
//     use super::*;

//     #[test]
//     fn test_solution() {
//         let input = "2413432311323
//                      3215453535623
//                      3255245654254
//                      3446585845452
//                      4546657867536
//                      1438598798454
//                      4457876987766
//                      3637877979653
//                      4654967986887
//                      4564679986453
//                      1224686865563
//                      2546548887735
//                      4322674655533";

//         assert_eq!(solution(input), NonZeroUsize::new(102));
//     }
// }
