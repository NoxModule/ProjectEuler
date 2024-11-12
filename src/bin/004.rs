/// Solution to [Largest Palindrome Product](https://projecteuler.net/problem=4). (Problem 4)
fn main() {
    let mut answer = 0;

    for i in 100..999 {
        for j in 100..999 {
            let product = i * j;

            if product > answer {
                let product_str = product.to_string();
                let product_str = product_str.chars();

                if product_str
                    .clone()
                    .zip(product_str.rev())
                    .all(|(i, j)| i == j)
                {
                    answer = product;
                }
            }
        }
    }

    println!("Answer: {}", answer);
}
