use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufRead};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args);

    let mut file = File::open(config.filename).expect("Unable to open file");
    let mut buf = String::new();
    file.read_to_string(&mut buf)
        .expect("Unable to load file contents {}");

    let measurements = io::Cursor::new(buf)
        .lines()
        .map(|l| l.unwrap().parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    let i = measurements
        .windows(2)
        .filter(|pair| pair[0] < pair[1])
        .count();

    print!(
        "there are {} measurements that are larger than the previous measurement",
        i
    )
}

struct Config {
    filename: String,
}
// https://doc.rust-lang.org/book/ch12-03-improving-error-handling-and-modularity.html#creating-a-constructor-for-config
impl Config {
    fn new(args: &[String]) -> Config {
        let filename = args[1].clone();

        Config { filename }
    }
}
