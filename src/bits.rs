use bitops::set_bit;
use num_traits::PrimInt;
use std::mem::size_of;

/// Bits representation.
pub struct Bits {
    bits: Vec<Option<bool>>,
    size: usize,
}

impl Bits {
    /// Create a new representation of Bits.
    /// # Examples
    /// ```
    /// extern crate xor_distance_exercise;
    /// use xor_distance_exercise::bits::Bits;
    ///
    /// Bits::new::<u64>;
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

    /// Return bit size of the type being represent in bits.
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
        let bit_size = byte_size * 8;

        bit_size
    }

    /// Get bit value for the index.
    ///
    /// # Panics
    ///
    /// Panics if `index` is out of range.
    pub fn get_bit(&self, index: usize) -> Option<bool> {
        self.bits[index]
    }

    /// Set new bit value for the index.
    ///
    /// # Panics
    ///
    /// Panics if `index` is out of range.
    pub fn set_bit(&mut self, index: usize, val: bool) {
        self.bits[index] = Some(val);
    }

    /// Set new bit value for the index.
    ///
    /// # Panics
    ///
    /// Panics if `index` is out of range.
    pub fn set_bit_within_constrains(&mut self, index: usize, val: bool) -> bool {
        match self.bits[index] {
            // Existing bit with a different value is a breach of constrains.
            Some(bit) if bit != val => return false,
            // The value is already present, nothing to do here.
            Some(_) => {}
            // No value set as yet so just assign it.
            None => self.bits[index] = Some(val),
        }

        true
    }

    /// Is bit decided already?
    ///
    /// # Panics
    ///
    /// Panics if `index` is out of range.
    pub fn is_bit_decided(&self, index: usize) -> bool {
        let bit = self.bits[index];

        bit.is_some()
    }

    /// Form and return a number based on bits representation, pad/fill undecided bits by zeros.
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
    fn incorporate_bit<T: PrimInt>(&self, index: usize, number: &mut T) {
        let bit = self.bits[index];

        // Set only "1" bit as the "0" bit is there by default.
        match bit {
            Some(bit) if bit == true => {
                set_bit::<T>(number, index);
            }
            _ => {}
        }
    }
}
