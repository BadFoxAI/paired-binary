use wasm_bindgen::prelude::*;
use crate::{InitialPattern, Propagator, HierarchyError, PairedEntity};
use num_bigint::BigUint;
use std::collections::HashSet;
use std::str::FromStr;
use rand::RngCore; 

// --- Simple Seedable PRNG for WASM ---
struct SimpleSeededRng {
    seed: u32,
}

impl SimpleSeededRng {
    fn new(seed: u32) -> Self {
        SimpleSeededRng { seed: if seed == 0 { 1 } else { seed } } 
    }
}

impl RngCore for SimpleSeededRng {
    fn next_u32(&mut self) -> u32 {
        self.seed = self.seed.wrapping_mul(1103515245).wrapping_add(12345);
        self.seed
    }

    fn next_u64(&mut self) -> u64 {
        ((self.next_u32() as u64) << 32) | (self.next_u32() as u64)
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        for chunk in dest.chunks_mut(4) {
            let rand_val = self.next_u32();
            let bytes = rand_val.to_le_bytes(); 
            let len_to_copy = chunk.len().min(bytes.len());
            chunk[..len_to_copy].copy_from_slice(&bytes[..len_to_copy]);
            if chunk.len() > bytes.len() { // Zero out remaining bytes in the chunk if any
                for byte_val in chunk[bytes.len()..].iter_mut() {
                    *byte_val = 0;
                }
            }
        }
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand::Error> {
        self.fill_bytes(dest);
        Ok(())
    }
}
// --- End Simple PRNG ---


// Helper to convert Rust Result<T, HierarchyError> to JsValue Result<JsValue, JsValue>
// This helper is for cases where the Ok variant should be a general JsValue
fn to_js_result_generic<T, F>(rust_result: Result<T, HierarchyError>, success_converter: F) -> Result<JsValue, JsValue>
where
    F: FnOnce(T) -> Result<JsValue, JsValue>,
{
    match rust_result {
        Ok(val) => success_converter(val),
        Err(err) => Err(JsValue::from_str(&format!("HierarchyError: {:?}", err))),
    }
}

static mut GLOBAL_PROPAGATOR: Option<Propagator> = None;
static mut GLOBAL_RNG_SEED: u32 = 12345; 

#[wasm_bindgen]
pub fn setup_propagator(s_base_values_str: &str, n_base_bits: usize) -> Result<(), JsValue> {
    let mut s_base = HashSet::new();
    for val_str in s_base_values_str.split(',') {
        let val_trimmed = val_str.trim();
        if val_trimmed.is_empty() { continue; }
        match BigUint::from_str(val_trimmed) {
            Ok(b_val) => { s_base.insert(b_val); }
            Err(e) => return Err(JsValue::from_str(&format!("Invalid BigUint in s_base: '{}', error: {}", val_trimmed, e))),
        }
    }

    match InitialPattern::new(s_base, n_base_bits) {
        Ok(pattern) => {
            let propagator = Propagator::new(pattern);
            unsafe {
                GLOBAL_PROPAGATOR = Some(propagator);
            }
            Ok(())
        }
        Err(e) => Err(JsValue::from_str(&format!("Error creating InitialPattern: {:?}", e))),
    }
}

#[wasm_bindgen]
pub fn is_member(x_target_str: &str, n_target_bits: usize) -> Result<bool, JsValue> {
    let propagator = unsafe { GLOBAL_PROPAGATOR.as_ref().ok_or_else(|| JsValue::from_str("Propagator not initialized. Call setup_propagator first."))? };
    
    let x_target = BigUint::from_str(x_target_str)
        .map_err(|e| JsValue::from_str(&format!("Invalid BigUint string for x_target: {}", e)))?;
    
    match propagator.is_member(&x_target, n_target_bits) {
        Ok(is_mem) => Ok(is_mem),
        Err(e) => Err(JsValue::from_str(&format!("{:?}", e))),
    }
}

/// Decomposes an S_N member to its S_base components.
/// Returns a js_sys::Array of strings (decimal representation of BigUint components).
#[wasm_bindgen]
pub fn decompose_to_base(x_target_str: &str, n_target_bits: usize) -> Result<js_sys::Array, JsValue> {
    let propagator = unsafe { GLOBAL_PROPAGATOR.as_ref().ok_or_else(|| JsValue::from_str("Propagator not initialized."))? };

    let x_target = BigUint::from_str(x_target_str)
        .map_err(|e| JsValue::from_str(&format!("Invalid BigUint string for x_target: {}", e)))?;

    // Direct handling for this specific return type
    match propagator.decompose_to_base(&x_target, n_target_bits) {
        Ok(components_biguint) => {
            let js_array = js_sys::Array::new_with_length(components_biguint.len() as u32);
            for (i, comp) in components_biguint.iter().enumerate() {
                js_array.set(i as u32, JsValue::from_str(&comp.to_string()));
            }
            Ok(js_array) // Directly return js_sys::Array
        }
        Err(err) => Err(JsValue::from_str(&format!("HierarchyError: {:?}", err))),
    }
}

/// Composes an S_N member from an array of S_base component strings.
/// s_base_components_js_array: js_sys::Array of strings.
/// Returns a JS object { value: string, n_bits: number }.
#[wasm_bindgen]
pub fn compose_from_base(s_base_components_js_array: js_sys::Array) -> Result<JsValue, JsValue> {
    let propagator = unsafe { GLOBAL_PROPAGATOR.as_ref().ok_or_else(|| JsValue::from_str("Propagator not initialized."))? };

    let mut s_base_components_biguint: Vec<BigUint> = Vec::new();
    for i in 0..s_base_components_js_array.length() {
        let js_val = s_base_components_js_array.get(i);
        let comp_str = js_val.as_string().ok_or_else(|| JsValue::from_str("Component is not a string or is undefined"))?;
        let comp_biguint = BigUint::from_str(&comp_str)
            .map_err(|e| JsValue::from_str(&format!("Invalid BigUint string for component '{}': {}", comp_str, e)))?;
        s_base_components_biguint.push(comp_biguint);
    }
    
    // Using the generic helper here is fine as the return type is Result<JsValue, JsValue>
    to_js_result_generic(propagator.compose_from_base(&s_base_components_biguint), |(composed_val, composed_n_bits)| {
        let result_obj = js_sys::Object::new();
        // Using .map_err for the Reflect::set operations to convert potential JS exceptions into our Result's Err type
        js_sys::Reflect::set(&result_obj, &JsValue::from_str("value"), &JsValue::from_str(&composed_val.to_string()))
            .map_err(|e| JsValue::from_str(&format!("JS Reflect Error: {:?}", e)))?;
        js_sys::Reflect::set(&result_obj, &JsValue::from_str("n_bits"), &JsValue::from(composed_n_bits as u32))
            .map_err(|e| JsValue::from_str(&format!("JS Reflect Error: {:?}", e)))?;
        Ok(JsValue::from(result_obj))
    })
}

/// Generates a random S_N member.
/// Returns the decimal string representation of the BigUint.
#[wasm_bindgen]
pub fn generate_random_member(target_n_bits: usize, seed_offset: u32) -> Result<String, JsValue> {
    let propagator = unsafe { GLOBAL_PROPAGATOR.as_ref().ok_or_else(|| JsValue::from_str("Propagator not initialized."))? };
    
    let current_seed = unsafe { 
        GLOBAL_RNG_SEED = GLOBAL_RNG_SEED.wrapping_add(seed_offset); 
        GLOBAL_RNG_SEED 
    };
    let mut rng = SimpleSeededRng::new(current_seed); 

    match propagator.generate_random_s_n_member(target_n_bits, &mut rng) {
        Ok(val) => Ok(val.to_string()),
        Err(e) => Err(JsValue::from_str(&format!("{:?}", e))),
    }
}

/// Creates a PairedEntity and returns it as a JS object { x: string, x_prime: string, n_bits: number }.
#[wasm_bindgen]
pub fn create_paired_entity(x_str: &str, n_bits: usize) -> Result<JsValue, JsValue> {
    let x_val = BigUint::from_str(x_str)
        .map_err(|e| JsValue::from_str(&format!("Invalid BigUint string for x: {}", e)))?;
    
    // Using the generic helper here
    to_js_result_generic(PairedEntity::new(x_val, n_bits), |pe| {
        let result_obj = js_sys::Object::new();
        js_sys::Reflect::set(&result_obj, &JsValue::from_str("x"), &JsValue::from_str(&pe.x.to_string()))
             .map_err(|e| JsValue::from_str(&format!("JS Reflect Error: {:?}", e)))?;
        js_sys::Reflect::set(&result_obj, &JsValue::from_str("x_prime"), &JsValue::from_str(&pe.x_prime.to_string()))
             .map_err(|e| JsValue::from_str(&format!("JS Reflect Error: {:?}", e)))?;
        js_sys::Reflect::set(&result_obj, &JsValue::from_str("n_bits"), &JsValue::from(pe.n_bits as u32))
             .map_err(|e| JsValue::from_str(&format!("JS Reflect Error: {:?}", e)))?;
        Ok(JsValue::from(result_obj))
    })
}