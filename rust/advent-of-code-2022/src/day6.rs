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

/*
AOC 2022
Day 6 - Part 1 : 1282
        generator: 4.304µs,
        runner: 298.151µs
*/

#[cfg(test)]
mod tests {

    use super::*;

    const INPUT1: &str = r#"bvwbjplbgvbhsrlpgdmjqwftvncz"#;
    #[test]
    fn sample1() {
        assert_eq!(solve_part1(&input_generator(INPUT1)), 5);
    }
}
