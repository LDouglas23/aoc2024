use std::ops::{Add, Index, Mul, Sub};

pub trait CellContents {
    fn from(x: usize, y: usize, c: &char) -> Self;
}

#[derive(Debug, Clone, Copy)]
pub struct Cell<T: CellContents + Copy> {
    x: usize,
    y: usize,
    contents: T,
}

impl<T: CellContents + Copy> Cell<T> {
    pub fn position(&self) -> Vector2D {
        (self.x as i32, self.y as i32).into()
    }

    pub fn contents(&self) -> T {
        self.contents
    }
}

pub enum CellType<T: CellContents + Copy> {
    Wall,
    Cell(Cell<T>),
}

#[derive(Debug, Clone)]
pub struct Grid<T: CellContents + Copy> {
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

impl<T: CellContents + Copy> Grid<T> {
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
                            contents: T::from(i, j, c),
                        })
                        .collect()
                })
                .collect(),
        }
    }

    pub fn safe_index(&self, loc: impl Into<Vector2D>) -> CellType<T> {
        let idx = loc.into();

        if idx.x < 0 || idx.y < 0 || idx.x >= self.dim as i32 || idx.y >= self.dim as i32 {
            return CellType::Wall;
        }

        CellType::Cell(self[idx])
    }

    pub fn cell_from(&self, start: impl Into<Vector2D>, delta: impl Into<Vector2D>) -> CellType<T> {
        return self.safe_index(start.into() + delta.into());
    }

    pub fn dim(&self) -> usize {
        self.dim
    }
}

pub struct GridIntoIterator<T: CellContents + Copy> {
    grid: Grid<T>,
    dim: usize,
    x: usize,
    y: usize,
}

impl<T: CellContents + Copy> Iterator for GridIntoIterator<T> {
    type Item = Cell<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut x = self.x;
        let mut y = self.y;

        if x >= self.dim {
            return None;
        };

        let result = self.grid.cells[x][y];

        y = (y + 1) % self.dim;

        if y == 0 {
            x += 1;

            self.x = x;
        }

        self.y = y;

        Some(result)
    }
}

impl<T: CellContents + Copy> IntoIterator for Grid<T> {
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

impl<T: CellContents + Copy> Index<Vector2D> for Grid<T> {
    type Output = Cell<T>;

    fn index(&self, idx: Vector2D) -> &Self::Output {
        let x = idx.x;
        let y = idx.y;

        &self.cells[x as usize][y as usize]
    }
}
