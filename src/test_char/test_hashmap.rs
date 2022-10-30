use std::{
    collections::HashMap,
    fs::File,
    io::{self, ErrorKind, Read},
};

pub fn hash_map() {
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let intial_scores = vec![10, 50];

    let scores: HashMap<_, _> = teams.iter().zip(intial_scores.iter()).collect();

    let fenshu = scores.get(&String::from("Blue"));

    match fenshu {
        Some(v) => println!("分数是{}", v),
        None => println!("是个空"),
    }

    for (k, v) in scores {
        println!("{}, {}", k, v);
    }

    // 替换现有的value
    let mut new_map = HashMap::new();
    new_map.insert("Blue", &15);
    new_map.insert("green", &25);
    // new_map.insert("white", &2550);

    // for (k, v) in new_map {
    //     println!("替换现有的value{}, {}", k, v);
    // }

    // new_map.insert("Blue", &14);
    // for (k, v) in new_map {
    //     println!("替换现有的value{}, {}", k, v);
    // }

    new_map.entry("white").or_insert_with(|| &50);

    for (k, v) in new_map {
        println!("替换现有的value{}, {}", k, v);
    }

    // test_word_space();
    // test_file();
    test_read();
}

fn test_word_space() {
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("测试空格切割：{:#?}", map);
}

fn test_file() {
    // let f = File::open("123.text");

    // let f = match f {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("123.text") {
    //             Ok(fc) => fc,
    //             Err(e) => panic!("创建文件失败{:?}", e),
    //         },
    //         other_error => panic!("打开文件错误{:?}", other_error),
    //     },
    // };

    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("创建文件失败{:?}", error);
            })
        } else {
            panic!("打开文件错误{:?}", error)
        }
    });

    // let f = File::open("123.text").expect("无法找到文件");

    println!("{:?}", f)
}

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => match e.kind() {
            ErrorKind::NotFound => File::create("hello.txt").expect("创建失败"),
            _other => return Err(e),
        },
    };

    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn simple_read() -> Result<String, io::Error> {
  let mut f = File::open("Cargo.toml")?;

  let mut s = String::new();
  f.read_to_string(&mut s)?;
  Ok(s)
}

fn test_read() {

    // let t = read_username_from_file();
    let t = simple_read();
    println!("读取文件内容{:?}", t)
}
