use std::cmp::Ordering;
use std::fmt::Debug;
use std::ops::{Add, Div, Index, Mul, Sub};

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

impl<T> Into<Vector2di> for &Cell<T> {
    fn into(self) -> Vector2di {
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

#[derive(Debug, Copy, Clone, PartialEq, Default)]
pub struct Vector2di {
    pub x: i64,
    pub y: i64,
}

impl Vector2di {
    pub fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }
}

impl From<(i32, i32)> for Vector2di {
    fn from(value: (i32, i32)) -> Self {
        Vector2di::from((value.0 as i64, value.1 as i64))
    }
}

impl From<(i64, i64)> for Vector2di {
    fn from(value: (i64, i64)) -> Self {
        Self {
            x: value.0,
            y: value.1,
        }
    }
}

impl From<(usize, usize)> for Vector2di {
    fn from(value: (usize, usize)) -> Self {
        Self {
            x: value.0 as i64,
            y: value.1 as i64,
        }
    }
}

impl Add for Vector2di {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Vector2di {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Mul<i64> for Vector2di {
    type Output = Self;

    fn mul(self, rhs: i64) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl Mul<usize> for Vector2di {
    type Output = Self;

    fn mul(self, rhs: usize) -> Self::Output {
        self * rhs as i64
    }
}

impl Div<Vector2di> for Vector2di {
    type Output = Self;

    fn div(self, rhs: Vector2di) -> Self::Output {
        Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum OrthoDirection {
    #[default]
    Up,
    Right,
    Down,
    Left,
}

impl OrthoDirection {
    pub fn left(&self) -> Self {
        match self {
            OrthoDirection::Up => Self::Left,
            OrthoDirection::Right => Self::Up,
            OrthoDirection::Down => Self::Right,
            OrthoDirection::Left => Self::Down,
        }
    }

    pub fn right(&self) -> Self {
        match self {
            OrthoDirection::Up => Self::Right,
            OrthoDirection::Right => Self::Down,
            OrthoDirection::Down => Self::Left,
            OrthoDirection::Left => Self::Up,
        }
    }

    pub fn next(&self, winding_mode: &WindingMode) -> Self {
        match winding_mode {
            WindingMode::Clockwise => self.right(),
            WindingMode::Anticlockwise => self.left(),
        }
    }
}

impl Into<Vector2di> for OrthoDirection {
    fn into(self) -> Vector2di {
        match self {
            OrthoDirection::Up => Vector2di::new(0, -1),
            OrthoDirection::Right => Vector2di::new(1, 0),
            OrthoDirection::Down => Vector2di::new(0, 1),
            OrthoDirection::Left => Vector2di::new(-1, 0),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum WindingMode {
    #[default]
    Clockwise,
    Anticlockwise,
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct NeighbourMode {
    pub start_direction: OrthoDirection,
    pub winding_mode: WindingMode,
}

impl<T> Grid<T> {
    pub fn safe_index(&self, loc: impl Into<Vector2di>) -> Option<&Cell<T>> {
        let idx = loc.into();

        if idx.x < 0 || idx.y < 0 || idx.x >= self.dim as i64 || idx.y >= self.dim as i64 {
            return None;
        }

        Some(&self[idx])
    }

    pub fn cell_from(
        &self,
        start: impl Into<Vector2di>,
        delta: impl Into<Vector2di>,
    ) -> Option<&Cell<T>> {
        return self.safe_index(start.into() + delta.into());
    }

    pub fn dim(&self) -> usize {
        self.dim
    }

    pub fn safe_ortho_neighbours(&self, loc: impl Into<Vector2di>) -> Vec<&Cell<T>> {
        let idx = loc.into();

        let neighbours = vec![
            self.cell_from(idx, (0, -1)),
            self.cell_from(idx, (1, 0)),
            self.cell_from(idx, (0, 1)),
            self.cell_from(idx, (-1, 0)),
        ];

        neighbours
            .iter()
            .filter_map(|c| *c)
            .collect::<Vec<&Cell<T>>>()
    }

    pub fn ortho_neighbours(
        &self,
        cell: &Cell<T>,
        pattern: NeighbourMode,
    ) -> Vec<Option<&Cell<T>>> {
        let mut result = vec![];

        let mut direction = pattern.start_direction;

        for _ in 0..4 {
            result.push(self.ortho_neighbour(cell, direction));
            direction = direction.next(&pattern.winding_mode);
        }

        result
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
    pub fn region(&'a self, loc: impl Into<Vector2di>) -> Region<'a, T> {
        let mut result: Vec<&Cell<T>> = vec![];

        if let Some(n) = self.safe_index(loc) {
            result.push(n);

            let mut buffer: Vec<&Cell<T>> = vec![n];

            while !buffer.is_empty() {
                let n = buffer.pop().unwrap();
                let neighbours = self.safe_ortho_neighbours(n);

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

impl<T> Index<Vector2di> for Grid<T> {
    type Output = Cell<T>;

    fn index(&self, idx: Vector2di) -> &Self::Output {
        let x = idx.x;
        let y = idx.y;

        &self.cells[y as usize][x as usize]
    }
}

impl<T> Index<(i32, i32)> for Grid<T> {
    type Output = Cell<T>;

    fn index(&self, index: (i32, i32)) -> &Self::Output {
        &self[Vector2di::from(index)]
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
