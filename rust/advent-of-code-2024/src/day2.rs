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
                return is_safe(r);
            } else {
                return false;
            }
        })
        .filter(|safe_reports| *safe_reports)
        .count() as u32
}

fn is_safe(data: &[u32]) -> bool {
    return data
        .windows(2)
        .filter(|w| {
            let abs = w[0].abs_diff(w[1]);
            abs > 3 || abs == 0
        })
        .count()
        == 0;
}

#[derive(Eq, PartialEq, Debug)]
enum ReportKind {
    Safe,
    Unsafe(usize),
}

fn is_safe_2(data: &[u32]) -> ReportKind {
    // use std::cmp::Reverse;
    // let mut asc_sorted = data.clone();
    // asc_sorted.sort();

    // let mut desc_sorted = data.clone();
    // desc_sorted.sort_by_key(|w| Reverse(*w));

    // if data == asc_sorted || data == desc_sorted {
    match data.windows(2).position(|w| {
        let abs = w[0].abs_diff(w[1]);
        ![1, 2, 3].contains(&abs)
    }) {
        Some(idx) => return ReportKind::Unsafe(idx),
        None => return ReportKind::Safe,
    }
    // }
}

#[aoc(day2, part2)]
pub fn solve_part2(input: &Reports) -> u32 {
    use std::cmp::Reverse;
    input
        .iter()
        .map(|r| {
            let mut asc_sorted = r.clone();
            asc_sorted.sort();

            let mut desc_sorted = r.clone();
            desc_sorted.sort_by_key(|w| Reverse(*w));

            if *r == asc_sorted || *r == desc_sorted {
                match is_safe_2(r) {
                    ReportKind::Safe => true,
                    ReportKind::Unsafe(index) => {
                        ReportKind::Safe == {
                            let mut new_line = r.clone();
                            new_line.remove(index);
                            is_safe_2(&new_line)
                        }
                    }
                }
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
    use rstest::rstest;

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

    #[test]
    fn sample2() {
        assert_eq!(solve_part2(&input_generator(INPUT1)), 4);
    }

    #[rstest]
    #[case(&[7, 6, 4, 2, 1].to_vec(), true)]
    #[case(&[7, 6, 5, 1, 1].to_vec(), false)]
    fn test_is_safe(#[case] input: &[u32], #[case] expected: bool) {
        assert_eq!(is_safe(input), expected);
    }

    #[rstest]
    #[case(&[7, 6, 4, 2, 1].to_vec(), ReportKind::Safe)]
    #[case(&[1, 2, 7, 8, 9].to_vec(), ReportKind::Unsafe(1))]
    #[case(&[9, 7, 6, 2, 1].to_vec(), ReportKind::Unsafe(2))]
    // check for removing badly ordered of each elements
    #[case(&[1, 3, 2, 4, 5].to_vec(), ReportKind::Unsafe(1))]
    #[case(&[8, 6, 4, 4, 1].to_vec(), ReportKind::Unsafe(2))]
    #[case(&[1, 3, 6, 7, 9].to_vec(), ReportKind::Safe)]
    fn test_is_safe_2(#[case] input: &[u32], #[case] expected: ReportKind) {
        assert_eq!(is_safe_2(input), expected);
    }
}
