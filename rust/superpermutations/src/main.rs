use std::time::{Instant};
use std::env;
use std::iter::Map;
use std::collections::HashMap;

fn main() {
    // Start timing
    let start = Instant::now();

    // Get arguments
    let args: Vec<String> = env::args().collect();
    let count: usize = (&args[1].parse::<usize>().expect("Unable to parse count of elements")).clone();

    let mut vec: Vec<usize> = vec![0; count]
        .iter_mut()
        .enumerate()
        .map(|(i, _)| i)
        .collect();
    let mut permutations: Vec<String> = Vec::new();
    generate(count, &mut vec, &mut permutations);
    println!("Number of permutations: {}", permutations.len());
    permutations.sort();
    println!("Super Permutation: {}", find_super_perm(&mut permutations));

    // Finish timing
    let duration = start.elapsed();
    println!("Time elapsed: {:?}", duration);
}

// Following Heap's Algorithm for generating permutations - https://en.wikipedia.org/wiki/Heap%27s_algorithm
fn generate(n : usize, a : &mut Vec<usize>, perm: &mut Vec<String>) {
    if n == 1 {
        perm.push(usize_vec_to_string(a));
    }
    else {
        for i in  0 .. n - 1 {
            generate(n - 1, a, perm);

            if n % 2 == 0 {
                a.swap(i, n - 1);
            }
            else {
                a.swap(0, n - 1);
            }
        }
        generate(n - 1, a, perm);
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

    // THIS IS THE ONE
    // let mut optional = find_largest_overlap(&test_string, permutations);
    // loop {
    //     match optional {
    //         Some((best_overlap, count)) => {
    //             // Take the overlap number of chars and push into super perm
    //             let str: &str = &best_overlap[0..(count as usize)];
    //             println!("str: {}", str);
    //             super_perm.push_str(str);
    //
    //             // Remove the best_overlap string from the permutations
    //             let index_of_overlap = permutations
    //                 .iter()
    //                 .position(|x| x.eq(&best_overlap))
    //                 .expect("Couldn't find the best overlap in the permutation list.");
    //             permutations.remove(index_of_overlap);
    //
    //             optional = find_largest_overlap(&best_overlap, permutations);
    //         },
    //         _ => { break; }
    //     }
    // }

    let mut overlap = find_largest_overlap_new(&test_string, permutations);
    while permutations.len() > 0 {
        // Take the overlap number of chars and push into super perm
        let str = &overlap.0[(overlap.1)..(overlap.0).len()];
        super_perm.push_str(str);
        println!("{}", overlap.0);

        // Remove the best_overlap string from the permutations
        if permutations.len() > 1 {
            let index_of_overlap = permutations
                .iter()
                .position(|x| x.eq(&overlap.0))
                .expect("Couldn't find the best overlap in the permutation list.");
            permutations.remove(index_of_overlap);
        } else {
            permutations.remove(0);
        }
        if permutations.len() > 0 {
            overlap = find_largest_overlap_new(&overlap.0, permutations);
        }
    }

    return String::from(super_perm);
}

fn find_largest_overlap_new(target_string: &str, permutations: &Vec<String>) -> (String, usize) {
    let mut iterations = 1;
    let mut chopped_targeted = &target_string[1..target_string.len()];

    // Short circuit this because there is only one permutation left
    if permutations.len() == 1 {
        return (permutations
                    .get(0)
                    .expect("Failed to get the only remaining perm")
                    .clone(), 2);
    }

    let mut matched_permutations: Vec<usize> = permutations
        .iter()
        .map(|x| x.matches(chopped_targeted).count())
        .collect();
    let mut overlap_found = matched_permutations.iter().any(|x| x > &(0 as usize));
    while !overlap_found {
        chopped_targeted = &chopped_targeted[1..chopped_targeted.len()];
        matched_permutations = permutations
            .iter()
            .map(|x| x.matches(chopped_targeted).count())
            .collect();
        overlap_found = matched_permutations
            .iter()
            .any(|x| x > &(0 as usize));
        iterations += 1;
    }

    let all_match = matched_permutations
        .iter()
        .all(|x| x == &(1 as usize));

    let mut index_of_match = 0;
    if all_match {
        let last_char = &target_string[target_string.len() - 1..target_string.len()];
        let index = permutations
            .iter()
            .position(|x| x[0..1].eq(last_char))
            .expect("Could not find a permutation - this shouldn't happen")
            .clone();
        index_of_match = index;
    } else {
        index_of_match = matched_permutations
            .iter()
            .position(|&r| r == 1 as usize)
            .expect("Could not find the index of the match")
            .clone()
    }

    return (permutations
        .get(index_of_match)
        .expect("Could not get permutation for index of match")
        .clone(), (target_string.len() - iterations));
}

fn find_largest_overlap(target_string: &str, test_strings: &Vec<String>) -> Option<(String, i32)> {
    let mut best_overlap = 1;
    let mut best_string_overlap: String = String::new();
    println!("Target String: {}", target_string);

    /* Enumerate through the test strings to build a map of the chopped strings.
    We do this so that we can check all of the permutations for maximum overlap before moving
    to the next chopped length.
    */
    let mut chopped_map: HashMap<i32, Vec<(String, String)>> = HashMap::new();
    for j in 1..target_string.len() {
        chopped_map.insert(j as i32, Vec::new());
    }
    for (_, perm_string) in test_strings.iter().enumerate() {
        for j in 1..target_string.len() {
            chopped_map
                .get_mut(&((perm_string.len() - j) as i32))
                .expect("Failed to find vector of chopped strings for length")
                .push((String::from(&perm_string[0..perm_string.len() - j]), String::from(perm_string)));
        }
    }
    for iter in (1..=(target_string.len() - 1)).rev() {
        let chopped_strings = chopped_map
            .get(&(iter as i32))
            .expect("Failed to get the perms for a given length.");
        for i in 0..chopped_strings.len() {
            let test_string = chopped_strings.get(i).expect("").clone();
            let overlap = match target_string.find(&test_string.0) {
                None => -1,
                Some(index) => (target_string.len() as i32) - (index as i32)
            };
            if overlap >= 1 {
                if overlap > best_overlap {
                    best_overlap = overlap;
                    best_string_overlap = test_string.1.clone();
                    println!("Set the best: {} {}", best_overlap, best_string_overlap);
                }

                if overlap == (target_string.len() as i32) - 1 {
                    best_string_overlap = test_string.1.clone();
                    break;
                }
            } else {
                continue;
            }
        }
        if best_overlap == (target_string.len() as i32) - 1 {
            break;
        }
    }

    // for i in 0..test_strings.len() {
    //     let test_string = test_strings.get(i).expect("");
    //     let mut chopped_string = String::from(&test_string[0..test_string.len() - 1]).clone();
    //     let overlap = match target_string.find(&chopped_string) {
    //         None => -1,
    //         Some(index) => (target_string.len() as i32) - (index as i32)
    //     };
    //
    //     if overlap >= 1 {
    //         if overlap > best_overlap {
    //             best_overlap = overlap;
    //             best_string_overlap = test_strings
    //                 .get(i)
    //                 .expect("Failed to get string from test string vector")
    //                 .clone();
    //         }
    //
    //         if overlap == (target_string.len() as i32) - 1 {
    //             best_string_overlap = test_string.clone();
    //             break;
    //         }
    //     } else {
    //         continue;
    //     }
    // }
    return if best_overlap == 1 {
        None
    } else {
        Some((best_string_overlap.clone(), best_overlap as i32))
    }
}


fn usize_vec_to_string(numbers: &Vec<usize>) -> String {
    return numbers.clone()
        .iter()
        .map(|&x| (x + 1).to_string())
        .collect();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_super_perm_3() {
        let mut vec: Vec<usize> = vec![0; 3]
            .iter_mut()
            .enumerate()
            .map(|(i, _)| i)
            .collect();
        let mut permutations: Vec<String> = Vec::new();
        generate(3, &mut vec, &mut permutations);
        assert_eq!(find_super_perm(&mut permutations), String::from("123121321"));
    }

    #[test]
    fn test_super_perm_4() {
        let mut vec: Vec<usize> = vec![0; 4]
            .iter_mut()
            .enumerate()
            .map(|(i, _)| i)
            .collect();
        let mut permutations: Vec<String> = Vec::new();
        generate(4, &mut vec, &mut permutations);
        assert_eq!(find_super_perm(&mut permutations), String::from("123412314231243121342132413214321"));
    }
}
