//Тут виникають помилки, хоча на хаккерранк все працює =(
//https://www.hackerrank.com/challenges/simple-array-sum/problem?isFullScreen=true

use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

fn simpleArraySum(ar: &[i32]) -> i32 {
    let length = ar.len();
    let mut result = 0;

    for i in 0..length{
        result += ar[i];
    }

    return result;
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let _ar_count = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let ar: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let result = simpleArraySum(&ar);

    writeln!(&mut fptr, "{}", result).ok();
}