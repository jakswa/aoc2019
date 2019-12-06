use std::fs::File;
use std::io::{BufReader, BufRead, Error};

fn main() -> Result<(), Error> {
    let input = File::open("./input.txt")?;
    let buffered = BufReader::new(input);

    let all: Vec<String> = buffered.lines().filter_map(std::io::Result::ok).collect();
    let mut cnt = 0;

    all.iter().for_each(|o| {
        let mut curr = &o[0..3];
        loop {
            if curr.starts_with("COM") {
                break;
            } else {
                curr = all.iter().find(|l| {
                    l.ends_with(&curr[0..3])
                }).unwrap();
                cnt += 1;
            }
        }
        cnt += 1;
    });

    println!("cnt: {}", cnt);
    Ok(())
}
