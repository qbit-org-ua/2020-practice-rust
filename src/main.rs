use std::collections::{HashMap, HashSet};
use std::convert::TryInto;

use num_rational::Ratio;

fn main() {
    // фиксированный массив
    let four_ints: [i32; 4] = [1, 2, 3, 4];
    //let four_ints: [i32; 30] = [0; 30];
    println!("{:?}", four_ints);

    // динамический массив
    let mut vector: Vec<i32> = vec![1, 2, 3, 4];
    vector.push(5);
    if vector.len() == 5 {
        println!("YAY!");
    }
    //let vector = four_ints.to_vec();
    println!("{:?}", vector);

    let x = 10;
    let y = 20;
    let mut point: (i32, i32) = (x, y);
    point.0 = 30;
    println!("Point: {{ {}, {} }}", point.0, point.1);
    let (x1, y1) = point;
    println!("x1 = {}, y1 = {}", x1, y1);

    let find_value: i32 = 5;
    /*let mut index = 0;
    for (i, element) in vector.iter().enumerate() {
        if *element == find_value {
            index = i;
            break;
        }
    }*/
    //let index = vector.iter().find
    // vector.find == O(N)
    // M numbers are in Vec = O(N*M)
    //println!("index of {} value is {} in {:?}", find_value, index, vector);

    let numbers = vec![1, 2, 2, 2, 3, 4, 5];
    println!(
        "Unique: {}",
        numbers.iter().cloned().collect::<HashSet<i32>>().len()
    );
    let these_numbers = vec![0, 1, 5, 6];
    let mut counter = 0;
    // find == O(1)
    let numbers = numbers.iter().cloned().collect::<HashSet<i32>>();
    println!("{:?}", numbers);
    for m in &these_numbers {
        if numbers.contains(m) {
            counter += 1;
        }
    }
    println!("{}", counter);

    let these_numbers_set = these_numbers.iter().cloned().collect::<HashSet<i32>>();
    println!(
        "set numbers: {:?}; set these_number: {:?}",
        numbers, these_numbers_set
    );
    println!(
        "Пересечение: {:?}",
        numbers
            .intersection(&these_numbers_set)
            .collect::<Vec<&i32>>()
    );
    println!(
        "Объединение: {:?}",
        numbers.union(&these_numbers_set).collect::<Vec<&i32>>()
    );

    let numbers: Vec<i32> = vec![1, 2, 2, 2, 3, 3, 3];
    let mut counts: HashMap<i32, u64> = HashMap::new();
    for number in &numbers {
        let record_in_counts = counts.entry(*number).or_default();
        *record_in_counts += 1;
        /*
        if counts.contains_key(number) {
            let previous_count = counts.get(number).unwrap();
            counts.insert(*number, previous_count + 1);
        } else {
            counts.insert(*number, 1);
        }*/
    }
    println!("Counts: {:?}", counts);

    let numbers: Vec<&str> = vec!["Vasya", "Petya", "Vlad", "Vlad", "Vlad"];
    let mut counts: HashMap<&str, u64> = HashMap::new();
    for number in &numbers {
        let record_in_counts = counts.entry(*number).or_default();
        *record_in_counts += 1;
    }
    println!("Names Counts: {:?}", counts);

    let numbers = vec!["Vasya", "Petya", "Vlad", "Vlad", "Vlad"];
    let mut counts = HashMap::<_, u64>::new();
    for number in &numbers {
        *counts.entry(*number).or_default() += 1;
    }
    println!("Names Counts: {:?}", counts);

    let a = Ratio::new_raw(11, 10);
    let b = Ratio::new_raw(9, 10);
    println!("{}", a + b);

    vector.sort_unstable();
    vector.reverse();
    let new_vector: Vec<_> = vector.into_iter().rev().collect();
    let b: bool = true; // false;
}

fn _old3() {
    let s = "abcd";
    println!("\" {:?}", s.chars().collect::<Vec<char>>());
    for i in (3..7 + 1).rev() {
        println!("{}", i);
    }

    println!("{:.3}", 0.33993333);

    let mut line = String::new();
    loop {
        line.clear();
        std::io::stdin().read_line(&mut line);
        if line.is_empty() {
            //if read_result.unwrap() == 0 {
            break;
        }
        println!("NExt");
    }
    println!("END");
}

fn _old2() {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).expect("Ooops");
    /*
    let split_line: Vec<&str> = line.split_ascii_whitespace().collect();
    let mut numbers: Vec<i64> = Vec::new();
    for item in &split_line {
        numbers.push(item.parse().unwrap());
    }
    */
    let mut numbers: Vec<i64> = Vec::new();
    for item in line.split_ascii_whitespace() {
        numbers.push(item.parse().unwrap());
    }

    let numbers = line
        .split_ascii_whitespace()
        .map(|item| item.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();
    println!("{:?}", numbers);

    //let data: String = std::fs::read_to_string("input.txt").unwrap();

    //std::fs::write("output.txt", "asd").unwrap();
}

fn _old() {
    let x = 1i64;
    let y: i32 = x.try_into().unwrap();
    let z;
    z = x;
    println!("{} {}", x + z, y);

    let x = "hello world!";
    println!("{}", x);

    let _s: String = "hello world".to_string();
    let _s: String = String::from("hello world");
    let s = String::from("hello world");
    println!("{}", s);
    // Нельзя!
    // s = 123;
    // s += 4;
    // s = String::from("qwe");

    let s2: String = s + "qwe";
    println!("{}", &s2[0..3]);

    assert!(s2 == "hello worldqwe");
    assert!(s2 != 123.to_string());

    let mut s3 = String::new();
    s3.push_str("Hello!");
    println!("{}", s3);

    let mut line = String::new();
    std::io::stdin().read_line(&mut line).expect("Ooops");

    //let line = line.trim();

    //let maybe_number = line.parse();
    let _number: i64 = line.trim().parse().unwrap();

    println!("USER INPUT: '{}'", line);

    let parts: Vec<&str> = line.split(":").collect();
    println!("{:?}", parts);
    let x = 12i32;
    println!("{}", x.pow(3));
}
