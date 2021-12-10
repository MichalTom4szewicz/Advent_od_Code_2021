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

                // transpose board to check amrked columns
                let mut transposed = vec![vec![(0u32, false); 5]; 5];
                for x in 0..5 {
                    for y in 0..5 {
                        transposed[y][x] = vvectors[i][x][y].clone();
                    }
                }

                // if we marked a number in a row/column check if that row/column is fully marked
                if marked
                    && (vvectors[i][j]
                        .iter()
                        .map(|item| item.1 as u32)
                        .collect::<Vec<u32>>()
                        .iter()
                        .sum::<u32>()
                        == 5
                        || transposed[j]
                            .iter()
                            .map(|item| item.1 as u32)
                            .collect::<Vec<u32>>()
                            .iter()
                            .sum::<u32>()
                            == 5)
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
}
