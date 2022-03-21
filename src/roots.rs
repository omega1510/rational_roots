/*
Find the rational roots of a polynomial.
By Hayaan Rizvi
*/

/*
Find the factors of a number.
@param n: the number to factor
@return: a vector of factors
*/
fn factor(n: i64) -> Vec<i64> {
    let mut factors = Vec::new();

    let n = n.abs();

    for i in 1..n + 1 {
        if n % i == 0 {
            factors.push(i);
            factors.push(-i)
        }
    }

    factors
}

/*
Evaluate a polynomial at a given point.
@param x: the point at which to evaluate
@param coeffs: the coefficients of the polynomial
@return: the value of the polynomial at x
*/
fn evaluate(x: f64, coeffs: &Vec<i64>) -> f64 {
    let mut result = 0.0;

    for i in 0..coeffs.len() {
        result += (coeffs[i] as f64) * x.powi(i as i32);
    }

    result
}

/*
Find the rational roots of a polynomial.
@param coeffs: the coefficients of the polynomial
@return: a vector of rational roots
*/
pub fn find_roots(coeffs: &Vec<i64>) -> Vec<f64> {
    let leading_factors = factor(coeffs[0]);
    let constant_factors = factor(coeffs[coeffs.len() - 1]);

    let mut possible_values = Vec::new();

    for term1 in leading_factors.iter() {
        for term2 in constant_factors.iter() {
            possible_values.push((term2 / term1) as f64);
        }
    }

    let mut roots = Vec::new();

    for value in possible_values.iter() {
        if evaluate(*value, coeffs) == 0.0 {
            if !roots.contains(value) {
                roots.push(*value);
            }
        }
    }

    roots
}
