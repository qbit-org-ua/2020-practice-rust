//! Авторы похожих решений:
//! * Торяник Георгий

fn main() {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();

    let mut chars: Vec<char> = line.trim().chars().collect();
    chars.sort();

    // ['1', '2', '3', '4']
    // [(0, '1'), (1, '2'), (2, '3'), (3, '4')]

    //  0    1    2
    // ['0', '0', '1', '2', '3', '4']
    // swap(0, 2)
    // ['1', '0', '0', '2', '3', '4']

    for (index, current_char) in chars.iter().enumerate() {
        if *current_char != '0' {
            chars.swap(0, index);
            break;
        }
    }
    // ['1', '0', '0', '2', '3', '4']

    println!("{}", chars.into_iter().collect::<String>());
}
