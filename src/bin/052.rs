/// Solution to [Permuted Multiples](https://projecteuler.net/problem=52). (Problem 52)
fn main() {
    let mut answer = 0;

    let mut i = 1;
    while answer == 0 {
        let all_multipliers_have_same_digits = (2..=6).all(|multiplier| {
            let mut result = (i * multiplier).to_string().chars().collect::<Vec<_>>();
            let mut i = i.to_string().chars().collect::<Vec<_>>();

            result.sort();
            i.sort();

            result == i
        });

        if all_multipliers_have_same_digits {
            answer = i;
        }

        i += 1;
    }

    println!("Answer: {}", answer);
}
