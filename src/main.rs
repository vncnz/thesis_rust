mod main_plain;
mod main_de;

#[path = "common.rs"] mod common;

use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_fasta_sequence_solid(path: &str) -> Option<String> {
    // let file = File::open(path).expect(&format!("Error opening {}", path));
    let file = File::open(path).unwrap_or_else(|err| panic!("Error opening {}: {}", path, err));
    let reader = BufReader::new(file);

    let sequence: String = reader
        .lines()
        .filter_map(Result::ok)
        .filter(|line| !line.starts_with('>')) // ignora intestazioni
        .collect::<Vec<_>>()
        .join("");

    Some(sequence)
}

fn main() {

    let x_fasta_path = "let-60_N2.txt";
    let x_sequence = read_fasta_sequence_solid(x_fasta_path).unwrap();
    let x_sequence_str: &str = &x_sequence;

    let y_fasta_path = "let-60_CB4856.txt";
    let y_sequence = read_fasta_sequence_solid(y_fasta_path).unwrap();
    let y_sequence_str: &str = &y_sequence;

    // thesis example fig 3.2
    // let x_sequence_str: &str = "CCTACA";
    // let y_sequence_str: &str = "ACCTTCCATACCAGTCA";

    // thesis example fig 3.5
    let y_sequence_str: &str = "CCTACA";
    let x_sequence_str: &str = "ACCTTCCTAAACAGTCA";

    // let x_sequence_str = "AAATTAGA";
    // let y_sequence_str: &str = "AAA[TTT|CC]AAATGGAAA";

    // For de-string creator (test)
    // let x_sequence_str = "AAAATTAGACCAATCG";
    // let y_sequence_str: &str = "AAAATTAAAACAATCG";

    let (score, max_pos);

    let degenerate: bool = y_sequence_str.contains('[');

    if degenerate {
        (score, max_pos) = main_de::build_tree(&x_sequence_str[0..], &y_sequence_str[0..], 1, -1, -1);
    } else {
        (score, max_pos) = main_plain::build_tree(&x_sequence_str[0..], &y_sequence_str[0..], 1, -1, -1);
        // (score, max_pos) = main_plain::build_tree(&x_sequence_str[0..], &y_sequence_str[0..], 1, i32::MIN, 0);
    }
    // let (score, max_pos) = main_plain::build_tree(&X[0..], &Y[0..], 1, -1, -1);
    // let (score, max_pos) = main_de::build_tree(&X[0..], &Y[0..], 1, -1, -1);

    println!("Alignment Score: {}   Position: {}", score, max_pos);
    println!("Is degenerate: {}", degenerate);
    println!("Is only-branching-mode: {}", common::TREE_MODE);
}

// example 0
// let x = String::from("AGT");
// let y = String::from("ATCGT");

// example 1
// let x = String::from("CCTA");
// let y = String::from("ACCTTCCATACCAGTCA");

// example 2
// let x = String::from("GAAAAAAATAACCAGCATTTA");
// let y = String::from("ACCTTCCATACCAGTCAAGGGGGGAAAAAAACCCACAACAAACCAGCATTTAAACAAAAAATGGAGAAGTGATAGATATTTTTGCTGTGTGTGTTTGTAGCATAGAAACTGCCGCGCAGGTGAAGAAAATGAAGAACTCGAAAAGAAAAGTGTGGGGTTATACTACACTACGGGATGAGAGAGTACA");

// example 5
// static X: &str = "CCGGGTTTA";
// static Y: &str = "ACCTTCGGGCCAGTCATATTTCA";

// thesis example fig 4.3 - de-strings
// static X: &str = "AAATTAGA";
// static Y: &str = "AAA[TTT|CC]AAATGGAAA";

// Caso pessimo
// static X: &str = "AAAAAAAA";
// static Y: &str = "ATATATATATATATA";

// thesis example fig 3.2
// static X: &str = "CCTACA";
// static Y: &str = "ACCTTCCATACCAGTCA";

/*

fn dna_to_bytes(seq: &str) -> Vec<u8> {
    seq.chars().map(|c| match c {
        'A' => 0,
        'C' => 1,
        'G' => 2,
        'T' => 3,
        _ => panic!("Invalid character in DNA sequence"),
    }).collect()
}

*/
