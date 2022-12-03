#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Vec<(String, String)> {
    use itertools::Itertools;

    input
        .lines()
        .map(|s| {
            let t = s.trim_matches('\n');
            let v = t.split_at(t.len() / 2);
            (
                v.0.chars().into_iter().unique().collect(),
                v.1.chars().into_iter().unique().collect(),
            )
        })
        .collect()
}

#[aoc(day3, part1)]
pub fn solve_part1(input: &[(String, String)]) -> u32 {
    input
        .iter()
        .map(|compartments| {
            compartments.0.chars().fold(0, |acc, x| {
                if compartments.1.contains(x) {
                    let offset = match x.is_lowercase() {
                        true => 96,
                        false => 38,
                    };

                    return acc + x as u32 - offset;
                }

                acc
            })
        })
        .sum()
}

/*
AOC 2022
Day 3 - Part 1 : 8153
        generator: 668.467µs,
        runner: 30.877µs
*/

#[cfg(test)]
mod tests {

    use super::*;

    const INPUT1: &str = r#"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
"#;
    #[test]
    fn sample1() {
        assert_eq!(solve_part1(&input_generator(INPUT1)), 157);
    }
}
