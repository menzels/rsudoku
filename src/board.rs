use crate::field::Field;
use std::collections::HashSet;
use std::fmt;

#[derive(Clone)]
pub struct Board {
    rows: [[Field; 9]; 9],
}

fn all_unique<'a, I>(vals: I) -> bool
where
    I: IntoIterator<Item = &'a Field>,
{
    let mut uniq = HashSet::new();
    vals.into_iter()
        .filter(|f| f.is_settled())
        .all(|f| uniq.insert(f.values[0]))
}

impl Board {
    pub fn from_string(s: String) -> Board {
        if s.len() != 81 {
            panic!("string length is off! {}", s.len());
        }
        let mut b = Board {
            rows: Default::default(),
        };
        s.chars()
            .zip(b.rows.iter_mut().flatten())
            .for_each(|(v, r)| {
                let d = v.to_digit(10);
                match d {
                    Some(x) => *r = Field::settled(x as u8),
                    _ => *r = Field::default(),
                }
            });
        b
    }

    pub fn is_valid(&self) -> bool {
        self.rows.iter().flatten().all(|f| f.is_valid())
            && self.rows.iter().all(all_unique)
            && self.cols().all(all_unique)
            && self.squares().all(all_unique)
    }

    pub fn is_solved(&self) -> bool {
        self.rows.iter().flatten().all(|f| f.is_settled())
    }

    pub fn possible_values(&self) -> Vec<(usize, Vec<u8>)> {
        self.rows
            .iter()
            .flatten()
            .enumerate()
            .filter(|f| !f.1.is_settled())
            .map(|(index, f)| (index, f.values.to_vec()))
            .collect()
    }

    pub fn update(&mut self, index: usize, values: Vec<u8>) {
        let row = index / 9;
        let col = index % 9;
        self.rows[row][col].values = values;
    }

    pub fn solve(&mut self) -> bool {
        let mut progress = false;

        let settled_rows = self.settled_values(self.rows.iter().map(|r| r.iter()));
        let settled_cols = self.settled_values(self.cols());
        let settled_squares = self.settled_values(self.squares());

        self.rows
            .iter_mut()
            .flatten()
            .enumerate()
            .filter(|f| !f.1.is_settled())
            .for_each(|(index, f)| {
                let row = index / 9;
                let col = index % 9;
                let sq = row / 3 * 3 + col / 3;
                let values = (1..=9)
                    .filter(|v| {
                        !settled_rows[row].contains(v)
                            && !settled_cols[col].contains(v)
                            && !settled_squares[sq].contains(v)
                    })
                    .collect::<Vec<u8>>();
                // println!("values for {}/{}/{}: {:?}", row, col, sq, values);
                if values.len() == 1 {
                    // println!("updated {}/{} to {}", row, col, values[0]);
                    progress = true;
                }
                f.values = values;
            });
        progress
    }

    fn settled_values<'a>(
        &self,
        subset: impl Iterator<Item = impl Iterator<Item = &'a Field>>,
    ) -> Vec<Vec<u8>> {
        subset
            .map(|r| {
                r.filter(|f| f.is_settled())
                    .map(|f| f.values[0])
                    .collect::<Vec<u8>>()
            })
            .collect::<Vec<Vec<u8>>>()
    }

    // fn row(&self, index: usize) -> impl Iterator<Item = &Field> {
    //     self.rows[index / 9].iter()
    // }

    // fn col(&self, index: usize) -> impl Iterator<Item = &Field> {
    //     let col_idx = index % 9;
    //     self.rows.iter().map(move |v| &v[col_idx])
    // }

    fn minigrid(&self, index: usize) -> impl Iterator<Item = &Field> {
        let row_idx = index / 9;
        let col_idx = index % 9;
        let minigrid_col_idx = col_idx / 3;
        let minigrid_row_idx = row_idx / 3;
        let x = minigrid_row_idx * 3;
        let y = minigrid_col_idx * 3;
        (0..3)
            .map(move |ox| (0..3).map(move |oy| &self.rows[x + ox][y + oy]))
            .flatten()
    }

    fn cols(&self) -> impl Iterator<Item = impl Iterator<Item = &Field>> {
        (0..9).map(move |x| self.rows.iter().map(move |r| &r[x]))
    }

    fn squares(&self) -> impl Iterator<Item = impl Iterator<Item = &Field>> {
        (0..9).map(move |index| self.minigrid(index * 3 % 9 + index / 3 * 3 * 9))
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[")?;
        for (i, r) in self.rows.iter().enumerate() {
            if i > 0 {
                write!(f, "\n ")?;
            }
            write!(f, "[")?;
            for (j, v) in r.iter().enumerate() {
                write!(f, "{}", v)?;
                if j < 8 {
                    write!(f, " ")?;
                }
            }
            write!(f, "]")?;
        }
        write!(f, "]")
    }
}
