pub struct Fibonacci {
    previous_term: u64,
    current_term: u64,
}

impl Fibonacci {
    pub fn new() -> Self {
        Self {
            previous_term: 0,
            current_term: 1,
        }
    }
}

impl Iterator for Fibonacci {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        self.current_term += self.previous_term;
        self.previous_term = self.current_term - self.previous_term;

        Some(self.current_term)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fibonacci_iterator_returns_expected_terms() {
        // arrange
        let expected_terms: Vec<u64> = vec![1, 2, 3, 5, 8, 13, 21, 34, 55, 89];

        // action
        let actual_terms = Fibonacci::new().take(10).collect::<Vec<_>>();

        // assert
        assert_eq!(expected_terms, actual_terms);
    }
}
