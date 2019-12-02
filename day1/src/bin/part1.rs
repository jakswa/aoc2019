use std::fs::File;
use std::io::{BufReader, BufRead, Error};

fn main() -> Result<(), Error> {
    let input = File::open("./input.txt")?;
    let buffered = BufReader::new(input);

    let nums = buffered.lines().map(|l| {
        let num: f32 = l.unwrap().parse().unwrap();
        (num / 3.0).floor() - 2.0
    });


    println!("Done: {:?}", nums.fold(0.0, |i,j| i + j));

    Ok(())
}
