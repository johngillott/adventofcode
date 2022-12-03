// TODO refactor input_generator to isolate part 1 and part 2 logic
#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<(char, char)> {
    input
        .lines()
        .map(|x| {
            let mut strategy: (char, char) = (' ', ' ');

            match x.chars().nth(0).unwrap() {
                'A' => strategy.0 = 'R',
                'B' => strategy.0 = 'P',
                'C' => strategy.0 = 'S',
                _ => unreachable!(),
            };

            // : X means you need to lose,
            // Y means you need to end the round in a draw,
            // and Z means you need to win.

            match x.chars().nth(2).unwrap() {
                'X' => match strategy.0 {
                    // lose
                    'R' => strategy.1 = 'S',
                    'P' => strategy.1 = 'R',
                    'S' => strategy.1 = 'P',
                    _ => unreachable!(),
                },
                'Y' => strategy.1 = strategy.0,
                'Z' => match strategy.0 {
                    // win
                    'R' => strategy.1 = 'P',
                    'P' => strategy.1 = 'S',
                    'S' => strategy.1 = 'R',
                    _ => unreachable!(),
                },
                _ => unreachable!(),
            };

            strategy
        })
        .collect::<Vec<(char, char)>>()
}

use std::cmp::Ordering;

fn cmp(p: &(char, char)) -> Ordering {
    match (p.0, p.1) {
        ('R', 'P') => Ordering::Greater,
        ('R', 'R') => Ordering::Equal,
        ('R', 'S') => Ordering::Less,
        ('P', 'P') => Ordering::Equal,
        ('P', 'R') => Ordering::Less,
        ('P', 'S') => Ordering::Greater,
        ('S', 'P') => Ordering::Less,
        ('S', 'R') => Ordering::Greater,
        ('S', 'S') => Ordering::Equal,
        _ => unreachable!(),
    }
}

#[aoc(day2, part1)]
pub fn solve_part1(input: &[(char, char)]) -> u32 {
    use std::cmp::Ordering;

    input
        .iter()
        .map(|f| {
            let mut score = 0;

            match cmp(f) {
                Ordering::Less => match f.1 {
                    'R' => score = score + 1,
                    'P' => score = score + 2,
                    'S' => score = score + 3,
                    _ => unimplemented!(),
                },
                Ordering::Equal => match f.1 {
                    'R' => score = score + 1 + 3,
                    'P' => score = score + 2 + 3,
                    'S' => score = score + 3 + 3,
                    _ => unimplemented!(),
                },
                Ordering::Greater => match f.1 {
                    'R' => score = score + 1 + 6,
                    'P' => score = score + 2 + 6,
                    'S' => score = score + 3 + 6,
                    _ => unimplemented!(),
                },
            }

            score
        })
        .sum()
}

#[aoc(day2, part2)]
pub fn solve_part2(input: &[(char, char)]) -> u32 {
    use std::cmp::Ordering;

    input
        .iter()
        .map(|f| {
            let mut score = 0;

            match cmp(f) {
                Ordering::Less => match f.1 {
                    'R' => score = score + 1,
                    'P' => score = score + 2,
                    'S' => score = score + 3,
                    _ => unimplemented!(),
                },
                Ordering::Equal => match f.1 {
                    'R' => score = score + 1 + 3,
                    'P' => score = score + 2 + 3,
                    'S' => score = score + 3 + 3,
                    _ => unimplemented!(),
                },
                Ordering::Greater => match f.1 {
                    'R' => score = score + 1 + 6,
                    'P' => score = score + 2 + 6,
                    'S' => score = score + 3 + 6,
                    _ => unimplemented!(),
                },
            }

            score
        })
        .sum()
}

#[cfg(test)]
mod tests {

    /*
        A X Rock
        B Y Paper
        C Z Scissors

    */
    use super::*;

    const INPUT1: &str = r#"A Y
B X
C Z"#;
    #[test]
    fn sample1() {
        assert_eq!(solve_part1(&input_generator(INPUT1)), 15);
    }

    // your opponent will choose Rock (A), and you should choose Paper (Y).
    // This ends in a win for you with a score of 8 (2 because you chose Paper + 6 because you won).
    #[test]
    fn sample2() {
        assert_eq!(solve_part1(&input_generator(r#"A Y"#)), 8);
    }

    // In the second round, your opponent will choose Paper (B), and you should choose Rock (X).
    // This ends in a loss for you with a score of 1 (1 + 0).
    #[test]
    fn sample3() {
        assert_eq!(solve_part1(&input_generator(r#"B X"#)), 1);
    }

    // The third round is a draw with both players choosing Scissors, giving you a score of 3 + 3 = 6.
    #[test]
    fn sample4() {
        assert_eq!(solve_part1(&input_generator(r#"C Z"#)), 6);
    }

    // Draw every match
    const INPUT2: &str = r#"A X
B Y
C Z"#;
    #[test]
    fn sample5() {
        assert_eq!(solve_part1(&input_generator(INPUT2)), 15);
    }

    // Win every match
    const WIN_EVERY: &str = r#"A Y
B Z
C X"#;
    #[test]
    fn sample6() {
        assert_eq!(solve_part1(&input_generator(WIN_EVERY)), 24);
    }

    // Lose every match
    const LOSE_EVERY: &str = r#"A Z
B X
C Y"#;
    #[test]
    fn sample7() {
        assert_eq!(solve_part1(&input_generator(LOSE_EVERY)), 6);
    }

    //     const INPUT2: &str = r#"C X
    // B Y
    // C Z"#;
    //     #[test]
    //     fn sample2() {
    //         assert_eq!(solve_part1(&input_generator(INPUT2)), 12);
    //     }
}
