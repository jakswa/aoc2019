use std::fs::File;
use std::io::{BufReader, BufRead, Error};

fn main() -> Result<(), Error> {
    let input = File::open("./input.txt")?;
    let buffered = BufReader::new(input);
    let line = buffered.lines().next().unwrap().unwrap();
    let mut image: [u8;25 * 6] = [2; 25*6];

    let layer_size = 25 * 6;

    let mut image_ind = 0;
    for (ind, chr) in line.char_indices() {
        if ind % layer_size == 0 {
            image_ind = 0;
        }

        if image[image_ind] == 2 {
            match chr {
                '0' => image[image_ind] = 0,
                '1' => image[image_ind] = 1,
                _ => {}
            }
        }

        image_ind += 1;
    }

    (0..6).for_each(|i|{
        (0..25).for_each(|j| {
            let chr = match image[i*25+j] {
                1 => '%',
                0 => ' ',
                _ => ' '
            };
            print!("{}", chr);
        });
        print!("\n");
    });
    Ok(())
}
