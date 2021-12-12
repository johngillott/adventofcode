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

    let mut school = vec![0; 9];

    input
        .lines()
        .map(|s| s.split(','))
        .flatten()
        .for_each(|s| match s {
            "1" => school[0] += 1,
            "2" => school[1] += 1,
            "3" => school[2] += 1,
            "4" => school[3] += 1,
            "5" => school[4] += 1,
            _ => (),
        });

    for _ in 0..config.num_of_days {
        school[6] += school[8];
        school.rotate_left(1);
    }

    println!(
        "The number of fish in the school after {} day(s) is {}.",
        config.num_of_days,
        school.iter().sum::<i64>(),
    );
}

struct Config {
    filename: String,
    num_of_days: i64,
}

impl Config {
    fn new(args: &[String]) -> Config {
        let filename = args[1].clone();
        let num_of_days = args[2].parse::<i64>().unwrap();

        Config {
            filename,
            num_of_days,
        }
    }
}
