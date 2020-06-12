use std::io;
use std::io::BufRead;
use std::fs::File;
use std::env;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let test_to_run = &args[2];
    
    if let Ok(lines) = open_and_read(filename) {
        for line in lines {
            if let Ok(necklace) = line {
                let split: Vec<&str> = necklace.split(" ").collect();
                match test_to_run as &str {
                    "bonus_1" => {
                        println!("Necklace \"{}\" repeats {} times", necklace, repeats(split[0]));
                    }
                    "bonus_2" => {
                        println!("Not implemented yet.");
                    },
                    _ => {
                        println!("Necklace: {} | Test: {} | Result: {}", split[0], split[1], does_necklace_match((split[0], split[1])));
                    },

                }
            }
        }
    }

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
        if test_string == necklace {
            matches+=1;
        }
    }
    return matches;
}

fn enable_search() {
    
}
