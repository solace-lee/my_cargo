use rand::{thread_rng, Rng};
// use rand::Rng;
use std::io;

fn main() {
    println!("请输入数字");
    // let mut rng = thread_rng();
    // let secret_number = rng.gen_range(1..101);

    let secret_number = thread_rng().gen_range(1..101);

    println!("随机数：{}", secret_number);

    let mut numrand = String::new();

    io::stdin().read_line(&mut numrand).expect("无法读取行");

    println!("你猜的数是：{}", numrand);
}
