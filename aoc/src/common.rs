use std::cmp::Ordering;
use std::fmt::{Debug, Display, Write};
use std::ops::{Add, Div, Index, IndexMut, Mul, Sub};

#[derive(Debug, Clone, Copy, Eq)]
pub struct Cell<T> {
    pub x: usize,
    pub y: usize,
    pub contents: T,
}

impl<T> Cell<T> {
    fn position(&self) -> Vector2di {
        Vector2di::new(self.x as i64, self.y as i64)
    }
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
    //Row-major order
    fn cmp(&self, other: &Self) -> Ordering {
        self.y.cmp(&other.y).then(self.x.cmp(&other.x))
    }
}

impl<T: Eq> PartialOrd for Cell<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone)]
pub struct Grid<T> {
    dim: usize,
    cells: Vec<Vec<Cell<T>>>,
}

#[derive(Debug, Copy, Clone, PartialEq, Default, Eq, Hash)]
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
    pub fn get(&self, loc: impl Into<Vector2di>) -> Option<&Cell<T>> {
        let idx = loc.into();

        if idx.x < 0 || idx.y < 0 || idx.x >= self.dim as i64 || idx.y >= self.dim as i64 {
            return None;
        }

        Some(&self[idx])
    }

    pub fn get_mut(&mut self, loc: impl Into<Vector2di>) -> Option<&mut Cell<T>> {
        let idx = loc.into();

        if idx.x < 0 || idx.y < 0 || idx.x >= self.dim as i64 || idx.y >= self.dim as i64 {
            return None;
        }

        Some(&mut self[idx])
    }

    pub fn insert(&mut self, loc: impl Into<Vector2di>, value: Cell<T>) {
        self[loc.into()] = value;
    }

    pub fn dim(&self) -> usize {
        self.dim
    }

    pub fn safe_ortho_neighbours(&self, loc: impl Into<Vector2di>) -> Vec<&Cell<T>> {
        let idx = loc.into();

        let neighbours = vec![
            self.get(idx + (0, -1).into()),
            self.get(idx + (1, 0).into()),
            self.get(idx + (0, 1).into()),
            self.get(idx + (-1, 0).into()),
        ];

        neighbours
            .iter()
            .filter_map(|c| *c)
            .collect::<Vec<&Cell<T>>>()
    }

    pub fn ortho_neighbours(&self, cell: &Cell<T>, mode: NeighbourMode) -> Vec<Option<&Cell<T>>> {
        let mut result = vec![];

        let mut direction = mode.start_direction;

        for _ in 0..4 {
            result.push(self.get(cell.position() + direction.into()));
            direction = direction.next(&mode.winding_mode);
        }

        result
    }
}

impl<T: Copy> Grid<T> {
    pub fn from(input: Vec<Vec<T>>) -> Self {
        Self {
            dim: input.len(),
            cells: input
                .iter()
                .enumerate()
                .map(|(j, row)| {
                    row.iter()
                        .enumerate()
                        .map(|(i, t)| Cell {
                            x: i,
                            y: j,
                            contents: *t,
                        })
                        .collect()
                })
                .collect(),
        }
    }

    /**
    Replace the contents of the cell at the given position. Returns the original contents of the
    cell.
    */
    pub fn replace(&mut self, loc: impl Into<Vector2di>, value: T) -> Option<T> {
        let original = self.get_mut(loc.into())?;
        let contents = original.contents;

        original.contents = value;

        Some(contents)
    }

    /**
    Swap the contents of the cells at the source and destination positions. If the provided
    positions are the same, the swap is not attempted. If either of the positions are outside the
    boundaries of the grid, the swap is not attempted.
     */
    pub fn swap(
        &mut self,
        source: impl Into<Vector2di> + Copy,
        destination: impl Into<Vector2di> + Copy,
    ) {
        if source.into() == destination.into() {
            return;
        }

        let (t1, t2) = (self.get(source.into()), self.get(destination.into()));

        if t1.is_none() || t2.is_none() {
            return;
        }

        let t1 = t1.unwrap().contents;
        let t2 = t2.unwrap().contents;

        if let Some(c1) = self.get_mut(source.into()) {
            c1.contents = t2;
        }

        if let Some(c2) = self.get_mut(destination.into()) {
            c2.contents = t1;
        }
    }
}

pub type Region<'a, T> = Vec<&'a Cell<T>>;

impl<'a, T: PartialEq + Copy> Grid<T> {
    pub fn region(&'a self, loc: impl Into<Vector2di>) -> Region<'a, T> {
        let mut result: Vec<&Cell<T>> = vec![];

        if let Some(n) = self.get(loc) {
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

impl<T> IndexMut<Vector2di> for Grid<T> {
    fn index_mut(&mut self, idx: Vector2di) -> &mut Self::Output {
        let x = idx.x;
        let y = idx.y;

        &mut self.cells[y as usize][x as usize]
    }
}

impl<T> IndexMut<(i32, i32)> for Grid<T> {
    fn index_mut(&mut self, index: (i32, i32)) -> &mut Self::Output {
        &mut self[Vector2di::from(index)]
    }
}

impl<T: Copy + Into<char>> Display for Grid<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for j in &self.cells {
            for i in j {
                f.write_char(i.contents.into())?;
            }

            f.write_str("\n")?;
        }

        Ok(())
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
