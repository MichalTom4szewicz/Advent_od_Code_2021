use std::fs::File;
use std::io::BufRead;

// stack overflow transpose
fn transpose2<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}
fn main() {
    let file = File::open("input.txt").unwrap();

    let buf = std::io::BufReader::new(file);

    // collect input data into 2 dim vector
    let vector = buf
        .lines()
        .map(|l| l.unwrap().chars().collect())
        .collect::<Vec<Vec<char>>>();

    let s_len = vector[0].len();
    let transposed = transpose2(vector);

    let mut gamma: u32 = 0;
    let mut epsilon: u32 = 0;

    let mut i = 0;
    for line in transposed {
        let sum: u32 = line
            .iter()
            .map(|item| item.to_digit(10).unwrap())
            .collect::<Vec<u32>>()
            .iter()
            .sum();

        if 2 * sum > line.len() as u32 {
            gamma += 2u32.pow((s_len - 1 - i) as u32);
        } else {
            epsilon += 2u32.pow((s_len - 1 - i) as u32);
        }
        i += 1;
    }

    println!("{}", epsilon * gamma);
}
