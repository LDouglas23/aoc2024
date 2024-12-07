use regex::Regex;

pub struct Input {
    memory: String,
}

pub fn solution(input: Input) -> usize {
    let re = Regex::new("mul\\((\\d+),(\\d+)\\)").expect("failed to compile regex");

    re.captures_iter(&input.memory)
        .map(|caps| {
            let (_, [x, y]) = caps.extract();

            x.parse::<usize>().expect("failed to parse")
                * y.parse::<usize>().expect("failed to parse")
        })
        .sum()
}

pub fn solution_part_two(input: Input) -> usize {
    input
        .memory
        .split("do()")
        .map(|line| line.split("don't()").collect::<Vec<&str>>()[0])
        .map(|do_line| {
            solution(Input {
                memory: do_line.to_owned(),
            })
        })
        .sum()
}

pub fn parse_lines(lines: Vec<String>) -> Input {
    Input {
        memory: lines.join(""),
    }
}

#[cfg(test)]
mod test {
    use super::{solution, Input};

    #[test]
    pub fn test() {
        let s1 = "mul(3,4)";
        let s2 = "mul(3,4)mul(2,2)";
        let s3 = "mul(3,4)xxxxxmul(2,2)";

        assert_eq!(
            solution(Input {
                memory: s1.to_owned()
            }),
            12
        );

        assert_eq!(
            solution(Input {
                memory: s2.to_owned()
            }),
            16
        );

        assert_eq!(
            solution(Input {
                memory: s3.to_owned()
            }),
            16
        );
    }
}
