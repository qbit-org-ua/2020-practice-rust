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
#[derive(Clone, Copy)]
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
    /// Получить новое направление при повороте налево
    fn turn_right(self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }

    /// Получить новое направление при повороте налево
    fn turn_left(self) -> Direction {
        match self {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
        }
    }

    /// Получить вектор направления движения.
    fn to_vector(&self) -> Point {
        match self {
            Direction::North => Point { x: 0, y: 1 },
            Direction::East => Point { x: 1, y: 0 },
            Direction::South => Point { x: 0, y: -1 },
            Direction::West => Point { x: -1, y: 0 },
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
            "L" => self.direction = self.direction.turn_left(),
            "R" => self.direction = self.direction.turn_right(),
            "F" => self.position += self.direction.to_vector(),
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
