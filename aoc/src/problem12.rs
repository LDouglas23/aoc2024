use crate::common::{Cell, Grid};

#[derive(Debug, Clone, Copy, PartialEq)]
struct Plot {
    plant: char,
}

impl From<char> for Plot {
    fn from(value: char) -> Self {
        Self { plant: value }
    }
}

#[derive(Debug, Clone)]
struct Region {
    plots: Vec<Cell<Plot>>,
}

impl Region {
    fn area(&self) -> usize {
        self.plots.len()
    }

    fn perimeter(&self, grid: &Grid<Plot>) -> usize {
        let mut perimeter = 0;

        for plot in &self.plots {
            let neighbours = grid.neighbours(plot);

            // doing this as 4 - same-type neighbours instead of just counting different-type neighbours lets us skip having to check for the edges of the grid
            perimeter += 4 - neighbours
                .iter()
                .filter(|other| other.contents == plot.contents)
                .count();
        }

        perimeter
    }
}

pub struct Input {
    grid: Grid<Plot>,
}

impl From<Vec<String>> for Input {
    fn from(value: Vec<String>) -> Self {
        Self {
            grid: Grid::new(value.iter().map(|s| s.chars().collect()).collect()),
        }
    }
}

pub fn solution(input: Input) -> usize {
    let grid = input.grid;

    let regions = grid.regions();

    let plot_regions: Vec<Region> = regions
        .iter()
        .map(|r| Region {
            plots: r.iter().cloned().map(|p| *p).collect(),
        })
        .collect();

    plot_regions
        .iter()
        .map(|region| region.area() * region.perimeter(&grid))
        .sum()
}

#[cfg(test)]
mod test {
    use super::{solution, Input};

    const EXAMPLE: &'static str = "\
RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";

    #[test]
    pub fn test() {
        let input = Input::from(EXAMPLE.lines().map(String::from).collect::<Vec<String>>());

        assert_eq!(solution(input), 1930);
    }
}
