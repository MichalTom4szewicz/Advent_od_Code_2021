use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead; // 1.1.8
use std::time::Instant;

/*

To also check diagonally lines, firstly I chech wheter dostance between p1.x and p2.x is equal
to distance between p1.y and p2.y. If it is, it means that "interpolated" points coordinates
are changing by step (x = +-1, y =  +-1). x=1 / y=1 == Tan(45 deg) == 1.

Next step is to figure out the direction in which new points will be interpolated (starting
from first point in a pair of coordinates). If x coordinate of the first point is
greater/smaller than according coordinate of second point thats means the interpolated points will have
smaller/greater x coordinate than first point.
Same with y.

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

    let mut hm: HashMap<Vec<u32>, i32> = HashMap::new();

    for pair in vectors {
        if pair[0][0] == pair[1][0] {
            for i in 0..=(pair[0][1] as i32 - pair[1][1] as i32).abs() {
                let p = vec![pair[0][0], std::cmp::min(pair[0][1], pair[1][1]) + i as u32];
                if hm.contains_key(&p) {
                    hm.insert(p.clone(), *hm.get(&p).unwrap() + 1);
                } else {
                    hm.insert(p.clone(), 1);
                }
            }
        } else if pair[0][1] == pair[1][1] {
            for i in 0..=(pair[0][0] as i32 - pair[1][0] as i32).abs() {
                let p = vec![std::cmp::min(pair[0][0], pair[1][0]) + i as u32, pair[0][1]];
                if hm.contains_key(&p) {
                    hm.insert(p.clone(), *hm.get(&p).unwrap() + 1);
                } else {
                    hm.insert(p.clone(), 1);
                }
            }
        } else if (pair[0][0] as i32 - pair[1][0] as i32).abs()
            == (pair[0][1] as i32 - pair[1][1] as i32).abs()
        {
            for i in 0..=(pair[0][0] as i32 - pair[1][0] as i32).abs() {
                let mut x_direction = 1;
                let mut y_direction = 1;

                if pair[0][0] > pair[1][0] {
                    x_direction = -1;
                }

                if pair[0][1] > pair[1][1] {
                    y_direction = -1;
                }

                let p = vec![
                    pair[0][0] + (i * x_direction) as u32,
                    pair[0][1] + (i * y_direction) as u32,
                ];
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
    println!("{}", c);
    println!("{}", a);
}
