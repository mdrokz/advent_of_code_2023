use std::io::{self, BufRead};

fn part_1(stdin_iterator: &mut io::Lines<io::StdinLock>) {
    let mut result = 0;

    while let Some(Ok(line)) = stdin_iterator.next() {
        let mut first = String::new();
        let mut last = String::new();
        for c in line.chars() {
            if let Some(n) = c.to_digit(10) {
                if first.is_empty() {
                    first = n.to_string();
                } else {
                    last = n.to_string();
                }
            }
        }

        if last.is_empty() {
            first.push_str(&first.clone()) ;
            result += first.parse::<i32>().unwrap();
        } else {
            result += &(first + &last).parse().unwrap();
        }

        println!("{} {}", line, result);
    }
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    part_1(&mut stdin_iterator);
}
