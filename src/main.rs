use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead; // 1.1.8
use std::time::{Duration, Instant};

/*

Read data into array of pairs of points.

Check whether one of coordinates of p1 is equal to one of coordinates of p2.
If it is that means  points lie on the same line (vertical if p1.x == p2.x or horizontal if
p1.y == p2.y).

After that just interpolate points alongside vertical or horizontal line.

*/

fn main() {
    let now = Instant::now();
    let file = File::open("input.txt").unwrap();
    let buf = std::io::BufReader::new(file);

    let vectors = buf
        .lines()
        .map(|l| {
            l.unwrap()
                .split(" -> ")
                .map(|it| {
                    it.split(",")
                        .map(|i| i.parse().unwrap())
                        .collect::<Vec<u32>>()
                })
                .collect::<Vec<Vec<u32>>>()
        })
        .collect::<Vec<Vec<Vec<u32>>>>();

    // let mut points: Vec<Vec<u32>> = Vec::new();

    let mut hm: HashMap<Vec<u32>, i32> = HashMap::new();

    for pair in vectors {
        if pair[0][0] == pair[1][0] {
            for i in 0..=(pair[0][1] as i32 - pair[1][1] as i32).abs() {
                let p = vec![pair[0][0], std::cmp::min(pair[0][1], pair[1][1]) + i as u32];
                // points.push(p.clone());
                if hm.contains_key(&p) {
                    hm.insert(p.clone(), *hm.get(&p).unwrap() + 1);
                } else {
                    hm.insert(p.clone(), 1);
                }
            }
        } else if pair[0][1] == pair[1][1] {
            for i in 0..=(pair[0][0] as i32 - pair[1][0] as i32).abs() {
                let p = vec![std::cmp::min(pair[0][0], pair[1][0]) + i as u32, pair[0][1]];
                // points.push(p.clone());
                if hm.contains_key(&p) {
                    hm.insert(p.clone(), *hm.get(&p).unwrap() + 1);
                } else {
                    hm.insert(p.clone(), 1);
                }
            }
        }
    }

    let c = hm.values().filter(|item| **item >= 2).count();

    let a = now.elapsed().as_micros();
    // println!("points {}", points.len());
    println!("{}", c);
    println!("{}", a);

    // let count = points
    //     .iter()
    //     .map(|point| (point, points.iter().filter(|p| **p == *point).count()))
    //     .collect::<HashMap<&Vec<u32>, usize>>()
    //     .iter()
    //     .filter(|x| *x.1 >= 2)
    //     .count();

    // println!("{:?}", count);

    // let a = now.elapsed().as_millis();
    // println!("{}", a);
}
