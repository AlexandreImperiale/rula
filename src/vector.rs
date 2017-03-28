use std::vec::*;
use super::traits::*;

/// Computing dot product between two vectors. The operations is performed up to the smallest range
/// shared by the two input vectors.
///
/// * `u` - First vector.
/// * `v` - Second vector.
///
/// # Examples
/// ```
/// use rula::vector::*;
///
/// let v = vec![1, 2, 3];
/// let u = vec![0, 6, 2];
/// assert!(dot(&u, &v) == dot(&v, &u));
/// assert!(dot(&u, &v) == 18);
/// assert!(u == [0, 6, 2])
/// ```
///
/// ```
/// use rula::vector::*;
///
/// let v = vec![1.0, 2.0, 3.0, 4.0];
/// let u = vec![0.0, 6.0, 2.0];
/// assert!(dot(&u, &v) == dot(&v, &u));
/// assert!(dot(&u, &v) == 18.0);
/// assert!(u == [0.0, 6.0, 2.0])
/// ```
pub fn dot<T>(u: &Vec<T>, v: &Vec<T>) -> T where T: IsNumerical<T>
{
    u.iter().zip(v.iter()).fold(T::zero(), |res, (&x, &y)| res + x * y )
}

/// Computing square norm of a vector.
///
/// * `u` - input vector.
///
/// # Examples
/// ```
/// use rula::vector::*;
///
/// let u = vec![1.0, 2.0, 3.0];
/// assert!(square_norm(&u) == 14.0);
/// ```
pub fn square_norm<T>(u: &Vec<T>) -> T where T: IsNumerical<T> { dot(u, u) }

/// Computing norm of a vector.
///
/// * `u` - input vector.
///
/// # Examples
/// ```
/// use rula::vector::*;
///
/// let u = vec![1.0, 2.0, 3.0];
/// assert!(norm(&u) == (14.0 as f64).sqrt());
/// ```
pub fn norm(u: &Vec<f64>) -> f64 { square_norm(u).sqrt() }
