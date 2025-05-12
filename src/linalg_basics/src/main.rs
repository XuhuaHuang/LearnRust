/**
 * @file: main.rs
 * @brief:  This project demonstrates basic linear algebra operations
 * @author: Xuhua Huang
 * @date: April 18, 2025
 * @version: 1.0
 */
use nalgebra::{DMatrix, LU, Vector2, Vector3};

fn main() {
    let v = Vector2::new(3.0, 4.0);
    let unit_v = v.normalize();

    println!("Original: {:?}", v);
    println!("Normalized: {:?}", unit_v);
    println!("Length of normalized vector: {}", unit_v.norm()); // Will print 1.0
    println!("Length of original vector: {}", v.norm()); // Will print 5.0

    // Define two 3D vectors
    let v1 = Vector3::new(1.0, 2.0, 3.0);
    let v2 = Vector3::new(4.0, 5.0, 6.0);

    // Vector addition
    let sum = v1 + v2;

    // Vector subtraction
    let diff = v1 - v2;

    // Scalar multiplication
    let scaled = 2.0 * v1;

    // Dot product
    let dot = v1.dot(&v2);

    // Cross product
    let cross = v1.cross(&v2);

    // Magnitude (Euclidean norm)
    let magnitude = v1.norm();

    // Normalization (unit vector)
    let normalized = v1.normalize();

    // Print results
    println!("Sum: {}", sum);
    println!("Difference: {}", diff);
    println!("Scaled: {}", scaled);
    println!("Dot product: {}", dot);
    println!("Cross product: {}", cross);
    println!("Magnitude: {}", magnitude);
    println!("Normalized: {}", normalized);

    // Create a dynamic 3x3 matrix
    let m = DMatrix::from_row_slice(3, 3, &[2.0, -1.0, 0.0, -1.0, 2.0, -1.0, 0.0, -1.0, 2.0]);

    // Perform LU decomposition
    let lu = LU::new(m.clone());

    // Solve a linear system Ax = b
    let b = DMatrix::from_column_slice(3, 1, &[1.0, 0.0, 1.0]);
    let x = lu.solve(&b).expect("Matrix is singular");

    println!("Solution x:\n{}", x);

    // Verify A * x == b
    println!("A * x = \n{}", &m * &x);
}
