use nalgebra as na;
use nalgebra::base::{DMatrix, DMatrixSlice, DVector};

pub fn skew_symmetric<N: na::Real>(wx: N, wy: N, wz: N) -> na::MatrixN<N, na::U3> {
    na::Matrix3::new(N::zero(), -wz, wy, wz, N::zero(), -wx, -wy, wx, N::zero())
}

pub fn skew_symmetric_v<N: na::Real>(v: &na::Vector3<N>) -> na::MatrixN<N, na::U3> {
    na::Matrix3::new(
        N::zero(),
        -v.z,
        v.y,
        v.z,
        N::zero(),
        -v.x,
        -v.y,
        v.x,
        N::zero(),
    )
}

pub struct SymmetricBlockMatrix<T: na::Real + core::fmt::Debug = f64> {
    matrix: na::MatrixMN<T, na::Dynamic, na::Dynamic>,
    variable_col_offsets: Vec<usize>,
}

impl<T: na::Real + core::fmt::Debug> SymmetricBlockMatrix<T> {
    pub fn new() -> SymmetricBlockMatrix<T> {
        SymmetricBlockMatrix {
            matrix: na::DMatrix::identity(0, 0),
            variable_col_offsets: Vec::new(),
        }
    }

    fn fill_offsets(&mut self, dims: &Vec<usize>, append_one_dim: bool) {
        self.variable_col_offsets
            .resize(dims.len() + 1 + append_one_dim as usize, 0);
        self.variable_col_offsets[0] = 0;

        let mut j: usize = 0;

        for dim in dims {
            self.variable_col_offsets[j + 1] = self.variable_col_offsets[j] + dim;
            j += 1;
        }

        if append_one_dim {
            self.variable_col_offsets[j + 1] = self.variable_col_offsets[j] + 1;
        }
    }

    pub fn from_dimensions(dims: &Vec<usize>) -> SymmetricBlockMatrix<T> {
        let mut mat = SymmetricBlockMatrix {
            matrix: na::DMatrix::identity(0, 0),
            variable_col_offsets: Vec::new(),
        };

        mat.fill_offsets(dims, false);

        let d = *mat.variable_col_offsets.last().unwrap();

        mat.matrix = DMatrix::identity(d, d);

        mat
    }

    pub fn num_blocks(&self) -> usize {
        self.variable_col_offsets.len() - 1
    }

    pub fn offset(&self, block: usize) -> usize {
        self.variable_col_offsets[block]
    }

    pub fn calc_indices(
        &self,
        i: usize,
        j: usize,
        rows: usize,
        cols: usize,
    ) -> (usize, usize, usize, usize) {
        let di = self.offset(i);
        let dj = self.offset(j);
        let dr = self.offset(i + rows) - di;
        let dc = self.offset(j + cols) - dj;

        (di, dj, dr, dc)
    }

    pub fn block_(&self, i: usize, j: usize, rows: usize, cols: usize) -> DMatrixSlice<T> {
        let ind = self.calc_indices(i, j, rows, cols);

        self.matrix.slice((ind.0, ind.1), (ind.2, ind.3))
    }

    pub fn diagonal(&self, j: usize) -> DVector<T> {
        self.block_(j, j, 1, 1).diagonal()
    }
}

impl<T: na::Real> core::fmt::Debug for SymmetricBlockMatrix<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(
            f,
            "SymmetricBlockMatrix:\n{:}\n{:?}",
            self.matrix, self.variable_col_offsets
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn symmetric_block_matrix_new() {
        let s = SymmetricBlockMatrix::<f64>::new();
        println!("{:?}", s);
    }

    #[test]
    fn symmetric_block_matrix_from_dims() {
        let v = vec![3, 3, 2, 1];
        let s = SymmetricBlockMatrix::<f64>::from_dimensions(&v);
        println!("{:?}", s);
    }

    #[test]
    fn symmetric_block_matrix_get_indices() {
        let v = vec![3, 3, 2, 1];
        let s = SymmetricBlockMatrix::<f64>::from_dimensions(&v);

        let ind = s.calc_indices(1, 2, 1, 1);

        assert_eq!(ind, (3, 6, 3, 2));
    }
}
