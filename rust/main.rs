use std::time::{Instant};
use std::env;

fn main() {
    // Start timing
    let start = Instant::now();

    // Get arguments
    let args: Vec<String> = env::args().collect();
    let count: usize = (&args[1].parse::<usize>().expect("Unable to parse count of elements")).clone();

    let mut vec: Vec<i32> = vec![0; count]
        .iter_mut()
        .enumerate()
        .map(|(i, _)| i as i32)
        .collect();
    let mut arr = vec.as_mut_slice();
    let mut permutations: Vec<String> = Vec::new();
    generate_permutations(count as i32, &mut arr, &mut permutations);
    println!("Number of permutations: {}", permutations.len());
    println!("Superpermutation: {}", find_super_perm(&mut permutations));

    // Finish timing
    let duration = start.elapsed();
    println!("Time elapsed: {:?}", duration);
}

// Following Heap's Algorithm for generating permutations - https://en.wikipedia.org/wiki/Heap%27s_algorithm
fn generate_permutations(count: i32, arr: &mut[i32], perms: &mut Vec<String>) {
    if count == 1 {
        perms.push(i32_vec_to_string(arr));
    } else {
        generate_permutations(count - 1, arr, perms);
        for i in 0..count - 1 {
            if i % 2 == 0 {
                let temp = arr[i as usize];
                arr[i as usize] = arr[(count - 1) as usize];
                arr[(count - 1) as usize] = temp;
            } else {
                let temp = arr[0 as usize];
                arr[0 as usize] = arr[(count - 1) as usize];
                arr[(count - 1) as usize] = temp;
            }
            generate_permutations(count - 1, arr, perms);
        }
    }
}

fn find_super_perm(permutations: &mut Vec<String>) -> String {
    permutations.sort();

    let mut visited_permutations: Vec<String> = Vec::new();
    let mut test_string = permutations.get(0)
        .expect("Failed to get initial string for find super perm.").clone();
    permutations.remove(0);
    // let best_overlap = find_largest_overlap(&init_string, permutations)
    //     .expect("Failed to get the best overlap for test string.");
    // println!("{}", best_overlap);

    while let Some(best_overlap) = find_largest_overlap(&test_string, permutations) {
        println!("Overlap: {:?}", best_overlap);

    }

    return String::from("");
}

fn find_largest_overlap(target_string: &str, test_strings: &Vec<String>) -> Option<(String, i32)> {
    let mut best_overlap = 1;
    let mut best_string_overlap: String = String::new();
    for i in 0..test_strings.len() {
        let test_string = test_strings.get(i).expect("");
        let chopped_string = String::from(&test_string[0..test_string.len() - 1]).clone();
        let overlap = match target_string.find(&chopped_string) {
            None => -1,
            Some(index) => (target_string.len() as i32) - (index as i32)
        };
        if overlap >= 1 {
            if overlap > best_overlap {
                best_overlap = overlap;
                best_string_overlap = test_strings.get(i).expect("Failed to get string from test string vector").clone();
            }

            if overlap == (target_string.len() as i32) - 1 {
                best_string_overlap = test_string.clone();
                break;
            }
        } else {
            continue;
        }
    }
    return if best_overlap == 1 {
        None
    } else {
        Some((best_string_overlap.clone(), best_overlap))
    }
}

fn i32_vec_to_string(numbers: &[i32]) -> String {
    return numbers.clone().iter().map(|x| (x + 1).to_string()).collect();
}
