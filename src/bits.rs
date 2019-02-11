//! Bits representation for any `Integer`.

use crate::bitops::BitOps;
use num_traits::PrimInt;
use std::mem::size_of;

/// Bits representation.
///
/// # Examples
/// ```
/// extern crate xor_distance_exercise;
///
/// use xor_distance_exercise::bits::Bits;
///
/// // Find out bit size of specific integer type.
/// let size = Bits::bit_size::<i64>();
///
/// // Bit representation of `u64` integer.
/// let mut bit_rep = Bits::new::<u64>();
///
/// // Operations on the bit representation.
/// let bit = bit_rep.get_bit(4);
/// bit_rep.set_bit(4, true);
/// bit_rep.set_bit_within_constrains(5, true);
/// bit_rep.is_bit_decided(4);
/// let number = bit_rep.form_zero_padded_number::<u64>().unwrap();
/// ```
pub struct Bits {
    bits: Vec<Option<bool>>,
    size: usize,
}

impl Bits {
    /// Create a new representation of Bits.
    /// # Examples
    /// ```
    /// extern crate xor_distance_exercise;
    ///
    /// use xor_distance_exercise::bits::Bits;
    ///
    /// let bit_rep = Bits::new::<u64>;
    /// ```
    pub fn new<T: PrimInt>() -> Self {
        // Initialize the vector with known size.
        let size = Self::bit_size::<T>();
        let mut bits: Vec<Option<bool>> = Vec::with_capacity(size);

        // Initialize the vector with default values of None (undecided bit yet).
        for _ in 0..size {
            bits.push(None);
        }

        Bits { bits, size }
    }

    /// Return bit size of the type being represented in bits.
    /// # Examples
    /// ```
    /// extern crate xor_distance_exercise;
    ///
    /// use xor_distance_exercise::bits::Bits;
    ///
    /// assert_eq!(8, Bits::bit_size::<u8>());
    /// assert_eq!(32, Bits::bit_size::<u32>());
    /// assert_eq!(64, Bits::bit_size::<u64>());
    /// assert_eq!(64, Bits::bit_size::<i64>());
    /// ```
    pub fn bit_size<T: PrimInt>() -> usize {
        let byte_size = size_of::<T>();

        // Return the bit size.
        byte_size * 8
    }

    /// Get bit value for the index.
    ///
    /// # Examples
    /// ```
    /// extern crate xor_distance_exercise;
    ///
    /// use xor_distance_exercise::bits::Bits;
    ///
    /// let bit_rep = Bits::new::<u64>();
    /// let bit = bit_rep.get_bit(4);
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if `index` is out of range.
    pub fn get_bit(&self, index: usize) -> Option<bool> {
        self.bits[index]
    }

    /// Set new bit value for the index.
    ///
    /// # Examples
    /// ```
    /// extern crate xor_distance_exercise;
    ///
    /// use xor_distance_exercise::bits::Bits;
    ///
    /// let mut bit_rep = Bits::new::<u64>();
    /// bit_rep.set_bit(4, true);
    /// bit_rep.set_bit(5, false);
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if `index` is out of range.
    pub fn set_bit(&mut self, index: usize, val: bool) {
        self.bits[index] = Some(val);
    }

    /// Set new bit value complying with constrains, already decided bit value can not be changed.
    ///
    /// Returns `Ok(())` in case constrains were not violated, `Err(&str)` otherwise.
    ///
    /// # Examples
    /// ```
    /// extern crate xor_distance_exercise;
    ///
    /// use xor_distance_exercise::bits::Bits;
    ///
    /// let mut bit_rep = Bits::new::<u64>();
    /// bit_rep.set_bit_within_constrains(4, true);
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if `index` is out of range.
    pub fn set_bit_within_constrains(
        &mut self,
        index: usize,
        val: bool,
    ) -> Result<(), &'static str> {
        match self.bits[index] {
            // Existing bit with a different value is a breach of constrains.
            Some(bit) if bit != val => return Err("Already decided bit value can not be changed!"),
            // The value is already present, nothing to do here.
            Some(_) => {}
            // No value set as yet so just assign it.
            None => self.bits[index] = Some(val),
        }

        Ok(())
    }

    /// Is bit decided already?
    ///
    /// # Examples
    /// ```
    /// extern crate xor_distance_exercise;
    ///
    /// use xor_distance_exercise::bits::Bits;
    ///
    /// let bit_rep = Bits::new::<u64>();
    /// bit_rep.is_bit_decided(4);
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if `index` is out of range.
    pub fn is_bit_decided(&self, index: usize) -> bool {
        let bit = self.bits[index];

        bit.is_some()
    }

    /// Form and return a number based on bits representation, pad/fill undecided bits by zeros.
    ///
    /// # Examples
    /// ```
    /// extern crate xor_distance_exercise;
    ///
    /// use xor_distance_exercise::bits::Bits;
    ///
    /// let bit_rep = Bits::new::<u64>();
    /// let number = bit_rep.form_zero_padded_number::<u64>().unwrap();
    /// ```
    pub fn form_zero_padded_number<T: PrimInt>(&self) -> Result<T, &str> {
        if Self::bit_size::<T>() < self.size {
            return Err("Requested number type has not enough bits to represent the whole number!");
        }

        // Initialize the number with "0".
        let mut number: T = T::zero();

        // Construct the number by incorporating in all bits.
        for (index, _) in self.bits.iter().enumerate() {
            self.incorporate_bit(index, &mut number);
        }

        Ok(number)
    }

    /// Incorporate bit into the provided number.
    ///
    /// # Panics
    ///
    /// Panics if `index` is out of range.
    fn incorporate_bit<T: PrimInt + BitOps>(&self, index: usize, number: &mut T) {
        let bit = self.bits[index];

        // Set only `1` bit as `0` bits are present by default.
        match bit {
            Some(bit) if bit => {
                number.set_bit(index);
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::bits::Bits;

    #[test]
    fn bit_size() {
        assert_eq!(8, Bits::bit_size::<u8>());
        assert_eq!(16, Bits::bit_size::<u16>());
        assert_eq!(32, Bits::bit_size::<u32>());
        assert_eq!(64, Bits::bit_size::<u64>());
        assert_eq!(128, Bits::bit_size::<u128>());
    }

    #[test]
    fn new_bits_by_default_none() {
        let bit_rep = Bits::new::<u64>();

        for i in 0..Bits::bit_size::<u64>() {
            assert_eq!(
                None,
                bit_rep.get_bit(i),
                "Every bit should be empty in this phase, but the bit with index {} is not!",
                i
            );
        }
    }

    #[test]
    fn get_set_bit() {
        let mut bit_rep = Bits::new::<u64>();

        // By default all bits are None before being set otherwise.
        assert_eq!(None, bit_rep.get_bit(0));
        assert_eq!(None, bit_rep.get_bit(8));
        assert_eq!(None, bit_rep.get_bit(63));

        // Set 0-th bit to true.
        let index = 0;
        let val = true;
        bit_rep.set_bit(index, val);
        assert_eq!(Some(val), bit_rep.get_bit(index));

        // Set 22-nd bit to true.
        let index = 22;
        let val = false;
        bit_rep.set_bit(index, val);
        assert_eq!(Some(val), bit_rep.get_bit(index));

        // Set 63-rd bit to false.
        let index = 63;
        let val = false;
        bit_rep.set_bit(index, val);
        assert_eq!(Some(val), bit_rep.get_bit(index));

        // Override 63-rd bit to true.
        let index = 63;
        let val = true;
        bit_rep.set_bit(index, val);
        assert_eq!(Some(val), bit_rep.get_bit(index));
    }

    #[test]
    #[should_panic(expected = "index out of bounds: the len is 64 but the index is 64")]
    fn get_bit_index_out_of_range() {
        let bit_rep = Bits::new::<u64>();

        let index_out_of_range = 64;
        bit_rep.get_bit(index_out_of_range);
    }

    #[test]
    #[should_panic(expected = "index out of bounds: the len is 64 but the index is 64")]
    fn set_bit_index_out_of_range() {
        let mut bit_rep = Bits::new::<u64>();

        let index_out_of_range = 64;
        bit_rep.set_bit(index_out_of_range, true);
    }

    #[test]
    fn set_bit_within_constrains() {
        let mut bit_rep = Bits::new::<u64>();

        let index = 2;
        // Setting the bit value for the first time is OK as it wasn't decided yet.
        assert_eq!(Ok(()), bit_rep.set_bit_within_constrains(index, true));
        // Setting the same bit value for the second time is OK, as the value stays the same.
        assert_eq!(Ok(()), bit_rep.set_bit_within_constrains(index, true));
        // Setting the bit value with a different value then in previous step violates constrains.
        assert_eq!(
            Err("Already decided bit value can not be changed!"),
            bit_rep.set_bit_within_constrains(index, false)
        );
    }

    #[test]
    fn is_bit_decided() {
        let mut bit_rep = Bits::new::<u64>();
        let index = 0;

        assert!(
            !bit_rep.is_bit_decided(index),
            "Bit hasn't been decided already, so false must be returned!"
        );

        // Set the bit to be `1`.
        bit_rep.set_bit(index, true);

        assert!(
            bit_rep.is_bit_decided(index),
            "Bit has been decided already, so true must be returned!"
        );

        // Set the bit to be `0`.
        bit_rep.set_bit(index, false);

        assert!(
            bit_rep.is_bit_decided(index),
            "Bit has been decided already, so true must be returned!"
        );
    }

    #[test]
    fn form_zero_padded_number() {
        let mut bit_rep = Bits::new::<u64>();
        bit_rep.set_bit_within_constrains(1, true).unwrap();
        bit_rep.set_bit_within_constrains(2, true).unwrap();
        bit_rep.set_bit_within_constrains(6, true).unwrap();

        assert_eq!(70, bit_rep.form_zero_padded_number::<u64>().unwrap());
    }

    #[test]
    fn form_zero_padded_number_type_error() {
        let bit_rep = Bits::new::<u64>();

        // Error is expected.
        assert_eq!(
            Err("Requested number type has not enough bits to represent the whole number!"),
            bit_rep.form_zero_padded_number::<u32>()
        );
    }

    #[test]
    fn incorporate_bit() {
        let mut bit_rep = Bits::new::<u64>();
        bit_rep.set_bit_within_constrains(1, true).unwrap();
        bit_rep.set_bit_within_constrains(2, true).unwrap();

        let mut number: u64 = 0;

        // Incorporating `1` bit with index 1 adds value 2.
        bit_rep.incorporate_bit(1, &mut number);

        assert_eq!(2, number);

        // Incorporating `1` bit with index 2 adds value 4.
        bit_rep.incorporate_bit(2, &mut number);

        assert_eq!(6, number);

        // Incorporating `0` bit does not change number's value.
        bit_rep.incorporate_bit(3, &mut number);

        assert_eq!(6, number);
    }
}
