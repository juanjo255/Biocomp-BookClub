pub mod utils;

fn main() {
    println!("Ori finder!");
    // utils::frequent_words(
    //     String::from("atcaatgatcaacgtaagcttctaagcatgatcaaggtgctcacacagtttatccacaac
    //     ctgagtggatgacatcaagataggtcgttgtatctccttcctctcgtactctcatgacca
    //     cggaaagatgatcaagagaggatgatttcttggccatatcgcaatgaatacttgtgactt
    //     gtgcttccaattgacatcttcagcgccatattgcgctggccaaggtgacggagcgggatt
    //     acgaaagcatgatcatggctgttgttctgtttatcttgttttgactgagacttgttagga
    //     tagacggtttttcatcactgactagccaaagccttactctgcctgacatcgaccgtaaat
    //     tgataatgaatttacatgcttccgcgacgatttacctcttgatcatcgatccgattgaag
    //     atcttcaattgttaattctcttgcctcgactcatagccatgatgagctcttgatcatgtt
    //     tccttaaccctctattttttacggaagaatgatcaagctgctgctcttgatcatcgtttc"),
    //     9,
    // )
    utils::find_clumps("atcaatgatcaacgtaagcttctaagcatg", "atc", 4, 3)

}
