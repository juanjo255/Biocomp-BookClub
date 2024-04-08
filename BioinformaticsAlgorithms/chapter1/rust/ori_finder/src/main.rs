pub mod utils;

fn main() {
    println!("Ori finder!");
    // utils::frequent_words(
    //     String::from("gatcagcataagggtccCTGCAATGCATGACAAGCCTGCAGTtgttttac"),
    //     4, 25, 3 
    // )
    //utils::find_clumps("atcaatgatcaacgtaagcttctaagcatg", "atc", 4, 3)
    //utils::minimum_skew(String::from("CATGGGCATCGGCCATACGCC"));
    //utils::frequent_pattern_mismatches(String::from("AACAAGCATAAACATTAAAGAG"), 4, 0);
    utils::number_to_pattern(utils::pattern_to_number("ATA"), 3)
}
