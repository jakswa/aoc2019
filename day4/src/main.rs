fn main() {
    let mut curr = 168630;
    let max = 718098;
    let mut cnt = 0;


    while curr <= max {
        let mut repcnt = 0;
        let mut rep = false;
        let mut inc = true;
        let mut prev = '-';
        curr.to_string().chars().for_each(|c|{
            if c < prev {
                inc = false;
            }
            if c == prev {
                repcnt += 1;
            } else if repcnt != 0 {
                if repcnt == 1 {
                    rep = true;
                }
                repcnt = 0;
            }
            prev = c;
        });
        // last check
        if repcnt == 1 {
            rep = true;
        }
        if rep && inc {
            cnt += 1;
        }
        curr += 1;
    }
    println!("found {}", cnt);
}
