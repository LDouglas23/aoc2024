use std::cmp::Ordering;
use std::fmt::Debug;
use std::ops::{Add, Index, Mul, Sub};

#[derive(Debug, Clone, Copy, Eq)]
pub struct Cell<T> {
    pub x: usize,
    pub y: usize,
    pub contents: T,
}

impl<T> PartialEq for Cell<T> {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl<T> Into<Vector2D> for &Cell<T> {
    fn into(self) -> Vector2D {
        (self.x, self.y).into()
    }
}

impl<T: Eq> Ord for Cell<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.y.cmp(&other.y).then(self.x.cmp(&other.x))
    }
}

impl<T: Eq> PartialOrd for Cell<T> {
    //Row-major order
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone)]
pub struct Grid<T> {
    dim: usize,
    cells: Vec<Vec<Cell<T>>>,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vector2D {
    x: i32,
    y: i32,
}

impl Vector2D {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

impl From<(i32, i32)> for Vector2D {
    fn from(value: (i32, i32)) -> Self {
        Self {
            x: value.0,
            y: value.1,
        }
    }
}

impl From<(usize, usize)> for Vector2D {
    fn from(value: (usize, usize)) -> Self {
        Self {
            x: value.0 as i32,
            y: value.1 as i32,
        }
    }
}

impl Add for Vector2D {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Vector2D {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Mul<i32> for Vector2D {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OrthoDirection {
    N,
    E,
    S,
    W,
}

impl OrthoDirection {
    pub fn left(&self) -> Self {
        match self {
            OrthoDirection::N => Self::W,
            OrthoDirection::E => Self::N,
            OrthoDirection::S => Self::E,
            OrthoDirection::W => Self::S,
        }
    }

    pub fn right(&self) -> Self {
        match self {
            OrthoDirection::N => Self::E,
            OrthoDirection::E => Self::S,
            OrthoDirection::S => Self::W,
            OrthoDirection::W => Self::N,
        }
    }
}

impl Into<Vector2D> for OrthoDirection {
    fn into(self) -> Vector2D {
        match self {
            OrthoDirection::N => Vector2D::new(0, -1),
            OrthoDirection::E => Vector2D::new(1, 0),
            OrthoDirection::S => Vector2D::new(0, 1),
            OrthoDirection::W => Vector2D::new(-1, 0),
        }
    }
}

impl<T> Grid<T> {
    pub fn safe_index(&self, loc: impl Into<Vector2D>) -> Option<&Cell<T>> {
        let idx = loc.into();

        if idx.x < 0 || idx.y < 0 || idx.x >= self.dim as i32 || idx.y >= self.dim as i32 {
            return None;
        }

        Some(&self[idx])
    }

    pub fn cell_from(
        &self,
        start: impl Into<Vector2D>,
        delta: impl Into<Vector2D>,
    ) -> Option<&Cell<T>> {
        return self.safe_index(start.into() + delta.into());
    }

    pub fn dim(&self) -> usize {
        self.dim
    }

    pub fn ortho_neighbours(&self, loc: impl Into<Vector2D>) -> Vec<&Cell<T>> {
        let idx = loc.into();

        let neighbours = vec![
            self.cell_from(idx, (0, -1)),
            self.cell_from(idx, (0, 1)),
            self.cell_from(idx, (-1, 0)),
            self.cell_from(idx, (1, 0)),
        ];

        neighbours
            .iter()
            .filter_map(|c| *c)
            .collect::<Vec<&Cell<T>>>()
    }

    pub fn ortho_neighbour(&self, cell: &Cell<T>, dir: OrthoDirection) -> Option<&Cell<T>> {
        self.cell_from((cell.x, cell.y), dir)
    }
}

impl<T: From<char>> Grid<T> {
    pub fn new(input: Vec<Vec<char>>) -> Self {
        Self {
            dim: input.len(),
            cells: input
                .iter()
                .enumerate()
                .map(|(j, row)| {
                    row.iter()
                        .enumerate()
                        .map(|(i, c)| Cell {
                            x: i,
                            y: j,
                            contents: T::from(*c),
                        })
                        .collect()
                })
                .collect(),
        }
    }
}

pub type Region<'a, T> = Vec<&'a Cell<T>>;

impl<'a, T: PartialEq + Copy> Grid<T> {
    pub fn region(&'a self, loc: impl Into<Vector2D>) -> Region<'a, T> {
        let mut result: Vec<&Cell<T>> = vec![];

        if let Some(n) = self.safe_index(loc) {
            result.push(n);

            let mut buffer: Vec<&Cell<T>> = vec![n];

            while !buffer.is_empty() {
                let n = buffer.pop().unwrap();
                let neighbours = self.ortho_neighbours(n);

                for neighbour in neighbours {
                    if neighbour.contents == n.contents && !result.contains(&neighbour) {
                        result.push(neighbour);
                        buffer.push(neighbour);
                    }
                }
            }
        }

        result
    }

    pub fn regions(&'a self) -> Vec<Region<'a, T>> {
        let mut regions = vec![];

        for cell in self.clone() {
            if regions.iter().any(|r: &Region<'a, T>| r.contains(&&cell)) {
                continue;
            }

            regions.push(self.region(&cell));
        }

        regions
    }
}

pub struct GridIntoIterator<T> {
    grid: Grid<T>,
    dim: usize,
    x: usize,
    y: usize,
}

impl<T: Copy> Iterator for GridIntoIterator<T> {
    type Item = Cell<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut x = self.x;
        let mut y = self.y;

        if x >= self.dim {
            return None;
        };

        let result = &self.grid.cells[x][y];

        y = (y + 1) % self.dim;

        if y == 0 {
            x += 1;

            self.x = x;
        }

        self.y = y;

        Some(*result)
    }
}

impl<T: Copy> IntoIterator for Grid<T> {
    type Item = Cell<T>;
    type IntoIter = GridIntoIterator<T>;

    fn into_iter(self) -> Self::IntoIter {
        GridIntoIterator {
            dim: self.dim,
            x: 0,
            y: 0,
            grid: self,
        }
    }
}

impl<T> Index<Vector2D> for Grid<T> {
    type Output = Cell<T>;

    fn index(&self, idx: Vector2D) -> &Self::Output {
        let x = idx.x;
        let y = idx.y;

        &self.cells[y as usize][x as usize]
    }
}

impl<T> Index<(i32, i32)> for Grid<T> {
    type Output = Cell<T>;

    fn index(&self, index: (i32, i32)) -> &Self::Output {
        &self[Vector2D::from(index)]
    }
}

#[cfg(test)]
mod test {
    use super::Cell;

    #[test]
    fn test_cell_ord_and_eq() {
        let cell1 = Cell {
            x: 0,
            y: 1,
            contents: (),
        };
        let cell2 = Cell {
            x: 3,
            y: 1,
            contents: (),
        };
        let cell3 = Cell {
            x: 3,
            y: 2,
            contents: (),
        };
        let cell4 = Cell {
            x: 3,
            y: 2,
            contents: (),
        };

        assert!(cell1 < cell2);
        assert!(cell2 < cell3);
        assert!(cell3 == cell4);
    }
}
