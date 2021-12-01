use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

fn main() {
    let path = Path::new("../../../input/1.txt");
    let file = match File::open(path) {
        Err(why) => panic!("couldn't open {}: {}", path.display(), why),
        Ok(file) => file,
    };

    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    match buf_reader.read_to_string(&mut contents) {
        Err(why) => panic!("couldn't read {}: {}", path.display(), why),
        Ok(_) => (),
    }

    println!("hello world {}", contents);

    let vec: Vec<&str> = contents.split('\n').collect();

    let ints = vec
        .into_iter()
        .map(|s| {
            match s.parse() {
                Ok(num) => num,
                Err(err) => (),
            };
        })
        .Collect();
    // let result: i32;

    // for a in vec {

    //       let i: i32 = match a.parse() {
    //                 Ok(num) => num,
    //                 Err(_) => continue,
    //             };
    //         }
    //     }
    //     for b in vec {

    //     }
    // }

    // for v in iter {
    //     let mut c = v.chars();
    //     match c.next() {
    //         None => {}
    //         Some(f) => {
    //             let n: isize = match c.as_str().parse() {
    //                 Ok(num) => num,
    //                 Err(_) => continue,
    //             };

    //             // if f == '+' {
    //             //     result = result + n;
    //             // } else {
    //             //     result = result - n;
    //             // }
    //         }
    //     }
    // }

    println!("The frequency result is: {}", result); // 513
}
