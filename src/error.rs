use thiserror::Error;
use num_bigint::BigUint;

/// Custom error types for the hierarchical_info library.
#[derive(Error, Debug, PartialEq, Eq, Clone)]
pub enum HierarchyError {
    /// Error indicating that an N-bits specification (e.g., for n_base_bits or n_target_bits)
    /// is zero, which is invalid for most operations requiring positive bit width.
    #[error("N-bits value ({0}) must be positive.")]
    NonPositiveNBits(usize),

    /// Error indicating that the set of base values for an InitialPattern is empty.
    /// An initial pattern must contain at least one value.
    #[error("S_base_values set cannot be empty.")]
    EmptySBaseValues,

    /// Error indicating that a value provided for S_base (InitialPattern)
    /// is too large to be represented by n_base_bits.
    #[error("S_base value {value} (decimal) does not fit within n_base_bits {n_bits}. Maximum representable value is {max_val} (decimal).")]
    ValueExceedsNBaseBits { value: BigUint, n_bits: usize, max_val: BigUint },
    
    /// Error indicating that the target N-bits for an operation (e.g., is_member, decompose)
    /// is smaller than the N-bits of the base pattern.
    #[error("Target N-bits ({target_n_bits}) is smaller than base N-bits ({base_n_bits}).")]
    TargetNBitsTooSmall { target_n_bits: usize, base_n_bits: usize },

    /// Error indicating that the target N-bits is not a valid hierarchical level
    /// derivable from the base N-bits by the rule N_target = N_base * 2^k.
    #[error("Target N-bits ({target_n_bits}) is not a valid hierarchical level from base N-bits ({base_n_bits}). Must be base_n_bits * 2^k for some integer k >= 0.")]
    InvalidHierarchicalLevel { target_n_bits: usize, base_n_bits: usize },

    /// Error indicating that an input X value is too large to be represented
    /// by the specified number of bits (n_bits).
    /// An N-bit number must be less than 2^N.
    #[error("Input X value {value} (decimal) is too large for specified n_bits {n_bits}. Value must be < 2^{n_bits}.")]
    ValueTooLargeForNBits { value: BigUint, n_bits: usize },

    /// Error indicating that an input X value is not a member of the
    /// selected set S_N for the given InitialPattern and target N-bits.
    #[error("Input X value {0} (decimal) is not a member of the selected set S_N for the given N-bits and initial pattern.")]
    NotAMember(BigUint),

    /// Error indicating that a component provided for composition
    /// is not a valid member of the initial S_base pattern.
    #[error("Base component {0} (decimal) is not a valid member of the initial S_base pattern.")]
    InvalidBaseComponent(BigUint),

    /// Error indicating that the number of base components provided for composition
    /// is not a non-zero power of 2, which is required for hierarchical composition.
    #[error("Number of base components ({0}) must be a non-zero power of 2 (e.g., 1, 2, 4, 8...).")]
    InvalidComponentCount(usize),

    /// Error indicating that a value cannot be decomposed further because its half bit-width
    /// would be smaller than the base pattern's bit-width.
    #[error("Cannot decompose further: half N-bits ({half_n_bits}) is smaller than base N-bits ({base_n_bits}).")]
    DecompositionLimitReached { half_n_bits: usize, base_n_bits: usize },

    /// Error indicating that a pair of values provided to create a PairedEntity
    /// are not bitwise complements for the specified n_bits.
    #[error("Values {val1} (decimal) and {val2_complement} (decimal) are not N-bit complements for n_bits = {n_bits}. Their sum should be 2^{n_bits} - 1.")]
    NonComplementaryPair { val1: BigUint, val2_complement: BigUint, n_bits: usize },

    #[error("Cannot generate random member: S_base pattern is empty (should be caught by InitialPattern::new).")]
    EmptySBaseForRandomGeneration, // For random generation specifically
}