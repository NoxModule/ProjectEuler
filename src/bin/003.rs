use project_euler::Math;

/// Solution to [Largest Prime Factor](https://projecteuler.net/problem=3). (Problem 3)
fn main() {
    let prime_factors = Math::prime_factors(600_851_475_143);

    println!("Prime Factors: {:?}", prime_factors);

    let answer = prime_factors
        .keys()
        .max()
        .expect("should have at least one prime factor");

    println!("Answer: {}", answer);
}
