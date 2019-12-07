extern crate permutohedron;
use permutohedron::LexicalPermutation;

fn main() {
    let i: [i32;503] = [3,8,1001,8,10,8,105,1,0,0,21,42,55,64,85,98,179,260,341,422,99999,3,9,101,2,9,9,102,5,9,9,1001,9,2,9,1002,9,5,9,4,9,99,3,9,1001,9,5,9,1002,9,4,9,4,9,99,3,9,101,3,9,9,4,9,99,3,9,1002,9,4,9,101,3,9,9,102,5,9,9,101,4,9,9,4,9,99,3,9,1002,9,3,9,1001,9,3,9,4,9,99,3,9,1002,9,2,9,4,9,3,9,101,1,9,9,4,9,3,9,101,1,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,101,1,9,9,4,9,3,9,101,1,9,9,4,9,3,9,101,2,9,9,4,9,3,9,1001,9,1,9,4,9,3,9,1002,9,2,9,4,9,3,9,1001,9,2,9,4,9,99,3,9,1002,9,2,9,4,9,3,9,101,2,9,9,4,9,3,9,1001,9,2,9,4,9,3,9,101,2,9,9,4,9,3,9,102,2,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,101,1,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,102,2,9,9,4,9,3,9,101,2,9,9,4,9,99,3,9,1002,9,2,9,4,9,3,9,1002,9,2,9,4,9,3,9,101,1,9,9,4,9,3,9,1001,9,2,9,4,9,3,9,1002,9,2,9,4,9,3,9,101,1,9,9,4,9,3,9,101,2,9,9,4,9,3,9,101,2,9,9,4,9,3,9,102,2,9,9,4,9,3,9,102,2,9,9,4,9,99,3,9,102,2,9,9,4,9,3,9,102,2,9,9,4,9,3,9,1001,9,2,9,4,9,3,9,1001,9,1,9,4,9,3,9,1001,9,1,9,4,9,3,9,101,1,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,101,2,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,1002,9,2,9,4,9,99,3,9,1001,9,1,9,4,9,3,9,102,2,9,9,4,9,3,9,1001,9,1,9,4,9,3,9,1002,9,2,9,4,9,3,9,1002,9,2,9,4,9,3,9,101,2,9,9,4,9,3,9,1001,9,1,9,4,9,3,9,1002,9,2,9,4,9,3,9,102,2,9,9,4,9,3,9,102,2,9,9,4,9,99];
    //let i: [i32;57] = [3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10];

    let mut phases = [5,6,7,8,9];
    let mut max: i32 = std::i32::MIN;
    loop {
        let mut input = 0;
        let mut memory: [[i32;503];5] = [i,i,i,i,i];
        let mut positions = [0,0,0,0,0];

        phases.iter().enumerate().for_each(|(ind, &p)| {
            let res = run(positions[ind], &mut memory[ind], vec![input, p]);
            input = res.0;
            positions[ind] = res.1;
        });
        loop {
            phases.iter().enumerate().for_each(|(ind, _p)| {
                let res = run(positions[ind], &mut memory[ind], vec![input]);
                input = res.0;
                positions[ind] = res.1;
            });
            if input > max { max = input; }
            if input == 0 { break; }
        }

        if !phases.next_permutation() { break; }
    }

    println!("got {}", max);
}

fn run(mut pos: usize, i: &mut [i32;503], mut input: Vec<i32>) -> (i32, usize) {
    let mut output = 0;
    loop {
        let cmd = i[pos as usize];
        if cmd == 99 { output = -output; break; }

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
                let inp = match input.len() == 1 {
                    true => *input.get(0).unwrap(),
                    false => input.pop().unwrap()
                };
                i[i[pos+1] as usize] = inp;
                pos += 2;
            },
            4 => {
                output = first;
                pos += 2;
                break;
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
    (output, pos)
}
