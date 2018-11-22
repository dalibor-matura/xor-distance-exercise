use bitops::is_bit_set;
use bits::Bits;
use num_traits::{PrimInt, Unsigned};

pub struct XorDistance<T: PrimInt + Unsigned> {
    points: Vec<T>,
    bit_size: usize,
}

impl<T: PrimInt + Unsigned> XorDistance<T> {
    pub fn new(points: Vec<T>) -> Self {
        let bit_size = Bits::bit_size::<T>();

        Self { points, bit_size }
    }

    /// Return up to requested count of closest points to the provided `x`, ordered from the closest
    /// to the n-th closest, where `n` is the count.
    ///
    /// The returned closest points count my be lower than the specified count and equal to all
    /// points count only in the case that: `count > points.len()`.
    ///
    /// # Examples
    /// ```
    /// extern crate xor_distance_exercise;
    ///
    /// use xor_distance_exercise::xor_distance::XorDistance;
    ///
    /// let xor_distance: XorDistance<u64> = XorDistance::new(vec![
    ///     0, 1, 2, 4, 6, 8, 12, 18, 19, 20, 21, 22, 406, 407, 408, 409, 410, 444, 445,
    /// ]);
    ///
    /// let x = 200;
    /// let count = 10;
    ///
    /// let closest_points = xor_distance.closest(x, count);
    /// ```
    pub fn closest(&self, x: T, count: usize) -> Vec<T> {
        let mut closest_sorted = self.points.clone();
        closest_sorted.sort_by_key(|point| *point ^ x);
        closest_sorted.truncate(count);
        closest_sorted
    }

    /// Return a `Some(x)` such that `self.closest(x)` equals closest_points and return None in case
    /// such a `x` does not exists.
    ///
    /// # Examples
    /// ```
    /// extern crate xor_distance_exercise;
    ///
    /// use xor_distance_exercise::xor_distance::XorDistance;
    ///
    /// let xor_distance: XorDistance<u64> = XorDistance::new(vec![
    ///     0, 1, 2, 4, 6, 8, 12, 18, 19, 20, 21, 22, 406, 407, 408, 409, 410, 444, 445,
    /// ]);
    ///
    /// let x = 200;
    /// let count = 10;
    ///
    /// // Get closest points and reversed guess of `x`
    /// let closest_points = xor_distance.closest(x, count);
    /// let x_guess = xor_distance.reverse_closest(&closest_points).unwrap();
    ///
    /// // Check that both `x` and `guess_x` produce the same result.
    /// assert_eq!(closest_points, xor_distance.closest(x_guess, count));
    /// ```
    pub fn reverse_closest(&self, closest_points: &[T]) -> Option<T> {
        let inequalities = self.form_inequalities(closest_points);

        if let Some(bit_rep) = self.form_bits_restrictions_from_inequalities(&inequalities) {
            // Asking for the same number type as we are bit-representing is fine.
            let position = bit_rep.form_zero_padded_number::<T>().unwrap();

            return Some(position);
        }

        None
    }

    pub fn form_inequalities(&self, closest_points: &[T]) -> Vec<(T, T)> {
        let mut inequalities = self.compose_closest_points_inequalities(closest_points);
        let mut further_inequalities = self.compose_further_points_inequalities(closest_points);

        inequalities.append(&mut further_inequalities);

        inequalities
    }

    /// Compose inequalities pairs amongst closest points and their order.
    ///
    /// We have a set of all existing unique points, represented as:
    /// `P = [p1, p2, p3, p4, p5, ..., p(m-1), p(m)]`
    ///
    /// We have a position number represented by `x` and we also have a P subset of selected points
    /// that are the closest points to `x` by XOR distance metric.
    ///
    /// The closest points are represented as:
    /// `C = [c1, c2, c3, c4, c5, ..., c(n-1), c(n)]`
    ///
    /// and the following inequality applies:
    /// `c1 ^ x < c2 ^ x < c3 ^ x < c4 ^ x < c5 ^ x < ... < c(n-1) ^ x < c(n) ^ x`
    ///
    /// Separating it into simple `(n-1)` inequalities:
    /// `c1 ^ x < c2 ^ x`
    /// `c2 ^ x < c3 ^ x`
    /// `c3 ^ x < c4 ^ x`
    /// `c4 ^ x < c5 ^ x`
    /// `...`
    /// `c(n-1) ^ x < c(n) ^ x`
    ///
    /// These `(n-1)` inequalities are what this method returns.
    fn compose_closest_points_inequalities(&self, closest_points: &[T]) -> Vec<(T, T)> {
        // Prepare the inequalities container.
        let size = closest_points.len();
        let mut inequalities = Vec::with_capacity(size);

        // Collect pairs of inequalities.
        for i in 0..size - 1 {
            // Point `a` must be closer to the point `x` then point `b`. The inequality is:
            // `a ^ x < b ^ x` , where point `x` is the position being searched for.
            let a = closest_points[i];
            let b = closest_points[i + 1];

            inequalities.push((a, b));
        }

        inequalities
    }

    /// Compose inequalities pairs between last closest point and all further points.
    ///
    /// We have a set of all existing unique points, represented as:
    /// `P = [p1, p2, p3, p4, p5, ..., p(n-1), p(n)]`
    ///
    /// We have a position number represented by `x` and we also have a P subset of selected points
    /// that are the closest points to `x` by XOR distance metric.
    ///
    /// The closest points are represented as:
    /// `[c1, c2, c3, c4, c5, ..., c(n-1), c(n)]`
    ///
    /// The further points are all unselected points from P and are represented as (U = P - C):
    /// `U = [u1, u2, u3, u4, u5, ..., u(n-1), u(n)]`
    ///
    /// and the following inequalities applies:
    /// `c(n) ^ x < u1 ^ x`
    /// `c(n) ^ x < u2 ^ x`
    /// `c(n) ^ x < u3 ^ x`
    /// `c(n) ^ x < u4 ^ x`
    /// `c(n) ^ x < u5 ^ x`
    /// ...`
    /// `c(n) ^ x < u(m) ^ x`
    ///
    /// These inequalities are what this method returns.
    fn compose_further_points_inequalities(&self, closest_points: &[T]) -> Vec<(T, T)> {
        // Get the n-th closest point to `x` where the n is number of closest points.
        if let Some(a) = closest_points.last() {
            let further_points = self.get_further_points(closest_points);

            // Prepare the inequalities container.
            let size = further_points.len();
            let mut inequalities = Vec::with_capacity(size);

            // Collect pairs of inequalities.
            for b in further_points.iter() {
                // Point `a` must be closer to the point `x` then point `b`. The inequality is:
                // `a ^ x < b ^ x` , where point `x` is the position being searched for.
                inequalities.push((*a, *b));
            }

            return inequalities;
        }

        // There are no inequalities.
        Vec::new()
    }

    fn get_further_points(&self, closest_points: &[T]) -> Vec<T> {
        // Get further points (the ones that were not selected as the closest).
        let mut further_points = self.points.clone();
        // Exclude all closest points.
        further_points.retain(|x| !closest_points.contains(&x));

        further_points
    }

    /// Form bits restrictions as a bit representation based on provided inequalities.
    ///
    /// Returns `Some(b)` if bits restrictions can be constructed within constrains (no two
    /// inequalities contradict themselves), `None` otherwise.
    fn form_bits_restrictions_from_inequalities(&self, inequalities: &[(T, T)]) -> Option<Bits> {
        let mut bit_rep = Bits::new::<T>();

        // Combine all inequalities to form bits restrictions.
        for pair in inequalities.iter() {
            if self.add_bit_restriction_from_inequality(pair, &mut bit_rep).is_err() {
                // Required bit can not be set within constrains and thus valid Bits
                // can not be formed.
                return None;
            }
        }

        Some(bit_rep)
    }

    /// Incorporate bit restriction from provided inequality `a ^ x < b ^ x`, where `x` is the
    /// position being searched for.
    ///
    /// Returns `Ok(())` in case the inequality doesn't contradict any inequality processed so far,
    /// `Err(&str)` otherwise.
    ///
    fn add_bit_restriction_from_inequality(
        &self,
        &(a, b): &(T, T),
        bit_rep: &mut Bits,
    ) -> Result<(), &'static str> {
        let xor_distance: T = a ^ b;

        // Index of the first left hand-side bit in which `a` and `b` differ. The index starts by 0.
        let bit_index = (self.bit_size as u32 - xor_distance.leading_zeros() - 1) as usize;

        // As `a` is closer to the position we are searching for then `b`, we need to restrict
        // to bit value of `a`.
        let a_bit = is_bit_set(a, bit_index);

        // Required bit can not be set within constrains.
        if let Err(e) = bit_rep.set_bit_within_constrains(bit_index, a_bit) {
            return Err(e);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::XorDistance;

    #[test]
    fn compose_closest_points_inequalities() {
        let points: Vec<u8> = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];
        let xor_distance = XorDistance::new(points.clone());

        let closest_points: Vec<u8> = vec![0, 1, 2, 3, 4, 5, 6];

        // Test first example, count < number of points.
        let result = xor_distance.compose_closest_points_inequalities(&closest_points);
        let expected: Vec<(u8, u8)> = vec![(0, 1), (1, 2), (2, 3), (3, 4), (4, 5), (5, 6)];

        assert_eq!(expected, result);
    }

    #[test]
    fn compose_further_points_inequalities() {
        let points: Vec<u8> = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];
        let xor_distance = XorDistance::new(points.clone());

        let closest_points: Vec<u8> = vec![0, 1, 2, 3, 4, 5, 6];

        // Test first example, count < number of points.
        let result = xor_distance.compose_further_points_inequalities(&closest_points);
        let expected: Vec<(u8, u8)> = vec![(6, 7), (6, 8), (6, 9), (6, 10), (6, 11), (6, 12)];

        assert_eq!(expected, result);
    }

    #[test]
    fn closest_u64() {
        let points: Vec<u64> = vec![
            0, 1, 2, 4, 6, 8, 12, 18, 19, 20, 21, 22, 406, 407, 408, 409, 410, 444, 445,
        ];
        let xor_distance = XorDistance::new(points.clone());

        // Test first example, count < number of points.
        let result = xor_distance.closest(300, 4);
        let expected = vec![444, 445, 408, 409];

        assert_eq!(expected, result);

        // Test second example, count < number of points.
        let result = xor_distance.closest(10, 10);
        let expected = vec![8, 12, 2, 0, 1, 6, 4, 18, 19, 22];

        assert_eq!(expected, result);

        // Test third example, count < number of points.
        let result = xor_distance.closest(888, 12);
        let expected = vec![444, 445, 408, 409, 410, 406, 407, 18, 19, 20, 21, 22];

        assert_eq!(expected, result);

        // Test situation with count = 0.
        let result = xor_distance.closest(10, 0);
        let expected: Vec<u64> = Vec::new();

        assert_eq!(expected, result);

        // Test situation with count = number of points.
        let result = xor_distance.closest(10, points.len());
        let expected = vec![
            8, 12, 2, 0, 1, 6, 4, 18, 19, 22, 20, 21, 410, 408, 409, 406, 407, 444, 445,
        ];

        assert_eq!(expected, result);
        assert_eq!(points.len(), expected.len());

        // Test situation with count > number of points.
        let result = xor_distance.closest(10, points.len() + 1);
        let expected = vec![
            8, 12, 2, 0, 1, 6, 4, 18, 19, 22, 20, 21, 410, 408, 409, 406, 407, 444, 445,
        ];

        assert_eq!(expected, result);
        assert_eq!(points.len(), expected.len());
    }

    #[test]
    fn closest_u8() {
        let points: Vec<u8> = vec![
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 12, 20, 21, 22, 23, 24, 100, 220, 230, 240, 250,
        ];
        let xor_distance = XorDistance::new(points.clone());

        // Test first example, count < number of points.
        let result = xor_distance.closest(18, 8);
        let expected = vec![22, 23, 20, 21, 24, 2, 3, 0];

        assert_eq!(expected, result);

        // Test second example, count < number of points.
        let result = xor_distance.closest(200, 14);
        let expected = vec![220, 230, 250, 240, 100, 8, 9, 10, 12, 0, 1, 2, 3, 4];

        assert_eq!(expected, result);
    }

    #[test]
    fn reverse_closest_u64() {
        let xor_distance: XorDistance<u64> = XorDistance::new(vec![
            0, 1, 2, 4, 6, 8, 12, 18, 19, 20, 21, 22, 406, 407, 408, 409, 410, 444, 445,
        ]);

        let closest_points = vec![8, 12, 2, 0, 1, 6, 4, 18, 19, 22];
        let count = closest_points.len();
        let guess_pos = xor_distance.reverse_closest(&closest_points).unwrap();

        assert_eq!(closest_points, xor_distance.closest(guess_pos, count));
    }

    #[test]
    fn reverse_closest_u8() {
        let xor_distance: XorDistance<u64> = XorDistance::new(vec![
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 12, 20, 21, 22, 23, 24, 100, 220, 230, 240, 250,
        ]);

        let closest_points = vec![220, 230, 250, 240, 100, 8, 9, 10, 12, 0, 1, 2, 3, 4];
        let count = closest_points.len();
        let guess_pos = xor_distance.reverse_closest(&closest_points).unwrap();

        assert_eq!(closest_points, xor_distance.closest(guess_pos, count));
    }

    #[test]
    fn reverse_closest_invalid_input() {
        let xor_distance: XorDistance<u64> = XorDistance::new(vec![
            0, 1, 2, 4, 6, 8, 12, 18, 19, 20, 21, 22, 406, 407, 408, 409, 410, 444, 445,
        ]);

        let closest_points = vec![8, 2, 12, 6, 1, 0, 4, 18, 22];

        // The output is `None` as there's no `x` that would satisfy the provided closest points
        // input.
        assert!(xor_distance.reverse_closest(&closest_points).is_none());
    }
}
