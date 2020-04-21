use std::collections::HashMap;
use std::io::Write;

fn main() {
    let mut record: HashMap<String, i32> = HashMap::new();
    let mut line = String::new();
    loop {
        std::io::stdin().read_line(&mut line).unwrap();
        if line.is_empty() {
            break;
        }
        let vec: Vec<&str> = line.trim().split_whitespace().collect();
        //println!("{:?}", vec);
        for el in vec {
            *record.entry(el.to_string()).or_default() += 1;
        }
        line.clear();
    }

    let mut vec: Vec<(String, i32)> = record.into_iter().collect();

    vec.sort_by(|(s1, number1), (s2, number2)| number2.cmp(number1).then_with(|| s1.cmp(s2)));

    println!(
        "{}",
        vec.iter()
            .map(|(s, _)| s.as_str())
            .collect::<Vec<&str>>()
            .join("\n")
    );
}
