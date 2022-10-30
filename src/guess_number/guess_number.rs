use rand::Rng;
use std::{cmp::Ordering, io};

#[derive(Debug)]
struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("只能介于1-100之间{}", value);
        }

        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}

pub mod guess {
    use std::{io, cmp::Ordering};

    use rand::Rng;

    use super::Guess;

    pub fn simple_guess() {
        loop {
            let guess = "32";

            let guess: i32 = match guess.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };

            let guess = Guess::new(guess);
            println!("结果是：{:?}", guess);
            break;
        }
    }

    pub fn guess_number() {
        println!("猜数游戏");
        // let mut rng = thread_rng();
        // let secret_number = rng.gen_range(1..101);

        let secret_number = rand::thread_rng().gen_range(1..101);
        println!("神秘数字是{}", secret_number);
        println!("猜测一个数字");

        loop {
            let mut guess = String::new();

            io::stdin().read_line(&mut guess).expect("无法读取行");

            // shadow
            let guess: u32 = match guess.trim().parse() {
                Ok(num) => num,
                Err(e) => {
                    println!("err{}", e);
                    continue;
                }
            };

            println!("你猜测的数是{}", guess);

            match guess.cmp(&secret_number) {
                Ordering::Less => println!("Too small"),
                Ordering::Greater => println!("Too Big"),
                Ordering::Equal => {
                    println!("You Win");
                    break;
                }
            }
        }
    }
}
