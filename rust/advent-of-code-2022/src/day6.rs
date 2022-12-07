#[aoc_generator(day6)]
pub fn input_generator(input: &str) -> String {
    input.lines().next().unwrap().to_string()
}

#[aoc(day6, part1)]
pub fn solve_part1(input: &String) -> usize {
    use itertools::Itertools;
    let iter = input.chars().collect::<Vec<char>>();

    let marker = iter
        .windows(4)
        .take_while(|s| {
            let u = s.into_iter().unique().collect::<Vec<_>>();

            s.len() != u.len()
        })
        .count();

    marker + 4
}

#[aoc(day6, part2)]
pub fn solve_part2(input: &String) -> usize {
    use itertools::Itertools;
    let iter = input.chars().collect::<Vec<char>>();

    let marker = iter
        .windows(14)
        .take_while(|s| {
            let u = s.into_iter().unique().collect::<Vec<_>>();

            s.len() != u.len()
        })
        .count();

    marker + 14
}

/*
AOC 2022
Day 6 - Part 1 : 1282
        generator: 3.388µs,
        runner: 351.574µs

Day 6 - Part 2 : 3513
        generator: 633ns,
        runner: 3.93227ms
*/

#[cfg(test)]
mod tests {

    use super::*;

    const INPUT1: &str = r#"bvwbjplbgvbhsrlpgdmjqwftvncz"#;
    #[test]
    fn sample1() {
        assert_eq!(solve_part1(&input_generator(INPUT1)), 5);
    }

    #[test]
    fn sample2() {
        assert_eq!(solve_part2(&input_generator(INPUT1)), 19);
    }
}
