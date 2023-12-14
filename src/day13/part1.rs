struct Grid2D {
  data: Vec<Vec<char>>,
}

impl Grid2D {
  fn from_lines(lines: &[String]) -> Self {
      let data = lines.iter().map(|line| line.chars().collect()).collect();
      Grid2D { data }
  }

  fn width(&self) -> usize {
      self.data.first().map_or(0, |row| row.len())
  }

  fn height(&self) -> usize {
      self.data.len()
  }

  fn col_iterator(&self, col: usize) -> impl Iterator<Item = &char> {
      self.data.iter().map(move |row| &row[col])
  }

  fn row_iterator(&self, row: usize) -> impl Iterator<Item = &char> {
      self.data[row].iter()
  }

  fn get(&self, x: usize, y: usize) -> Option<&char> {
      self.data.get(y).and_then(|row| row.get(x))
  }

  fn replace(&mut self, x: usize, y: usize, value: char) {
      if let Some(row) = self.data.get_mut(y) {
          if let Some(cell) = row.get_mut(x) {
              *cell = value;
          }
      }
  }
}

fn is_vert_reflect(g: &Grid2D, idx: usize) -> bool {
    let rightmost_mirror = idx;
    let leftmost_mirror = idx + 1;

    let foo: Vec<_> = (0..=rightmost_mirror).rev().zip(leftmost_mirror..g.width()).collect();

    foo.into_iter().all(|a| {
        let first = a.0;
        let second = a.1;

        let collection: Vec<_> = g.col_iterator(first).zip(g.col_iterator(second)).collect();

        collection.iter().all(|a| a.0 == a.1)
    })
}

fn is_horiz_reflect(g: &Grid2D, idx: usize) -> bool {
    let rightmost_mirror = idx;
    let leftmost_mirror = idx + 1;

    let foo: Vec<_> = (0..=rightmost_mirror).rev().zip(leftmost_mirror..g.height()).collect();
    foo.into_iter().all(|a| {
        let first = a.0;
        let second = a.1;

        g.row_iterator(first).zip(g.row_iterator(second)).all(|a| a.0 == a.1)
    })
}

fn smudge_score(grid: &mut Grid2D) -> u64 {
    let old_score = reflection_score(grid);
    let old_score_value = old_score.first().unwrap();

    for x in 0..grid.width() {
        for y in 0..grid.height() {
            let curr = *grid.get(x, y).unwrap();
            let new = match curr {
                '#' => '.',
                '.' => '#',
                _ => panic!("invalid grid item: {curr}")
            };

            grid.replace(x, y, new);
            let new_score = reflection_score(grid);
            if !new_score.is_empty() && new_score != old_score {
                return new_score.into_iter().find(|x| x != old_score_value).unwrap()
            }

            grid.replace(x, y, curr);
        }
    }

    todo!()
}

fn reflection_score(grid: &Grid2D) -> Vec<u64> {
    let mut scores = Vec::new();
    (0..grid.width() - 1).filter_map(|idx| {
        if is_vert_reflect(grid, idx) {
            Some(idx as u64 + 1)
        } else {
            None
        }
    }).for_each(|x| scores.push(x));

    (0..grid.height() - 1).filter_map(|idx| {
        if is_horiz_reflect(grid, idx) {
            Some((idx as u64 + 1) * 100)
        } else {
            None
        }
    }).for_each(|x| scores.push(x));

    scores
}


pub fn solution(input:String) {


    let mut part1_answer: u64 = 0;
    let mut part2_answer: u64 = 0;

    input.split("\n\n").for_each(|c| {
        let lines: Vec<_> = c.lines().map(|l| l.to_string()).collect();
        let mut grid = Grid2D::from_lines(&lines);
        part1_answer += reflection_score(&grid).first().unwrap();
        part2_answer += smudge_score(&mut grid);
    });

    println!("{}", part1_answer);
    println!("{}", part2_answer);
}
