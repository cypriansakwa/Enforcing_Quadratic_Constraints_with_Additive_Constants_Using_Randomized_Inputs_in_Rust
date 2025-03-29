# Enforcing Quadratic Constraints with Additive Constants Using Randomized Inputs in Rust

This Rust program demonstrates how to enforce quadratic constraints with additive constants using randomized inputs. It verifies the constraint:

$z = xy + 2$

by representing it in matrix form and checking its validity.

## Features
- Generates random values for `x` and `y`
- Computes `z = xy + 2`
- Uses matrix multiplication to verify the constraint
- Ensures correctness using assertions

## Requirements
- Rust (latest stable version)
- [`ndarray`](https://crates.io/crates/ndarray) for matrix operations
- [`rand`](https://crates.io/crates/rand) for generating random values

## Installation
Clone the repository and navigate to the project directory:

```sh
git clone https://github.com/cypriansakwa/Enforcing_Quadratic_Constraints_with_Additive_Constants_Using_Randomized_Inputs_in_Rust.git
cd Enforcing_Quadratic_Constraints_with_Additive_Constants_Using_Randomized_Inputs_in_Rust
```
## Add dependencies by modifying Cargo.toml:
```toml
[dependencies]
ndarray = "0.16.1"
rand = "0.9.0"
```
## Usage
Run the program using:
```sh
cargo run
```
## Example output:
Constraint satisfied with x = 890, y = 882, z = 784982
