type Page = usize;

#[derive(Debug, Clone, Copy)]
struct Rule {
    first: Page,
    second: Page,
}

impl From<String> for Rule {
    fn from(value: String) -> Self {
        let pages = value.split("|").map(String::from).collect::<Vec<String>>();

        assert!(pages.len() == 2);

        Self {
            first: pages[0].parse().expect("failed to parse digits"),
            second: pages[1].parse().expect("failed to parse digits"),
        }
    }
}

impl Rule {
    fn applies_to(&self, page: Page) -> bool {
        self.first == page || self.second == page
    }
}

#[derive(Debug, Clone)]
struct Update {
    pages: Vec<Page>,
}

impl From<String> for Update {
    fn from(value: String) -> Self {
        Self {
            pages: value.split(",").map(|s| s.parse().unwrap()).collect(),
        }
    }
}

#[derive(Debug)]
pub struct Input {
    rules: Vec<Rule>,
    updates: Vec<Update>,
}

pub fn parse_lines(lines: Vec<String>) -> Input {
    let mut iter = lines.iter();

    let rules = iter
        .by_ref()
        .take_while(|line| !line.is_empty())
        .cloned()
        .map(|s| Rule::from(s))
        .collect();

    let updates = iter.cloned().map(Update::from).collect();

    Input { rules, updates }
}

impl Update {
    fn well_ordered(&self, rules: &[Rule]) -> bool {
        rules.iter().all(self.satisfies)
    }

    fn satisfies(&self, rule: &Rule) -> bool {
        if !self.pages.contains(&rule.first) || !self.pages.contains(&rule.second) {
            return true;
        };

        let i1 = self.pages.iter().position(|&p| p == rule.first).unwrap();

        let i2 = self.pages.iter().position(|&p| p == rule.second).unwrap();

        return i1 < i2;
    }

    fn middle_page(&self) -> Page {
        self.pages[self.pages.len() / 2]
    }

    fn sorted(&self, rules: &[Rule]) -> Self {
        let applicable_rules: Vec<Rule> = rules
            .iter()
            .cloned()
            .filter(|&rule| self.pages.iter().any(|&page| rule.applies_to(page)))
            .collect();

        let mut output: Vec<Page> = vec![];

        for page in &self.pages {
            let mut i = 0;

            for el in &output {
                if applicable_rules
                    .iter()
                    .any(|rule| rule.first == *page && rule.second == *el)
                {
                    break;
                }

                i += 1;
            }

            output.insert(i, *page);
        }

        Self { pages: output }
    }
}

pub fn solution(input: Input) -> usize {
    input
        .updates
        .iter()
        .filter(|update| update.well_ordered(&input.rules))
        .map(|good_update| good_update.middle_page())
        .sum()
}

pub fn solution_part_two(input: Input) -> usize {
    input
        .updates
        .iter()
        .filter(|update| !update.well_ordered(&input.rules))
        .map(|update| update.sorted(&input.rules))
        .map(|sorted_update| sorted_update.middle_page())
        .sum()
}

#[cfg(test)]
mod test {
    use super::{parse_lines, solution, solution_part_two};

    const TEST: &'static str = "\
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    #[test]
    fn test() {
        let input = parse_lines(TEST.split("\n").map(String::from).collect());

        assert_eq!(solution(input), 143);
    }

    #[test]
    fn test_2() {
        let input = parse_lines(TEST.split("\n").map(String::from).collect());

        assert_eq!(solution_part_two(input), 123);
    }
}
