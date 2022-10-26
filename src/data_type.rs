pub fn main_type() {
    let guess: u32 = "42".parse().expect("no a number");

    println!("{}哈哈哈", guess);

    let f = 2.0;
    // let f = f + 2;
    println!("{}", f);

    let a = 'd';
    println!("{}", a);

    let tup = (500, 6.4, 1);
    println!("{},{}, {}", tup.0, tup.1, tup.2);

    let (x, y, z) = tup;
    println!("{},{}, {}", x, y, z);

    let arr: [u32; 3] = [45, 88, 2];
    println!("{}-{}-{}", arr[0], arr[1], arr[2]);

    let b = another_func(9);
    println!("{}哈哈哈", b);
}

fn another_func(x: u8) -> u8 {
    if x > 8 {
        1
    } else if x == 9 {
        x + 8
    } else {
        x * 2
    }
}
