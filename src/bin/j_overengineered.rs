//! Задача "Ёжик в тумане"
//! ======================
//!
//! Ёжик пошёл на север из точки с координатами `(0; 0)`, но заблудился в тумане.
//!
//! Теперь он находится в поле, представляющем собой декартову плоскость и
//! перемещается строго между соседними точками, смещаясь ровно на единицу по
//! одной из координат. Также ёжик может поворачивать на `90` градусов в любом
//! из возможных направлений.
//!
//! Ёжик сделал одну очень хорошую вещь: он записывал каждое своё действие и
//! теперь просит помочь определить, в какой точке плоскости он сейчас находится.
//!
//! Каждая строка входного данных содержит один символ:
//!
//! * `F` - переместиться на один шаг вперёд,
//! * `L` - повернуть налево,
//! * `R` - повернуть направо.
//!
//! Выведите два числа через пробел - координаты ёжика после всех его перемещений.
//!
//! Тест #1:
//! --------
//!
//! ### Ввод
//!
//! ```skip
//! L
//! R
//! F
//! F
//! ```
//!
//! ### Ответ
//!
//! ```skip
//! 0 2
//! ```

/// Точка в двумерном пространстве
#[derive(Debug, Default, PartialEq, Eq)]
struct Point {
    /// Координата по горизонтали
    x: i64,
    /// Координата по вертикали
    y: i64,
}

impl std::ops::AddAssign for Point {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}

/// Направление движения
enum Direction {
    North,
    East,
    South,
    West,
}

impl Default for Direction {
    fn default() -> Self {
        Self::North
    }
}

impl Direction {
    /// Создать направление из числового представления.
    fn from_number(direction: u8) -> Self {
        match direction % 4 {
            0 => Direction::North,
            1 => Direction::East,
            2 => Direction::South,
            3 => Direction::West,
            _ => unreachable!(),
        }
    }

    /// Получить числовое представление направления.
    fn as_number(&self) -> u8 {
        match self {
            Direction::North => 0,
            Direction::East => 1,
            Direction::South => 2,
            Direction::West => 3,
        }
    }

    /// Получить вектор направления движения.
    fn as_vector(&self) -> Point {
        let number = i64::from(self.as_number());
        Point {
            x: ((number - 2) * -1) % 2,
            y: ((number - 1) * -1) % 2,
        }
    }
}

/// Игровой мир представляет из себя текущее положение Ёжика и его направление
/// движения.
#[derive(Default)]
struct Game {
    position: Point,
    direction: Direction,
}

impl Game {
    /// Применить следующий ход, записанный Ёжиком.
    fn apply(&mut self, action: &str) {
        match action {
            "L" => self.direction = Direction::from_number(self.direction.as_number() + 3),
            "R" => self.direction = Direction::from_number(self.direction.as_number() + 1),
            "F" => self.position += self.direction.as_vector(),
            _ => panic!("Unknown action"),
        }
    }
}

fn main() {
    let mut game = Game::default();
    let mut action = String::new();
    loop {
        action.clear();
        std::io::stdin().read_line(&mut action).unwrap();
        let action = action.trim();
        if action.is_empty() {
            break;
        }
        game.apply(action);
    }
    println!("{} {}", game.position.x, game.position.y);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_moves() {
        let mut game = Game::default();
        for _ in 0..10 {
            game.apply("L");
            assert_eq!(game.position, Point { x: 0, y: 0 });
        }
        for _ in 0..10 {
            game.apply("R");
            assert_eq!(game.position, Point { x: 0, y: 0 });
        }
    }

    #[test]
    fn no_turns() {
        let mut game = Game::default();
        for y in 1..10_i64 {
            game.apply("F");
            assert_eq!(game.position, Point { x: 0, y: y });
        }
    }

    #[test]
    fn back_to_starting() {
        let mut game = Game::default();
        for _ in 0..4 {
            game.apply("L");
            for _ in 1..10_i64 {
                game.apply("F");
            }
        }
        assert_eq!(game.position, Point { x: 0, y: 0 });
    }

    #[test]
    fn test_no1_from_problem() {
        let mut game = Game::default();
        game.apply("L");
        game.apply("R");
        game.apply("F");
        game.apply("F");
        assert_eq!(game.position, Point { x: 0, y: 2 });
    }
}
