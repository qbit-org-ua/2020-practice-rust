//! Авторы похожих решений:
//! * Ткачёв Захар

struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn distance(&self, other: &Point) -> f64 {
        ((other.x - self.x).powi(2) + (other.y - self.y).powi(2)).sqrt()
    }
}
// point: Point, point_2: Point
// point.distance(point_2)

fn distance(point_1: &Point, point_2: &Point) -> f64 {
    ((point_1.x - point_2.x).powi(2) + (point_1.y - point_2.y).powi(2)).sqrt()
}

fn main() {
    let mut line = String::new();
    let mut points: Vec<Point> = Vec::new();
    for _ in 0..3 {
        line.clear();
        std::io::stdin().read_line(&mut line).unwrap();
        let coordinates: Vec<f64> = line
            .trim()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        let point = Point {
            x: coordinates[0],
            y: coordinates[1],
        };
        points.push(point);
    }

    /*
    let points: Vec<Point> = (0..3)
        .map(|_| {
            let mut line = String::new();
            std::io::stdin().read_line(&mut line).unwrap();
            let coordinates: Vec<f64> = line
                .trim()
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();
            Point {
                x: coordinates[0],
                y: coordinates[1],
            }
        })
        .collect();
        */

    //let a = distance(&points[0], &points[1]);

    let a = points[0].distance(&points[1]);
    let b = points[1].distance(&points[2]);
    let c = points[2].distance(&points[0]);

    let p = (a + b + c) / 2.0;

    println!("{:.3}", (p * (p - a) * (p - b) * (p - c)).sqrt());
}
