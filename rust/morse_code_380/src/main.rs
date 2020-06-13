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

    print_vec!(code_map.get(&String::from("-...-....-.--.")).unwrap());

    // Execute tests
    match challenge_to_run as &str {
        "1" => {
            println!("Not yet implemented.");
        }
        "2" => {
            println!("Not yet implemented.");
        }
        "3" => {
            println!("Not yet implemented.");
        }
        "4" => {
            println!("Not yet implemented.");
        }
        "5" => {
            println!("Not yet implemented.");
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
}