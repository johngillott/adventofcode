use regex::Regex;

type Input = Vec<Instruction>;

enum Instruction {
    Enable,
    Disable,
    Multiply(i32, i32),
}

#[aoc_generator(day3)]
fn input_generator(input: &str) -> Input {
    Regex::new(r"((mul)|(do(n't)?))\(((\d+),(\d+))?\)")
        .unwrap()
        .captures_iter(input)
        .map(|cap| match cap.get(1).unwrap().as_str() {
            "don't" => Instruction::Disable,
            "do" => Instruction::Enable,
            "mul" => {
                let a = cap.get(6).unwrap().as_str().parse().unwrap();
                let b = cap.get(7).unwrap().as_str().parse().unwrap();
                Instruction::Multiply(a, b)
            }
            _ => panic!(),
        })
        .collect()
}

#[aoc(day3, part1)]
fn solve_part1(input: &Input) -> i32 {
    input
        .iter()
        .map(|v| match v {
            Instruction::Multiply(a, b) => a * b,
            _ => 0,
        })
        .sum()
}

#[aoc(day3, part2)]
fn solve_part2(input: &Input) -> i32 {
    let mut enabled = true;

    let mut sum = 0;

    for instruction in input.iter() {
        match instruction {
            Instruction::Multiply(a, b) => {
                if enabled {
                    sum = sum + a * b
                }
            }
            Instruction::Enable => enabled = true,
            Instruction::Disable => enabled = false,
        }
    }
    sum
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

    #[test]
    fn example_2() {
        assert_eq!(solve_part2(&input_generator(INPUT)), 161);
    }
}
