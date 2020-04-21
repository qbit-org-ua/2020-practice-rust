//! Авторы похожих решений:
//! * Овчинников Павло
//! * Попович Ярослав

fn main() {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let numbers: Vec<i64> = line
        .split_whitespace()
        .map(|item| item.parse().unwrap())
        .collect();

    let ans = (numbers[0] + numbers[1] * 90 + 360) % 360;
    println!("{}", ans);
}
