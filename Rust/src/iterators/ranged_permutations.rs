pub struct RangedPermutations<I>
where
    I: Iterator,
{
    current: Vec<usize>,
    has_exhausted: bool,
    length: usize,
    values: Vec<I::Item>,
}

impl<I> RangedPermutations<I>
where
    I: Iterator,
{
    pub fn new(iter: I, length: usize) -> Self {
        Self {
            current: vec![0; length],
            has_exhausted: false,
            length,
            values: iter.collect(),
        }
    }
}

impl<I> Iterator for RangedPermutations<I>
where
    I: Iterator,
    <I as Iterator>::Item: Copy,
{
    type Item = Vec<I::Item>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.has_exhausted {
            return None;
        }

        let permutation = self
            .current
            .iter()
            .map(|&i| self.values[i])
            .collect::<Vec<_>>();

        for i in (0..self.length).rev() {
            if self.current[i] < self.values.len() - 1 {
                self.current[i] += 1;

                break;
            } else {
                self.current[i] = 0;

                if i == 0 {
                    self.has_exhausted = true;
                }
            }
        }

        Some(permutation)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ranged_permutations_iterator_returns_expected_values() {
        // arrange
        let expected_values = vec![
            vec!['a', 'a', 'a'],
            vec!['a', 'a', 'b'],
            vec!['a', 'a', 'c'],
            vec!['a', 'b', 'a'],
            vec!['a', 'b', 'b'],
            vec!['a', 'b', 'c'],
            vec!['a', 'c', 'a'],
            vec!['a', 'c', 'b'],
            vec!['a', 'c', 'c'],
            vec!['b', 'a', 'a'],
            vec!['b', 'a', 'b'],
            vec!['b', 'a', 'c'],
            vec!['b', 'b', 'a'],
            vec!['b', 'b', 'b'],
            vec!['b', 'b', 'c'],
            vec!['b', 'c', 'a'],
            vec!['b', 'c', 'b'],
            vec!['b', 'c', 'c'],
            vec!['c', 'a', 'a'],
            vec!['c', 'a', 'b'],
            vec!['c', 'a', 'c'],
            vec!['c', 'b', 'a'],
            vec!['c', 'b', 'b'],
            vec!['c', 'b', 'c'],
            vec!['c', 'c', 'a'],
            vec!['c', 'c', 'b'],
            vec!['c', 'c', 'c'],
        ];

        // action
        let actual_values = RangedPermutations::new('a'..='c', 3).collect::<Vec<_>>();

        // assert
        assert_eq!(expected_values, actual_values);
    }

    #[test]
    fn ranged_permutations_iterator_returns_expected_values2() {
        // arrange
        let expected_values = vec![
            vec!['a', 'a'],
            vec!['a', 'b'],
            vec!['a', 'c'],
            vec!['b', 'a'],
            vec!['b', 'b'],
            vec!['b', 'c'],
            vec!['c', 'a'],
            vec!['c', 'b'],
            vec!['c', 'c'],
        ];

        // action
        let actual_values = RangedPermutations::new('a'..='c', 2).collect::<Vec<_>>();

        // assert
        assert_eq!(expected_values, actual_values);
    }
}
