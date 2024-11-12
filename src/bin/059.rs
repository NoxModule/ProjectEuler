use std::fs::{self};

/// Solution to [XOR Decryption](https://projecteuler.net/problem=59). (Problem 59)
fn main() {
    let mut answer = 0;

    let message_bytes = fs::read_to_string("data/059.txt")
        .expect("should be able to read input file")
        .split(",")
        .map(|number| {
            number
                .parse::<u8>()
                .expect("should be able to parse number")
        })
        .collect::<Vec<_>>();

    let bytes = (('a' as u8)..=('z' as u8)).collect::<Vec<_>>();
    let key_permutations = Permutations::new(bytes, 3);

    for key in key_permutations {
        let encryption_key = build_encryption_key(&key, message_bytes.len());

        let decrypted_bytes = message_bytes
            .iter()
            .zip(encryption_key.iter())
            .map(|(message_byte, key_byte)| message_byte ^ key_byte)
            .collect::<Vec<_>>();

        let percentage = decrypted_bytes.iter().fold(0, |valid_count: i32, &byte| {
            let char = byte as char;

            if char.is_alphabetic() || char.is_whitespace() {
                valid_count + 1
            } else {
                valid_count
            }
        }) as f32
            / decrypted_bytes.len() as f32;

        if percentage > 0.9 {
            answer = decrypted_bytes
                .iter()
                .fold(0, |answer, &byte| answer as u32 + byte as u32);

            break;
        }
    }

    println!("Answer: {}", answer);
}

fn build_encryption_key(key: &Vec<u8>, length: usize) -> Vec<u8> {
    let capacity = length / key.len();
    let capacity = if capacity % key.len() == 0 {
        capacity
    } else {
        capacity + key.len()
    };

    let mut encryption_key = Vec::with_capacity(capacity);
    while encryption_key.len() < length {
        encryption_key.extend_from_slice(key);
    }

    encryption_key.truncate(length);

    encryption_key
}

struct Permutations {
    bytes: Vec<u8>,
    current: Vec<usize>,
    has_exhausted: bool,
    length: usize,
}

impl Permutations {
    fn new(bytes: Vec<u8>, length: usize) -> Self {
        Self {
            bytes,
            current: vec![0; length],
            has_exhausted: false,
            length,
        }
    }
}

impl Iterator for Permutations {
    type Item = Vec<u8>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.has_exhausted {
            return None;
        }

        let result = self
            .current
            .iter()
            .map(|&i| self.bytes[i])
            .collect::<Vec<_>>();

        for i in (0..self.length).rev() {
            if self.current[i] < self.bytes.len() - 1 {
                self.current[i] += 1;

                break;
            } else {
                self.current[i] = 0;

                if i == 0 {
                    self.has_exhausted = true;
                }
            }
        }

        Some(result)
    }
}
