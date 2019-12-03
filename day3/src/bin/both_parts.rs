use std::fs::File;
use std::io::{BufReader, BufRead, Error};

fn main() -> Result<(), Error> {
    let input = File::open("./input.txt")?;
    let buffered = BufReader::new(input);
    let mut lines: Vec<Vec<[i32;2]>>= vec![];

    buffered.lines().for_each(|l| {
        let mut positions: Vec<[i32;2]> = vec![];
        l.unwrap().split(",").for_each(|j| {
            let mut chars = j.chars();
            let fchar = chars.next().unwrap();
            let dist: i32 = chars.as_str().parse().unwrap();
            let dir = match fchar {
                'U' => 0,
                'R' => 1,
                'D' => 2,
                'L' => 3,
                _ => panic!("unexpected first char {:?}", fchar)
            };
            positions.push([dir, dist]);
        });
        lines.push(positions);
    });

    let mut intersects: Vec<[i32;3]> = vec![];
    let mut l1pos: [i32;3] = [0,0,0];
    let mut l2pos: [i32;3] = [0,0,0];
    lines[0].iter().for_each(|i| {
        (0..i[1]).for_each(|_| {
            match i[0] {
                0 => l1pos[1] -= 1,
                1 => l1pos[0] += 1,
                2 => l1pos[1] += 1,
                3 => l1pos[0] -= 1,
                _ => panic!("unkonwn dir")
            }
            l1pos[2] += 1;
            l2pos[0] = 0; l2pos[1] = 0; l2pos[2] = 0;
            lines[1].iter().for_each(|j| {
                (0..j[1]).for_each(|_| {
                    match j[0] {
                        0 => l2pos[1] -= 1,
                        1 => l2pos[0] += 1,
                        2 => l2pos[1] += 1,
                        3 => l2pos[0] -= 1,
                        _ => panic!("unkonwn dir")
                    }
                    l2pos[2] += 1;
                    if l1pos[0] == l2pos[0] && l1pos[1] == l2pos[1] {
                        intersects.push(l1pos.clone());
                        println!("found one at {:?} - {:?}", l2pos, l1pos);
                    }
                })
            });
        })
    });

    println!("done, with {} intersects", intersects.len());
    Ok(())
}
