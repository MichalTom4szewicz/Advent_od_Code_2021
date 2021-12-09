use std::fs::File;
use std::io::BufRead;
use std::vec;

fn add(a: Vec<u32>, b: Vec<u32>) -> Vec<u32> {
    let mut c = vec![0; a.len()];

    for i in 0..a.len() {
        c[i] = a[i] + b[i];
    }
    c
}
fn main() {
    let file1 = File::open("input.txt").unwrap();
    let mut buf_test = std::io::BufReader::new(file1);
    let mut first_line = String::new();
    let _ = buf_test.read_line(&mut first_line);
    let len = (first_line.len() - 2) as u32;
    let height = buf_test.lines().count() as u32;

    let file = File::open("input.txt").unwrap();
    let buf = std::io::BufReader::new(file);

    let mut array = vec![0u32; len as usize];

    for (_i, line) in buf.lines().enumerate() {
        let vector = line
            .unwrap()
            .chars()
            .map(|item| item.to_digit(10).unwrap())
            .collect::<Vec<u32>>();

        array = add(vector, array);
    }

    let mut gamma: u32 = 0;
    let mut epsilon: u32 = 0;

    for (i, number) in array.iter().enumerate() {
        if 2 * number > height {
            gamma += 2u32.pow(len - 1 - i as u32);
        } else {
            epsilon += 2u32.pow(len - 1 - i as u32);
        }
    }
    println!("e {} g {}", epsilon, gamma);

    println!("{}", epsilon * gamma);
}
