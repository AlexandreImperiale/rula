use std::cmp::*;
use std::convert::*;
use std::vec::*;
use super::traits::*;

/// Computing dot product between two vectors. The operations is performed up to the smallest range
/// shared by the two input vectors. Input vector are assumed to be of the same type.
///
/// * `u` - First vector.
/// * `v` - Second vector.
///
/// # Examples
///
/// This first example shows the use of dot operation on vector of integers with the same size.
///
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
/// The dot operation being generic, it can also be applied to vector of floats. If one of a vector
/// is larger the extra values are omitted in the computation of the dot product.
///
/// ```
/// use rula::vector::*;
///
/// let v = vec![1.0, 2.0, 3.0, 4.0];
/// let u = vec![0.0, 6.0, 2.0];
/// assert!(dot(&u, &v) == dot(&v, &u));
/// assert!(dot(&u, &v) == 18.0);
/// ```
pub fn dot<U>(u: &Vec<U>, v: &Vec<U>) -> U
    where U: IsNumerical<U>
{
    u.iter().zip(v.iter()).fold(U::zero(), |res, (&x, &y)| res + x * y.into())
}

/// The operations is performed up to the smallest range shared by the two input vectors.
/// Input vectors may not be of the same type, however they need to satisfy a conversion trait into
/// the type of the value bearing the result.
///
/// * `d` - Value bearing the result of the dot product.
/// * `u` - First vector.
/// * `v` - Second vector.
///
/// # Examples
///
/// In this first example, the place to store the result has the same type as the input vectors;
///
/// ```
/// use rula::vector::*;
///
/// let v = vec![1.0, 2.0, 3.0];
/// let u = vec![0.0, 6.0, 2.0];
/// let mut d : f64 = 0.;
/// dot_in(&mut d, &u, &v);
/// assert!(dot(&u, &v) == d);
/// ```
///
/// In this second example, the place to store the result is larger that the size of the type of
/// the input vectors values.
///
/// ```
/// use rula::vector::*;
///
/// let v = vec![1, 2, 3];
/// let u = vec![0, 6, 2];
/// let mut d : f64 = 0.;
/// dot_in(&mut d, &u, &v);
/// assert!(dot(&u, &v) == d as i32);
/// ```
pub fn dot_in<D, U, V>(d: &mut D, u: &Vec<U>, v: &Vec<V>)
    where D: IsNumerical<D>, U: IsNumerical<U> + Into<D>, V: IsNumerical<V> + Into<D>
{
    *d = D::zero();
    u.iter().zip(v.iter()).fold((), |(), (&x, &y)| *d += x.into() * y.into())
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
pub fn square_norm<T>(u: &Vec<T>) -> T
    where T: IsNumerical<T>
{
    dot(u, u)
}

/// Computing norm of a vector.
///
/// * `u` - input vector.
///
/// # Examples
/// ```
/// use rula::vector::*;
///
/// let u = vec![1, 2, 3];
/// assert!(norm(&u) == (dot(&u, &u) as f64).sqrt());
/// ```
pub fn norm<T>(u: &Vec<T>) -> f64
    where T: IsNumerical<T> + Into<f64>
{
    (square_norm(u).into()).sqrt()
}

/// Creating vector obtained by linear combination of two input vectors, i.e. w <- a * u + b * v.
/// Types of vector elements are assumed to be identical.
///
/// * `a` - first coefficient in linear combination, applied to first input vector
/// * `u` - first input vector,
/// * `b` - second coefficient in linear combination, applied to second input vector,
/// * `v` - second input vector.
///
/// # Examples
/// ```
/// use rula::vector::*;
///
/// let u = vec![1, 2];
/// let v = vec![2, 9, 1];
/// let w = lin_com(2, &u, 1, &v);
/// assert!(w.len() == 2);
/// assert!(w == [4, 13]);
/// ```
pub fn lin_com<T>(a: T, u: &Vec<T>, b: T, v: &Vec<T>) -> Vec<T>
    where T: IsNumerical<T>
{
    let sz = min(u.len(), v.len());
    let mut w = Vec::with_capacity(sz);
    for i in 0..sz
    {
        w.push(a * u[i] + b * v[i]);
    }
    return w;
}
