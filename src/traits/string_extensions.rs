pub trait StringExtensions {
    fn contains_same<O>(&self, other: O) -> bool
    where
        O: AsRef<str>;

    fn is_palindrome(&self) -> bool;
}

impl<S> StringExtensions for S
where
    S: AsRef<str>,
{
    fn contains_same<O>(&self, other: O) -> bool
    where
        O: AsRef<str>,
    {
        let mut str_1 = self.as_ref().chars().collect::<Vec<_>>();
        let mut str_2 = other.as_ref().chars().collect::<Vec<_>>();

        str_1.sort();
        str_2.sort();

        str_1 == str_2
    }

    fn is_palindrome(&self) -> bool {
        let chars = self.as_ref().chars();

        chars.clone().zip(chars.rev()).all(|(a, b)| a == b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn contains_same_returns_true_when_string_contains_same_sorted_characters() {
        // assert
        assert_eq!(true, "!dorlW ,oellH".contains_same("Hello, World!"));
    }

    #[test]
    fn contains_same_returns_false_when_string_does_not_contain_same_sorted_characters() {
        // assert
        assert_eq!(false, "dorlW oellH".contains_same("Hello, World!"));
    }

    #[test]
    fn is_palindrome_returns_true_when_string_is_a_palindrome() {
        // assert
        assert_eq!(true, "101000101".is_palindrome())
    }

    #[test]
    fn is_palindrome_returns_false_when_string_is_not_a_palindrome() {
        // assert
        assert_eq!(false, "101011101".is_palindrome())
    }
}
