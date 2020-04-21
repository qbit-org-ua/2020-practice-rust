use std::collections::HashMap;

fn most_frequent(x: HashMap<char, u64>) -> (char, u64) {
    x.into_iter()
        .max_by_key(|(character, count)| (*count, -(*character as i32)))
        .unwrap()
}

    fn main() {
    let mut x: Vec<(&str, u64)> = vec![("aab", 4), ("aaa", 4)];

    assert!("aaa" < "b");
    assert!("aaa" < "aab");
    assert!("aaa" < "aaaa");
    x.sort_by(|(word_1, number_1), (word_2, number_2)| {
        number_2.cmp(number_1).then_with(|| word_1.cmp(word_2))
    });
    println!("{:?}", x);

    for i in (0..=4).rev() {
        println!("{}", i);
    }

    let x: HashMap<char, u64> = [('a', 4), ('b', 5)].iter().cloned().collect();

    let y: Vec<(char, u64)> = x.into_iter().collect();

    for element in y {
        println!("{}", element.0);
    }

    //println!("{:?}", most_frequent(x));
    let char_a = 'a';
    let char_b = 'b';
    assert!(char_a < char_b);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]

    fn test_most_frequent() {
        let tests = vec![
            (vec![('a', 10)], 'a'),
            (vec![('b', 3), ('a', 10)], 'a'),
            (vec![('a', 10), ('b', 3)], 'a'),
            (vec![('a', 10), ('b', 10)], 'a'),
            (vec![('a', 10), ('b', 11)], 'b'),
        ];
        for (frequences, expected_char) in tests {
            let frequences = frequences.into_iter().collect();
            assert_eq!(most_frequent(frequences).0, expected_char);
        }
    }
}
