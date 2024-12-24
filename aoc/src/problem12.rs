use crate::common::{Cell, Grid, OrthoDirection};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
            let neighbours = grid.ortho_neighbours(plot);

            // doing this as 4 - same-type neighbours instead of just counting different-type neighbours lets us skip having to check for the edges of the grid
            perimeter += 4 - neighbours
                .iter()
                .filter(|other| other.contents == plot.contents)
                .count();
        }

        perimeter
    }

    fn number_of_sides(&self, grid: &Grid<Plot>) -> usize {
        let mut number_of_sides = 0;

        /*
            Follow left wall strategy requires starting in the top-left. I wasn't able to find an algorithm that could generalise to any starting square so we instead use a consistent ordering of cells in order to find the top-most left-most square and start from there, facing right
        */
        let mut cells = self.plots.clone();
        cells.sort();

        let mut position = &cells[0];
        let mut direction = OrthoDirection::E;

        loop {
            let left = grid.ortho_neighbour(position, direction.left());

            if left.is_some_and(|cell| cell.contents == position.contents) {
                direction = direction.left();
                number_of_sides += 1;
            }

            let next = grid.ortho_neighbour(position, direction);

            if next.is_some_and(|cell| cell.contents == position.contents) {
                position = next.unwrap();
                continue;
            }

            direction = direction.right();
            number_of_sides += 1;

            if position == &cells[0] && direction == OrthoDirection::E {
                break;
            }
        }

        number_of_sides
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

pub fn solution_part_two(input: Input) -> usize {
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
        .map(|plot| plot.area() * plot.number_of_sides(&grid))
        .sum()
}

#[cfg(test)]
mod test {
    use super::{solution, solution_part_two, Input};

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

    #[test]
    pub fn test2() {
        let input = Input::from(EXAMPLE.lines().map(String::from).collect::<Vec<String>>());

        assert_eq!(solution_part_two(input), 1206);
    }

    // Counter example with island that defeats left-hand wall strategy
    const EXAMPLE2: &'static str = "\
AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA    
";

    #[test]
    fn test3() {
        let input = Input::from(EXAMPLE2.lines().map(String::from).collect::<Vec<String>>());

        assert_eq!(solution_part_two(input), 368);
    }

    // Counter example that defeats strategies for detecting unexplored walls
    const EXAMPLE3: &'static str = "\
AAAA
ABBA
ABBA
AAAA    
";

    #[test]
    fn test4() {
        let input = Input::from(EXAMPLE3.lines().map(String::from).collect::<Vec<String>>());

        assert_eq!(solution_part_two(input), (4 * 4) + (12 * 8));
    }
}
