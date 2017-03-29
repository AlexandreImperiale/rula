use super::traits::*;

/// Definition of full matric type.
/// The ordering by default is row-major.
pub struct FullMatrix<T> where T: IsNumerical<T> {
    /// Associated data.
    data: Vec<T>,
    /// Associated sizes.
    pub nrow: usize, pub ncol: usize,
}

impl<T> FullMatrix<T> where T: IsNumerical<T> {

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

    /// Creating zero matrix as full matrix.
    ///
    /// * `nrow` - number of rows.
    /// * `ncol` - number of columns.
    ///
    /// # Examples
    /// ```
    ///
    /// ```
    pub fn zero(nrow: usize, ncol: usize) -> Self
    {
        Self { data: vec![T::zero(); nrow * ncol], nrow: nrow, ncol: ncol }
    }
}
