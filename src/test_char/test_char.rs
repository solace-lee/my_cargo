pub fn char_test() {
    let hello = "哈你好";
    let a = &hello[0..9];

    println!("hello{}", a);

    for i in hello.chars() {
        println!("i是{}", i);
    }
}
