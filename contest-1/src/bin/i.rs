fn main() {
    for _ in 0..3 {
        let mut name = String::new();
        std::io::stdin().read_line(&mut name).unwrap();
        let mut age = String::new();
        std::io::stdin().read_line(&mut age).unwrap();
        let age: u64 = age.trim().parse().expect("failed to parse");
        println!("{} will reach adulthood in {}", name, age + 18);
    }
}
