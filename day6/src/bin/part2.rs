use std::fs::File;
use std::io::{BufReader, BufRead, Error};

fn main() -> Result<(), Error> {
    let input = File::open("./input.txt")?;
    let buffered = BufReader::new(input);

    let all: Vec<String> = buffered.lines().filter_map(std::io::Result::ok).collect();
    let me: &str = all.iter().find(|l| l.ends_with("YOU")).unwrap();
    let targ: &str = all.iter().find(|l| l.ends_with("SAN")).unwrap();

    let mut my_path: Vec<&str> = vec![];
    let mut curr = me;
    loop {
        if curr.starts_with("COM") {
            break;
        } else {
            curr = all.iter().find(|l| {
                l.ends_with(&curr[0..3])
            }).unwrap();
            my_path.push(curr);
        }
    }

    let mut san_path: Vec<&str> = vec![];
    curr = targ;
    loop {
        if curr.starts_with("COM") {
            break;
        } else {
            curr = all.iter().find(|l| {
                l.ends_with(&curr[0..3])
            }).unwrap();
            san_path.push(curr);
        }
    }

    loop {
        let m: &str = my_path.pop().unwrap();
        let s: &str = san_path.pop().unwrap();
        if m != s { break; }
    }

    println!("cnt: {}, {}", my_path.len(), san_path.len());
    Ok(())
}
