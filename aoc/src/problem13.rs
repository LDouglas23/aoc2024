use crate::common::Vector2di;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref BUTTON_A_RE: Regex = Regex::new(r"Button A: X\+(\d+), Y\+(\d+)").unwrap();
    static ref BUTTON_B_RE: Regex = Regex::new(r"Button B: X\+(\d+), Y\+(\d+)").unwrap();
    static ref PRIZE_RE: Regex = Regex::new(r"Prize: X=(\d+), Y=(\d+)").unwrap();
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Strategy {
    a: i64,
    b: i64,
}

impl Strategy {
    fn token_cost(&self) -> usize {
        (self.a * 3 + self.b) as usize
    }
}

#[derive(Debug, Clone, Copy)]
struct Machine {
    button_a: Vector2di,
    button_b: Vector2di,
    prize: Vector2di,
}

impl Machine {
    fn is_winning(&self, strategy: &Strategy) -> bool {
        (self.button_a * strategy.a) + (self.button_b * strategy.b) == self.prize
    }

    fn modify_precision(&mut self) {
        self.prize.x += 10000000000000;
        self.prize.y += 10000000000000;
    }
}

impl From<&[String]> for Machine {
    fn from(value: &[String]) -> Self {
        let button_a = BUTTON_A_RE
            .captures_iter(&value[0])
            .map(|caps| {
                let (_, [x, y]) = caps.extract();

                Vector2di::new(x.parse().unwrap(), y.parse().unwrap())
            })
            .collect::<Vec<Vector2di>>()[0];

        let button_b = BUTTON_B_RE
            .captures_iter(&value[1])
            .map(|caps| {
                let (_, [x, y]) = caps.extract();

                Vector2di::new(x.parse().unwrap(), y.parse().unwrap())
            })
            .collect::<Vec<Vector2di>>()[0];

        let prize = PRIZE_RE
            .captures_iter(&value[2])
            .map(|caps| {
                let (_, [x, y]) = caps.extract();

                Vector2di::new(x.parse().unwrap(), y.parse().unwrap())
            })
            .collect::<Vec<Vector2di>>()[0];

        Self {
            button_a,
            button_b,
            prize,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Input {
    machines: Vec<Machine>,
}

impl From<Vec<String>> for Input {
    fn from(value: Vec<String>) -> Self {
        let machines = value
            .split(|line| line.is_empty())
            .map(Machine::from)
            .collect();

        Self { machines }
    }
}

pub fn solution(input: Input) -> usize {
    let mut tokens = 0;

    for machine in &input.machines {
        let a = machine.button_a;
        let b = machine.button_b;

        let x_candidate = machine.prize.x / a.x;
        let y_candidate = machine.prize.y / a.y;

        let mut strategies: Vec<Strategy> = vec![];

        for x in (0..x_candidate).rev() {
            let remaining_x = machine.prize.x - (x * a.x);

            if remaining_x % b.x == 0 {
                strategies.push(Strategy {
                    a: x,
                    b: (remaining_x / b.x),
                })
            };
        }

        for y in (0..y_candidate).rev() {
            let remaining_y = machine.prize.y - (y * a.y);

            if remaining_y % b.y == 0 {
                let strategy = Strategy {
                    a: y,
                    b: (remaining_y / b.y),
                };

                if !strategies.contains(&strategy) {
                    strategies.push(strategy);
                }
            }
        }

        tokens += strategies
            .iter()
            .filter(|strat| machine.is_winning(strat))
            .map(|strat| strat.token_cost())
            .min()
            .unwrap_or(0);
    }

    tokens
}

/*
    Each axis of the machine is a linear equation with integer solutions:

        AX + BY = T1 (x-axis)
        CX + BY = T2 (y-axis)

    where X is the number of A button presses and Y is the number of B button presses. Since these
    equations are linear, they intersect only once. If the coordinates of intersection (X,Y) are
    integers, then the machine has a valid solution, otherwise no solution exists.

    We can rearrange the equations to find expressions for X and Y in terms of A,B,C,D,T1,T2 which
    are all known:

        X = (T1D - T2B) / AD - BC
        Y = (T2A - T1C) / AD - BC

*/
pub fn solution_part_two(input: Input) -> usize {
    let mut modified_input = input.clone();
    let mut token_cost = 0;

    modified_input
        .machines
        .iter_mut()
        .for_each(|machine| machine.modify_precision());

    for m in &modified_input.machines {
        let (a, b, c, d, t1, t2) = (
            m.button_a.x,
            m.button_b.x,
            m.button_a.y,
            m.button_b.y,
            m.prize.x,
            m.prize.y,
        );

        let quotient = a * d - b * c;

        let x = t1 * d - t2 * b;
        let y = t2 * a - t1 * c;

        if x % quotient == 0 && y % quotient == 0 {
            token_cost += (x / quotient * 3) + y / quotient;
        }
    }

    token_cost as usize
}

#[cfg(test)]
mod test {

    use super::{solution, solution_part_two, Input};

    const EXAMPLE: &'static str = "\
Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279
";

    #[test]
    fn test() {
        let input = Input::from(EXAMPLE.lines().map(String::from).collect::<Vec<String>>());

        assert_eq!(solution(input), 480);
    }

    #[test]
    fn test2() {
        let input = Input::from(EXAMPLE.lines().map(String::from).collect::<Vec<String>>());

        let result = solution_part_two(input);

        println!("{}", result);
    }
}
