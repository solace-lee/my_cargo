// use rand::{thread_rng, Rng};
use rand::Rng;
use std::{cmp::Ordering, io};
mod data_type;
mod vector;

fn main() {
    data_type::main_type();
    // vector::vector_demo::vector_test();
    vector::vector_demo::enum_test();
    println!("猜数游戏");
    return;
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
