#[derive(Debug)]
struct BaseBall {
    nums: Vec<u8>,
}

impl BaseBall {
    fn new() -> BaseBall {
        let mut b = BaseBall {nums: (0..3).collect()};
        b.init();
        b
    }
    fn init(&mut self) {
        use rand::prelude::*;
        let mut nums:Vec<u8> = (0..10).collect();
        nums.shuffle(&mut rand::thread_rng());
        self.nums = nums[0..3].to_vec();
    }
    fn answer(&self) -> &Vec<u8> {
        &self.nums
    }
    fn guess(&self, g:&Vec<u8>) -> bool {
        let n = &self.nums;
        assert_eq!(n.len(), g.len());
        assert_ne!(g[0], g[1]);
        assert_ne!(g[0], g[2]);
        assert_ne!(g[1], g[2]);
        let mut strike = 0u8;
        let mut ball = 0u8;
        for i in 0..3 {
            for j in 0..3 {
                if n[i]==g[j] {
                    if i==j {strike+=1;}
                    else {ball+=1;}
                }
            }
        }
        if strike<3 {
            println!("{} Strike(s), {} Ball(s)!", strike, ball);
            return false;
        } else {
            println!("Correct: {:?}", self.answer());
            return true;
        }
    }
}

fn main() {
    use std::io::{stdin,stdout,Write};
    let b = BaseBall::new();
    loop {
        print!("Guess 3 numbers in [0-9]: ");
        let _ = stdout().flush();
        let mut s = String::new();
        stdin().read_line(&mut s).expect("Wrong input!");
        let nums:Vec<u8> = s.split_whitespace()
                .map(|x| match x.parse::<u8>() {
                             Ok(i) => i,
                             Err(e) => panic!("{}", e),
                         })
                .collect();
        if b.guess(&nums) {
            break;
        }
    }
}
