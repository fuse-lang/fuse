use std::ops::Add;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum Precedence {
    /// Default precedence of any expression
    /// used as 0 value of enum.
    Expression,
    Assignment,
    LogicalOr,
    LogicalAnd,
    BitwiseOr,
    BitwiseXor,
    BitwiseAnd,
    Equality,
    Relational,
    Shift,
    Add,
    Multiply,
    Exponential,
    Member,
}

impl Precedence {
    pub fn is_right_associative(&self) -> bool {
        matches!(self, Self::Assignment | Self::Member)
    }

    pub fn is_left_associative(&self) -> bool {
        matches! {
            self,
            | Self::LogicalOr
            | Self::LogicalAnd
            | Self::BitwiseOr
            | Self::BitwiseXor
            | Self::BitwiseAnd
            | Self::Equality
            | Self::Relational
            | Self::Shift
            | Self::Add
            | Self::Multiply
        }
    }
}

/// Last variant in `Precedence` type.
const PRECEDENCE_MAX: u8 = Precedence::Multiply as u8;

impl From<u8> for Precedence {
    fn from(value: u8) -> Self {
        let value = if value > PRECEDENCE_MAX {
            PRECEDENCE_MAX
        } else {
            value
        };
        // SAFETY: `Precedence` type have `repr(u8)` attribute,
        // And we clap the upper bound of value to the last enum variant.
        unsafe { std::mem::transmute(value) }
    }
}

impl Add<u8> for Precedence {
    type Output = Self;
    fn add(self, rhs: u8) -> Self::Output {
        let res = self as u8 + rhs;
        let res = if res > PRECEDENCE_MAX {
            PRECEDENCE_MAX
        } else {
            res
        };
        res.into()
    }
}
