use std::collections::HashSet;
use num_bigint::BigUint;
use num_traits::One; 
use crate::error::HierarchyError;

/// Represents the initial pattern (S_base) at a specific bit-width (N_base).
/// This pattern is the seed for generating hierarchical structures at higher N-levels.
#[derive(Debug, Clone)]
pub struct InitialPattern {
    /// The set of X-values (as BigUint) that constitute the base pattern.
    /// These are typically the numerically smaller values of canonical Paired Entities.
    pub s_base_values: HashSet<BigUint>,
    /// The bit-width (N) of the X-values in `s_base_values`.
    pub n_base_bits: usize,
}

impl InitialPattern {
    /// Creates a new `InitialPattern`.
    ///
    /// # Arguments
    /// * `s_base_values`: A set of `BigUint` X-values for the base pattern.
    /// * `n_base_bits`: The bit-width N for these base X-values.
    ///
    /// # Errors
    /// Returns `HierarchyError` if:
    /// * `n_base_bits` is 0.
    /// * `s_base_values` is empty.
    /// * Any value in `s_base_values` cannot be represented within `n_base_bits`
    ///   (i.e., value >= 2^`n_base_bits`).
    pub fn new(s_base_values: HashSet<BigUint>, n_base_bits: usize) -> Result<Self, HierarchyError> {
        if n_base_bits == 0 {
            return Err(HierarchyError::NonPositiveNBits(n_base_bits));
        }
        if s_base_values.is_empty() {
            return Err(HierarchyError::EmptySBaseValues);
        }

        let one = BigUint::one();
        // limit_exclusive represents 2^n_base_bits.
        // Values in s_base_values must be < limit_exclusive.
        let limit_exclusive = &one << n_base_bits;

        for val in &s_base_values {
            if *val >= limit_exclusive {
                // max_representable_value is 2^n_base_bits - 1.
                // Since n_base_bits >= 1, limit_exclusive >= 2, so subtracting 1 is safe.
                let max_representable_value = limit_exclusive - &one;
                return Err(HierarchyError::ValueExceedsNBaseBits {
                    value: val.clone(),
                    n_bits: n_base_bits,
                    max_val: max_representable_value,
                });
            }
        }
        Ok(Self { s_base_values, n_base_bits })
    }
}