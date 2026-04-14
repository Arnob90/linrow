use std::fmt::Display;
use std::ops::{AddAssign, DivAssign, Index, IndexMut, Mul, MulAssign, SubAssign};
#[derive(Clone, Debug)]
pub struct Row {
    pub columns: Vec<f64>,
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
impl Index<usize> for Row {
    type Output = f64;
    fn index(&self, index: usize) -> &Self::Output {
        &self.columns[index]
    }
}
impl IndexMut<usize> for Row {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.columns[index]
    }
}
