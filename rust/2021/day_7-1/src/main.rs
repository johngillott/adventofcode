use std::collections::HashMap;
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

    let mut map: HashMap<u64, u32> = HashMap::new();

    input.lines().map(|s| s.split(',')).flatten().for_each(|s| {
        *map.entry(s.parse::<u64>().unwrap()).or_insert(0) += 1;
    });

    // a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
    // make vec from 0 to max?

    map.iter()
        .zip(map.iter())
        .filter(|(x, y)| x.0 == y.0)
        .map(|(x, y)| {
            let k1 = x.0;
            let k2 = y.0;
            // buckets if k2 is preferred horizon then k1 val * abs_diff, store in k2 bucket

            (k1).abs_diff(k2)
        })
        .collect();

    // 1, 2, 3, 3, 4

    // println!("{:?}", map);

    //    for (n, count) in map {

    //     for ()

    //    }

    // find mod then compute the diff between all other elements from mode to get total fuel.
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
