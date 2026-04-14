use crate::constants::EPSILON;
use derive_more::{Deref, DerefMut};
use std::fmt::Display;
use std::ops::{AddAssign, DivAssign, Mul, MulAssign, SubAssign};
#[derive(Clone, Debug, Deref, DerefMut)]
pub struct Row {
    pub columns: Vec<f64>,
}
impl Row {
    pub fn new(columns: Vec<f64>) -> Self {
        Row { columns }
    }
}
impl AddAssign<&Row> for Row {
    fn add_assign(&mut self, rhs: &Row) {
        for i in 0..self.columns.len() {
            self.columns[i] += rhs.columns[i];
        }
    }
}
impl AddAssign<Row> for Row {
    fn add_assign(&mut self, rhs: Row) {
        *self += &rhs
    }
}

impl Mul<f64> for Row {
    type Output = Row;
    fn mul(mut self, rhs: f64) -> Self::Output {
        self *= rhs;
        self
    }
}

impl MulAssign<f64> for Row {
    fn mul_assign(&mut self, rhs: f64) {
        for i in 0..self.columns.len() {
            self.columns[i] *= rhs;
        }
    }
}

impl SubAssign<&Row> for Row {
    fn sub_assign(&mut self, rhs: &Row) {
        for i in 0..self.columns.len() {
            self.columns[i] -= rhs.columns[i];
        }
    }
}

impl SubAssign<Row> for Row {
    fn sub_assign(&mut self, rhs: Row) {
        *self -= &rhs
    }
}

impl DivAssign<f64> for Row {
    fn div_assign(&mut self, rhs: f64) {
        *self *= 1.0 / rhs;
    }
}

impl Display for Row {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?; // Start the bracket
        for (i, col) in self.columns.iter().enumerate() {
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
    fn into_iter(self) -> Self::IntoIter {
        self.columns.into_iter()
    }
}
impl PartialEq for Row {
    fn eq(&self, other: &Self) -> bool {
        if self.columns.len() != other.columns.len() {
            return false;
        }
        self.iter().zip(other.columns.iter()).all(|(a, b)| a == b)
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_row_addition() {
        let mut a = Row {
            columns: vec![1.0, 2.0, 3.0],
        };
        let b = Row {
            columns: vec![4.0, 5.0, 6.0],
        };
        a += &b;
        assert_eq!(a.columns, vec![5.0, 7.0, 9.0]);
    }

    #[test]
    fn test_row_subtraction() {
        let mut a = Row {
            columns: vec![5.0, 7.0, 9.0],
        };
        let b = Row {
            columns: vec![1.0, 2.0, 3.0],
        };
        a -= &b;
        assert_eq!(a.columns, vec![4.0, 5.0, 6.0]);
    }

    #[test]
    fn test_row_scalar_multiplication() {
        let mut a = Row {
            columns: vec![1.0, 2.0, 3.0],
        };
        a *= 2.0;
        assert_eq!(a.columns, vec![2.0, 4.0, 6.0]);
    }

    #[test]
    fn test_row_scalar_division() {
        let mut a = Row {
            columns: vec![2.0, 4.0, 6.0],
        };
        a /= 2.0;
        assert_eq!(a.columns, vec![1.0, 2.0, 3.0]);
    }

    #[test]
    fn test_row_equality() {
        let a = Row {
            columns: vec![1.0, 2.0, 3.0],
        };
        let b = Row {
            columns: vec![1.0, 2.0, 3.0],
        };
        let c = Row {
            columns: vec![3.0, 2.0, 1.0],
        };
        assert_eq!(a, b);
        assert_ne!(a, c);
    }
}
