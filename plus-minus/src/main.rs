use std::io::{self, BufRead};

/*
 * Complete the 'plusMinus' function below.
 *
 * The function accepts INTEGER_ARRAY arr as parameter.
 */

fn plusMinus(arr: &[i32]) {
    let mut positive = 0;
    let mut negative = 0;
    let mut zero = 0;

    for element in arr {
        if *element == 0 {
            zero = zero + 1;
        } else {
            if *element < 0 {
                negative = negative + 1;
            } else {
                positive = positive + 1;
            }
        }
    }

    println!("{}", positive/arr.len());
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let arr: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    plusMinus(&arr);
}
