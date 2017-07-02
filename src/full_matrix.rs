use super::traits::*;
use std::iter::*;

///////////////////////////////////////////////////////////////////////////////////////////////////
///////////////////////////////////////////////////////////////////////////////////////////////////
/// Definition of full matrix type.
///////////////////////////////////////////////////////////////////////////////////////////////////
///////////////////////////////////////////////////////////////////////////////////////////////////

/// Definition of full matrix type.
/// The ordering by default is row-major.
pub struct FullMatrix<T> where T: IsField<T> {
    /// Associated data.
    data: Vec<T>,
    /// Associated sizes.
    pub nrow: usize, pub ncol: usize,
}

/// Definition of square full matrix.

///////////////////////////////////////////////////////////////////////////////////////////////////
///////////////////////////////////////////////////////////////////////////////////////////////////
/// Definition of iterators.
///////////////////////////////////////////////////////////////////////////////////////////////////
///////////////////////////////////////////////////////////////////////////////////////////////////

/// Definition of row iterators.
pub struct RowIter<'a, T> where T: IsField<T> + 'a {
    /// Associated reference to underlying full matrix.
    matrix: &'a FullMatrix<T>,
    /// Associated column and row index.
    i: usize, j: usize,
}

/// Definition of column iterators.
pub struct ColIter<'a, T> where T: IsField<T> + 'a {
    /// Associated reference to underlying full matrix.
    matrix: &'a FullMatrix<T>,
    /// Associated column and row index.
    i: usize, j: usize,
}

///////////////////////////////////////////////////////////////////////////////////////////////////
///////////////////////////////////////////////////////////////////////////////////////////////////
/// Implementation of iterators.
///////////////////////////////////////////////////////////////////////////////////////////////////
///////////////////////////////////////////////////////////////////////////////////////////////////

/// Implementation of column iterator.
impl<'a, T> Iterator for ColIter<'a, T> where T: IsField<T> {
    /// Definition of item types.
    type Item = &'a T;
    /// Implementation of the next() method.
    fn next(&mut self) -> Option<&'a T> {
        if self.i < self.matrix.nrow {
            let value : &'a T = self.matrix.get(self.i, self.j);
            self.i += 1;
            return Some(value);
        } else {
            None
        }
    }
}

/// Implementation of row iterator.
impl<'a, T> Iterator for RowIter<'a, T> where T: IsField<T> {
    /// Definition of item types.
    type Item = &'a T;
    /// Implementation of the next() method.
    fn next(&mut self) -> Option<&'a T> {
        if self.j < self.matrix.ncol {
            let value : &'a T = self.matrix.get(self.i, self.j);
            self.j += 1;
            return Some(value);
        } else {
            None
        }
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////
///////////////////////////////////////////////////////////////////////////////////////////////////
/// Implementation of full matrices.
///////////////////////////////////////////////////////////////////////////////////////////////////
///////////////////////////////////////////////////////////////////////////////////////////////////

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
    /// assert_eq!(*m.get(0, 1), 0.);
    /// ```
    pub fn get(&self, i: usize, j: usize) -> &T
    {
        &self.data[i * self.ncol + j]
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
    pub fn iter_over_row<'a>(&'a self, i: usize) -> RowIter<'a, T>
    {
        RowIter { matrix: &self, i: i, j: 0 }
    }

    /// Accessing iterator over a column.
    ///
    /// * `j` - column index.
    ///
    /// # Examples
    /// ```
    /// use rula::full_matrix::*;
    ///
    /// let m : FullMatrix<f64> = FullMatrix::zero(1, 2);
    /// for v in m.iter_over_column(0)
    /// {
    ///     assert_eq!(*v, 0.);
    /// }
    /// ```
    pub fn iter_over_column<'a>(&'a self, j: usize) -> ColIter<'a, T>
    {
        ColIter { matrix: &self, i: 0, j: j }
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
