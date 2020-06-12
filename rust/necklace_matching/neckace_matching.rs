use std::io;
use std::io::BufRead;
use std::fs::File;
use std::env;
use std::collections::HashMap;
use std::time::{Duration, Instant};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let test_to_run = &args[2];

    let mut word_map: HashMap<String, Vec<String>> = HashMap::new();
    let start = Instant::now();
    
    if let Ok(lines) = open_and_read(filename) {
        for line in lines {
            if let Ok(necklace) = line {
                let split: Vec<&str> = necklace.split(" ").collect();
                match test_to_run as &str {
                    "bonus_1" => {
                        println!("Necklace \"{}\" repeats {} times", necklace, repeats(split[0]));
                    }
                    "bonus_2" => {
                        if necklace.len() >= 4 {
                            let rotated: String = rotate_to_sort(split[0]);
                            if word_map.get(&(rotated.clone())).is_none() {
                                word_map.insert(rotated.clone(), Vec::new());
                            } 
                            word_map.get_mut(&(rotated.clone())).unwrap().push(String::from(split[0]));
                        }
                    },
                    _ => {
                        println!("Necklace: {} | Test: {} | Result: {}", split[0], split[1], does_necklace_match((split[0], split[1])));
                    },

                }
            }
        }
    }

    if test_to_run == "bonus_2" {
        for (key, val) in word_map.iter() {
            if (val.len() as i32) == 4 {
                println!("[{}]", val.iter().fold(String::new(), |acc, st| acc + &st.to_string() + ", "));
            }
        }
    }

    let duration = start.elapsed();
    println!("Time elapse: {:?}", duration);
    Ok(())
}

fn open_and_read(filename: &str) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(&filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn does_necklace_match((necklace, test): (&str, &str)) -> bool {
    if necklace.len() != test.len() {
        return false;
    }
    return format!("{}{}", necklace, necklace).contains(test);
}

fn repeats(necklace: &str) -> i32 {
    if necklace.is_empty() {
        return 1;
    }

    let mut test_string: String;
    let mut matches: i32 = 0;
    for i in 0..necklace.len() {
        let back = String::from(necklace.get(..i+1).unwrap());
        let front = String::from(necklace.get(i+1..).unwrap());
        test_string = front + &back;
        println!("{}", test_string);
        if test_string == necklace {
            matches+=1;
        }
    }
    return matches;
}

fn rotate_to_sort(necklace: &str) -> String {
    let mut best_string: Option<String> = Some(String::from(""));
    let mut best_char: char = 'z';
    let mut test_string: String;
    for i in 0..necklace.len() {
        let back = String::from(necklace.get(..i+1).unwrap());
        let front = String::from(necklace.get(i+1..).unwrap());
        test_string = front + &back;

        let vec: Vec<char> = test_string.chars().collect();
        if (vec[0] as i32) < (best_char as i32) {
            match best_string.as_mut() {
                Some(v) => *v = test_string,
                None => {},
            }
            best_char = vec[0];
        }
    }
    return best_string.unwrap();
}
