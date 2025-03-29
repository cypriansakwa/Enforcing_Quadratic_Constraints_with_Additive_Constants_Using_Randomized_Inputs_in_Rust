use ndarray::{array};
use rand::Rng;

// Function to generate random x and y
fn generate_xy() -> (i32, i32) {
    let mut rng = rand::rng();
    let x = rng.random_range(1..=1000);
    let y = rng.random_range(1..=1000);
    (x, y)
}

fn main() {
    // Define the matrices
    let l = array![[0, 0, 1, 0]];
    let r = array![[0, 0, 0, 1]];
    let o = array![[-2, 1, 0, 0]];

    // Generate random values for x and y
    let (x, y) = generate_xy();
    let z = x * y + 2; // Compute z for witness vector

    // Define the witness vector
    let a = array![1, z, x, y];

    // Perform matrix operations
    let lhs = o.dot(&a);
    let rhs = (l.dot(&a) * r.dot(&a))[0]; // Ensuring scalar multiplication

    // Check the constraint
    assert_eq!(lhs[0], rhs, "Result contains an inequality");

    println!("Constraint satisfied with x = {}, y = {}, z = {}", x, y, z);
}
