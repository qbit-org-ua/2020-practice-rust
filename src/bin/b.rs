//! Авторы похожих решений:
//! * Овчинников Павло
//! * Злобинцев Илья
//! * Торяник Георгий

fn from_string_to_int(input_str: &str) -> i64 {
    return input_str.parse().unwrap();
}

fn main() {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let numbers: Vec<i64> = line
        .trim()
        .split_whitespace()
        // ["1", "5", ...]
        .map(|x| x.parse().unwrap())
        // [1, 5, ...]
        .collect();

    let (mut a, b) = (numbers[0], numbers[1]);

    println!("{} very important numbers:", (a - b).abs() + 1);

    let step = if a > b { -1 } else { 1 };
    while a != b {
        println!("{}", a);
        a += step;
    }
    println!("{}", b);

    /* Не работает :(
    let range = if a < b { a..=b } else { (b..=a).rev() };
    for i in range {
        println!("{}", i);
    }
    */

    /*
    if a < b {
        for i in a..=b {
            println!("{}", i);
        }
    } else {
        for i in (b..=a).rev() {
            println!("{}", i);
        }
    }
    */
}
