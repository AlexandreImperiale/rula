use super::traits::*;
use std::iter::*;
use std::slice::*;

/// Definition of full matric type.
/// The ordering by default is row-major.
pub struct FullMatrix<T> where T: IsField<T> {
    /// Associated data.
    data: Vec<T>,
    /// Associated sizes.
    pub nrow: usize, pub ncol: usize,
}

impl<T> FullMatrix<T> where T: IsField<T> {

    /// Accessing matrix value from its row and column index.
    ///
    /// * `i` - row index.
    /// * `j` - column index.
    ///
    /// # Examples
    /// ```
    /// use rula::full_matrix::*;
    ///
    /// let m : FullMatrix<f64> = FullMatrix::zero(1, 2);
    /// assert_eq!(m.get(0, 1), 0.);
    /// ```
    pub fn get(&self, i: usize, j: usize) -> T
    {
        self.data[i * self.ncol + j]
    }

    /// Accessing iterator over a row.
    ///
    /// * `i` - row index.
    ///
    /// # Examples
    /// ```
    /// use rula::full_matrix::*;
    ///
    /// let m : FullMatrix<f64> = FullMatrix::zero(1, 2);
    /// for v in m.iter_over_row(0)
    /// {
    ///     assert_eq!(*v, 0.);
    /// }
    /// ```
    pub fn iter_over_row<'a>(&'a self, i: usize) -> Take<Skip<Iter<'a, T>>>
    {
        self.data.iter().skip(i * self.ncol).take(self.ncol)
    }

    /// Creating zero matrix as full matrix.
    ///
    /// * `nrow` - number of rows.
    /// * `ncol` - number of columns.
    ///
    /// # Examples
    /// ```
    /// use rula::full_matrix::*;
    ///
    /// let m : FullMatrix<f64> = FullMatrix::zero(1, 2);
    /// for i in 0..m.nrow
    /// {
    ///     for v in m.iter_over_row(i)
    ///     {
    ///         assert_eq!(*v, 0.);
    ///     }
    /// }
    /// ```
    pub fn zero(nrow: usize, ncol: usize) -> Self
    {
        Self { data: vec![T::zero(); nrow * ncol], nrow: nrow, ncol: ncol }
    }
}
