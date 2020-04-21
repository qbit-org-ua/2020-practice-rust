//! Авторы похожих решений:
//! * Ткачёв Захар

fn main() {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let numbers: Vec<i64> = line
        .split_whitespace()
        .map(|item| item.parse().unwrap())
        .collect();

    // a=3 b=9 (3, 6, 9) = 3
    // (0; 9] = 9 / 3 = 3
    // (0; 3) = (0; 2] = 2 / 3 = 0
    let ans = numbers[1].div_euclid(3) - (numbers[0] - 1).div_euclid(3);
    println!("{}", ans);
}
