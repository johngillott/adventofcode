use std::fs;

fn main() {
    let filename = "input.txt";

    let contents = fs::read_to_string(filename).expect("Something went wrong reading file.");

    let hzs: Vec<&str> = contents.split('\n').collect();

    let mut result: isize = 0;

    let iter = hzs.iter();

    for v in iter {
        let mut c = v.chars();
        match c.next() {
            None => {}
            Some(f) => {
                let n: isize = match c.as_str().parse() {
                    Ok(num) => num,
                    Err(_) => continue,
                };

                if f == '+' {
                    result = result + n;
                } else {
                    result = result - n;
                }
            }
        }
    }

    println!("The frequency result is: {}", result); // 513
}
