/* tslint:disable */
/* eslint-disable */
export function setup_propagator(s_base_values_str: string, n_base_bits: number): void;
export function is_member(x_target_str: string, n_target_bits: number): boolean;
/**
 * Decomposes an S_N member to its S_base components.
 * Returns a js_sys::Array of strings (decimal representation of BigUint components).
 */
export function decompose_to_base(x_target_str: string, n_target_bits: number): Array<any>;
/**
 * Composes an S_N member from an array of S_base component strings.
 * s_base_components_js_array: js_sys::Array of strings.
 * Returns a JS object { value: string, n_bits: number }.
 */
export function compose_from_base(s_base_components_js_array: Array<any>): any;
/**
 * Generates a random S_N member.
 * Returns the decimal string representation of the BigUint.
 */
export function generate_random_member(target_n_bits: number, seed_offset: number): string;
/**
 * Creates a PairedEntity and returns it as a JS object { x: string, x_prime: string, n_bits: number }.
 */
export function create_paired_entity(x_str: string, n_bits: number): any;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly setup_propagator: (a: number, b: number, c: number) => [number, number];
  readonly is_member: (a: number, b: number, c: number) => [number, number, number];
  readonly decompose_to_base: (a: number, b: number, c: number) => [number, number, number];
  readonly compose_from_base: (a: any) => [number, number, number];
  readonly generate_random_member: (a: number, b: number) => [number, number, number, number];
  readonly create_paired_entity: (a: number, b: number, c: number) => [number, number, number];
  readonly __wbindgen_exn_store: (a: number) => void;
  readonly __externref_table_alloc: () => number;
  readonly __wbindgen_export_2: WebAssembly.Table;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __externref_table_dealloc: (a: number) => void;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
  readonly __wbindgen_start: () => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
*
* @returns {InitOutput}
*/
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
*
* @returns {Promise<InitOutput>}
*/
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
