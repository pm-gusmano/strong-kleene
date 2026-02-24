use std::{
    fmt::Display,
    ops::{BitOr, Not},
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Trit {
    True,
    Unknown,
    False,
}

impl Display for Trit {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Trit::True => write!(f, "true"),
            Trit::Unknown => write!(f, "unknown"),
            Trit::False => write!(f, "false"),
        }
    }
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

impl BitOr for Trit {
    type Output = Self;

    fn bitor(self, other: Trit) -> Self::Output {
        match (self, other) {
            (Trit::True, _) | (_, Trit::True) => Trit::True,
            (Trit::False, Trit::False) => Trit::False,
            _ => Trit::Unknown,
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

    #[rstest]
    #[case(Trit::True, Trit::True, Trit::True)]
    #[case(Trit::True, Trit::Unknown, Trit::True)]
    #[case(Trit::True, Trit::False, Trit::True)]
    #[case(Trit::Unknown, Trit::True, Trit::True)]
    #[case(Trit::Unknown, Trit::Unknown, Trit::Unknown)]
    #[case(Trit::Unknown, Trit::False, Trit::Unknown)]
    #[case(Trit::False, Trit::True, Trit::True)]
    #[case(Trit::False, Trit::Unknown, Trit::Unknown)]
    #[case(Trit::False, Trit::False, Trit::False)]
    fn test_or_truth_table(#[case] a: Trit, #[case] b: Trit, #[case] expected: Trit) {
        assert_eq!(a | b, expected);
    }

    #[rstest]
    fn test_or_commutative(
        #[values(Trit::True, Trit::Unknown, Trit::False)] a: Trit,
        #[values(Trit::True, Trit::Unknown, Trit::False)] b: Trit,
    ) {
        assert_eq!(a | b, b | a);
    }

    #[rstest]
    fn test_or_identity_false(#[values(Trit::True, Trit::Unknown, Trit::False)] a: Trit) {
        assert_eq!(a | Trit::False, a);
    }

    #[rstest]
    fn test_or_domination_true(#[values(Trit::True, Trit::Unknown, Trit::False)] a: Trit) {
        assert_eq!(a | Trit::True, Trit::True);
    }

    #[rstest]
    fn test_or_idempotent(#[values(Trit::True, Trit::Unknown, Trit::False)] a: Trit) {
        assert_eq!(a | a, a);
    }
}
