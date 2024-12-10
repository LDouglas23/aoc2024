use crate::common::{CellContents, CellType, Grid, Vector2D};

#[derive(Debug, Clone, Copy, PartialEq)]
struct Antenna {
    frequency: char,
    position: Vector2D,
}

type GridCell = Option<Antenna>;

impl CellContents for GridCell {
    fn from(x: usize, y: usize, c: &char) -> Self {
        if *c == '.' {
            None
        } else {
            Some(Antenna {
                frequency: *c,
                position: (x as i32, y as i32).into(),
            })
        }
    }
}

pub struct Input {
    grid: Grid<GridCell>,
}

impl Input {
    fn get_antennas(&self) -> Vec<Antenna> {
        self.grid
            .clone()
            .into_iter()
            .filter(|&cell| cell.contents().is_some())
            .map(|cell| cell.contents().unwrap())
            .collect::<Vec<Antenna>>()
    }
}

impl From<Vec<String>> for Input {
    fn from(value: Vec<String>) -> Self {
        Self {
            grid: Grid::new(value.iter().map(|s| s.chars().collect()).collect()),
        }
    }
}

pub fn solution(input: Input) -> usize {
    let antennas = input.get_antennas();

    let mut antinode_positions: Vec<Vector2D> = vec![];

    for antenna in &antennas {
        let matches: Vec<&Antenna> = antennas
            .iter()
            .filter(|&other| other != antenna && other.frequency == antenna.frequency)
            .collect();

        for matched in matches {
            let delta = matched.position - antenna.position;
            let antinode_position = antenna.position - delta;

            if let CellType::Cell(_) = input.grid.safe_index(antinode_position) {
                if !antinode_positions.contains(&antinode_position) {
                    antinode_positions.push(antinode_position);
                }
            }
        }
    }

    antinode_positions.len()
}

pub fn solution_part_two(input: Input) -> usize {
    let antennas = input.get_antennas();

    let mut antinode_positions: Vec<Vector2D> = vec![];

    for antenna in &antennas {
        let matches: Vec<&Antenna> = antennas
            .iter()
            .filter(|&other| other != antenna && other.frequency == antenna.frequency)
            .collect();

        for matched in matches {
            let delta = matched.position - antenna.position;
            let mut next_pos = antenna.position;

            while let CellType::Cell(_) = input.grid.safe_index(next_pos) {
                if !antinode_positions.contains(&next_pos) {
                    antinode_positions.push(next_pos)
                }

                next_pos = next_pos - delta;
            }
        }
    }

    // ---- This one has an output that's nice to look at
    let mut counter = 0;
    let dim = input.grid.dim();

    for cell in input.grid {
        if counter % dim == 0 {
            println!();
        }
        if let Some(antenna) = cell.contents() {
            print!("{}", antenna.frequency);
        } else if antinode_positions.contains(&cell.position()) {
            print!("#");
        } else {
            print!(".");
        }

        counter += 1;
    }
    // ----------------------------------------------------

    antinode_positions.len()
}

#[cfg(test)]
mod test {
    const EXAMPLE: &'static str = "\
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
";

    #[test]
    fn test() {
        let input = super::Input::from(EXAMPLE.lines().map(String::from).collect::<Vec<String>>());

        assert_eq!(super::solution(input), 14);
    }

    #[test]
    fn test2() {
        let input = super::Input::from(EXAMPLE.lines().map(String::from).collect::<Vec<String>>());

        assert_eq!(super::solution_part_two(input), 34);
    }
}
