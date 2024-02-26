pub mod utils;

fn main() {
    println!("Ori finder!");
    utils::frequent_words(
        String::from("gatcagcataagggtccCTGCAATGCATGACAAGCCTGCAGTtgttttac"),
        4, 25, 3 
    )
    //utils::find_clumps("atcaatgatcaacgtaagcttctaagcatg", "atc", 4, 3)

}
