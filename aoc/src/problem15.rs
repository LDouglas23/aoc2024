use std::fmt::{Debug, Display, Write};

use crate::common::{Grid, OrthoDirection, Vector2di};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum WarehouseEntity {
    Nothing,
    Wall,
    Box,
}

#[derive(Debug, Clone)]
struct Warehouse {
    grid: Grid<WarehouseEntity>,
    robot: Vector2di,
}

impl From<char> for WarehouseEntity {
    fn from(value: char) -> Self {
        match value {
            '#' => WarehouseEntity::Wall,
            'O' => WarehouseEntity::Box,
            _ => WarehouseEntity::Nothing,
        }
    }
}

impl Into<char> for WarehouseEntity {
    fn into(self) -> char {
        match self {
            WarehouseEntity::Nothing => '.',
            WarehouseEntity::Wall => '#',
            WarehouseEntity::Box => 'O',
        }
    }
}

impl From<Vec<String>> for Warehouse {
    fn from(value: Vec<String>) -> Self {
        let entities: Vec<Vec<WarehouseEntity>> = value
            .iter()
            .map(|s| s.chars().map(WarehouseEntity::from).collect())
            .collect();

        let row = value.iter().position(|s| s.contains('@')).unwrap();
        let col = value[row].chars().position(|c| c == '@').unwrap();

        Self {
            grid: Grid::from(entities),
            robot: Vector2di {
                x: col as i64,
                y: row as i64,
            },
        }
    }
}

impl Display for Warehouse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for cell in self.grid.clone().into_iter() {
            if cell.x == 0 {
                f.write_str("\n")?;
            }

            if self.robot == Vector2di::new(cell.x as i64, cell.y as i64) {
                f.write_char('@')?;
            } else {
                f.write_char(cell.contents.into())?
            }
        }

        Ok(())
    }
}

impl Warehouse {
    fn move_robot(&mut self, dir: OrthoDirection) {
        let target_square = self.robot + dir.into();

        let push_result = self.push_result(target_square, dir);

        self.grid.swap(self.robot, push_result);
        self.robot = push_result;
    }

    /*
        Pushes the entity at the target square and returns the position that the pushing entity
        should move to
    */
    fn push_result(&mut self, push_target: Vector2di, dir: OrthoDirection) -> Vector2di {
        let entity = self
            .grid
            .get(push_target)
            .expect("attempted to push outside grid")
            .contents;

        match entity {
            WarehouseEntity::Nothing => push_target,
            WarehouseEntity::Wall => push_target - dir.into(),
            WarehouseEntity::Box => {
                let next_target = self.push_result(push_target + dir.into(), dir);

                self.grid.swap(push_target, next_target);

                next_target - dir.into()
            }
        }
    }

    fn get_total_gps(&self) -> usize {
        let mut total = 0;

        for cell in self.grid.clone().into_iter() {
            if cell.contents == WarehouseEntity::Box {
                total += cell.x + cell.y * 100;
            }
        }

        total
    }
}

#[derive(Debug, Clone)]
struct Moves {
    moves: Vec<OrthoDirection>,
}

impl From<Vec<String>> for Moves {
    fn from(value: Vec<String>) -> Self {
        let m = value.join("");

        Self {
            moves: m
                .chars()
                .map(|c| match c {
                    '<' => OrthoDirection::Left,
                    '^' => OrthoDirection::Up,
                    '>' => OrthoDirection::Right,
                    'v' => OrthoDirection::Down,
                    _ => panic!("unrecognised direction {}", c),
                })
                .collect(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Input {
    warehouse: Warehouse,
    moves: Moves,
}

impl From<Vec<String>> for Input {
    fn from(value: Vec<String>) -> Self {
        let mut iter = value.split(|s| s.is_empty());

        let warehouse = iter.next().unwrap().to_owned();
        let moves = iter.next().unwrap().to_owned();

        Self {
            warehouse: Warehouse::from(warehouse),
            moves: Moves::from(moves),
        }
    }
}

impl Input {
    fn solve(&mut self) {
        let mut iter = self.moves.moves.iter();

        while let Some(dir) = iter.next() {
            self.warehouse.move_robot(*dir);
        }

        println!("{}", self.warehouse);
    }
}

pub fn solution(input: Input) -> usize {
    let mut input = input.clone();

    input.solve();

    input.warehouse.get_total_gps()
}

#[cfg(test)]
mod test {
    use super::{solution, Input};

    const EXAMPLE: &'static str = "\
##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
";

    #[test]
    fn test() {
        let input = Input::from(EXAMPLE.lines().map(String::from).collect::<Vec<String>>());

        assert_eq!(solution(input), 10092);
    }
}
