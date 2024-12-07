#[derive(Debug)]
pub struct Input {
    rows: Vec<String>,
}

/* This probably would have worked but I just didn't like it. The edge cases were weird, and the other solution seems more fun anyway */
pub fn solution_old(input: Input) -> usize {
    let rows = input.rows.to_owned();
    let rows_reversed = rows
        .clone()
        .iter()
        .map(|s| s.chars().rev().collect::<String>())
        .collect::<Vec<String>>();

    let mut cols = vec![];

    for i in 0..rows[0].len() {
        let col = rows
            .iter()
            .map(|r| r.chars().collect::<Vec<char>>()[i])
            .collect::<String>();

        cols.push(col);
    }

    let cols_reversed = cols
        .clone()
        .iter()
        .map(|s| s.chars().rev().collect::<String>())
        .collect::<Vec<String>>();

    let d1 = get_diagonals(&rows);
    let d2 = get_diagonals(&rows.iter().rev().cloned().collect());
    let d3 = get_diagonals(&cols.iter().rev().cloned().collect());
    let d4 = get_diagonals(&cols);

    println!("{:?}", rows);
    println!("{:?}", rows_reversed);

    println!("{:?}", cols);
    println!("{:?}", cols_reversed);

    println!("{:?}", d1);
    println!("{:?}", d2);
    println!("{:?}", d3);
    println!("{:?}", d4);

    rows.iter().map(count_matches).sum::<usize>()
        + rows_reversed.iter().map(count_matches).sum::<usize>()
        + cols.iter().map(count_matches).sum::<usize>()
        + cols_reversed.iter().map(count_matches).sum::<usize>()
        + d1.iter().map(count_matches).sum::<usize>()
        + d2.iter().map(count_matches).sum::<usize>()
        + d3.iter().map(count_matches).sum::<usize>()
        + d4.iter().map(count_matches).sum::<usize>()
}

fn get_diagonals(rows: &Vec<String>) -> Vec<String> {
    let mut diagonals: Vec<String> = vec![];

    for i in 0..rows[0].len() {
        // The wordsearches are square so we can do this
        let mut x = 0;

        let mut chars = vec![];

        chars.push(rows[x].chars().nth(i).unwrap());

        while x < i {
            x += 1;

            chars.push(rows[x].chars().nth(i - x).unwrap());
        }

        diagonals.push(chars.iter().collect())
    }

    diagonals.iter().filter(|&s| s.len() > 3).cloned().collect()
}

fn count_matches(candidate: &String) -> usize {
    candidate.matches("XMAS").count()
}

// ---------------------------------------------------------

#[derive(Copy, Clone)]
struct Cell {
    x: usize,
    y: usize,
    contents: char,
}

#[derive(Clone)]
struct Grid {
    rows: Vec<Vec<Cell>>,
    x_dim: usize,
    y_dim: usize,
}

impl IntoIterator for Grid {
    type Item = Cell;
    type IntoIter = GridIntoIterator;

    fn into_iter(self) -> Self::IntoIter {
        GridIntoIterator {
            dim: self.rows.len(),
            grid: self,
            index: 0,
        }
    }
}

struct GridIntoIterator {
    grid: Grid,
    dim: usize,
    index: usize,
}

impl Iterator for GridIntoIterator {
    type Item = Cell;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.dim.pow(2) {
            return None;
        };

        let result = self.grid.rows[self.index / self.dim][self.index % self.dim];

        self.index += 1;

        Some(result)
    }
}

#[derive(Clone, Copy)]
enum Direction {
    N,
    NE,
    E,
    SE,
    S,
    SW,
    W,
    NW,
}

impl Direction {
    const DIRS: [Direction; 8] = [
        Self::N,
        Self::NE,
        Self::E,
        Self::SE,
        Self::S,
        Self::SW,
        Self::W,
        Self::NW,
    ];

    fn iterator() -> impl Iterator<Item = Direction> {
        Direction::DIRS.iter().copied()
    }
}

impl Grid {
    fn new(chars: Vec<Vec<char>>) -> Self {
        let cells: Vec<Vec<Cell>> = chars
            .iter()
            .enumerate()
            .map(|(i, row)| {
                row.iter()
                    .enumerate()
                    .map(|(j, c)| Cell {
                        x: i,
                        y: j,
                        contents: *c,
                    })
                    .collect()
            })
            .collect();

        Self {
            x_dim: cells[0].len(),
            y_dim: cells.len(),
            rows: cells,
        }
    }

    fn get_neighbour(&self, cell: Cell, dir: Direction) -> Option<Cell> {
        let delta: (isize, isize) = match dir {
            Direction::N => (0, -1),
            Direction::NE => (1, -1),
            Direction::E => (1, 0),
            Direction::SE => (1, 1),
            Direction::S => (0, 1),
            Direction::SW => (-1, 1),
            Direction::W => (-1, 0),
            Direction::NW => (-1, -1),
        };

        let proj_x = cell.x as isize + delta.0;
        let proj_y = cell.y as isize + delta.1;

        if proj_x < 0 || proj_x >= self.x_dim as isize {
            return None;
        };

        if proj_y < 0 || proj_y >= self.y_dim as isize {
            return None;
        };

        Some(self.rows[proj_x as usize][proj_y as usize])
    }

    fn is_edge(&self, cell: Cell) -> bool {
        cell.x == 0 || cell.y == 0 || cell.x == self.x_dim - 1 || cell.y == self.y_dim - 1
    }
}

pub fn solution(input: Input) -> usize {
    let grid = Grid::new(
        input
            .rows
            .iter()
            .map(|r| r.clone().chars().collect())
            .collect(),
    );

    let mut count = 0;

    for cell in grid.clone() {
        if cell != 'X' {
            continue;
        }

        for dir in Direction::iterator() {
            if let Some(c) = grid.get_neighbour(cell, dir) {
                if c == 'M' {
                    if let Some(c2) = grid.get_neighbour(c, dir) {
                        if c2 == 'A' {
                            if let Some(c3) = grid.get_neighbour(c2, dir) {
                                if c3 == 'S' {
                                    count += 1;
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    count
}

impl PartialEq<char> for Cell {
    fn eq(&self, other: &char) -> bool {
        &self.contents == other
    }
}

impl Into<char> for Cell {
    fn into(self) -> char {
        self.contents
    }
}

pub fn solution_part_two(input: Input) -> usize {
    let grid = Grid::new(
        input
            .rows
            .iter()
            .map(|r| r.clone().chars().collect())
            .collect(),
    );

    let mut count = 0;

    for cell in grid.clone() {
        if grid.is_edge(cell) || cell.contents != 'A' {
            continue;
        }

        // We can unwrap because we already checked that the cell is not on the edge of the grid
        let nw = grid.get_neighbour(cell, Direction::NW).unwrap();
        let se = grid.get_neighbour(cell, Direction::SE).unwrap();

        let s1 = vec![nw.contents, cell.contents, se.contents]
            .iter()
            .cloned()
            .collect::<String>();

        let ne = grid.get_neighbour(cell, Direction::NE).unwrap();
        let sw = grid.get_neighbour(cell, Direction::SW).unwrap();

        let s2 = vec![ne.contents, cell.contents, sw.contents]
            .iter()
            .cloned()
            .collect::<String>();

        if (s1 == "MAS" || s1 == "SAM") && (s2 == "MAS" || s2 == "SAM") {
            count += 1;
        }
    }

    count
}

pub fn parse_lines(lines: Vec<String>) -> Input {
    Input { rows: lines }
}

#[cfg(test)]
mod test {
    use super::{solution, solution_part_two, Input};

    #[test]
    pub fn test() {
        let wordsearch  = "MMMSXXMASM\nMSAMXMSMSA\nAMXSXMAAMM\nMSAMASMSMX\nXMASAMXAMM\nXXAMMXXAMA\nSMSMSASXSS\nSAXAMASAAA\nMAMMMXMMMM\nMXMXAXMASX";

        let input = Input {
            rows: wordsearch.split("\n").map(String::from).collect(),
        };

        assert_eq!(solution(input), 18)
    }

    #[test]
    pub fn test2() {
        let wordsearch  = "MMMSXXMASM\nMSAMXMSMSA\nAMXSXMAAMM\nMSAMASMSMX\nXMASAMXAMM\nXXAMMXXAMA\nSMSMSASXSS\nSAXAMASAAA\nMAMMMXMMMM\nMXMXAXMASX";

        let input = Input {
            rows: wordsearch.split("\n").map(String::from).collect(),
        };

        assert_eq!(solution_part_two(input), 9);
    }
}
