use std::collections::HashMap;

pub struct Math;

impl Math {
    pub fn is_prime(number: u64) -> bool {
        number >= 2 && !(2..=((number as f64).sqrt() as u64)).any(|i| number % i == 0)
    }

    pub fn prime_factors(number: u64) -> HashMap<u64, u64> {
        let mut prime_factors = HashMap::new();
        let mut number = number;

        fn add_prime_factor(prime_factors: &mut HashMap<u64, u64>, prime_factor: u64) {
            let factor_count = prime_factors.entry(prime_factor).or_insert(0);
            *factor_count += 1;
        }

        while number % 2 == 0 {
            add_prime_factor(&mut prime_factors, 2);

            number /= 2;
        }

        let mut index = 3_u64;
        while index <= (number as f64).sqrt() as u64 {
            while number % index == 0 {
                add_prime_factor(&mut prime_factors, index);

                number /= index;
            }

            index += 2;
        }

        if number > 2 {
            add_prime_factor(&mut prime_factors, number);
        }

        prime_factors
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_prime_returns_false_for_composite_numbers() {
        // assert
        assert!(vec![4, 6, 8, 9, 10, 12]
            .iter()
            .all(|number| !Math::is_prime(*number)));
    }

    #[test]
    fn is_prime_returns_true_for_prime_numbers() {
        // assert
        assert!(vec![2, 3, 5, 7, 11, 13]
            .iter()
            .all(|number| Math::is_prime(*number)));
    }

    #[test]
    fn prime_factors_returns_expected_factors() {
        // arrange
        let mut expected_factors = HashMap::<u64, u64>::new();
        expected_factors.insert(2, 1);
        expected_factors.insert(5, 1);
        expected_factors.insert(7, 1);
        expected_factors.insert(13, 1);
        expected_factors.insert(29, 1);

        // action
        let actual_factors = Math::prime_factors(26_390);

        // assert
        assert_eq!(expected_factors, actual_factors);
    }
}
