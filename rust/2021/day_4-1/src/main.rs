use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args);
    let mut file = File::open(dbg!(config.filename)).expect("Unable to open file");

    let mut input = String::new();
    file.read_to_string(&mut input)
        .expect("Unable to load file contents {}");

    let (numbers, mut squares) = parse_input(&input);

    // print!("{:?}\n{:?}", numbers, &squares);

    for n in numbers {
        for sq in squares.iter_mut() {
            mark(&n, sq);

            if check(sq) {
                let sum = sq.iter().flatten().filter(|x| **x != -1).sum::<i32>();
                println!("sum={} n={}, product={}", sum, n, sum * n,);
                return;
            }
        }
    }
}

fn parse_input(input: &str) -> (Vec<i32>, Vec<Vec<Vec<i32>>>) {
    let mut iter = input.lines();
    let list_of_numbers = iter.next().expect("first line must be be stringy numbers");

    let numbers = list_of_numbers
        .split(',')
        .map(|s| s.parse::<i32>().expect("each value must be a number"))
        .collect::<Vec<i32>>();

    let result = iter.collect::<Vec<&str>>();

    // TODO: inline iter.next()
    let mut iter = result.chunks(6);
    let mut squares = vec![];
    while let Some(s) = iter.next() {
        squares.push(
            s.iter()
                .skip(1)
                .map(|x| {
                    x.split(' ')
                        .filter(|x| !x.is_empty())
                        .map(|s| s.parse::<i32>().expect("each value must be a number"))
                        .collect::<Vec<i32>>()
                })
                .collect::<Vec<Vec<i32>>>(),
        );
    }

    (numbers, squares)
}

fn mark<'a>(number: &i32, square: &mut Vec<Vec<i32>>) {
    for (i, row) in square.iter().enumerate() {
        for (j, _) in row.iter().enumerate() {
            if square[i][j] == *number {
                square[i][j] = -1;
                return;
            }
        }
    }
}

fn check(square: &Vec<Vec<i32>>) -> bool {
    let want = vec![-1; 5];

    // has row line
    for i in 0..5 {
        if square[i] == want {
            return true;
        }
    }

    for j in 0..5 {
        let mut count = 0;
        for i in 0..5 {
            if count == 5 {
                return true;
            }
            if square[j][i] == -1 {
                count += 1;
            }
        }
    }

    for (_, row) in square.iter().enumerate() {
        for (_, _) in row.iter().enumerate() {
            if *row == want {
                return true;
            }
        }
    }

    false
}
struct Config {
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Config {
        let filename = args[1].clone();

        Config { filename }
    }
}
