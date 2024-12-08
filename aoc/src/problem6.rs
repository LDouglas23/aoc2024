trait GridCell {
    fn from(x: usize, y: usize, contents: &char) -> Self;
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum CellContents {
    Nothing,
    Obstacle,
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Cell {
    x: usize,
    y: usize,
    contents: CellContents,
}

impl From<&char> for CellContents {
    fn from(value: &char) -> Self {
        match value {
            '#' => CellContents::Obstacle,
            _ => CellContents::Nothing,
        }
    }
}

impl GridCell for Cell {
    fn from(x: usize, y: usize, contents: &char) -> Self {
        Self {
            x,
            y,
            contents: contents.into(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Direction {
    UP,
    RIGHT,
    DOWN,
    LEFT,
}

#[derive(Clone, Copy, Debug)]
struct Guard {
    x: usize,
    y: usize,
    facing: Direction,
}

impl Guard {
    fn new(map: &[String]) -> Self {
        for (j, s) in map.iter().enumerate() {
            if s.contains('^') {
                return Self {
                    x: s.chars().position(|c| c == '^').unwrap(),
                    y: j,
                    facing: Direction::UP,
                };
            }
        }

        panic!();
    }

    fn step(&mut self, grid: &Grid<Cell>) -> Option<Cell> {
        if let Some(cell) = grid.neighbour(self.x, self.y, self.facing) {
            if let CellContents::Nothing = cell.contents {
                self.x = cell.x;
                self.y = cell.y;

                return Some(cell.clone());
            } else {
                self.turn();

                return self.step(grid);
            }
        }

        None
    }

    fn turn(&mut self) {
        match self.facing {
            Direction::UP => self.facing = Direction::RIGHT,
            Direction::RIGHT => self.facing = Direction::DOWN,
            Direction::DOWN => self.facing = Direction::LEFT,
            Direction::LEFT => self.facing = Direction::UP,
        };
    }
}

#[derive(Clone, Debug)]
struct Grid<T: GridCell> {
    cells: Vec<Vec<T>>,
    dim: usize,
}

impl<T: GridCell> Grid<T> {
    fn new(rows: &[String]) -> Self {
        let cells: Vec<Vec<T>> = rows
            .iter()
            .enumerate()
            .map(|(j, row)| {
                row.chars()
                    .enumerate()
                    .map(|(i, col)| T::from(i, j, &col))
                    .collect()
            })
            .collect();

        Self {
            dim: cells[0].len(),
            cells,
        }
    }

    fn neighbour(&self, x: usize, y: usize, dir: Direction) -> Option<&T> {
        let (dx, dy) = match dir {
            Direction::UP => (0, -1),
            Direction::RIGHT => (1, 0),
            Direction::DOWN => (0, 1),
            Direction::LEFT => (-1, 0),
        };

        let (x_step, y_step) = (x as isize + dx, y as isize + dy);

        if x_step < 0 || y_step < 0 || x_step >= self.dim as isize || y_step >= self.dim as isize {
            None
        } else {
            Some(&self.cells[y_step as usize][x_step as usize])
        }
    }

    fn cell(&self, x: isize, y: isize) -> Option<&T> {
        if x < 0 || y < 0 || x >= self.dim as isize || y >= self.dim as isize {
            return None;
        }

        Some(&self.cells[x as usize][y as usize])
    }

    fn cell_mut(&mut self, x: isize, y: isize) -> Option<&mut T> {
        if x < 0 || y < 0 || x >= self.dim as isize || y >= self.dim as isize {
            return None;
        }

        Some(&mut self.cells[x as usize][y as usize])
    }
}

pub struct Input {
    grid: Grid<Cell>,
    guard: Guard,
}

pub fn solution(input: Input) -> usize {
    let mut visited: Vec<Cell> = vec![];

    let grid = input.grid;
    let mut guard = input.guard;

    // Push the starting cell for completeness
    visited.push(
        grid.cell(guard.x as isize, guard.y as isize)
            .unwrap()
            .clone(),
    );

    while let Some(cell) = guard.step(&grid) {
        if !visited.contains(&cell) {
            visited.push(cell);
        }
    }

    visited.len()
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Position {
    x: usize,
    y: usize,
    facing: Direction,
}

impl From<Guard> for Position {
    fn from(value: Guard) -> Self {
        Self {
            x: value.x,
            y: value.y,
            facing: value.facing,
        }
    }
}

pub fn solution_part_two(input: Input) -> usize {
    let mut loops_found = 0;

    let grid = input.grid;

    for x in 0..grid.dim {
        for y in 0..grid.dim {
            println!("{}, {}", x, y);
            let mut guard = input.guard.clone();
            let mut grid_cp = grid.clone();

            if x == guard.x && y == guard.y {
                continue;
            }

            let cell = grid_cp.cell_mut(x as isize, y as isize).unwrap();

            if let CellContents::Obstacle = cell.contents {
                continue;
            }

            cell.contents = CellContents::Obstacle;

            let mut encountered_positions: Vec<Position> = vec![];
            encountered_positions.push(Position::from(guard));

            let mut loop_counter = 0;
            while let Some(_) = guard.step(&grid_cp) {
                if loop_counter > 10000 {
                    println!("Hit loop limiter, x: {}, y: {}", x, y);
                    panic!();
                }

                let new_pos = Position::from(guard);

                if encountered_positions.contains(&new_pos) {
                    println!("Loop found after {}", loop_counter);
                    loops_found += 1;
                    break;
                }

                encountered_positions.push(new_pos);

                loop_counter += 1;
            }
        }
    }

    loops_found
}

pub fn parse_lines(lines: Vec<String>) -> Input {
    Input {
        grid: Grid::new(&lines),
        guard: Guard::new(&lines),
    }
}

#[cfg(test)]
mod test {

    const EXAMPLE: &'static str = "\
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    #[test]
    fn test() {
        let input = super::parse_lines(EXAMPLE.lines().map(String::from).collect());

        assert_eq!(super::solution(input), 41);
    }

    #[test]
    fn test2() {
        let input = super::parse_lines(EXAMPLE.lines().map(String::from).collect());

        assert_eq!(super::solution_part_two(input), 6);
    }
}
