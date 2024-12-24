use crate::common::{Cell, Grid, NeighbourMode, OrthoDirection, WindingMode};

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
            let neighbours = grid.safe_ortho_neighbours(plot);

            // doing this as 4 - same-type neighbours instead of just counting different-type neighbours lets us skip having to check for the edges of the grid
            perimeter += 4 - neighbours
                .iter()
                .filter(|other| other.contents == plot.contents)
                .count();
        }

        perimeter
    }

    /*
    This method works for solid shapes with no holes, but I couldn't easily adapt it to also work
    for shapes containing holes without being able to come up with a trivial counter-example so I
    chose to discard it.
    */
    fn number_of_sides_old(&self, grid: &Grid<Plot>) -> usize {
        let mut number_of_sides = 0;

        /*
            Follow left wall strategy requires starting in the top-left. I wasn't able to find an algorithm that could generalise to any starting square so we instead use a consistent ordering of cells in order to find the top-most left-most square and start from there, facing right
        */
        let mut cells = self.plots.clone();
        cells.sort();

        let mut position = &cells[0];
        let mut direction = OrthoDirection::Right;

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

            if position == &cells[0] && direction == OrthoDirection::Right {
                break;
            }
        }

        number_of_sides
    }

    /*
       Corner-counting strategy

       Since for any regular polygon the number of corners and the number of sides is always equal,
       we can instead detect every cell that forms a corner with other cells and use that as the
       result instead.
    */
    fn number_of_sides(&self, grid: &Grid<Plot>) -> usize {
        let mut num_corners = 0;

        for plot in &self.plots {
            // It's important that the neighbouring cells are retrieved in order so we will make it explicit
            let north = grid.ortho_neighbour(plot, OrthoDirection::Up);
            let east = grid.ortho_neighbour(plot, OrthoDirection::Right);
            let south = grid.ortho_neighbour(plot, OrthoDirection::Down);
            let west = grid.ortho_neighbour(plot, OrthoDirection::Left);

            let junctions = vec![
                vec![north, grid.cell_from(plot, (1, -1)), east],
                vec![east, grid.cell_from(plot, (1, 1)), south],
                vec![south, grid.cell_from(plot, (-1, 1)), west],
                vec![west, grid.cell_from(plot, (-1, -1)), north],
            ];

            for junction in junctions {
                // We are only interested in cases where the neighbours in question are BOTH equal
                // or BOTH unequal to the inspected cell. If only one is equal, then there is no
                // corner at that junction

                // If the junction plots are equal to the inspected plot, then there is only a
                // corner in the case that the cell between the junction plots is unequal to
                // the inspected plot. This is the most irritating corner type to detect.
                if junction[0].is_some_and(|cell| cell.contents == plot.contents)
                    && junction[1].is_some_and(|cell| cell.contents != plot.contents)
                    && junction[2].is_some_and(|cell| cell.contents == plot.contents)
                {
                    num_corners += 1;
                } else if (junction[0].is_none()
                    || junction[0].is_some_and(|cell| cell.contents != plot.contents))
                    && (junction[2].is_none()
                        || junction[2].is_some_and(|cell| cell.contents != plot.contents))
                {
                    // On the other hand, if both are unequal or none then there is definitely a
                    // corner and we can just carry on.

                    num_corners += 1;
                }
            }
        }

        num_corners
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
