/// Solution to [Even Fibonacci Numbers](https://projecteuler.net/problem=2). (Problem 2)
fn main() {
    let mut answer = 0;

    let mut prev = 1;
    let mut next = 1;

    while next < 4_000_000 {
        if next % 2 == 0 {
            answer += next
        }

        next += prev;
        prev = next - prev;
    }

    println!("Answer: {}", answer);
}
