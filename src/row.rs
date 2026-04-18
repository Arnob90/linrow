use derive_more::{Deref, DerefMut};
use std::fmt::Display;
use std::ops::{AddAssign, DivAssign, Mul, MulAssign, SubAssign};
/// Represents a single row in a matrix.
///
/// A `Row` is essentially a wrapper around a `Vec<f64>`, providing
/// convenient methods and operator overloads for row-specific operations
/// like scalar multiplication, addition, and subtraction.
///
/// It derives `Deref` and `DerefMut` to allow direct access to the
/// underlying `Vec<f64>` methods.
///
/// # Examples
///
/// ```
/// use linrow::row::Row;
///
/// let row = Row::new(vec![1.0, 2.0, 3.0]);
/// assert_eq!(row[0], 1.0);
/// ```
#[derive(Clone, Debug, Deref, DerefMut)]
pub struct Row {
    pub row_elems: Vec<f64>,
}
impl Row {
    /// Creates a new `Row` from a `Vec<f64>`.
    ///
    /// # Arguments
    ///
    /// * `columns` - A `Vec<f64>` representing the elements of the row.
    ///
    /// # Examples
    ///
    /// ```
    /// use linrow::row::Row;
    ///
    /// let row = Row::new(vec![1.0, 2.0, 3.0]);
    /// assert_eq!(row.row_elems, vec![1.0, 2.0, 3.0]);
    /// ```
    pub fn new(columns: Vec<f64>) -> Self {
        Row { row_elems: columns }
    }
}
impl AddAssign<&Row> for Row {
    /// Performs in-place addition of another `Row` to this `Row`.
    ///
    /// This operation adds corresponding elements of the `rhs` row to `self`.
    /// Both rows must have the same number of columns.
    ///
    /// # Arguments
    ///
    /// * `rhs` - A reference to the `Row` to add.
    ///
    /// # Panics
    ///
    /// Panics if the number of columns in `self` and `rhs` are not equal.
    ///
    /// # Examples
    ///
    /// ```
    /// use linrow::row::Row;
    ///
    /// let mut r1 = Row::new(vec![1.0, 2.0, 3.0]);
    /// let r2 = Row::new(vec![4.0, 5.0, 6.0]);
    /// r1 += &r2;
    /// assert_eq!(r1, Row::new(vec![5.0, 7.0, 9.0]));
    /// ```
    fn add_assign(&mut self, rhs: &Row) {
        for i in 0..self.row_elems.len() {
            self.row_elems[i] += rhs.row_elems[i];
        }
    }
}
impl AddAssign<Row> for Row {
    /// Performs in-place addition of another `Row` (by value) to this `Row`.
    ///
    /// This is a convenience implementation that delegates to `AddAssign<&Row>`.
    ///
    /// # Arguments
    ///
    /// * `rhs` - The `Row` to add.
    ///
    /// # Panics
    ///
    /// Panics if the number of columns in `self` and `rhs` are not equal.
    fn add_assign(&mut self, rhs: Row) {
        *self += &rhs
    }
}

impl Mul<f64> for Row {
    type Output = Row;
    /// Performs scalar multiplication on a `Row`, returning a new `Row`.
    ///
    /// Each element in the row is multiplied by the scalar `rhs`.
    ///
    /// # Arguments
    ///
    /// * `rhs` - The `f64` scalar to multiply by.
    ///
    /// # Examples
    ///
    /// ```
    /// use linrow::row::Row;
    ///
    /// let r1 = Row::new(vec![1.0, 2.0, 3.0]);
    /// let r2 = r1 * 2.0;
    /// assert_eq!(r2, Row::new(vec![2.0, 4.0, 6.0]));
    /// ```
    fn mul(mut self, rhs: f64) -> Self::Output {
        self *= rhs;
        self
    }
}

impl MulAssign<f64> for Row {
    /// Performs in-place scalar multiplication on this `Row`.
    ///
    /// Each element in the row is multiplied by the scalar `rhs`.
    ///
    /// # Arguments
    ///
    /// * `rhs` - The `f64` scalar to multiply by.
    ///
    /// # Examples
    ///
    /// ```
    /// use linrow::row::Row;
    ///
    /// let mut r1 = Row::new(vec![1.0, 2.0, 3.0]);
    /// r1 *= 2.0;
    /// assert_eq!(r1, Row::new(vec![2.0, 4.0, 6.0]));
    /// ```
    fn mul_assign(&mut self, rhs: f64) {
        for i in 0..self.row_elems.len() {
            self.row_elems[i] *= rhs;
        }
    }
}

impl SubAssign<&Row> for Row {
    /// Performs in-place subtraction of another `Row` from this `Row`.
    ///
    /// This operation subtracts corresponding elements of the `rhs` row from `self`.
    /// Both rows must have the same number of columns.
    ///
    /// # Arguments
    ///
    /// * `rhs` - A reference to the `Row` to subtract.
    ///
    /// # Panics
    ///
    /// Panics if the number of columns in `self` and `rhs` are not equal.
    ///
    /// # Examples
    ///
    /// ```
    /// use linrow::row::Row;
    ///
    /// let mut r1 = Row::new(vec![5.0, 7.0, 9.0]);
    /// let r2 = Row::new(vec![1.0, 2.0, 3.0]);
    /// r1 -= &r2;
    /// assert_eq!(r1, Row::new(vec![4.0, 5.0, 6.0]));
    /// ```
    fn sub_assign(&mut self, rhs: &Row) {
        for i in 0..self.row_elems.len() {
            self.row_elems[i] -= rhs.row_elems[i];
        }
    }
}

impl SubAssign<Row> for Row {
    /// Performs in-place subtraction of another `Row` (by value) from this `Row`.
    ///
    /// This is a convenience implementation that delegates to `SubAssign<&Row>`.
    ///
    /// # Arguments
    ///
    /// * `rhs` - The `Row` to subtract.
    ///
    /// # Panics
    ///
    /// Panics if the number of columns in `self` and `rhs` are not equal.
    fn sub_assign(&mut self, rhs: Row) {
        *self -= &rhs
    }
}

impl DivAssign<f64> for Row {
    /// Performs in-place scalar division on this `Row`.
    ///
    /// Each element in the row is divided by the scalar `rhs`.
    ///
    /// # Arguments
    ///
    /// * `rhs` - The `f64` scalar to divide by.
    ///
    /// # Panics
    ///
    /// Panics if `rhs` is zero.
    ///
    /// # Examples
    ///
    /// ```
    /// use linrow::row::Row;
    ///
    /// let mut r1 = Row::new(vec![2.0, 4.0, 6.0]);
    /// r1 /= 2.0;
    /// assert_eq!(r1, Row::new(vec![1.0, 2.0, 3.0]));
    /// ```
    fn div_assign(&mut self, rhs: f64) {
        *self *= 1.0 / rhs;
    }
}

impl Display for Row {
    /// Formats the row for display.
    ///
    /// The elements of the row are displayed within square brackets,
    /// separated by commas.
    ///
    /// # Examples
    ///
    /// ```
    /// use linrow::row::Row;
    ///
    /// let row = Row::new(vec![1.0, 2.0, 3.0]);
    /// assert_eq!(format!("{}", row), "[1,2,3]");
    /// ```
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?; // Start the bracket
        for (i, col) in self.row_elems.iter().enumerate() {
            if i > 0 {
                write!(f, ",")?; // Add comma before all but the first element
            }
            write!(f, "{}", col)?; // Write the number directly to the buffer
        }
        write!(f, "]") // End the bracket
    }
}
impl IntoIterator for Row {
    type Item = f64;
    type IntoIter = std::vec::IntoIter<Self::Item>;
    /// Consumes the `Row` and returns an iterator over its `f64` elements.
    ///
    /// This allows a `Row` to be used in `for` loops and other iterator contexts.
    ///
    /// # Examples
    ///
    /// ```
    /// use linrow::row::Row;
    ///
    /// let row = Row::new(vec![1.0, 2.0, 3.0]);
    /// let mut sum = 0.0;
    /// for val in row {
    ///     sum += val;
    /// }
    /// assert_eq!(sum, 6.0);
    /// ```
    fn into_iter(self) -> Self::IntoIter {
        self.row_elems.into_iter()
    }
}
impl PartialEq for Row {
    /// Compares two `Row`s for equality.
    ///
    /// Two rows are considered equal if they have the same number of columns
    /// and all corresponding elements are equal. Floating-point comparison
    /// is done directly, which might lead to precision issues.
    ///
    /// # Arguments
    ///
    /// * `other` - The other `Row` to compare against.
    ///
    /// # Examples
    ///
    /// ```
    /// use linrow::row::Row;
    ///
    /// let r1 = Row::new(vec![1.0, 2.0, 3.0]);
    /// let r2 = Row::new(vec![1.0, 2.0, 3.0]);
    /// let r3 = Row::new(vec![3.0, 2.0, 1.0]);
    ///
    /// assert_eq!(r1, r2);
    /// assert_ne!(r1, r3);
    /// ```
    fn eq(&self, other: &Self) -> bool {
        if self.row_elems.len() != other.row_elems.len() {
            return false;
        }
        // Direct comparison for f64, consider using an epsilon-based comparison for robustness
        // if floating point inaccuracies are a concern.
        self.iter().zip(other.row_elems.iter()).all(|(a, b)| a == b)
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_row_addition() {
        let mut a = Row {
            row_elems: vec![1.0, 2.0, 3.0],
        };
        let b = Row {
            row_elems: vec![4.0, 5.0, 6.0],
        };
        a += &b;
        assert_eq!(a.row_elems, vec![5.0, 7.0, 9.0]);
    }

    #[test]
    fn test_row_subtraction() {
        let mut a = Row {
            row_elems: vec![5.0, 7.0, 9.0],
        };
        let b = Row {
            row_elems: vec![1.0, 2.0, 3.0],
        };
        a -= &b;
        assert_eq!(a.row_elems, vec![4.0, 5.0, 6.0]);
    }

    #[test]
    fn test_row_scalar_multiplication() {
        let mut a = Row {
            row_elems: vec![1.0, 2.0, 3.0],
        };
        a *= 2.0;
        assert_eq!(a.row_elems, vec![2.0, 4.0, 6.0]);
    }

    #[test]
    fn test_row_scalar_division() {
        let mut a = Row {
            row_elems: vec![2.0, 4.0, 6.0],
        };
        a /= 2.0;
        assert_eq!(a.row_elems, vec![1.0, 2.0, 3.0]);
    }

    #[test]
    fn test_row_equality() {
        let a = Row {
            row_elems: vec![1.0, 2.0, 3.0],
        };
        let b = Row {
            row_elems: vec![1.0, 2.0, 3.0],
        };
        let c = Row {
            row_elems: vec![3.0, 2.0, 1.0],
        };
        assert_eq!(a, b);
        assert_ne!(a, c);
    }
}
