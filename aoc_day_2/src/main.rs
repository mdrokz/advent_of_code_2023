use std::io::{self, BufRead};

#[derive(Debug)]
enum Cube {
    RED,
    GREEN,
    BLUE,
}

fn part_1(stdin_iterator: &mut io::Lines<io::StdinLock>) {
    let mut games = 0;
    let mut id = 1;
    while let Some(Ok(line)) = stdin_iterator.next() {
        // commenting system
        if line.starts_with("#") {
            continue;
        }


        let chars = line.chars();

        let mut colon_found = false;

        let mut previous = ' ';

        let mut current_digit = String::new();

        let mut cube_configs = vec![];

        let mut sets = vec![];

        for (idx,char) in chars.enumerate() {
            if char == ':' {
                colon_found = true;
            }

            if char == ';' {
                cube_configs.push(sets);
                sets = vec![];
            }

            if sets.len() > 0 && idx == line.len() - 1 {
                cube_configs.push(sets);
                sets = vec![];
            }

            if colon_found {
                if let Some(digit) = char.to_digit(10) {
                    current_digit += &digit.to_string();
                } else {
                    if !current_digit.is_empty() {
                        match char {
                            'r' => {
                                let digit = current_digit.parse::<u32>().unwrap();
                                if digit > 0  && previous != 'g' {
                                    current_digit = String::new();
                                    sets.push((Cube::RED, digit));
                                }
                            }
                            'g' => {

                                let digit = current_digit.parse::<u32>().unwrap();
                                if digit > 0 {
                                    current_digit = String::new();
                                    sets.push((Cube::GREEN, digit));
                                }
                            }
                            'b' => {
                                let digit = current_digit.parse::<u32>().unwrap();
                                if digit > 0 {
                                    current_digit = String::new();
                                    sets.push((Cube::BLUE, digit));
                                }
                            }
                            _ => {}
                        };
                    }

                }
            }
            previous = char;
        }

        let mut is_red_valid = true;
        let mut is_green_valid = true;
        let mut is_blue_valid = true;

        let mut cube_len = cube_configs.len();

        // println!("cubes: {:?}", cube_configs);

        for set in &cube_configs {

            for (cube, digit) in set {
                match cube {
                    Cube::RED => {
                        if *digit <= 12 {
                            is_red_valid = true;
                        } else {
                            is_red_valid = false;
                        }
                    },
                    Cube::GREEN => {
                        if *digit <= 13 {
                            is_green_valid = true;
                        } else {
                            is_green_valid = false;
                        }
                    },
                    Cube::BLUE => {
                        if *digit <= 14 {
                            is_blue_valid = true;
                        } else {
                            is_blue_valid = false;
                        }
                    },
                }
            }
            if is_red_valid && is_green_valid && is_blue_valid {
                cube_len -= 1;
            }

        }

        if cube_len == 0 {
            games += id;
        }

        id+=1;
    }

    println!("games: {}", games);
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    part_1(&mut stdin_iterator);
}
