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

    let (mut depth, mut horizontal_position, mut aim) = (0, 0, 0);

    input.lines().for_each(|s| {
        let command = s
            .split_once(' ')
            .expect("Expected line value to be direction and distance string values");
        // https://doc.rust-lang.org/rust-by-example/flow_control/match/destructuring/destructure_tuple.html

        let distance = command
            .1
            .parse::<i32>()
            .expect("Distance should be a number");

           
        match command.0 {
            "forward" => {
                horizontal_position += distance;
                depth = depth + distance*aim;
           },
            "down" => aim += distance,
            "up" => aim -= distance,
            _ => (),
        };
    });

    println!("you would have a horizontal position of {horizontal_position} and a depth of {depth}. (Multiplying these together produces {product}).",
    horizontal_position = horizontal_position,
    depth = depth, 
    product = horizontal_position * depth,
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
