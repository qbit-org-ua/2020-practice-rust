use std::convert::TryInto;

#[derive(Default)]
struct Reader {
    line: String,
}

impl Reader {
    fn read_numbers(&mut self) -> impl Iterator<Item = i64> + '_ {
        std::io::stdin().read_line(&mut self.line).unwrap();
        self.line
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<i64>().unwrap())
    }
}

struct Range {
    start: i64,
    stop: i64,
    step: i64,
}

impl Iterator for Range {
    type Item = i64;

    fn next(&mut self) -> Option<Self::Item> {
        let x = self.start;
        if self.start - self.step == self.stop {
            None
        } else {
            self.start += self.step;
            Some(x)
        }
    }
}

impl ExactSizeIterator for Range {
    fn len(&self) -> usize {
        ((self.stop - self.start) / self.step + 1)
            .try_into()
            .unwrap()
    }
}

fn main() {
    let mut reader = Reader::default();
    let mut numbers = reader.read_numbers();
    let start = numbers.next().unwrap();
    let stop = numbers.next().unwrap();
    let range = Range {
        start,
        stop,
        step: if start < stop { 1 } else { -1 },
    };

    println!("{} very important numbers:", range.len());
    for x in range {
        println!("{}", x);
    }
}
