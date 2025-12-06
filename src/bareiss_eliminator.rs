use std::{
    error::Error, fmt::{self, Display, Formatter}, iter::repeat_n, ops::{
        Index,
        IndexMut,
        Range,
        RangeFrom
    }
};


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnrectangularMatrixError;
impl Display for UnrectangularMatrixError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "The matrix isn't rectangular")
    }
}
impl Error for UnrectangularMatrixError {}


#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BareissSolverError {
    SingularPivot { column: usize },
    RankDeficient { expected: usize, found: usize },
}
impl Display for BareissSolverError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::SingularPivot { column } =>
                write!(f, "No pivot found at column {column}"),
            Self::RankDeficient { expected, found } =>
                write!(f, "Expected {expected} ranks, got {found}"),
        }
    }
}
impl Error for BareissSolverError {}


#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BareissEliminatorError {
    UnrectangularMatrix(UnrectangularMatrixError),
    BareissSolver(BareissSolverError),
}
impl Display for BareissEliminatorError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::UnrectangularMatrix(unrectangular_matrix_error) =>
                write!(f, "{unrectangular_matrix_error}"),
            Self::BareissSolver(bareiss_solver_error) =>
                write!(f, "{bareiss_solver_error}"),
        }
    }
}
impl From<UnrectangularMatrixError> for BareissEliminatorError {
    fn from(value: UnrectangularMatrixError) -> Self {
        Self::UnrectangularMatrix(value)
    }
}
impl From<BareissSolverError> for BareissEliminatorError {
    fn from(value: BareissSolverError) -> Self {
        Self::BareissSolver(value)
    }
}
impl Error for BareissEliminatorError {}


/// A helper macro that only prints with the `debug-print` fature enabled.
#[macro_export]
macro_rules! debug_println {
    ($($arg:tt)*) => {
        #[cfg(feature = "debug-print")]
        {
            println!($($arg)*);
        }
    };
}

#[derive(Debug, PartialEq)]
pub struct RectangularMatrix {
    rows: Box<[Box<[f64]>]>,
}
impl<T> TryFrom<&[T]> for RectangularMatrix
where
    T: AsRef<[f64]>,
{
    type Error = UnrectangularMatrixError;
    fn try_from(value: &[T]) -> Result<Self, UnrectangularMatrixError> {
        if !value[1..].iter().all(|row| row.as_ref().len() == value[0].as_ref().len()) {
            return Err(UnrectangularMatrixError)
        }
        Ok(Self {
            rows: value.iter().map(|row| row.as_ref().into()).collect()
        })
    }
}

type List = Box<[f64]>;
impl RectangularMatrix {
    pub fn switch_dimensions(&self) -> Self {
        Self {
            rows: (0..self.rows[0].len()).map(|index|
                self.rows.iter().map(|row| row[index]).collect()).collect()
        }
    }
    fn len(&self) -> usize {
        self.rows.len()
    }
    fn split_at_mut(&mut self, mid: usize) -> (&mut [List], &mut [List]) {
        self.rows.split_at_mut(mid)
    }
    fn swap(&mut self, a: usize, b: usize) {
        self.rows.swap(a, b);
    }
    fn iter(&self) -> std::slice::Iter<'_, Box<[f64]>> {
        self.rows.iter()
    }
    pub fn bareiss_solve(&mut self) -> Result<Box<[f64]>, BareissSolverError> {
        let size = self[0].len();

        for index in 0..self[0].len() - 1 {
            // debug_println!("Stepped: {:?}", self.rows);

            if self[index][index] == 0.0 {
                match self[index + 1..self.len()].iter().position(|row| row[index] != 0.0) {
                    Some(valid_row_index) => {
                        self.swap(index, valid_row_index + index + 1);
                    }
                    None => {
                        return Err(BareissSolverError::SingularPivot { column: index })
                    }
                }
                // debug_println!("Swapped: {:?}", self.rows);
            }

            let (prev_rows, next_rows) = self.split_at_mut(index + 1);
            let prev_row = &mut prev_rows[index];
            let (prev_left_elements, prev_right_elements) = prev_row.split_at(index + 1);
            let prev_element = prev_left_elements[index];

            for next_row in next_rows {
                let (next_left_elements, next_right_elements) = next_row.split_at_mut(index + 1);
                let next_element = &mut next_left_elements[index];

                for (prev_right_element, next_right_element) in prev_right_elements.iter().zip(next_right_elements) {
                    *next_right_element = prev_element.mul_add(*next_right_element, -(prev_right_element * *next_element));
                }
                *next_element = 0.0;
            }
        };
        let last_index = self.len() - self.iter().rev().position(|row| !row.iter().all(|element| element.abs() < f64::from(f32::EPSILON))).unwrap();
        // debug_println!("Finally: {:?}", self.split_at(last_index).0);
        if size != last_index + 1 {
            return Err(BareissSolverError::RankDeficient { expected: size, found: last_index + 1 })
        }

        let mut solutions: Box<[f64]> = repeat_n(0.0, size - 1).collect();
        for index in (0..size - 1).rev() {
            solutions[index] = (self[index][size - 1] - (index + 1..size - 1).map(|inner_index| self[index][inner_index] * solutions[inner_index]).sum::<f64>()) / self[index][index];
        }

        Ok(solutions)
    }
}
impl Index<usize> for RectangularMatrix {
    type Output = Box<[f64]>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.rows[index]
    }
}
impl Index<Range<usize>> for RectangularMatrix {
    type Output = [Box<[f64]>];
    fn index(&self, index: Range<usize>) -> &Self::Output {
        &self.rows[index]
    }
}
impl Index<RangeFrom<usize>> for RectangularMatrix {
    type Output = [Box<[f64]>];
    fn index(&self, index: RangeFrom<usize>) -> &Self::Output {
        &self.rows[index]
    }
}
impl IndexMut<usize> for RectangularMatrix {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.rows[index]
    }
}


#[cfg(test)]
mod tests {
    use crate::{dim, {
        bareiss_eliminator::*, dimension::DimensionalAnalysable, dimensions::le_systeme_international_d_unites::{JOULE, base_units::{AMPERE, KILOGRAM, METER, SECOND}}
    }};

    fn test_solvable(rows: &[&[f64]], solution: &[f64]) {
        assert_eq!(RectangularMatrix::try_from(rows).expect("Sould be a rectanglar matrix").bareiss_solve().expect("Should be solvable").as_ref(), solution);
    }
    fn test_unsolvable(rows: &[&[f64]], error: BareissSolverError) {
        assert_eq!(RectangularMatrix::try_from(rows).expect("Should be rectanglar matrix").bareiss_solve(), Err(error));
    }
    fn test_unrectangular_matrix(rows: &[&[f64]]) {
        assert_eq!(RectangularMatrix::try_from(rows), Err(UnrectangularMatrixError));
    }

    #[test]
    fn simple_equations() {
        // y = 2.0
        // x = 3.0
        test_solvable(&[
            &[0.0, 1.0, 2.0],
            &[1.0, 0.0, 3.0]
        ], &[3.0, 2.0]);
    }

    #[test]
    fn simple_unsolvable_equation_system() {
        //  x +  y = 3.0
        // 2x + 2y = 5.0
        test_unsolvable(&[
            &[1.0, 1.0, 3.0],
            &[2.0, 2.0, 4.0]
        ], BareissSolverError::SingularPivot { column: 1 });
    }

    #[test]
    fn not_power_symmetric() {
        // x = 2.0
        // 2x = 4.0
        // -4x = -4.0
        // -2x = -4.0
        test_unsolvable(&[
            &[1.0, 2.0],
            &[2.0, 4.0],
            &[-4.0, -4.0],
            &[-2.0, -4.0]
        ], BareissSolverError::RankDeficient { expected: 2, found: 4 });
    }

    #[test]
    fn three_variables() {
        // x + z = 0.0
        // 2x - 3z = 1.0
        // -2x + y = 0.0
        test_solvable(&[
            &[1.0, 0.0, 1.0, 0.0],
            &[2.0, 0.0, -3.0, 1.0],
            &[-2.0, 1.0, 0.0, 0.0]
        ], &[0.2, 0.4, -0.2]);
    }

    #[test]
    fn power_symmetry() {
        // -x = -2.0
        // 2x = 4.0
        // -2x = -4.0
        test_solvable(&[
            &[-1.0, -2.0],
            &[2.0, 4.0],
            &[-2.0, -4.0]
        ], &[2.0]);
    }

    #[test]
    fn identity_matrix() {
        // x = 1.0
        // y = 2.0
        // z = 3.0
        test_solvable(&[
            &[1.0, 0.0, 0.0, 1.0],
            &[0.0, 1.0, 0.0, 2.0],
            &[0.0, 0.0, 1.0, 3.0],
        ], &[1.0, 2.0, 3.0]);
    }

    #[test]
    fn zero_matrix() {
        // 0x + 0y = 0.0
        // 0x + 0y = 0.0
        test_unsolvable(&[
            &[0.0, 0.0, 0.0],
            &[0.0, 0.0, 0.0],
        ], BareissSolverError::SingularPivot { column: 0 });
    }

    #[test]
    fn overdetermined_but_consistent() {
        // x + y = 2.0
        // 2x + 2y = 4.0
        // −3x −2y = −2.0
        test_solvable(&[
            &[1.0, 1.0, 2.0],
            &[2.0, 2.0, 4.0],
            &[-3.0, -2.0, -2.0],
        ], &[-2.0, 4.0]);
    }

    #[test]
    fn underdetermined_system() {
        // x + y + z = 1
        // 2x + 2y + 2z = 2
        test_unsolvable(&[
            &[1.0, 1.0, 1.0, 1.0],
            &[2.0, 2.0, 2.0, 2.0],
        ], BareissSolverError::SingularPivot { column: 1 });
    }

    #[test]
    fn non_rectangle_input() {
        // x + y = 2
        // x = 1 ???
        test_unrectangular_matrix(&[
            &[1.0, 1.0, 2.0],
            &[1.0, 1.0],
        ]);
    }

    #[test]
    fn dimensional_analysis_example() {
        let joule = dim!(JOULE);
        let second = dim!(SECOND);
        let density = dim!(KILOGRAM METER^-3);
        let ampere_per_meter = dim!(AMPERE METER^-1);
        let meter = dim!(METER);
        let rows = [joule, second, density, ampere_per_meter, meter].exponents();
        debug_println!("{:?}", rows);
        let rows_matrix = RectangularMatrix::try_from(rows.as_ref()).expect("Already rectangular");
        debug_println!("{:?}", rows_matrix.rows);
        let mut rows_corrected_matrix = rows_matrix.switch_dimensions();
        debug_println!("{:?}", rows_corrected_matrix.rows);
        assert_eq!(rows_corrected_matrix.rows, [
            [1.0, 0.0, 1.0, 0.0, 0.0].into(), // kilogram exponent
            [2.0, 0.0, -3.0, -1.0, 1.0].into(), // meter exponent
            [-2.0, 1.0, 0.0, 0.0, 0.0].into(), // second exponent
            [0.0, 0.0, 0.0, 1.0, 0.0].into(), // ampere unit
        ].into());
        
        assert_eq!(rows_corrected_matrix.bareiss_solve(), Ok([0.2, 0.4, -0.2, 0.0].into()));
    }
}