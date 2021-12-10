use std::fs::File;
use std::io::BufRead; // 1.1.8

fn main() {
    let file = File::open("input.txt").unwrap();
    let mut buf = std::io::BufReader::new(file);

    // ranodm numbers from 1st line
    let mut first_line = String::new();
    buf.read_line(&mut first_line).unwrap();

    let numbers = first_line
        .split(',')
        .map(|item| item.trim().parse().unwrap())
        .collect::<Vec<u32>>();

    // bingo matrices vector
    let mut vvectors: Vec<Vec<Vec<(u32, bool)>>> = Vec::new();

    // temporary bingo matrix
    let mut tmp_matrix: Vec<Vec<(u32, bool)>> = Vec::new();

    // save input into vector of bingo boards
    for line_result in buf.lines() {
        let line = line_result.unwrap();

        // skip whiteline
        if line.len() < 5 {
            continue;
        }

        // store elements of bingo matrices as tuple of (value: u32, marked: bool)
        let row = line
            .split_whitespace()
            .map(|item| (item.to_string().parse().unwrap(), false))
            .collect::<Vec<(u32, bool)>>();

        tmp_matrix.push(row);
        if tmp_matrix.len() == 5 {
            vvectors.push(tmp_matrix.clone());
            tmp_matrix.clear();
        }
    }

    // println!("{:?}", numbers);
    // println!("{:?}", vvectors);

    for number in numbers {
        for i in 0..vvectors.len() {
            for j in 0..vvectors[i].len() {
                let mut marked = false;
                for k in 0..vvectors[i][j].len() {
                    if vvectors[i][j][k].0 == number {
                        marked = true;
                        // mark every picked numbers on all bingo matrices
                        vvectors[i][j][k] = (vvectors[i][j][k].0, true);
                    }
                }

                // if we marked a number in a row check if that row is fully marked
                if marked
                    && vvectors[i][j]
                        .iter()
                        .map(|item| item.1 as u32)
                        .collect::<Vec<u32>>()
                        .iter()
                        .sum::<u32>()
                        == 5
                {
                    // calculate the score
                    let score: u32 = vvectors[i]
                        .iter()
                        .map(|row| {
                            row.iter()
                                .map(|item| if !item.1 { item.0.clone() } else { 0 })
                                .collect::<Vec<u32>>()
                                .iter()
                                .sum::<u32>()
                        })
                        .collect::<Vec<u32>>()
                        .iter()
                        .sum();

                    println!(
                        "score {} number {} result {}",
                        score,
                        number,
                        score * number
                    );
                    return;
                }
            }
        }
    }

    // collect input data into 2 dim vector
    // let mut vectors = buf
    //     .lines()
    //     .map(|l| {
    //         l.unwrap()
    //             .chars()
    //             .map(|item| item.to_digit(10).unwrap())
    //             .collect()
    //     })
    //     .collect::<Vec<Vec<u32>>>();

    // let mut vectors_copy = vectors.clone();

    // let mut ones_i: Vec<usize> = Vec::new();
    // let mut zeroes_i: Vec<usize> = Vec::new();

    // for i in 0..vectors[0].len() {
    //     if vectors.len() == 1 {
    //         break;
    //     }
    //     for j in 0..vectors.len() {
    //         if vectors[j][i] == 0 {
    //             zeroes_i.push(j);
    //         } else {
    //             ones_i.push(j);
    //         }
    //     }

    //     let mut new_vectors: Vec<Vec<u32>> = Vec::new();
    //     if ones_i.len() >= zeroes_i.len() {
    //         for idx in &mut ones_i {
    //             new_vectors.push(vectors[idx.clone()].clone());
    //         }
    //     } else {
    //         for idx in &mut zeroes_i {
    //             new_vectors.push(vectors[idx.clone()].clone());
    //         }
    //     }
    //     vectors.clear();
    //     vectors = new_vectors;

    //     // println!("{:?}", vectors);
    //     // println!("-----------------");
    //     ones_i.clear();
    //     zeroes_i.clear();
    // }

    // for i in 0..vectors_copy[0].len() {
    //     if vectors_copy.len() == 1 {
    //         break;
    //     }
    //     for j in 0..vectors_copy.len() {
    //         if vectors_copy[j][i] == 0 {
    //             zeroes_i.push(j);
    //         } else {
    //             ones_i.push(j);
    //         }
    //     }

    //     let mut new_vectors: Vec<Vec<u32>> = Vec::new();
    //     if ones_i.len() >= zeroes_i.len() {
    //         for idx in &mut zeroes_i {
    //             new_vectors.push(vectors_copy[idx.clone()].clone());
    //         }
    //     } else {
    //         for idx in &mut ones_i {
    //             new_vectors.push(vectors_copy[idx.clone()].clone());
    //         }
    //     }
    //     vectors_copy.clear();
    //     vectors_copy = new_vectors;

    //     // println!("{:?}", vectors_copy);
    //     // println!("-----------------");
    //     ones_i.clear();
    //     zeroes_i.clear();
    // }

    // let a = u32::from_str_radix(
    //     &vectors[0]
    //         .iter()
    //         .map(|item| char::from_digit(*item, 2).unwrap())
    //         .collect::<Vec<char>>()
    //         .iter()
    //         .collect::<String>()[..],
    //     2,
    // )
    // .unwrap();

    // let b = u32::from_str_radix(
    //     &vectors_copy[0]
    //         .iter()
    //         .map(|item| char::from_digit(*item, 2).unwrap())
    //         .collect::<Vec<char>>()
    //         .iter()
    //         .collect::<String>()[..],
    //     2,
    // )
    // .unwrap();

    // println!("{} {} {}", a, b, a * b);
}
