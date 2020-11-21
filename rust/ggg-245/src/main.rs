use std::collections::HashMap;
use std::time::{Instant};
use std::io;
use std::io::BufRead;
use std::fs::File;
use std::env;
use rand::distributions::{Distribution, Uniform};
use rand::seq::SliceRandom;

extern crate rand;

fn main() {
    // Start timing
    let start = Instant::now();

    // Get arguments
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let mut action = String::new();
    if args.len() == 3 {
        action = String::from(&args[2]);
    }

    match action.as_str() {
        "d" => {
            // Get the ggg key and the encoding
            let mut key: String = String::new();
            let mut encoding: String = String::new();
            if let Ok(lines) = open_and_read(filename) {
                let mut index: i32 = 0;
                for line in lines {
                    if let Ok(line) = line {
                        if index == 0 {
                            key = line;
                        } else {
                            encoding.push_str(&line);
                        }
                        index += 1;
                    }
                }
            }

            // Print out encoded
            println!("Encoded: {}", encoding);

            // Get + print decoded string
            let decoded_string = decode(&key_to_map(&key), &encoding);
            println!("Decoded: {}", decoded_string);
        },
        "e" => {
            let mut string_to_encode = String::new();
            if let Ok(lines) = open_and_read(filename) {
                for line in lines {
                    if let Ok(line) = line {
                        string_to_encode = line;
                    }
                }
            }

            // Print out string to encode
            println!("Encoding: {}", string_to_encode);

            // Encode and print
            let (encoded_string, key) = encode(&string_to_encode);
            println!("Key: {}", key);
            println!("Encoded: {}", encoded_string);
        },
        _ => {
            panic!("You did not choose to encode or decode!");
        }
    }

    // Finish timing
    let duration = start.elapsed();
    println!("Time elapsed: {:?}", duration);
}

fn open_and_read(filename: &str) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn key_to_map(key: &str) -> HashMap<&str, &str> {
    // Split the key into str for processing
    let split_key: Vec<&str> = key
        .split(" ").collect::<Vec<&str>>();

    // Partition into even/odd vectors so we can put into hashmap
    let mut even: Vec<&str> = Vec::new();
    let mut odd: Vec<&str> = Vec::new();
    for (index, key) in split_key.iter().enumerate() {
        if index % 2 == 0 {
            even.push(key.clone())
        } else {
            odd.push(key.clone())
        }
    }

    let mut map: HashMap<&str, &str> = HashMap::new();
    // Iterate through the vectors
    for (_, coded) in even
        .iter()
        .zip(odd.iter()).enumerate() {
        map.insert(coded.1, coded.0);
    }
    return map;
}

fn decode(key_map: &HashMap<&str, &str>, coded_string: &str) -> String {
    let mut coded_clone: String = String::from(coded_string);
    let mut decoded_string: String = String::new();

    let string_length = coded_string.len();
    while !coded_clone.is_empty() {
        for i in 1..string_length {

            // Pull a slice off starting from the beginning
            let slice = &coded_clone[0..i];

            // If the slice is a character that is not 'G' or 'g, pass it through to the decoded string
            if !valid_slice(slice) {
                decoded_string.push_str(slice);
                coded_clone = coded_clone.replacen(slice, "", 1);
                break;
            }

            // Otherwise, decode it and push it in
            let mapped_char = key_map.get(slice);
            if !mapped_char.is_none() {
                decoded_string.push_str(mapped_char.expect("Failed to get the mapped character"));
                coded_clone = coded_clone.replacen(slice, "", 1);
                break;
            }
        }
    }
    return decoded_string;
}

fn encode(string_to_encode: &str) -> (String, String) {
    let encoding_map = generate_encoding_map();
    let mut encoded_string = String::new();
    let mut key = String::new();
    let mut visited_chars: Vec<char> = vec![];
    for c in string_to_encode.chars() {

        // Pull characters off of the string, encode it and push it into the encoded string
        if valid_char(&c) {
            let code = encoding_map.get(&c.to_string()).expect("Failed to get encoding for char");
            encoded_string.push_str(code);

            if !visited_chars.contains(&c) {
                let key_for_char = format!("{} {} ", c, code);
                key.push_str(&key_for_char);
                visited_chars.push(c);
            }
        } else {
            encoded_string.push_str(&c.to_string());
        }
    }
    return (encoded_string, String::from(key.trim()));
}

fn generate_encoding_map() -> HashMap<String, String> {
    let mut map: HashMap<String, String> = HashMap::new();
    for i in (65 as u8)..=(90 as u8) {
        generate_encoding_for_char(i, &mut map);
    }

    // a-z
    for j in (97 as u8)..=(122 as u8) {
        generate_encoding_for_char(j, &mut map);
    }

    return map;
}

fn generate_encoding_for_char(ascii_val: u8, encoding_map: &mut HashMap<String, String>) {
    let possible_chars = vec!['G', 'g', 'G', 'g', 'G', 'g', 'G', 'g', 'G', 'g', 'G', 'g'];
    let mut found_encoding = false;
    while !found_encoding {
        let sampled_chars: Vec<char> = possible_chars
            .choose_multiple(&mut rand::thread_rng(), 6)
            .map(|char| char.clone())
            .collect();
        let encoding = sampled_chars.into_iter().collect();
        let ascii = format!("{}", ascii_val as char);

        // Make sure that encoding is not a subslice of previous encoding
        let no_subslices = encoding_map
            .values()
            .all(|val| !val.starts_with(&encoding));

        if no_subslices {
            encoding_map.insert(ascii, encoding);
            found_encoding = true;
        }
    };
}

fn valid_slice(slice: &str) -> bool {
    return slice.contains("G") || slice.contains("g");
}

fn valid_char(c: &char) -> bool {
    let ascii_val = *c as u32;
    return (ascii_val >= 65 && ascii_val <=90) || (ascii_val >= 97 && ascii_val <= 122);
}
