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
            first.push_str(&first.clone());
            result += first.parse::<i32>().unwrap();
        } else {
            result += &(first + &last).parse().unwrap();
        }

        println!("{} {}", line, result);
    }
}

fn part_2(stdin_iterator: &mut io::Lines<io::StdinLock>) {
    let mut result = 0;

    let mut digit_mappings = std::collections::HashMap::new();

    digit_mappings.insert("one", "1");
    digit_mappings.insert("two", "2");
    digit_mappings.insert("three", "3");
    digit_mappings.insert("four", "4");
    digit_mappings.insert("five", "5");
    digit_mappings.insert("six", "6");
    digit_mappings.insert("seven", "7");
    digit_mappings.insert("eight", "8");
    digit_mappings.insert("nine", "9");

    while let Some(Ok(line)) = stdin_iterator.next() {
        let mut v = vec![];
        let mut acc = String::new();
        let mut chars = line.as_bytes();

        let mut start = 0;
        let mut end = line.len() - 1;

        while start <= line.len() - 1 {
            let c_start = chars[start] as char;
            let c_end = chars[end] as char;
            if let Some(n) = c_start.to_digit(10) {
                v.push(n.to_string());
                acc.clear();
            } else {
                if acc.is_empty() {
                    // println!("{}",c_start);
                    if c_start == 'o'
                        || c_start == 't'
                        || c_start == 'f'
                        || c_start == 's'
                        || c_start == 'e'
                        || c_start == 'n'
                    {
                        // println!("{}",c_start);
                        acc.push(c_start);
                    }
                }
                // println!("{} {}",start,acc.len());

                if acc.len() == 1 {
                    if acc == "o" && c_start == 'n' {
                        acc.push(c_start);
                    } else if acc == "t" && c_start == 'w' {
                        acc.push(c_start);
                    } else if acc == "t" && c_start == 'h' {
                        acc.push(c_start);
                    } else if acc == "f" && c_start == 'o' {
                        acc.push(c_start);
                    } else if acc == "f" && c_start == 'i' {
                        acc.push(c_start);
                    } else if acc == "s" && c_start == 'i' {
                        acc.push(c_start);
                    } else if acc == "s" && c_start == 'e' {
                        acc.push(c_start);
                    } else if acc == "e" && c_start == 'i' {
                        acc.push(c_start);
                    } else if acc == "n" && c_start == 'i' {
                        acc.push(c_start);
                    } else {
                        if acc != c_start.to_string() {
                            acc.clear();
                            acc = c_start.to_string();
                        }
                    }
                }

                if acc.len() == 2 {
                    // println!("{} {}", acc, c_start);
                    if acc == "on" && c_start == 'e' {
                        acc.push(c_start);
                    } else if acc == "tw" && c_start == 'o' {
                        acc.push(c_start);
                    } else if acc == "th" && c_start == 'r' {
                        acc.push(c_start);
                    } else if acc == "fo" && c_start == 'u' {
                        acc.push(c_start);
                    } else if acc == "fi" && c_start == 'v' {
                        acc.push(c_start);
                    } else if acc == "si" && c_start == 'x' {
                        acc.push(c_start);
                    } else if acc == "se" && c_start == 'v' {
                        acc.push(c_start);
                    } else if acc == "ei" && c_start == 'g' {
                        acc.push(c_start);
                    } else if acc == "ni" && c_start == 'n' {
                        acc.push(c_start);
                    } else {
                        let a = &acc[1..];
                        if a != c_start.to_string() {
                            acc = a.to_string() + &c_start.to_string();
                        }
                    }
                }

                if acc.len() == 3 {
                    if acc == "thr" && c_start == 'e' {
                        acc.push(c_start);
                    } else if acc == "fou" && c_start == 'r' {
                        acc.push(c_start);
                    } else if acc == "fiv" && c_start == 'e' {
                        acc.push(c_start);
                    } else if acc == "sev" && c_start == 'e' {
                        acc.push(c_start);
                    } else if acc == "eig" && c_start == 'h' {
                        acc.push(c_start);
                    } else if acc == "nin" && c_start == 'e' {
                        acc.push(c_start);
                    }
                }

                if acc.len() == 4 {
                    if acc == "thre" && c_start == 'e' {
                        acc.push(c_start);
                        start += 1;
                    } else if acc == "seve" && c_start == 'n' {
                        acc.push(c_start);
                    } else if acc == "eigh" && c_start == 't' {
                        acc.push(c_start);
                    }
                }

                if let Some(digit) = digit_mappings.get(acc.as_str()) {
                    v.push(digit.to_string());
                    let acc_len = acc.len();
                    acc.clear();
                    // if start == line.len() - 2 {
                    //     break;
                    // }
                    start = start - (acc_len - 1);
                }
            }
            start += 1;
            // end -= 1;
        }

        println!("{:?}", v);

        if v.len() == 1 {
            let n = &v[0];

            let res = n.to_owned() + n.as_str();

            result += res.parse::<i32>().unwrap();
        } else {
            let first = &v[0];
            let last = v.last().unwrap();

            let res = first.to_owned() + last.as_str();

            result += res.parse::<i32>().unwrap();
        }

        println!("{} {}", line, result);
    }
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    // part_1(&mut stdin_iterator);
    part_2(&mut stdin_iterator);
}
