#[derive(Debug)]
pub enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

pub mod vector_demo {
    pub fn vector_test() {
        let v: Vec<u32> = Vec::new();
        let a = vec![1, 2, 3];

        let mut b = Vec::new();
        b.push(55);
        b.push(66);

        match b.get(10) {
            Some(value) => println!("匹配到的值{}", value),
            None => println!("不存在该元素"),
        }

        // b.push(77);

        for v in &mut b {
            *v = *v + 1;
            println!("测试for循环{}, {}", v, v)
        }

        let first = &b[0];

        println!("集合{:?}{:?}{:?}", v, a, first);
    }

    use crate::vector::SpreadsheetCell;

    pub fn enum_test() {
        let row = vec![
            SpreadsheetCell::Int(3),
            SpreadsheetCell::Text(String::from("blue")),
            SpreadsheetCell::Float(10.12),
        ];

        println!("枚举的测试{:?}，{:?}，{:?}", row[0], row[1], row[2]);
    }
}
