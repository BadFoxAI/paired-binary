use num_bigint::BigUint;
use num_traits::One; // Zero is not used in this file
use rand::seq::SliceRandom;
use rand::Rng;
use crate::pattern::InitialPattern;
use crate::error::HierarchyError;

/// `Propagator` is responsible for applying the hierarchical propagation rules
/// based on a given `InitialPattern` (S_base).
/// It determines membership in higher-level selected sets (S_N),
/// decomposes S_N members into their S_base components, and composes
/// S_N members from S_base components.
#[derive(Debug, Clone)]
pub struct Propagator {
    initial_pattern: InitialPattern,
}

impl Propagator {
    /// Creates a new `Propagator` with a specific `InitialPattern`.
    pub fn new(initial_pattern: InitialPattern) -> Self {
        Self { initial_pattern }
    }

    /// Returns a reference to the `InitialPattern` used by this propagator.
    pub fn initial_pattern(&self) -> &InitialPattern {
        &self.initial_pattern
    }

    /// Checks if `target_n_bits` is a valid hierarchical level that can be derived
    /// from `self.initial_pattern.n_base_bits` by successive doublings.
    /// A valid level means `target_n_bits = n_base_bits * 2^k` for some integer `k >= 0`.
    fn is_valid_hierarchical_level(&self, target_n_bits: usize) -> bool {
        let base_n_bits = self.initial_pattern.n_base_bits; 
        if target_n_bits < base_n_bits {
            return false;
        }
        if target_n_bits == base_n_bits {
            return true;
        }
        
        if base_n_bits == 0 { return false; } 
        if target_n_bits % base_n_bits != 0 {
            return false;
        }
        let factor = target_n_bits / base_n_bits;
        factor.is_power_of_two()
    }

    /// Checks if a given X-value (`x_target`) is a member of the selected set S_N
    /// at `n_target_bits`, according to the propagation rules and the `InitialPattern`.
    pub fn is_member(&self, x_target: &BigUint, n_target_bits: usize) -> Result<bool, HierarchyError> {
        if n_target_bits == 0 {
             return Err(HierarchyError::InvalidHierarchicalLevel { 
                target_n_bits: n_target_bits, // Corrected: field_name: variable_value
                base_n_bits: self.initial_pattern.n_base_bits 
            });
        }
        
        let limit_exclusive = BigUint::one() << n_target_bits;
        if *x_target >= limit_exclusive {
            return Err(HierarchyError::ValueTooLargeForNBits {
                value: x_target.clone(),
                n_bits: n_target_bits,
            });
        }

        if !self.is_valid_hierarchical_level(n_target_bits) { // This was error line 54/63 previously
            return Err(HierarchyError::InvalidHierarchicalLevel { 
                target_n_bits: n_target_bits, // Corrected: field_name: variable_value
                base_n_bits: self.initial_pattern.n_base_bits 
            });
        }
        // Note: The compiler reported error on line 69 as well for this.
        // The above is the only place it's constructed in is_member.
        // If line 69 is still an error, it must be in a different function or a test.
        // However, the function seems complete as is. Let's assume this fixes both.
        
        Ok(self._is_member_recursive(x_target, n_target_bits))
    }
    
    fn _is_member_recursive(&self, x_current: &BigUint, n_current_bits: usize) -> bool {
        if n_current_bits == self.initial_pattern.n_base_bits {
            return self.initial_pattern.s_base_values.contains(x_current);
        }

        let n_half_bits = n_current_bits / 2;

        let one = BigUint::one();
        let mask = (&one << n_half_bits) - &one;
        
        let h_upper = x_current >> n_half_bits;
        let h_lower = x_current & &mask;

        self._is_member_recursive(&h_upper, n_half_bits) && 
        self._is_member_recursive(&h_lower, n_half_bits)
    }

    /// Decomposes a given X-value (`x_target`), known to be a member of S_N,
    /// into its constituent S_base components.
    pub fn decompose_to_base(&self, x_target: &BigUint, n_target_bits: usize) -> Result<Vec<BigUint>, HierarchyError> {
        if !self.is_member(x_target, n_target_bits)? {
            return Err(HierarchyError::NotAMember(x_target.clone()));
        }

        let mut components = Vec::new();
        self._decompose_recursive_collect(x_target, n_target_bits, &mut components);
        Ok(components)
    }

    fn _decompose_recursive_collect(&self, current_x: &BigUint, current_n_bits: usize, components: &mut Vec<BigUint>) {
        if current_n_bits == self.initial_pattern.n_base_bits {
            components.push(current_x.clone());
            return;
        }

        let n_half_bits = current_n_bits / 2;
        
        let one = BigUint::one();
        let mask = (&one << n_half_bits) - &one;
        let h_upper = current_x >> n_half_bits;
        let h_lower = current_x & &mask;

        self._decompose_recursive_collect(&h_upper, n_half_bits, components);
        self._decompose_recursive_collect(&h_lower, n_half_bits, components);
    }

    /// Composes an S_N member from a sequence of its S_base components.
    pub fn compose_from_base(&self, s_base_components: &[BigUint]) -> Result<(BigUint, usize), HierarchyError> {
        let num_components = s_base_components.len();
        if num_components == 0 || !num_components.is_power_of_two() {
            return Err(HierarchyError::InvalidComponentCount(s_base_components.len()));
        }

        let one = BigUint::one();
        let limit_exclusive_base = &one << self.initial_pattern.n_base_bits;

        for comp in s_base_components {
            if !self.initial_pattern.s_base_values.contains(comp) {
                return Err(HierarchyError::InvalidBaseComponent(comp.clone()));
            }
            if *comp >= limit_exclusive_base {
                 let max_val = limit_exclusive_base - &one;
                return Err(HierarchyError::ValueExceedsNBaseBits {
                    value: comp.clone(),
                    n_bits: self.initial_pattern.n_base_bits,
                    max_val,
                });
            }
        }
        
        Ok(self._compose_recursive(s_base_components))
    }

    fn _compose_recursive(&self, components_slice: &[BigUint]) -> (BigUint, usize) {
        if components_slice.len() == 1 {
            return (components_slice[0].clone(), self.initial_pattern.n_base_bits);
        }

        let mid = components_slice.len() / 2;
        let (upper_half_val, upper_n_bits) = self._compose_recursive(&components_slice[0..mid]);
        let (lower_half_val, _lower_n_bits) = self._compose_recursive(&components_slice[mid..]);
        
        let composed_n_bits = upper_n_bits * 2; 
        let composed_val = (upper_half_val << upper_n_bits) | lower_half_val;
        
        (composed_val, composed_n_bits)
    }

    /// Generates a random member of the selected set S_N at `target_n_bits`.
    pub fn generate_random_s_n_member<R: Rng + ?Sized>(&self, target_n_bits: usize, rng: &mut R) -> Result<BigUint, HierarchyError> {
        if !self.is_valid_hierarchical_level(target_n_bits) {
            return Err(HierarchyError::InvalidHierarchicalLevel {
                target_n_bits: target_n_bits, // Corrected: field_name: variable_value
                base_n_bits: self.initial_pattern.n_base_bits,
            });
        }
        if self.initial_pattern.s_base_values.is_empty() {
            return Err(HierarchyError::EmptySBaseForRandomGeneration);
        }

        Ok(self._generate_random_recursive(target_n_bits, rng))
    }

    fn _generate_random_recursive<R: Rng + ?Sized>(&self, current_n_bits: usize, rng: &mut R) -> BigUint {
        if current_n_bits == self.initial_pattern.n_base_bits {
            let s_base_vec: Vec<&BigUint> = self.initial_pattern.s_base_values.iter().collect();
            return (*s_base_vec.choose(rng).expect("S_base_values cannot be empty due to earlier check")).clone();
        }

        let n_half_bits = current_n_bits / 2;
        let h_upper = self._generate_random_recursive(n_half_bits, rng);
        let h_lower = self._generate_random_recursive(n_half_bits, rng);

        (h_upper << n_half_bits) | h_lower
    }
}