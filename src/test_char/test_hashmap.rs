use std::collections::HashMap;

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

    test_word_space()
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
