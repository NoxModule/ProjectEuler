use crate::Math;

pub struct Primes {
    current: u64,
}

impl Primes {
    pub fn new() -> Self {
        Self { current: 2 }
    }
}

impl Iterator for Primes {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current == 2 {
            self.current = 3;

            return Some(2);
        }

        while !Math::is_prime(self.current) {
            self.current += 2;
        }

        let next_prime = self.current;
        self.current += 2;
        Some(next_prime)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn primes_iterator_returns_expected_prime_numbers() {
        // arrange
        let expected_terms: Vec<u64> = vec![2, 3, 5, 7, 11, 13];

        // action
        let actual_terms = Primes::new().take(6).collect::<Vec<_>>();

        // assert
        assert_eq!(expected_terms, actual_terms);
    }
}
