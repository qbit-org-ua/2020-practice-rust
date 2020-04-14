//! Авторы похожих решений:
//! * Першина Настя
//! * Ткачёв Захар
//! * Лысуха Виталий
//! * Фролов Ваня

fn main() {
    let (mut direction, mut x, mut y) = (0, 0, 0);
    let mut action = String::new();
    loop {
        action.clear();
        std::io::stdin().read_line(&mut action).unwrap();
        let action = action.trim();
        if action.is_empty() {
            break;
        }
        match action {
            "L" => direction = (direction + 270) % 360,
            "R" => direction = (direction + 90) % 360,
            "F" => match direction {
                0 => y += 1,
                90 => x += 1,
                180 => y -= 1,
                270 => x -= 1,
                _ => unreachable!(),
            },
            unknown_action => panic!("Unknown action '{}'", unknown_action),
        }
    }
    println!("{} {}", x, y);
}
