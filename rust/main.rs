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
    let mut super_perm: String = String::new();
    let mut test_string = permutations.get(0)
        .expect("Failed to get initial string for find super perm.").clone();
    permutations.remove(0);
    super_perm.push_str(&test_string);

    // let mut optional = find_largest_overlap(&test_string, permutations);
    // while permutations.len() > 0 {
    //     println!("---");
    //     match optional {
    //         Some((best_overlap, count)) => {
    //             // Take the overlap number of chars and push into super perm
    //             let str: &str = &best_overlap[0..(count as usize)];
    //             super_perm.push_str(str);
    //
    //             // Remove the best_overlap string from the permutations
    //             let index_of_overlap = permutations
    //                 .iter()
    //                 .position(|x| x.eq(&best_overlap))
    //                 .expect("Couldn't find the best overlap in the permutation list.");
    //             permutations.remove(index_of_overlap);
    //             optional = find_largest_overlap(&best_overlap, permutations)
    //         },
    //         _ => break
    //     }
    //     println!("Superpermutation: {}", super_perm);
    // }
    let mut optional = find_largest_overlap(&test_string, permutations);
    loop {
        match optional {
            Some((best_overlap, count)) => {
                // Take the overlap number of chars and push into super perm
                let str: &str = &best_overlap[0..(count as usize)];
                super_perm.push_str(str);

                // Remove the best_overlap string from the permutations
                let index_of_overlap = permutations
                    .iter()
                    .position(|x| x.eq(&best_overlap))
                    .expect("Couldn't find the best overlap in the permutation list.");
                permutations.remove(index_of_overlap);

                optional = find_largest_overlap(&best_overlap, permutations);
            },
            _ => { break; }
        }
    }

    return String::from(super_perm);
}

fn find_largest_overlap(target_string: &str, test_strings: &Vec<String>) -> Option<(String, i32)> {
    let mut best_overlap = 1;
    let mut best_string_overlap: String = String::new();
    println!("Target String: {}", target_string);
    for i in 0..test_strings.len() {
        let test_string = test_strings.get(i).expect("");
        let mut chopped_string = String::from(&test_string[0..test_string.len() - 1]).clone();

        let overlap = match target_string.find(&chopped_string) {
            None => -1,
            Some(index) => (target_string.len() as i32) - (index as i32)
        };
        if overlap >= 1 {
            if overlap > best_overlap {
                best_overlap = overlap;
                best_string_overlap = test_strings
                    .get(i)
                    .expect("Failed to get string from test string vector")
                    .clone();
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
        Some((best_string_overlap.clone(), best_overlap as i32))
    }
}

fn i32_vec_to_string(numbers: &[i32]) -> String {
    return numbers.clone().iter().map(|x| (x + 1).to_string()).collect();
}
