use core::{
    error::Error,
    fmt::Display,
    ops::{BitAnd, BitOr, BitXor, Not},
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Trit {
    False = 0,
    True = 1,
    Unknown = 2,
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

impl From<bool> for Trit {
    fn from(value: bool) -> Self {
        if value { Trit::True } else { Trit::False }
    }
}

impl TryFrom<Trit> for bool {
    type Error = UnknownToBoolError;

    fn try_from(value: Trit) -> Result<Self, Self::Error> {
        match value {
            Trit::True => Ok(true),
            Trit::False => Ok(false),
            Trit::Unknown => Err(UnknownToBoolError),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnknownToBoolError;

impl Display for UnknownToBoolError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "Cannot convert Trit::Unknown to bool; handle Unknown explicitly!"
        )
    }
}

impl Error for UnknownToBoolError {}

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

impl BitAnd for Trit {
    type Output = Self;

    fn bitand(self, other: Trit) -> Self::Output {
        match (self, other) {
            (Trit::False, _) | (_, Trit::False) => Trit::False,
            (Trit::True, Trit::True) => Trit::True,
            _ => Trit::Unknown,
        }
    }
}

impl BitXor for Trit {
    type Output = Self;

    fn bitxor(self, other: Trit) -> Self::Output {
        match (self, other) {
            (Trit::Unknown, _) | (_, Trit::Unknown) => Trit::Unknown,
            (Trit::True, Trit::True) | (Trit::False, Trit::False) => Trit::False,
            _ => Trit::True,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(true, Trit::True)]
    #[case(false, Trit::False)]
    fn test_from_bool(#[case] input: bool, #[case] expected: Trit) {
        assert_eq!(Trit::from(input), expected);
    }

    #[rstest]
    #[case(Trit::True, Ok(true))]
    #[case(Trit::False, Ok(false))]
    #[case(Trit::Unknown, Err(UnknownToBoolError))]
    fn from_trit_to_bool(#[case] input: Trit, #[case] expected: Result<bool, UnknownToBoolError>) {
        assert_eq!(bool::try_from(input), expected);
    }

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

    #[rstest]
    #[case(Trit::True, Trit::True, Trit::True)]
    #[case(Trit::True, Trit::Unknown, Trit::Unknown)]
    #[case(Trit::True, Trit::False, Trit::False)]
    #[case(Trit::Unknown, Trit::True, Trit::Unknown)]
    #[case(Trit::Unknown, Trit::Unknown, Trit::Unknown)]
    #[case(Trit::Unknown, Trit::False, Trit::False)]
    #[case(Trit::False, Trit::True, Trit::False)]
    #[case(Trit::False, Trit::Unknown, Trit::False)]
    #[case(Trit::False, Trit::False, Trit::False)]
    fn and_truth_table(#[case] a: Trit, #[case] b: Trit, #[case] expected: Trit) {
        assert_eq!(a & b, expected);
    }

    #[rstest]
    #[case(Trit::True, Trit::True, Trit::False)]
    #[case(Trit::True, Trit::Unknown, Trit::Unknown)]
    #[case(Trit::True, Trit::False, Trit::True)]
    #[case(Trit::Unknown, Trit::True, Trit::Unknown)]
    #[case(Trit::Unknown, Trit::Unknown, Trit::Unknown)]
    #[case(Trit::Unknown, Trit::False, Trit::Unknown)]
    #[case(Trit::False, Trit::True, Trit::True)]
    #[case(Trit::False, Trit::Unknown, Trit::Unknown)]
    #[case(Trit::False, Trit::False, Trit::False)]
    fn xor_truth_table(#[case] a: Trit, #[case] b: Trit, #[case] expected: Trit) {
        assert_eq!(a ^ b, expected);
    }
}
