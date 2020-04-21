use std::collections::HashMap;

fn main() {
    let mut string = String::new();
    std::io::stdin().read_line(&mut string).unwrap();

    string.clear();
    std::io::stdin().read_line(&mut string).unwrap();

    let mut chars: HashMap<char, i64> = HashMap::new();

    for this_char in string.trim().chars() {
        *chars.entry(this_char).or_insert(0) += 1;
    }

    let most_frequent_char = chars
        .into_iter()
        .max_by_key(|(chr, count)| (*count, -(*chr as i32)))
        .unwrap();
    println!("{}", most_frequent_char.0 as u8);
}
