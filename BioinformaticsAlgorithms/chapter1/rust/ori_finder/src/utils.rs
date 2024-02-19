
pub fn pattern_count (text:&str, pattern:&str) -> i64 {
  // Counter for each time an instance of "pattern" is found
  let mut count:i64 = 0;
  
  // Pattern size (k-mer)
  let k = pattern.len();

  // Reverse complement
  let rev_compl_pattern = reverse_complement(pattern);

  // Iterate for each k-mer in text
  for i in 0..=(text.len()-k){
    let pattern_2 = &text[i..i+k];
    if (pattern_2 == pattern) || (pattern_2 == rev_compl_pattern) {
      count += 1;
      //println!("{pattern_2}, {i}");
    }
  }
  //println!("{count}");
  return count
}

pub fn reverse_complement(pattern:&str) -> String{
  let mut complement:String = String::new();
  //println!("{pattern}");
  for i in pattern.chars(){
    match i {
      'A' => {complement.push('T')},
      'T' => {complement.push('A')},
      'C' => {complement.push('G')},
      'G' => {complement.push('C')},
      _ => {panic!("There are non-canonical bases")}
    }
  }
  let rev_compl:String = complement.chars().rev().collect();
  return rev_compl
}

pub fn frequent_words (text: String, k:u8){
  let mut text = text.to_ascii_uppercase();
  text.retain(|x| !x.is_whitespace());
  
  // Store every k-mer found in text
  let mut frequent_pattern:Vec<&str> = Vec::new();

  // Vec to store the counts for each kmer
  let mut count :Vec<i64> = Vec::new();

  for i in 0..=(text.len()-k as usize){
    let pattern = &text[i..i + (k as usize)];
    //println!("{pattern}");
    count.push(pattern_count(&text, pattern));
  }
  //println!("{:?}",count);
  let max_num = count.iter().max().unwrap().to_owned();
  for i in 0..=(text.len()-k as usize){
    if count[i] == max_num{
      frequent_pattern.push(&text[i..i + (k as usize)]);
    }
  }
  frequent_pattern.sort();
  frequent_pattern.dedup();

  println!("{max_num}");
  println!("{:?}", frequent_pattern);

}

pub fn find_clumps (text:&str, pattern:&str, w:usize, t:u8){
  
  // take chuncks and look for k-mer clumps (number of k-mers in w equal or greater than t).
  for i in 0..=(text.len() - w){
    let chunk = &text[i..i + w];
    for j in 0..=(chunk.len() - pattern.len()){
      let pattern = &chunk[j..j+pattern.len()];
//      count.push(pattern_count(&text, pattern));
    }
  }
}