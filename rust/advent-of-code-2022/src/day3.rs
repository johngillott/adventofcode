use std::collections::HashSet;

#[aoc_generator(day3, part1)]
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

#[aoc_generator(day3, part2)]
pub fn input_generator_part_2(input: &str) -> Vec<Vec<HashSet<char>>> {
    let v = input
        .lines()
        .map(|l| HashSet::from_iter(l.chars().collect::<Vec<char>>()))
        .collect::<Vec<HashSet<char>>>();

    v.chunks_exact(3)
        .map(|f| {
            let first = f.get(0).unwrap();
            let second = f.get(1).unwrap();
            let third = f.get(2).unwrap();

            vec![first.clone(), second.clone(), third.clone()]
        })
        .collect()
}

#[aoc(day3, part2)]
pub fn solve_part2(input: &[Vec<HashSet<char>>]) -> u32 {
    input
        .iter()
        .map(|f| {
            let mut intersection = f[0]
                .iter()
                .filter(move |c| f[1..].iter().all(|s| s.contains(c)));

            let badge = *intersection.next().unwrap();

            let offset = match badge.is_lowercase() {
                true => 96,
                false => 38,
            };

            badge as u32 - offset
        })
        .sum()
}

/*
AOC 2022
Day 3 - Part 1 : 8153
        generator: 638.245µs,
        runner: 22.928µs

Day 3 - Part 2 : 2342
        generator: 613.812µs,
        runner: 20.761µs
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
