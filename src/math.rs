use std::collections::HashMap;

pub struct Math;

impl Math {
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
