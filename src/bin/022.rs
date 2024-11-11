use std::{fs::File, io::Read};

/// Solution to [Names Scores](https://projecteuler.net/problem=22). (Problem 22)
fn main() {
    let mut input_names_file =
        File::open("data/022.txt").expect("should be able to open input file");
    let mut input_names = String::new();

    input_names_file
        .read_to_string(&mut input_names)
        .expect("should be able to read input file");

    let mut names = input_names
        .trim_matches('\"')
        .split("\",\"")
        .collect::<Vec<_>>();

    names.sort();

    let answer = names.iter().enumerate().fold(0, |answer, (index, &name)| {
        let name_total = name.chars().fold(0, |name_total, char| {
            name_total + char.to_ascii_lowercase() as u32 - 'a' as u32 + 1
        });

        answer + (name_total * (index as u32 + 1))
    });

    println!("Answer: {}", answer);
}
