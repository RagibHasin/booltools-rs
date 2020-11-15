#![forbid(unsafe_code, future_incompatible)]
#![deny(nonstandard_style, unused, missing_docs)]
#![feature(bool_to_option, total_cmp, iter_map_while, array_value_iter)]
#![allow(clippy::float_cmp)]

//! Secondary boolean operations for [`bool`] primitive type.

mod private {
    pub trait Sealed {}

    impl Sealed for bool {}
}

/// Secondary boolean operations for [`bool`] primitive type.
pub trait BoolTools: private::Sealed {
    /// The logical [`implication`] operation.
    ///
    /// Truth table:
    ///
    /// | self | rhs | output |
    /// |------|-----|--------|
    /// | [`true`]  | [`true`]  | [`true`]  |
    /// | [`true`]  | [`false`] | [`false`] |
    /// | [`false`] | [`true`]  | [`true`] |
    /// | [`false`] | [`false`] | [`true`] |
    ///
    ///
    /// [`implication`]: https://en.wikipedia.org/wiki/Boolean_algebra#Operations
    fn implication(self, rhs: Self) -> Self;

    /// The logical [`xor`] operation.
    ///
    /// Truth table:
    ///
    /// | self | rhs | output |
    /// |------|-----|--------|
    /// | [`true`]  | [`true`]  | [`false`]  |
    /// | [`true`]  | [`false`] | [`true`] |
    /// | [`false`] | [`true`]  | [`true`] |
    /// | [`false`] | [`false`] | [`false`] |
    ///
    ///
    /// [`xor`]: https://en.wikipedia.org/wiki/Boolean_algebra#Operations
    fn xor(self, rhs: Self) -> Self;

    /// The logical [`equivalence`] operation.
    ///
    /// Truth table:
    ///
    /// | self | rhs | output |
    /// |------|-----|--------|
    /// | [`true`]  | [`true`]  | [`true`]  |
    /// | [`true`]  | [`false`] | [`false`] |
    /// | [`false`] | [`true`]  | [`false`] |
    /// | [`false`] | [`false`] | [`true`] |
    ///
    ///
    /// [`equivalence`]: https://en.wikipedia.org/wiki/Boolean_algebra#Operations
    fn equivalence(self, rhs: Self) -> Self;
}

impl BoolTools for bool {
    #[inline]
    fn implication(self, rhs: Self) -> Self {
        !self || rhs
    }

    #[inline]
    fn xor(self, rhs: Self) -> Self {
        (self || rhs) && (!self || !rhs)
    }

    #[inline]
    fn equivalence(self, rhs: Self) -> Self {
        !self.xor(rhs)
    }
}
