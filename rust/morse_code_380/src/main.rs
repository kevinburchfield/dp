use std::env;
use std::time::{Instant};
use std::io;
use std::io::BufRead;
use std::fs::File;
use std::collections::HashMap;

macro_rules! print_vec {
    ($vec: expr) => {
        println!("[{}]", $vec.iter().fold(String::new(), |acc, st| acc + &st.to_string() + ", "));
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let challenge_to_run = &args[2];

    // Create map to be used for tests
    let mut code_map: HashMap<String, Vec<String>> = HashMap::new();

    // Start timing
    let start = Instant::now();

    // Iterate and assign to map
    if let Ok(lines) = open_and_read(filename) {
        for line in lines {
            if let Ok(line) = line {
                let code = dot_dash(&line);
                if code_map.get(&code.clone()).is_none() {
                    code_map.insert(code.clone(), Vec::new());
                }
                code_map.get_mut(&code.clone()).map(|x| x.push(line.clone()));
            }
        }
    }

    // Execute tests
    match challenge_to_run as &str {
        "1" => {
            for (key, val) in code_map.iter() {
                if val.len() == 13 {
                    println!("Only sequence for 13 words: {}", key);
                }
            }
        }
        "2" => {
            for (key, val) in code_map.iter() {
                // Possible to do regex for match?
                if key.contains("---------------") {
                    println!("Only word that encodes with 15 dashes in a row: {}", val[0]);
                }
            }
        }
        "3" => {
            println!("Not yet implemented.");
        }
        "4" => {
            for (key, val) in code_map.iter() {
                if seq_is_palindrome(key) && val.iter().any(|w| w.len() == 13) {
                    println!("The only 13 letter word that is a palindrome is: {}",
                             val.iter().find(|w| w.len() == 13).unwrap());
                }
            }
        }
        "5" => {
            let possible_chars: Vec<&str> = vec![".", "-"];
            let mut strings: Vec<String> = Vec::new();
            for x in 0..(2_usize.pow(13)) {
                let mut string = "".to_string();
                let mut integer = x;

                // Create a string based off of the binary presentation of the number mapped to chars
                for _ in 0..13 {
                    let char_index = integer % 2;
                    integer = integer >> 1;
                    string.push_str(possible_chars[char_index]);
                }
                println!("{}", string.len());
            }
        }
        _ => {
            println!("Not a valid challenge.");
        }
    }

    // Finish timing
    let duration = start.elapsed();
    println!("Time elapsed: {:?}", duration);
}

fn open_and_read(filename: &str) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(&filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn dot_dash(string: &str) -> String {
    let res: Vec<char> = string
        .chars()
        .collect();
    let code = res
        .iter()
        .fold(String::from(""), |acc, c| acc + &get_morse_code(c));
    return code;
}

fn seq_is_palindrome(sequence: &str) -> bool {
    let sequence_length = sequence.len();
    return match sequence_length % 2 {
        0 => {
            let front = sequence.get(..(sequence_length / 2)).unwrap();
            let mut back: Vec<char> = sequence.get((sequence_length / 2)..).unwrap().chars().collect();
            back.reverse();
            let rev_back: String = back.iter().collect();
            front.eq(&rev_back)
        },
        1 => {
            let front = sequence.get(..(sequence_length / 2)).unwrap();
            let mut back: Vec<char> = sequence.get((sequence_length / 2) + 1..).unwrap().chars().collect();
            back.reverse();
            let rev_back: String = back.iter().collect();
            front.eq(&rev_back)
        },
        _ => {
            println!("This is literally not possible.");
            false
        }
    }
}

fn get_morse_code(char: &char) -> String {
    match char {
        'a' => String::from(".-"),
        'b' => String::from("-..."),
        'c' => String::from("-.-."),
        'd' => String::from("-.."),
        'e' => String::from("."),
        'f' => String::from("..-."),
        'g' => String::from("--."),
        'h' => String::from("...."),
        'i' => String::from(".."),
        'j' => String::from(".---"),
        'k' => String::from("-.-"),
        'l' => String::from(".-.."),
        'm' => String::from("--"),
        'n' => String::from("-."),
        'o' => String::from("---"),
        'p' => String::from(".--."),
        'q' => String::from("--.-"),
        'r' => String::from(".-."),
        's' => String::from("..."),
        't' => String::from("-"),
        'u' => String::from("..-"),
        'v' => String::from("...-"),
        'w' => String::from(".--"),
        'x' => String::from("-..-"),
        'y' => String::from("-.--"),
        'z' => String::from("--.."),
        _ => String::from(""),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sos() {
        assert_eq!(dot_dash("sos"), String::from("...---..."));
    }

    #[test]
    fn test_daily() {
        assert_eq!(dot_dash("daily"), String::from("-...-...-..-.--"));
    }

    #[test]
    fn test_programmer() {
        assert_eq!(dot_dash("programmer"), String::from(".--..-.-----..-..-----..-."));
    }

    #[test]
    fn test_bits() {
        assert_eq!(dot_dash("bits"), String::from("-.....-..."));
    }

    #[test]
    fn test_three() {
        assert_eq!(dot_dash("three"), String::from("-.....-..."));
    }

    #[test]
    fn test_seq_is_palindrome() {
        assert!(seq_is_palindrome("..."));
        assert!(seq_is_palindrome("--"));
        assert!(!seq_is_palindrome("--."));
        assert!(!seq_is_palindrome("--.-"));
    }
}