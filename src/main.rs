use std::fs::File;
use std::io::BufRead;

/*

Put data into specific data structure and follow instructions? Nothing special.

Used "switch" to make action corresponding to a word.

*/

fn main() {
    let file = File::open("input.txt").unwrap();

    let buf = std::io::BufReader::new(file);

    // collect input data into 2 dim vector
    let vector = buf
        .lines()
        .map(|l| l.unwrap().split(" ").map(|item| item.to_string()).collect())
        .collect::<Vec<Vec<String>>>();

    let mut depth = 0;
    let mut position = 0;
    let mut aim = 0;

    for pair in vector {
        // destructure vector into tuple
        let (key, value): (&str, i32) = (&pair[0][..], pair[1].parse().unwrap());

        // different meaning of commands
        match key {
            "forward" => {
                position += value;
                depth += aim * value;
            }
            "up" => aim -= value,
            "down" => aim += value,
            _ => println!("oops"),
        }
    }

    println!(
        "depth: {}, position: {}, result: {}",
        depth,
        position,
        position * depth
    )
}
