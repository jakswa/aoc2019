extern crate permutohedron;
use permutohedron::LexicalPermutation;

fn main() {
    let i: [i32;503] = [3,8,1001,8,10,8,105,1,0,0,21,42,55,64,85,98,179,260,341,422,99999,3,9,101,2,9,9,102,5,9,9,1001,9,2,9,1002,9,5,9,4,9,99,3,9,1001,9,5,9,1002,9,4,9,4,9,99,3,9,101,3,9,9,4,9,99,3,9,1002,9,4,9,101,3,9,9,102,5,9,9,101,4,9,9,4,9,99,3,9,1002,9,3,9,1001,9,3,9,4,9,99,3,9,1002,9,2,9,4,9,3,9,101,1,9,9,4,9,3,9,101,1,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,101,1,9,9,4,9,3,9,101,1,9,9,4,9,3,9,101,2,9,9,4,9,3,9,1001,9,1,9,4,9,3,9,1002,9,2,9,4,9,3,9,1001,9,2,9,4,9,99,3,9,1002,9,2,9,4,9,3,9,101,2,9,9,4,9,3,9,1001,9,2,9,4,9,3,9,101,2,9,9,4,9,3,9,102,2,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,101,1,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,102,2,9,9,4,9,3,9,101,2,9,9,4,9,99,3,9,1002,9,2,9,4,9,3,9,1002,9,2,9,4,9,3,9,101,1,9,9,4,9,3,9,1001,9,2,9,4,9,3,9,1002,9,2,9,4,9,3,9,101,1,9,9,4,9,3,9,101,2,9,9,4,9,3,9,101,2,9,9,4,9,3,9,102,2,9,9,4,9,3,9,102,2,9,9,4,9,99,3,9,102,2,9,9,4,9,3,9,102,2,9,9,4,9,3,9,1001,9,2,9,4,9,3,9,1001,9,1,9,4,9,3,9,1001,9,1,9,4,9,3,9,101,1,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,101,2,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,1002,9,2,9,4,9,99,3,9,1001,9,1,9,4,9,3,9,102,2,9,9,4,9,3,9,1001,9,1,9,4,9,3,9,1002,9,2,9,4,9,3,9,1002,9,2,9,4,9,3,9,101,2,9,9,4,9,3,9,1001,9,1,9,4,9,3,9,1002,9,2,9,4,9,3,9,102,2,9,9,4,9,3,9,102,2,9,9,4,9,99];

    let mut phases = [0,1,2,3,4];
    let mut max: i32 = std::i32::MIN;
    loop {
        let mut input = 0;
        phases.iter().for_each(|&p| {
            input = run(i.clone(), vec![input, p]);
        });

        if input > max { max = input; }
        if !phases.next_permutation() { break; }
    }
    println!("got {}", max);
}

fn run(mut i: [i32;503], mut input: Vec<i32>) -> i32 {
    let mut pos = 0;
    let mut output = 0;
    loop {
        let cmd = i[pos as usize];
        if cmd == 99 { break; }

        let mut first = i[pos+1 as usize];
        if cmd % 1000 < 100 { first = i[first as usize]; }

        match cmd % 100 {
            1 => {
                let mut second = i[pos+2 as usize];
                if cmd % 10000 < 1000 { second = i[second as usize]; }
                i[i[pos+3] as usize] = first + second;
                pos += 4;
            },
            2 => {
                let mut second = i[pos+2 as usize];
                if cmd % 10000 < 1000 { second = i[second as usize]; }
                i[i[pos+3] as usize] = first * second;
                pos += 4;
            },
            3 => {
                i[i[pos+1] as usize] = input.pop().unwrap();
                pos += 2;
            },
            4 => {
                output = first;
                pos += 2;
            },
            5 => {
                let mut second = i[pos+2 as usize];
                if cmd % 10000 < 1000 { second = i[second as usize]; }
                if first != 0 {
                    pos = second as usize;
                } else {
                    pos += 3;
                }
            },
            6 => {
                let mut second = i[pos+2 as usize];
                if cmd % 10000 < 1000 { second = i[second as usize]; }
                if first == 0 {
                    pos = second as usize;
                } else {
                    pos += 3;
                }
            },
            7 => {
                let mut second = i[pos+2 as usize];
                if cmd % 10000 < 1000 { second = i[second as usize]; }
                let set = match first < second {
                    true => 1,
                    false => 0
                };
                i[i[pos+3] as usize] = set;
                pos += 4;
            },
            8 => {
                let mut second = i[pos+2 as usize];
                if cmd % 10000 < 1000 { second = i[second as usize]; }
                let set = match first == second {
                    true => 1,
                    false => 0
                };
                i[i[pos+3] as usize] = set;
                pos += 4;
            },
            _ => panic!("unknown op code {}", cmd)
        }
    }
    output
}
