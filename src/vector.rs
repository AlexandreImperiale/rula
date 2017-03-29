use std::convert::*;
use std::iter::*;
use std::vec::*;
use super::traits::*;

/// Creating new vector by copying input resource.
///
/// * `u` - Input vector to be copied.
///
/// # Examples
/// ```
/// use rula::vector::*;
///
/// let mut u = vec![1.0, 2.0, 3.0];
/// let mut v = copy(&u);
/// v[0] = 10.0;
/// assert!(u == [1.0, 2.0, 3.]);
/// assert!(v == [10., 2.0, 3.]);
/// ```
pub fn copy<T>(u: &Vec<T>) -> Vec<T>
    where T: Clone
{
    u.iter().cloned().collect()
}

/// Creating new vector by copying input resource filtered using input predicate.
///
/// * `u` - Input vector to be copied.
/// * `f` - Filter to be applied. Return true if element should be copied, false otherwise.
///
/// # Examples
/// ```
/// use rula::vector::*;
///
/// let mut u = vec![1.0, -2.0, 3.0];
/// let mut v = filtered_copy(&u, |&x| x > 0.);
/// assert!(v == [1.0, 3.0]);
/// ```
pub fn filtered_copy<T, P>(u: &Vec<T>, predicate: P) -> Vec<T>
    where T: Clone, P: FnMut(&T) -> bool
{
    u.iter().cloned().filter(predicate).collect()
}

/// Scaling input vector with single value.
///
/// * `u` - Input vector to be scaled.
/// * `a` - Scaling coefficient.
///
/// # Examples
/// ```
/// use rula::vector::*;
///
/// let mut v = vec![1.0, 2.0, 3.0];
/// scale(&mut v, 2.0);
/// assert!(v == [2., 4., 6.])
/// ```
pub fn scale<T>(u: &mut Vec<T>, a: T)
    where T: IsNumerical<T>
{
    for e in u
    {
        *e *= a;
    }
}

/// Set input vector to zero.
///
/// * `u` - Input vector to be zeroed.
///
/// # Examples
/// ```
/// use rula::vector::*;
///
/// let mut v = vec![1, 2, 3];
/// zero(&mut v);
/// assert!(v == [0, 0, 0])
/// ```
pub fn zero<T>(u: &mut Vec<T>)
    where T: IsNumerical<T>
{
    scale(u, T::zero())
}

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
    let uv_iter = u.iter().zip(v.iter());
    uv_iter.fold(U::zero(), |res, (&x, &y)| res + x * y)
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
    let uv_iter = u.iter().zip(v.iter());
    uv_iter.map(|(&x, &y)| a * x + b * y).collect()
}

/// Implementation of the mlt add operation, ie u <- u + a * v.
///
/// * `u` - first input vector bearing the result.
/// * `a` - coefficient applied to second input vector,
/// * `v` - second input vector.
///
/// # Examples
///
/// In this first example input types are identical.
///
/// ```
/// use rula::vector::*;
///
/// let mut u = vec![1, 2];
/// let v = vec![2, 9, 1];
/// mlt_add(&mut u, 2, &v);
/// assert!(u == [5, 20]);
/// ```
///
/// In this second example, input types are different.
///
/// ```
/// use rula::vector::*;
///
/// let mut u = vec![1.1, 2.0];
/// let v = vec![2, 9, 1];
/// mlt_add(&mut u, 2, &v);
/// assert!(u == [5.1, 20.0]);
/// ```
pub fn mlt_add<U, A, V>(u: &mut Vec<U>, a: A, v: &Vec<V>)
    where U: IsNumerical<U>, A: IsNumerical<A> + Into<U>, V: IsNumerical<V> + Into<U>
{
    for (eu, ev) in u.iter_mut().zip(v.iter())
    {
        *eu += a.into() * (*ev).into();
    }
}
