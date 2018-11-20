//! Miscellaneous bit operations for any [`Integer`].
//!
//! # Examples
//!
//! ```
//! extern crate xor_distance_exercise;
//!
//! use xor_distance_exercise::bitops::{is_flag, is_bit_set, is_flag_set, set_bit, set_flag};
//!
//! let x = 0b1000_0000_1001_1010;
//! let flag = 0b1000_0000;
//!
//! assert!(is_flag(flag));
//! assert!(is_bit_set(flag, 7));
//! assert!(is_flag_set(x, flag));
//!
//! let mut y = 0b1000_0000_0001_1010;
//!
//! set_flag(&mut y, flag);
//! assert_eq!(0b1000_0000_1001_1010, y);
//!
//! set_bit(&mut y, 0);
//! assert_eq!(0b1000_0000_1001_1011, y);
//! ```

use num_traits::PrimInt;

/// Returns whether this number only has one bit set.
///
/// # Examples
///
/// ```
/// extern crate xor_distance_exercise;
///
/// use xor_distance_exercise::bitops::is_flag;
///
/// // Flag must have exactly one bit set to "1".
/// assert!(is_flag(0b0010));
/// assert!(!is_flag(0b0101));
/// ```
#[inline]
pub fn is_flag<T: PrimInt>(x: T) -> bool {
    // Flag must satisfy following criteria:
    // - at lest one bit must be set "1"
    // - exactly one bit must be set to "1", subtracting one will move this bit right and thus
    //   bitwise "&" with the original value must return zero
    x > T::zero() && (x & (x - T::one())) == T::zero()
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
/// use xor_distance_exercise::bitops::is_flag_set;
///
/// let x = 0b1101;
/// let flag = 0b0001;
///
/// assert!(is_flag_set(x, flag));
/// ```
#[inline]
pub fn is_flag_set<T: PrimInt>(x: T, flag: T) -> bool {
    // The self has the "1" bit set on the same position as the flag.
    x & flag > T::zero()
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
/// use xor_distance_exercise::bitops::set_flag;
///
/// let mut x = 0b1101;
/// let flag = 0b0010;
///
/// set_flag(&mut x, flag);
///
/// assert_eq!(0b1111, x);
/// ```
#[inline]
pub fn set_flag<T: PrimInt>(x: &mut T, flag: T) {
    *x = *x | flag;
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
/// use xor_distance_exercise::bitops::is_bit_set;
///
/// assert!(is_bit_set(0b1000, 3));
/// ```
#[inline]
pub fn is_bit_set<T: PrimInt>(x: T, bit_index: usize) -> bool {
    // Create flag one and move its "1" bit from most left hand side to left by the requested
    // bit index number.
    let flag = T::one() << bit_index;
    // Check out if the prepared flag is set.
    is_flag_set(x, flag)
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
/// use xor_distance_exercise::bitops::set_bit;
///
/// let mut x = 0b1000;
/// set_bit(&mut x, 1);
///
/// // The second bit should be added/set now.
/// assert_eq!(0b1010, x);
/// ```
#[inline]
pub fn set_bit<T: PrimInt>(x: &mut T, bit_index: usize) {
    // Create flag one and move its "1" bit from most left hand side to left by the requested
    // bit index number.
    let flag = T::one() << bit_index;
    // Check out if the prepared flag is set.
    set_flag(x, flag);
}

#[cfg(test)]
mod tests {
    use bitops::{is_bit_set, is_flag, is_flag_set, set_bit, set_flag};

    #[test]
    fn check_is_flag() {
        let zero = 0;

        // Zero is not a flag.
        assert!(!is_flag(zero));

        // Number having more than one "1" bit is not a flag.
        let x = 0b0111;
        assert!(!is_flag(x));

        let flag = 0b0100;

        // Any number that has exactly one "1" bit set is a flag.
        assert!(is_flag(flag));
    }

    #[test]
    fn check_is_flag_set() {
        let zero = 0;

        // Zero is not a flag.
        assert!(!is_flag(zero));
        // Zero can not be set as a flag to anything.
        assert!(!is_flag_set(0b0000, 0));

        let flag = 0b0010;
        let x = 0b1110;
        // Valid flag.
        assert!(is_flag(flag));
        // The flag is set.
        assert!(is_flag_set(x, flag));
    }

    #[test]
    fn check_set_flag() {
        let mut x = 0b0000;

        // Set flag for first bit.
        set_flag(&mut x, 0b0001);
        // Check first bit.
        assert_eq!(0b0001, x);

        // Set flag for second bit.
        set_flag(&mut x, 0b0010);
        // Check second bit.
        assert_eq!(0b0011, x);

        // Set flag for fourth bit.
        set_flag(&mut x, 0b1000);
        // Check fourth bit.
        assert_eq!(0b1011, x);
    }

    #[test]
    fn check_is_bit_set() {
        let x = 0b1011;

        // Anything which is not Zero is a flag.
        assert!(is_bit_set(x, 0));
        assert!(is_bit_set(x, 1));
        assert!(!is_bit_set(x, 2));
        assert!(is_bit_set(x, 3));
        // Test singed numbers.
        assert!(is_bit_set(0b1011i8, 0));
        assert!(is_bit_set(0b1011i16, 0));
        assert!(is_bit_set(0b1011i32, 0));
        assert!(is_bit_set(0b1011i64, 0));
        assert!(is_bit_set(0b1011i128, 0));
        assert!(is_bit_set(0b1011isize, 0));
        // Test unsigned numbers.
        assert!(is_bit_set(0b1011u8, 0));
        assert!(is_bit_set(0b1011u16, 0));
        assert!(is_bit_set(0b1011u32, 0));
        assert!(is_bit_set(0b1011u64, 0));
        assert!(is_bit_set(0b1011u128, 0));
        assert!(is_bit_set(0b1011usize, 0));
    }

    #[test]
    fn check_set_bit() {
        let mut x = 0b0000;

        // Set first bit.
        set_bit(&mut x, 0);
        // Check first bit.
        assert_eq!(0b0001, x);

        // Set second bit.
        set_bit(&mut x, 1);
        // Check second bit.
        assert_eq!(0b0011, x);

        // Set fourth bit.
        set_bit(&mut x, 3);
        // Check fourth bit.
        assert_eq!(0b1011, x);
    }

    #[test]
    #[should_panic]
    fn is_bit_set_index_out_of_range() {
        let bit_out_of_range = 64;

        // Bit are indexed from 0 so bit on position 64 has bit index 63.
        is_bit_set(0u64, bit_out_of_range);
    }
}
