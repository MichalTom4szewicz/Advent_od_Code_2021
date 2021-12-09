use std::fs::File;
use std::io::BufRead;

// sum of 2 vectors column-wise
fn add(a: Vec<u32>, b: &Vec<u32>) -> Vec<u32> {
    let mut c = vec![0; a.len()];
    for i in 0..a.len() {
        c[i] = a[i] + b[i];
    }
    c
}

fn main() {
    let file = File::open("input.txt").unwrap();

    let buf = std::io::BufReader::new(file);

    // collect input data into 2 dim vector
    let vectors = buf
        .lines()
        .map(|l| {
            l.unwrap()
                .chars()
                .map(|item| item.to_digit(10).unwrap())
                .collect()
        })
        .collect::<Vec<Vec<u32>>>();

    let mut array = vec![0u32; vectors[0].len()];

    // sum columns
    for data in &vectors {
        array = add(array, data)
    }

    let mut gamma: u32 = 0;
    let mut epsilon: u32 = 0;

    let len = vectors[0].len();
    let height = vectors.len();

    // if sum of column value is greater than half of column height then gamma 1 else epsilon 1
    for (i, num) in array.iter().enumerate() {
        if 2 * num > height as u32 {
            gamma += 2u32.pow((len - 1 - i) as u32);
        } else {
            epsilon += 2u32.pow((len - 1 - i) as u32);
        }
    }

    println!("{}", epsilon * gamma);
}
