# Paired Binary: A Framework for Information

This repository contains the implementation for (["A Framework for Information Based on Complementary Pairing and Hierarchical Structures"](https://zenodo.org/records/15543486)) exploring binary complements and compositional patterning. The core idea revolves around N-bit Paired Entities (an N-bit binary value `X` and its bitwise complement `X’`) and how they form structured, self-similar hierarchies based on a compositional rule.

This project aims to provide the foundational tools to explore, generate, and analyze these structures, with potential applications in understanding information, novel encoding schemes, or models of emergent complexity.

## Project Overview

The framework proposes that an initial pattern, defined at a low bit-width (e.g., N=3), can propagate to higher bit-widths (e.g., N=6, N=12), selecting specific subsets of Paired Entities at each level. This creates a self-similar, hierarchical structure. For instance, an N=3 pattern selecting 3 `X`-values might lead to `3*3=9` selected Paired Entities at N=6, and this N=6 subset could, in turn, select `9*9=81` Paired Entities at N=12.

This implementation provides the core logic to:
*   Define initial patterns (`S_base`).
*   Propagate these patterns to higher N-levels.
*   Check for membership within these generated sets (`S_N`).
*   Decompose `S_N` members into their `S_base` constituents.
*   Compose `S_N` members from `S_base` constituents.
*   Generate random `S_N` members.
*   Work with N-bit Paired Entities (`X`, `X'`).

## Key Concepts

*   **N-bit Paired Entity `X(X’)`:** An N-bit binary value `X` always linked to its bitwise complement `X’`. A key property is that their sum `X + X′` equals `2^N − 1` (a binary number of N ones).
*   **Canonical Set:** To avoid duplicates (like `X(X')` vs `X'(X)`), a unique set of Paired Entities is defined for each N, typically by choosing `X` as the numerically smaller value of the pair.
*   **Initial Pattern (`S_base`):** A user-defined set of `X`-values at a base N-level (e.g., `S3 = {0, 1, 2}` for `N_base=3`) that acts as the seed for the entire hierarchical structure.
*   **Propagation Rule:** The core rule determining how patterns from a lower level (`N/2`) dictate the selected entities at a higher level (`N`). An `X`-value `X_N` is selected into the pattern `S_N` if, and only if, both its upper half (`H_upper`) and lower half (`H_lower`) components are members of the selected pattern `S_{N/2}` from the level below.
*   **Selected Sets (`S_N`):** The specific, sparse subset of Paired Entities chosen by the propagation rule at each N-level. These sets possess a high degree of structure due to their generative origin.
*   **Composition and Decomposition:** The framework allows for the construction (composition) of higher-level `S_N` members from their `S_base` components and, conversely, the deconstruction (decomposition) of an `S_N` member back into its unique `S_base` "path" or "genetic code."

## Features of this Implementation

*   **Arbitrary Precision:** Utilizes `num-bigint` for handling N-bit values, allowing for exploration of very large N.
*   **Core Logic Encapsulation:** Provides `InitialPattern` and `Propagator` types to manage and apply the framework's rules.
*   **Key Operations:** Includes functions for membership testing, decomposition, composition, and random member generation within the `S_N` sets.
*   **Paired Entity Representation:** Offers a `PairedEntity` type for working with `X(X')` pairs.
*   **Comprehensive Error Handling:** Employs custom error types for clarity.
*   **WebAssembly (WASM) Compatibility:** Designed with WASM in mind, including `wasm-bindgen` wrappers to allow the core Rust logic to be used in web applications.

## Getting Started

### Prerequisites

*   **Rust:** Ensure you have the Rust programming language toolchain installed. You can get it from [rustup.rs](https://rustup.rs/).
*   **`wasm-pack` (for WebAssembly):** If you intend to build for or use the WebAssembly version, install `wasm-pack` by following the instructions on the [official `wasm-pack` site](https://rustwasm.github.io/wasm-pack/installer/).

### Building and Running

1.  **Clone the Repository:**
    ```bash
    git clone <your-repo-url>
    cd paired_binary
    ```
2.  **Build the Rust Library & Run Tests:**
    To build the native Rust library and execute its unit tests (recommended to verify correctness):
    ```bash
    cargo build
    cargo test
    ```
3.  **Build for WebAssembly (WASM):**
    To compile the library into a WebAssembly module for use in web browsers:
    ```bash
    wasm-pack build --target web
    ```
    This command creates a `pkg` directory in your project root. This directory contains the `.wasm` file, the JavaScript glue code, and a `package.json`, making it ready for web integration.

### Using the WASM Module in a Web Page

An example `index.html` is provided in this repository. You will typically need to serve the `index.html` and the `pkg` directory via a local HTTP server due to browser security policies for loading WASM modules. The JavaScript in `index.html` demonstrates how to import and call the exported WASM functions.

## Structure of the Code

The Rust source code is organized as follows:

*   `src/error.rs`: Defines custom error types used throughout the library.
*   `src/pattern.rs`: Contains the definition and logic for `InitialPattern` (`S_base`).
*   `src/entity.rs`: Defines the `PairedEntity` struct for `X(X')` pairs.
*   `src/propagator.rs`: Implements the `Propagator` which holds the core logic for applying the propagation rules, checking membership, decomposing, composing, and generating random members.
*   `src/wasm_api.rs`: Provides the `#[wasm_bindgen]` annotated functions that serve as the interface between Rust and JavaScript when compiled to WASM.
*   `src/lib.rs`: The crate root, organizing and re-exporting the public API of the library.
