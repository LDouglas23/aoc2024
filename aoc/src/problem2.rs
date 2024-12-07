pub type Report = Vec<usize>;

pub struct Input {
    reports: Vec<Report>,
}

trait Safe {
    fn safe(&self) -> bool;

    fn safe_dampened(&self) -> bool;
}

impl Safe for Report {
    fn safe(&self) -> bool {
        let diffs = self
            .windows(2)
            .map(|w| w[1] as isize - w[0] as isize)
            .collect::<Vec<isize>>();

        if !diffs.iter().all(|&d| d < 0) && !diffs.iter().all(|&d| d > 0) {
            return false;
        }

        if diffs.iter().any(|d| d.abs() > 3 || d.abs() < 1) {
            return false;
        }

        true
    }

    fn safe_dampened(&self) -> bool {
        self.iter()
            .enumerate()
            .map(|(i, _)| {
                let mut new_list = self.clone();
                new_list.remove(i);

                new_list.safe()
            })
            .any(|o| o)
    }
}

pub fn solution(input: Input) -> usize {
    input.reports.iter().filter(|&r| r.safe()).count()
}

pub fn solution_part_two(input: Input) -> usize {
    input.reports.iter().filter(|&r| r.safe_dampened()).count()
}

pub fn parse_lines(lines: Vec<String>) -> Input {
    let mut reports = vec![];

    for line in lines {
        let report: Report = line
            .split(" ")
            .map(|s| s.parse().expect("failed to parse"))
            .collect();

        reports.push(report);
    }

    Input { reports }
}

#[cfg(test)]
mod test {
    use super::{Report, Safe};
    use std::vec;

    #[test]
    fn test_safe() {
        let r1: Report = vec![7, 6, 4, 3, 1];
        let r2: Report = vec![1, 2, 7, 8, 9];
        let r3: Report = vec![9, 7, 6, 2, 1];
        let r4: Report = vec![1, 3, 2, 4, 5];
        let r5: Report = vec![8, 6, 4, 4, 1];
        let r6: Report = vec![1, 3, 6, 7, 9];

        assert!(r1.safe(), "r1 should have been safe, was unsafe");
        assert!(!r2.safe(), "r2 should have been unsafe, was safe");
        assert!(!r3.safe(), "r3 should have been unsafe, was safe");
        assert!(!r4.safe(), "r4 should have been unsafe, was safe");
        assert!(!r5.safe(), "r5 should have been unsafe, was safe");
        assert!(r6.safe(), "r6 should have been safe, was unsafe");
    }

    #[test]
    fn test_safe_dampened() {
        let r1: Report = vec![7, 6, 4, 3, 1];
        let r2: Report = vec![1, 2, 7, 8, 9];
        let r3: Report = vec![9, 7, 6, 2, 1];
        let r4: Report = vec![1, 3, 2, 4, 5];
        let r5: Report = vec![8, 6, 4, 4, 1];
        let r6: Report = vec![1, 3, 6, 7, 9];

        assert!(r1.safe_dampened(), "r1 should have been safe, was unsafe");
        assert!(!r2.safe_dampened(), "r2 should have been unsafe, was safe");
        assert!(!r3.safe_dampened(), "r3 should have been unsafe, was safe");
        assert!(r4.safe_dampened(), "r4 should have been safe, was unsafe");
        assert!(r5.safe_dampened(), "r5 should have been safe, was unsafe");
        assert!(r6.safe_dampened(), "r6 should have been safe, was unsafe");
    }
}
