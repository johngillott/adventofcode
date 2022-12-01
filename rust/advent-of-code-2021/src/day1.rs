#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<u32> {
    input
        .lines()
        .map(|l| l.parse::<u32>().unwrap())
        .collect::<Vec<u32>>()
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &[u32]) -> usize {
    input.windows(2).filter(|pair| pair[0] < pair[1]).count()
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &[u32]) -> usize {
    let sum_of_measurements = input
        .windows(3)
        .map(|tuple| tuple.iter().sum())
        .collect::<Vec<u32>>();

    sum_of_measurements
        .windows(2)
        .filter(|pair| pair[0] < pair[1])
        .count()
}

/*
AOC 2021
Day 1 - Part 1 : 1475
        generator: 40.558µs,
        runner: 2.279µs

Day 1 - Part 2 : 1516
        generator: 36.966µs,
        runner: 1.274µs
*/

#[cfg(test)]
mod tests {

    use super::*;

    const INPUT1: &str = r#"199
200
208
210
200
207
240
269
260
263
"#;
    #[test]
    fn sample1() {
        assert_eq!(solve_part1(&input_generator(INPUT1)), 7);
    }
}
