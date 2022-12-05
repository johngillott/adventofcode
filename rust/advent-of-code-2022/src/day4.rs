#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Vec<u32> {
    input
        .lines()
        .flat_map(|s| s.split(',').collect::<Vec<_>>())
        .flat_map(|s| s.split('-').collect::<Vec<_>>())
        .map(|s: _| s.parse::<u32>().unwrap())
        .collect()
}

#[aoc(day4, part1)]
pub fn solve_part1(input: &[u32]) -> u32 {
    input
        .chunks_exact(4)
        .map(|f| {
            let first = f.first().unwrap();
            let second = f.get(1).unwrap();
            let third = f.get(2).unwrap();
            let fourth = f.get(3).unwrap();

            first >= third && second <= fourth || third >= first && fourth <= second
        })
        .filter(|b| *b)
        .count() as u32
}

#[aoc(day4, part2)]
pub fn solve_part2(input: &[u32]) -> u32 {
    input
        .chunks_exact(4)
        .map(|f| {
            let first = f.first().unwrap();
            let second = f.get(1).unwrap();
            let third = f.get(2).unwrap();
            let fourth = f.get(3).unwrap();

            first <= fourth && third <= second
        })
        .filter(|b| *b)
        .count() as u32
}

/*
AOC 2022
Day 4 - Part 1 : 431
        generator: 474.39µs,
        runner: 8.776µs

Day 4 - Part 2 : 823
        generator: 560.936µs,
        runner: 1.769µs
*/

#[cfg(test)]
mod tests {

    use super::*;

    const INPUT1: &str = r#"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
"#;
    #[test]
    fn sample1() {
        assert_eq!(solve_part1(&input_generator(INPUT1)), 2);
    }

    const INPUT2: &str = r#"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
"#;
    #[test]
    fn sample2() {
        assert_eq!(solve_part2(&input_generator(INPUT2)), 4);
    }

    const INPUT3: &str = r#"78-78,27-77
"#;
    #[test]
    fn sample3() {
        assert_eq!(solve_part2(&input_generator(INPUT3)), 0);
    }

    const INPUT4: &str = r#"2-4,6-8
"#;
    #[test]
    fn sample4() {
        assert_eq!(solve_part2(&input_generator(INPUT4)), 0);
    }
}
