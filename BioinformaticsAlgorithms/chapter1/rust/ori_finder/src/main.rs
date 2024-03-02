pub mod utils;

fn main() {
    println!("Ori finder!");
    // utils::frequent_words(
    //     String::from("gatcagcataagggtccCTGCAATGCATGACAAGCCTGCAGTtgttttac"),
    //     4, 25, 3 
    // )
    //utils::find_clumps("atcaatgatcaacgtaagcttctaagcatg", "atc", 4, 3)
    //utils::minimum_skew(String::from("CATGGGCATCGGCCATACGCC"));
    utils::approx_pattern_match("AAAAA",String::from("AACAAGCATAAACATTAAAGAG"), 1);
}
