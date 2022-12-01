type Elf = Vec<u32>;

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<Elf> {
    let mut es = vec![];
    let mut elf = vec![];
    input.lines().for_each(|e| match e.parse::<u32>() {
        Ok(cal) => elf.push(cal),
        Err(_) => {
            es.push(elf.clone());
            elf = vec![];
        }
    });

    es
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &[Elf]) -> u32 {
    input.iter().map(|elf| elf.iter().sum()).max().unwrap()
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &[Elf]) -> u32 {
    let mut sorted: Vec<u32> = input.iter().map(|elf| elf.iter().sum()).collect();

    sorted.sort_by(|a, b| b.cmp(a));

    sorted.iter().take(3).sum()
}

/*
AOC 2022
Day 1 - Part 1 : 71124
        generator: 131.83µs,
        runner: 3.669µs

Day 1 - Part 2 : 204639
        generator: 124.526µs,
        runner: 11.441µs
*/

#[cfg(test)]
mod tests {

    use super::*;

    const INPUT1: &str = r#"1000
    2000
    3000
    
    4000
    
    5000
    6000
    
    7000
    8000
    9000
    
    10000"#;
    #[test]
    fn sample1() {
        assert_eq!(solve_part1(&input_generator(INPUT1)), 1000);
    }
}
