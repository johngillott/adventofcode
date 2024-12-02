type LocationIds = Vec<u32>;

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> (LocationIds, LocationIds) {
    let mut left = vec![];
    let mut right = vec![];
    input.lines().for_each(|line: &str| {
        let mut res: Vec<u32> = line
            .split("   ")
            .map(|v| v.parse::<u32>().unwrap())
            .collect();

        left.push(res.pop().unwrap());

        right.push(res.pop().unwrap());
    });

    left.sort();
    right.sort();

    (left, right)
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &(LocationIds, LocationIds)) -> u32 {
    // input.iter().map(|elf| elf.iter().sum()).max().unwrap()

    let iter = input.0.iter().zip(input.1.iter());

    iter.map(|(a, b)| a.abs_diff(*b)).sum()
}

/*
AOC 2024
*/

#[cfg(test)]
mod tests {

    use super::*;

    const INPUT1: &str = r#"3   4
4   3
2   5
1   3
3   9
3   3"#;
    #[test]
    fn sample1() {
        assert_eq!(solve_part1(&input_generator(INPUT1)), 11);
    }
}
