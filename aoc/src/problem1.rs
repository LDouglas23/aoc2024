pub struct Input {
    left_list: Vec<usize>,
    right_list: Vec<usize>,
}

pub fn solution(input: Input) -> usize {
    let mut left_list = input.left_list;
    let mut right_list = input.right_list;

    assert!(left_list.len() == right_list.len());

    left_list.sort();
    right_list.sort();

    left_list
        .iter()
        .zip(right_list)
        .map(|(x, y)| x.abs_diff(y))
        .sum()
}

pub fn solution_part_two(input: Input) -> usize {
    let mut score: usize = 0;

    for x in input.left_list {
        let similarity = x * input.right_list.iter().filter(|&y| *y == x).count();

        score += similarity
    }

    score
}

pub fn parse_lines(lines: Vec<String>) -> Input {
    let mut left = vec![];
    let mut right = vec![];

    for line in lines {
        let nums: Vec<&str> = line.split("   ").collect();

        assert!(nums.len() == 2);

        left.push(nums[0].parse().expect("failed to parse"));
        right.push(nums[1].parse().expect("failed to parse"));
    }

    Input {
        left_list: left,
        right_list: right,
    }
}
