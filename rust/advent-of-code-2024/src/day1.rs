use std::collections::HashMap;

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
    let iter = input.0.iter().zip(input.1.iter());

    iter.map(|(a, b)| a.abs_diff(*b)).sum()
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &(LocationIds, LocationIds)) -> u32 {
    let mut m: HashMap<u32, u32> = HashMap::new();

    input.1.iter().for_each(|v| *m.entry(*v).or_insert(0) += 1);

    input
        .0
        .iter()
        .map(|a| {
            let count = m.get(a).unwrap_or(&0u32);

            a * count
        })
        .sum()
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

    #[test]
    fn sample2() {
        assert_eq!(solve_part2(&input_generator(INPUT1)), 31);
    }
}
