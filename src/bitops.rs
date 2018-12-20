//! Miscellaneous bit operations for any `Integer`.
//!
//! # Examples
//!
//! ```
//! extern crate xor_distance_exercise;
//!
//! use xor_distance_exercise::bitops::BitOps;
//!
//! let x = 0b1000_0000_1001_1010;
//! let flag = 0b1000_0000;
//!
//! assert!(flag.is_flag());
//! assert!(flag.is_bit_set(7));
//! assert!(x.is_flag_set(flag));
//!
//! let mut y = 0b1000_0000_0001_1010;
//!
//! y.set_flag(flag);
//! assert_eq!(0b1000_0000_1001_1010, y);
//!
//! y.set_bit(0);
//! assert_eq!(0b1000_0000_1001_1011, y);
//! ```

use num_traits::PrimInt;

/// Bit operations trait for any `Integer` type.
pub trait BitOps: PrimInt {
    /// Returns whether this number only has one bit set.
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate xor_distance_exercise;
    ///
    /// use xor_distance_exercise::bitops::BitOps;
    ///
    /// // Flag must have exactly one bit set to "1".
    /// assert!(0b0010.is_flag());
    /// assert!(!0b0101.is_flag());
    /// ```
    #[inline]
    fn is_flag(&self) -> bool {
        // Flag must satisfy following criteria:
        // - at lest one bit must be set "1"
        // - exactly one bit must be set to "1", subtracting one will move this bit right and thus
        //   bitwise "&" with the original value must return zero
        *self > Self::zero() && (*self & (*self - Self::one())) == Self::zero()
    }

    /// Returns whether the given flag is set.
    ///
    /// It doesn't check if the provided flag is really flag.
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate xor_distance_exercise;
    ///
    /// use xor_distance_exercise::bitops::BitOps;
    ///
    /// let x = 0b1101;
    /// let flag = 0b0001;
    ///
    /// assert!(x.is_flag_set(flag));
    /// ```
    #[inline]
    fn is_flag_set(&self, flag: Self) -> bool {
        // The self has the "1" bit set on the same position as the flag.
        *self & flag > Self::zero()
    }

    /// Set flag.
    ///
    /// It doesn't check if the provided flag is really flag.
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate xor_distance_exercise;
    ///
    /// use xor_distance_exercise::bitops::BitOps;
    ///
    /// let mut x = 0b1101;
    /// let flag = 0b0010;
    ///
    /// x.set_flag(flag);
    ///
    /// assert_eq!(0b1111, x);
    /// ```
    #[inline]
    fn set_flag(&mut self, flag: Self) {
        *self = *self | flag;
    }

    /// Returns whether the bit on specified bit index is set to "1".
    ///
    /// # Panics
    ///
    /// Panics if `bit` is greater than the number of bits in this Integer.
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate xor_distance_exercise;
    ///
    /// use xor_distance_exercise::bitops::BitOps;
    ///
    /// assert!(0b1000.is_bit_set(3));
    /// ```
    #[inline]
    fn is_bit_set(&self, bit_index: usize) -> bool {
        // Create flag one and move its "1" bit from most left hand side to left by the requested
        // bit index number.
        let flag = Self::one() << bit_index;
        // Check out if the prepared flag is set.
        self.is_flag_set(flag)
    }

    /// Set bit to "1" for specified bit index. Indexed from zero.
    ///
    /// # Panics
    ///
    /// Panics if `bit` is greater than the number of bits in this Integer.
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate xor_distance_exercise;
    ///
    /// use xor_distance_exercise::bitops::BitOps;
    ///
    /// let mut x = 0b1000;
    /// x.set_bit(1);
    ///
    /// // The second bit should be added/set now.
    /// assert_eq!(0b1010, x);
    /// ```
    #[inline]
    fn set_bit(&mut self, bit_index: usize) {
        // Create flag one and move its "1" bit from most left hand side to left by the requested
        // bit index number.
        let flag = Self::one() << bit_index;
        // Check out if the prepared flag is set.
        self.set_flag(flag);
    }
}

/// Implements the `BitOps` trait for all 'Integer' types.
impl<T> BitOps for T where T: PrimInt {}

#[cfg(test)]
mod tests {
    use crate::bitops::BitOps;

    #[test]
    fn check_is_flag() {
        let zero = 0;

        // Zero is not a flag.
        assert!(!zero.is_flag());

        // Number having more than one "1" bit is not a flag.
        let x = 0b0111;
        assert!(!x.is_flag());

        let flag = 0b0100;

        // Any number that has exactly one "1" bit set is a flag.
        assert!(flag.is_flag());
    }

    #[test]
    fn check_is_flag_set() {
        let zero = 0b0000;

        // Zero is not a flag.
        assert!(!zero.is_flag());
        // Zero can not be set as a flag to anything.
        assert!(!zero.is_flag_set(0));

        let flag = 0b0010;
        let x = 0b1110;
        // Valid flag.
        assert!(flag.is_flag());
        // The flag is set.
        assert!(x.is_flag_set(flag));
    }

    #[test]
    fn check_set_flag() {
        let mut x = 0b0000;

        // Set flag for first bit.
        x.set_flag(0b0001);
        // Check first bit.
        assert_eq!(0b0001, x);

        // Set flag for second bit.
        x.set_flag(0b0010);
        // Check second bit.
        assert_eq!(0b0011, x);

        // Set flag for fourth bit.
        x.set_flag(0b1000);
        // Check fourth bit.
        assert_eq!(0b1011, x);
    }

    #[test]
    fn check_is_bit_set() {
        let x = 0b1011;

        // Anything which is not Zero is a flag.
        assert!(x.is_bit_set(0));
        assert!(x.is_bit_set(1));
        assert!(!x.is_bit_set(2));
        assert!(x.is_bit_set(3));
        // Test singed numbers.
        assert!(0b1011i8.is_bit_set(0));
        assert!(0b1011i16.is_bit_set(0));
        assert!(0b1011i32.is_bit_set(0));
        assert!(0b1011i64.is_bit_set(0));
        assert!(0b1011i128.is_bit_set(0));
        assert!(0b1011isize.is_bit_set(0));
        // Test unsigned numbers.
        assert!(0b1011u8.is_bit_set(0));
        assert!(0b1011u16.is_bit_set(0));
        assert!(0b1011u32.is_bit_set(0));
        assert!(0b1011u64.is_bit_set(0));
        assert!(0b1011u128.is_bit_set(0));
        assert!(0b1011usize.is_bit_set(0));
    }

    #[test]
    fn check_set_bit() {
        let mut x = 0b0000;

        // Set first bit.
        x.set_bit(0);
        // Check first bit.
        assert_eq!(0b0001, x);

        // Set second bit.
        x.set_bit(1);
        // Check second bit.
        assert_eq!(0b0011, x);

        // Set fourth bit.
        x.set_bit(3);
        // Check fourth bit.
        assert_eq!(0b1011, x);
    }

    #[test]
    #[should_panic]
    fn is_bit_set_index_out_of_range() {
        let bit_out_of_range = 64;

        // Bit are indexed from 0 so bit on position 64 has bit index 63.
        0u64.is_bit_set(bit_out_of_range);
    }
}
