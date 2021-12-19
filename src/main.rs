use std::fs::File;
use std::io::BufRead;

/*

Scubber rating and generator rating are calculated separately. :(

Save indices of "zero" and "ones" vectors in separate arrays just to rebuild new vector
and overwrite old one. Repeated after each column


*/

fn main() {
    let file = File::open("input.txt").unwrap();

    let buf = std::io::BufReader::new(file);

    // collect input data into 2 dim vector
    let mut vectors = buf
        .lines()
        .map(|l| {
            l.unwrap()
                .chars()
                .map(|item| item.to_digit(10).unwrap())
                .collect()
        })
        .collect::<Vec<Vec<u32>>>();

    let mut vectors_copy = vectors.clone();

    let mut ones_i: Vec<usize> = Vec::new();
    let mut zeroes_i: Vec<usize> = Vec::new();

    for i in 0..vectors[0].len() {
        if vectors.len() == 1 {
            break;
        }
        for j in 0..vectors.len() {
            if vectors[j][i] == 0 {
                zeroes_i.push(j);
            } else {
                ones_i.push(j);
            }
        }

        let mut new_vectors: Vec<Vec<u32>> = Vec::new();
        if ones_i.len() >= zeroes_i.len() {
            for idx in &mut ones_i {
                new_vectors.push(vectors[idx.clone()].clone());
            }
        } else {
            for idx in &mut zeroes_i {
                new_vectors.push(vectors[idx.clone()].clone());
            }
        }
        vectors.clear();
        vectors = new_vectors;

        // println!("{:?}", vectors);
        // println!("-----------------");
        ones_i.clear();
        zeroes_i.clear();
    }

    for i in 0..vectors_copy[0].len() {
        if vectors_copy.len() == 1 {
            break;
        }
        for j in 0..vectors_copy.len() {
            if vectors_copy[j][i] == 0 {
                zeroes_i.push(j);
            } else {
                ones_i.push(j);
            }
        }

        let mut new_vectors: Vec<Vec<u32>> = Vec::new();
        if ones_i.len() >= zeroes_i.len() {
            for idx in &mut zeroes_i {
                new_vectors.push(vectors_copy[idx.clone()].clone());
            }
        } else {
            for idx in &mut ones_i {
                new_vectors.push(vectors_copy[idx.clone()].clone());
            }
        }
        vectors_copy.clear();
        vectors_copy = new_vectors;

        // println!("{:?}", vectors_copy);
        // println!("-----------------");
        ones_i.clear();
        zeroes_i.clear();
    }

    let a = u32::from_str_radix(
        &vectors[0]
            .iter()
            .map(|item| char::from_digit(*item, 2).unwrap())
            .collect::<Vec<char>>()
            .iter()
            .collect::<String>()[..],
        2,
    )
    .unwrap();

    let b = u32::from_str_radix(
        &vectors_copy[0]
            .iter()
            .map(|item| char::from_digit(*item, 2).unwrap())
            .collect::<Vec<char>>()
            .iter()
            .collect::<String>()[..],
        2,
    )
    .unwrap();

    println!("{} {} {}", a, b, a * b);
}
