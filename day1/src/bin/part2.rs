use std::fs::File;
use std::io::{BufReader, BufRead, Error};

fn main() -> Result<(), Error> {
    let input = File::open("./input.txt")?;
    let buffered = BufReader::new(input);

    let nums = buffered.lines().map(|l| {
        let num: f32 = l.unwrap().parse().unwrap();
        (num / 3.0).floor() - 2.0
    });


    let sum: f32 = nums.fold(0.0, |acc, mut x| {
        let mut loopsum = 0.0;
        loop {
            loopsum += x;
            x = (x / 3.0).floor() - 2.0;
            if x < 0.0 { break; }
        }
        loopsum + acc
    });

    println!("Done: {:?}", sum);

    Ok(())
}
