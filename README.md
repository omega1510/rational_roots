# Rational Roots
Find the rational roots of a polynomial where all the coefficients are integers and the constant term is not zero.

## Usage
1. Import the `roots.rs` file:
```rust
mod roots;
```

2. Create a polynomial as a vector, where each value is a coefficient. For example, `3x^3 - 2x + 1` would be written as:
```rust
let polynomial: Vec<i64> = vec![3, 0, -2, 1];
```

3. Call the `find_roots()` function on the reference to the polynomial:
```rust
roots::find_roots(&polynomial)
```

The `find_roots()` function returns a `Vec<f64>` vector.
