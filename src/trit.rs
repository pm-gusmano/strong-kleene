use std::ops::Not;

#[derive(Debug, PartialEq, Eq)]
pub enum Trit {
    True,
    Unknown,
    False,
}

impl Not for Trit {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Trit::True => Trit::False,
            Trit::Unknown => Trit::Unknown,
            Trit::False => Trit::True,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(Trit::True, Trit::False)]
    #[case(Trit::Unknown, Trit::Unknown)]
    #[case(Trit::False, Trit::True)]
    fn test_not(#[case] input: Trit, #[case] expected: Trit) {
        assert_eq!(!input, expected);
    }
}
