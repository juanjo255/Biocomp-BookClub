use std::{cmp::min, iter::zip};

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

// Problem 1F
// Find a position in a genome where the skew diagram attains a minimum.
pub fn minimum_skew(dna: String) -> i8 {
    let mut skew_values: Vec<i8> = Vec::new();
    for nucl in dna.chars() {
        match nucl {
            'G' => skew_values.push(match skew_values.last() {
                Some(i) => i + 1,
                None => 1,
            }),
            'C' => skew_values.push(match skew_values.last() {
                Some(i) => i - 1,
                None => -1,
            }),
            _ => skew_values.push((skew_values.last().unwrap_or(&0)).to_owned()),
        }
    }
    return skew_values.iter().min().unwrap().to_owned();
}

// Problem 1G
// Compute the Hamming distance between two strings.
pub fn hamming_distance(pattern: &str, pattern2: &str) -> i32 {
    let mut score = 0;
    let pattern_iter = pattern.chars();
    let pattern2_iter = pattern2.chars();
    for (char1, char2) in zip(pattern_iter, pattern2_iter) {
        if char1 != char2 {
            score += 1;
        }
    }
    //println!("Hamming distance: {score}");
    return score;
}

// Problem 1H
// Find all approximate occurrences of a pattern in a string.
pub fn approx_pattern_match(text: &str, pattern: &str, d: i32) -> (u64, Vec<u64>) {
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
        if hamming_distance(pattern_2, pattern) <= d
            || hamming_distance(pattern_2, &rev_compl_pattern) <= d
        {
            count += 1;
            pattern_pos.push(i as u64);
        }
    }
    //println!("Instances found: {count}");
    return (count, pattern_pos)
}

// Problem 1I and 1J
// Find the most frequent k-mers (with mismatches and reverse complements) in a string.

// Compute an Index for a pattern (hash strategy)
pub fn nucl_to_number(nucl:&str) -> u64{
    let mut score: u64 = 0;
    for base in nucl.chars(){
        match base{
            'A' => score += 0,
            'C' => score += 1,
            'G' => score += 2,
            'T'=> score += 3,
            _ => panic!("Non cannonical base found! {base}")
        }
    }
    return score
}

pub fn pattern_to_number(pattern:&str) -> u64{
    if pattern.is_empty(){
        return 0
    }
    let suffix = &pattern[pattern.len()-1..pattern.len()];
    let prefix = &pattern[..pattern.len()-1];
    //println!("prefix: {prefix}, suffix: {suffix}");
    return 4 * pattern_to_number(prefix) + nucl_to_number(&suffix);
}

// Inverse from above: Compute a pattern from an Index
pub fn number_to_nucl(number:u8) -> String{
    let nucl:String;
    match number{
            0 => nucl= String::from("A"),
            1 => nucl= String::from("C"),
            2 => nucl= String::from("G"),
            3 => nucl= String::from("T"),
            _ => panic!("Non cannonical number found! {number}")
        }
    return nucl
}

pub fn number_to_pattern(index:u64, k:u32){
    let mut counter = k;
    let mut index_copy = index.clone();
    let mut remainder:u8;
    let mut quotient:u64;
    let mut pattern:String= String::from("");
    while counter > 0 {
        remainder = (index_copy % 4) as u8;
        pattern = number_to_nucl(remainder) + &pattern;
        quotient = index_copy / 4;
        index_copy = quotient;
        counter-= 1;
    }
    println!("{pattern}");

    todo!()
}

pub fn frequent_pattern_mismatches (text: String, k: u8, d:i32){

    // Remove whitespaces
    let mut text = text.to_ascii_uppercase();
    text.retain(|x| !x.is_whitespace());

    // Store every k-mer found in text
    let mut frequent_pattern: Vec<(&str, Vec<u64>)> = Vec::new();

    // Store the counts for each kmer
    let mut count: Vec<(u64, Vec<u64>)> = Vec::new();

    // Traverse the genome in windows of l length
    for i in 0..=(text.len() - k as usize) {
            
        // Take the minimun between forward and reverse to avoid redundancy
        let pattern = (&text[i..i + (k as usize)]).to_string();

        // Count instances of pattern
        let count_pos = approx_pattern_match(&text, &pattern, d);
        count.push(count_pos.clone());

    }

    let max_num = count.iter().map(|x| x.0).max().unwrap().to_owned();
    println!("max_num: {max_num}");
    for i in 0..=(text.len() - k as usize) {
        if count[i].0 == max_num
            && !(frequent_pattern
                .iter()
                .map(|x| x.0)
                .collect::<Vec<&str>>()
                .contains(&min(&&text[i..i + (k as usize)], &reverse_complement(&text[i..i + (k as usize)]) )))
        {
            frequent_pattern.push((&text[i..i + (k as usize)], count[i].1.clone()));
        }
    }

    println!("Frequent patterns {:?}", frequent_pattern);
}