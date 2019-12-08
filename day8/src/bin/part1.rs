use std::fs::File;
use std::io::{BufReader, BufRead, Error};

fn main() -> Result<(), Error> {
    let input = File::open("./input.txt")?;
    let buffered = BufReader::new(input);
    let line = buffered.lines().next().unwrap().unwrap();

    let layer_size = 25 * 6;

    let mut min = std::i32::MAX;
    let mut cnt = 0;
    let mut one_twos = [0, 0];
    for (ind, chr) in line.char_indices() {
        if ind > 0 && ind % layer_size == 0 {
            if cnt < min {
                min = cnt;
                println!("found {} on layer {}, one_twos: {:?}", min, ind / layer_size, one_twos);
            }
            cnt = 0;
            one_twos = [0,0];
        }

        match chr {
            '0' => cnt += 1,
            '1' => one_twos[0] += 1,
            '2' => one_twos[1] += 1,
            _ => {}
        }
    }
    Ok(())
}
