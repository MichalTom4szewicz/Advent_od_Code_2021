use std::fs::File;
use std::io::BufRead;

fn main() {
    let file = File::open("input.txt").unwrap();

    let buf = std::io::BufReader::new(file);

    // parse each element (String) to i32
    let vector: Vec<i32> = buf.lines().map(|l| l.unwrap().parse().unwrap()).collect();

    // stores result of sliding window sum
    let mut sums: Vec<i32> = Vec::new();

    // stores result
    let mut sum = 0;

    // fill sums vector which is 2 elements shorter than input vector
    for i in 0..vector.len() - 2 {
        sums.push(vector[i] + vector[i + 1] + vector[i + 2])
    }

    // sum++ if current is bigger than previous
    for i in 1..sums.len() {
        if sums[i] > sums[i - 1] {
            sum += 1;
        }
    }

    println!("{}", sum);
}
