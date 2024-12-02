type Reports = Vec<Vec<u32>>;

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Reports {
    input
        .lines()
        .map(|line: &str| line.split(" ").map(|v| v.parse::<u32>().unwrap()).collect())
        .collect()
}

#[aoc(day2, part1)]
pub fn solve_part1(input: &Reports) -> u32 {
    use std::cmp::Reverse;

    input
        .iter()
        .map(|r| {
            let mut asc_sorted = r.clone();
            asc_sorted.sort();

            let mut desc_sorted = r.clone();
            desc_sorted.sort_by_key(|w| Reverse(*w));

            if *r == asc_sorted || *r == desc_sorted {
                return r
                    .windows(2)
                    .filter(|w| {
                        let abs = w[0].abs_diff(w[1]);
                        abs > 3 || abs == 0
                    })
                    .count()
                    == 0;
            } else {
                return false;
            }
        })
        .filter(|safe_reports| *safe_reports)
        .count() as u32
}

#[cfg(test)]
mod tests {

    use super::*;

    const INPUT1: &str = r#"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"#;
    #[test]
    fn sample1() {
        assert_eq!(solve_part1(&input_generator(INPUT1)), 2);
    }
}
