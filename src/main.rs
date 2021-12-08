use std::fs::File;
use std::io::BufRead;

fn main() {
    let file = File::open("input.txt").unwrap();

    let buf = std::io::BufReader::new(file);

    // parse each element (String) to i32
    let vector: Vec<i32> = buf.lines().map(|l| l.unwrap().parse().unwrap()).collect();

    // stores result
    let mut sum = 0;

    // sum++ if current is bigger than previous
    for i in 1..vector.len() {
        if vector[i] > vector[i - 1] {
            sum += 1;
        }
    }

    println!("{}", sum);
}
