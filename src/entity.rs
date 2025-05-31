use num_bigint::BigUint;
use num_traits::One; 
use crate::error::HierarchyError;

/// Represents an N-bit Paired Entity, consisting of an N-bit value X
/// and its bitwise complement X'.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PairedEntity {
    /// The N-bit X-value. In canonical representations, this is often
    /// chosen as the numerically smaller value of the pair (X, X').
    pub x: BigUint,
    /// The N-bit bitwise complement of X.
    pub x_prime: BigUint,
    /// The bit-width N of X and X'.
    pub n_bits: usize,
}

impl PairedEntity {
    /// Creates a new `PairedEntity` from an X-value and its bit-width N.
    /// The complement X' is calculated automatically.
    ///
    /// # Arguments
    /// * `x`: The `BigUint` X-value.
    /// * `n_bits`: The bit-width N. Must be greater than 0.
    ///
    /// # Errors
    /// Returns `HierarchyError` if:
    /// * `n_bits` is 0.
    /// * `x` cannot be represented within `n_bits` (i.e., x >= 2^`n_bits`).
    pub fn new(x: BigUint, n_bits: usize) -> Result<Self, HierarchyError> {
        if n_bits == 0 {
            return Err(HierarchyError::NonPositiveNBits(n_bits));
        }

        let one = BigUint::one();
        let limit_exclusive = &one << n_bits;
        if x >= limit_exclusive {
            return Err(HierarchyError::ValueTooLargeForNBits { value: x.clone(), n_bits });
        }

        // Calculate complement: X' = (2^N - 1) - X
        // 2^N - 1 is a sequence of N ones.
        // Since n_bits >= 1, limit_exclusive >= 2, so subtracting one is safe.
        let all_ones = limit_exclusive - &one;
        let x_prime = all_ones - &x;

        Ok(PairedEntity { x, x_prime, n_bits })
    }

    /// Creates a new `PairedEntity` in its canonical form, where `x` is guaranteed
    /// to be the numerically smaller value of the (value, complement) pair.
    ///
    /// # Arguments
    /// * `value`: A `BigUint` value, which could be either X or X'.
    /// * `n_bits`: The bit-width N. Must be greater than 0.
    ///
    /// # Errors
    /// Returns `HierarchyError` if `n_bits` is 0 or `value` is too large for `n_bits`.
    pub fn new_canonical_from_x(value: BigUint, n_bits: usize) -> Result<Self, HierarchyError> {
        if n_bits == 0 {
            return Err(HierarchyError::NonPositiveNBits(n_bits));
        }
        let one = BigUint::one();
        let limit_exclusive = &one << n_bits;
        if value >= limit_exclusive {
            return Err(HierarchyError::ValueTooLargeForNBits { value: value.clone(), n_bits });
        }

        // Since n_bits >= 1, limit_exclusive >= 2, so subtracting one is safe.
        let all_ones = limit_exclusive - &one; 
        let complement = &all_ones - &value;

        if value <= complement {
            Ok(PairedEntity { x: value, x_prime: complement, n_bits })
        } else {
            Ok(PairedEntity { x: complement, x_prime: value, n_bits })
        }
    }

    /// Creates a `PairedEntity` from two values, asserting they are complements.
    /// This constructor is useful if X and X' are already known and their
    /// complementarity has been verified or is trusted.
    /// It will pick the smaller value as `self.x` for canonical representation.
    ///
    /// # Arguments
    /// * `val1`: One N-bit value.
    /// * `val2_supposed_complement`: The other N-bit value, assumed to be the complement of `val1`.
    /// * `n_bits`: The bit-width N.
    ///
    /// # Errors
    /// Returns `HierarchyError` if `n_bits` is 0, if values are too large for `n_bits`,
    /// or if `val1` and `val2_supposed_complement` are not valid N-bit complements.
    pub fn new_from_pair_assert_canonical(
        val1: BigUint, 
        val2_supposed_complement: BigUint, 
        n_bits: usize
    ) -> Result<Self, HierarchyError> {
        if n_bits == 0 {
            return Err(HierarchyError::NonPositiveNBits(n_bits));
        }
        let one = BigUint::one();
        let limit_exclusive = &one << n_bits;

        if val1 >= limit_exclusive {
            return Err(HierarchyError::ValueTooLargeForNBits { value: val1.clone(), n_bits });
        }
        if val2_supposed_complement >= limit_exclusive {
            return Err(HierarchyError::ValueTooLargeForNBits { value: val2_supposed_complement.clone(), n_bits });
        }
        
        // Since n_bits >= 1, limit_exclusive >= 2, so subtracting one is safe.
        let all_ones = limit_exclusive - &one;
        if &val1 + &val2_supposed_complement != all_ones {
            return Err(HierarchyError::NonComplementaryPair { 
                val1: val1.clone(), 
                val2_complement: val2_supposed_complement.clone(), 
                n_bits 
            });
        }

        if val1 <= val2_supposed_complement {
            Ok(PairedEntity { x: val1, x_prime: val2_supposed_complement, n_bits })
        } else {
            Ok(PairedEntity { x: val2_supposed_complement, x_prime: val1, n_bits })
        }
    }
}