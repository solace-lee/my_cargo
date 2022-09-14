use std::io;

fn main() {
    println!("请输入数字");
    let mut numrand = String::new();
    io::stdin().read_line(&mut numrand).expect("无法读取行");
    println!("你猜的数是：{}", numrand);
}