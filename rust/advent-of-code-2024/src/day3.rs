use regex::Regex;

type Input = Vec<(u32, u32)>;

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Input {
    Regex::new(r"mul\(((\d+),(\d+))\)")
        .unwrap()
        .captures_iter(input)
        .map(|cap| {
            let (_, [_, a, b]) = cap.extract();
            (a.parse::<u32>().unwrap(), b.parse::<u32>().unwrap())
        })
        .collect()
}

#[aoc(day3, part1)]
pub fn solve_part1(input: &Input) -> u32 {
    input.iter().map(|v| v.0 * v.1).sum()
}

#[cfg(test)]
mod tests {

    use super::*;

    const INPUT: &str =
        r#"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"#;
    #[test]
    fn example_1() {
        assert_eq!(solve_part1(&input_generator(INPUT)), 161);
    }
}
