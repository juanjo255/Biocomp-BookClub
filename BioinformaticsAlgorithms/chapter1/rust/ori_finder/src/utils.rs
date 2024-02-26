use std::{cmp::min, num::ParseFloatError};

pub fn pattern_count(text: &str, pattern: &str) -> (u64, Vec<u64>) {
    // Count how many time a given patter appears in text and positions in text

    // Counter for each time an instance of "pattern" is found
    let mut count: u64 = 0;

    // postions in text where pattern is found
    let mut pattern_pos: Vec<u64> = Vec::new();

    // Pattern size (k-mer)
    let k = pattern.len();

    // Reverse complement
    let rev_compl_pattern = reverse_complement(pattern);

    // Iterate for each k-mer in text
    for i in 0..=(text.len() - k) {
        let pattern_2 = &text[i..i + k];
        if (pattern_2 == pattern) || (pattern_2 == rev_compl_pattern) {
            count += 1;
            pattern_pos.push(i as u64);
        }
    }
    //println!("{count}");
    return (count, pattern_pos);
}

pub fn reverse_complement(pattern: &str) -> String {
    let mut complement: String = String::new();
    //println!("{pattern}");
    for i in pattern.chars() {
        match i {
            'A' => complement.push('T'),
            'T' => complement.push('A'),
            'C' => complement.push('G'),
            'G' => complement.push('C'),
            _ => {
                panic!("There are non-canonical bases")
            }
        }
    }
    let rev_compl: String = complement.chars().rev().collect();
    return rev_compl;
}

pub fn frequent_words(text: String, k: u8, l: u8, t: u64) {
    // Store and count each k-mer that appears in text

    // Remove whitespaces
    let mut text = text.to_ascii_uppercase();
    text.retain(|x| !x.is_whitespace());

    // Store every k-mer found in text
    let mut frequent_pattern: Vec<(&str, Vec<u64>)> = Vec::new();

    // Store the counts for each kmer
    let mut count: Vec<(u64, Vec<u64>)> = Vec::new();

    // Store k-mer that form clumps > t
    let mut clumps: Vec<(String, Vec<u64>)> = Vec::new();

    // Traverse the genome in windows of l length
    for i in 0..=(text.len() - l as usize) {
        let window_in_genome = &text[i..i + (l as usize)];

        // Traverse each window in genome
        for j in 0..=(window_in_genome.len() - k as usize) {
          
            let forward = (&text[j..j + (k as usize)]).to_string();
            let reverse = reverse_complement(&text[j..j + (k as usize)]);

            // Take the minimun between forward and reverse to avoid redundancy
            let pattern = min(forward, reverse);

            // Count instances of pattern
            let count_pos = pattern_count(&text, &pattern);

            count.push(count_pos.clone());

            // Check if pattern forms a clump and if it's not already in the group
            if (count_pos.0 >= t)
                && !(clumps
                    .iter()
                    .map(|x| &x.0)
                    .collect::<Vec<&String>>()
                    .contains(&&pattern))
            {
                clumps.push((pattern.to_owned(), count_pos.1.clone()));
            }
        }
    }

    let max_num = count.iter().map(|x| x.0).max().unwrap().to_owned();
    println!("max_num: {max_num}");
    for i in 0..=(text.len() - k as usize) {
        if count[i].0 == max_num
            && !(frequent_pattern
                .iter()
                .map(|x| x.0)
                .collect::<Vec<&str>>()
                .contains(&&text[i..i + (k as usize)]))
        {
            frequent_pattern.push((&text[i..i + (k as usize)], count[i].1.clone()));
        }
    }

    println!("Frequent patterns {:?}", frequent_pattern);
    println!("Clumps: {:?}", clumps);
}

// pub fn find_clumps (text:&str, pattern:&str, w:usize, t:u8){
//   clumps
//   // take chuncks and look for k-mer clumps (number of k-mers in w equal or greater than t).
//   for i in 0..=(text.len() - w){
//     let chunk = &text[i..i + w];
//     for j in 0..=(chunk.len() - pattern.len()){
//       let pattern = &chunk[j..j+pattern.len()];
//       if pattern_count(&text, pattern) > t{

//       }
//     }
//   }
// }
