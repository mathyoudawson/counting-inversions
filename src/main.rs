use std::fs::File;
use std::io::{BufReader, BufRead};

fn main() {
    let path = "src/integers.txt";
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let mut input_integers: Vec<i32> = Vec::new();

    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap(); // Ignore errors.
        input_integers.push(line.parse().unwrap());
    }

    let n: usize = input_integers.len();
    let inversion_count: i64 = count_inversions(n, &input_integers);
    println!("There are {} inversions", inversion_count);
}

fn count_inversions(n: usize, integers: &Vec<i32>) -> i64 {
    let mut inversions: i64 = 0;

    for i in 0..n {
        for j in i + 1..n {
            if integers[i] > integers [j] {
                inversions += 1;
            } 
        }
    }

    inversions
}
