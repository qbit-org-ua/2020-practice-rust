fn main() {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let numbers: Vec<f64> = line
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let (f, d) = (numbers[0], numbers[1]);
    println!("{}'{}\" = {:.2}m.", f, d, (f * 12. + d) * 25.4 / 1000.0);
}
