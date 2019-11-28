// Copyright 2018 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
use std::fmt;
use std::ops::Mul;

use log::error;

use crate::core::geometry::Vector3f;
use crate::core::pbrt::{Float, EPSILON};

/// Solve a 2x2 linear system in the form Ax = B.  For parameters `a` and `b`, the solution to `x`
/// will be returned if any exist.  None will be returned of the answer is numerically unstable or
/// doesn't exist.
///
/// # Examples
/// From http://www-users.math.umn.edu/~weim0024/pdf/37%20-%20Solve2x2.pdf
/// ```
/// # use pbrt::core::transform::solve_linear_system_2x2;
///
/// assert_eq!(solve_linear_system_2x2([[3., -5.],[1., -4.]], [4., -1.]), Some([3., 1.]));
/// assert_eq!(solve_linear_system_2x2([[2., -3.],[0., 4.]], [-8. ,8.]), Some([-1., 2.]));
/// assert_eq!(solve_linear_system_2x2([[5., -1.],[3., 2.]], [3., 20.]), Some([2., 7.]));
/// assert_eq!(solve_linear_system_2x2([[2., -1.],[-4., 2.]], [7., 6.]), None);
/// assert_eq!(solve_linear_system_2x2([[2., -1.],[-2., 1.]], [7., 3.]), None);
/// ```
pub fn solve_linear_system_2x2(a: [[Float; 2]; 2], b: [Float; 2]) -> Option<[Float; 2]> {
    let det = a[0][0] * a[1][1] - a[0][1] * a[1][0];
    if det.abs() < 1e-10 {
        return None;
    }
    let x0 = (a[1][1] * b[0] - a[0][1] * b[1]) / det;
    let x1 = (a[0][0] * b[1] - a[1][0] * b[0]) / det;
    if x0.is_nan() || x1.is_nan() {
        return None;
    }
    Some([x0, x1])
}

#[derive(Default, Clone, Copy)]
/// The matrix m is stored in row-major form, so element m[i][j] corresponds to mi , j , where i is
/// the row number and j is the column number.
pub struct Matrix4x4 {
    m: [[Float; 4]; 4],
}

impl Matrix4x4 {
    /// Create a `Matrix4x4` containing the identity, all zeros with ones along the diagonal.
    pub fn identity() -> Matrix4x4 {
        Matrix4x4::new(
            [1., 0., 0., 0.],
            [0., 1., 0., 0.],
            [0., 0., 1., 0.],
            [0., 0., 0., 1.],
        )
    }

    /// Create a `Matrix4x4` with each of the given rows.
    pub fn new(r0: [Float; 4], r1: [Float; 4], r2: [Float; 4], r3: [Float; 4]) -> Matrix4x4 {
        Matrix4x4 {
            m: [r0, r1, r2, r3],
        }
    }

    /// Transpose self, returning a new matrix that has been reflected across the diagonal.
    pub fn transpose(&self) -> Matrix4x4 {
        let m = self.m;
        Matrix4x4 {
            m: [
                [m[0][0], m[1][0], m[2][0], m[3][0]],
                [m[0][1], m[1][1], m[2][1], m[3][1]],
                [m[0][2], m[1][2], m[2][2], m[3][2]],
                [m[0][3], m[1][3], m[2][3], m[3][3]],
            ],
        }
    }

    /// Returns a new matrix that is the inverse of self. If self is A, inverse returns A^-1, where
    /// AA^-1 = I.
    /// Uses a numerically stable Gauss–Jordan elimination routine to compute the inverse.
    pub fn inverse(&self) -> Matrix4x4 {
        // TODO(wathiede): how come the C++ version doesn't need to deal with non-invertable
        // matrix.
        let mut indxc: [usize; 4] = Default::default();
        let mut indxr: [usize; 4] = Default::default();
        let mut ipiv: [usize; 4] = Default::default();
        let mut minv = self.m;

        for i in 0..4 {
            let mut irow: usize = 0;
            let mut icol: usize = 0;
            let mut big: Float = 0.;
            // Choose pivot
            for j in 0..4 {
                if ipiv[j] != 1 {
                    for (k, ipivk) in ipiv.iter().enumerate() {
                        if *ipivk == 0 {
                            if minv[j][k].abs() >= big {
                                big = minv[j][k].abs();
                                irow = j;
                                icol = k;
                            }
                        } else if *ipivk > 1 {
                            error!("Singular matrix in MatrixInvert");
                        }
                    }
                }
            }
            ipiv[icol] += 1;
            // Swap rows _irow_ and _icol_ for pivot
            if irow != icol {
                // Can't figure out how to make swap work here.
                #[allow(clippy::manual_swap)]
                for k in 0..4 {
                    let tmp = minv[irow][k];
                    minv[irow][k] = minv[icol][k];
                    minv[icol][k] = tmp;
                }
            }
            indxr[i] = irow;
            indxc[i] = icol;
            if minv[icol][icol] == 0. {
                error!("Singular matrix in MatrixInvert");
            }

            // Set $m[icol][icol]$ to one by scaling row _icol_ appropriately
            let pivinv: Float = minv[icol][icol].recip();
            minv[icol][icol] = 1.;
            for j in 0..4 {
                minv[icol][j] *= pivinv;
            }

            // Subtract this row from others to zero out their columns
            for j in 0..4 {
                if j != icol {
                    let save = minv[j][icol];
                    minv[j][icol] = 0.;
                    for k in 0..4 {
                        minv[j][k] -= minv[icol][k] * save;
                    }
                }
            }
        }
        // Swap columns to reflect permutation
        for j in (0..4).rev() {
            if indxr[j] != indxc[j] {
                for mi in &mut minv {
                    mi.swap(indxr[j], indxc[j])
                }
            }
        }
        Matrix4x4 { m: minv }
    }
}

impl fmt::Debug for Matrix4x4 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if f.alternate() {
            write!(
                f,
                "{:?}\n  {:?}\n  {:?}\n  {:?}",
                self.m[0], self.m[1], self.m[2], self.m[3]
            )
        } else {
            write!(
                f,
                "[{:?} {:?} {:?} {:?}]",
                self.m[0], self.m[1], self.m[2], self.m[3]
            )
        }
    }
}

impl Mul<Matrix4x4> for Matrix4x4 {
    type Output = Matrix4x4;
    fn mul(self, m2: Matrix4x4) -> Matrix4x4 {
        let m1 = self;
        let mut r: Matrix4x4 = Default::default();
        for i in 0..4 {
            for j in 0..4 {
                r.m[i][j] = m1.m[i][0] * m2.m[0][j]
                    + m1.m[i][1] * m2.m[1][j]
                    + m1.m[i][2] * m2.m[2][j]
                    + m1.m[i][3] * m2.m[3][j];
            }
        }
        r
    }
}

impl PartialEq for Matrix4x4 {
    fn eq(&self, rhs: &Matrix4x4) -> bool {
        let l = self.m;
        let r = rhs.m;
        for i in 0..4 {
            for j in 0..4 {
                let d = (l[i][j] - r[i][j]).abs();
                if d > EPSILON {
                    return false;
                }
            }
        }
        true
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Transform {
    m: Matrix4x4,
    m_inv: Matrix4x4,
}

impl Transform {
    /// Returns a new transform with m and m_inv set to identity.
    pub fn identity() -> Transform {
        Transform {
            m: Matrix4x4::identity(),
            m_inv: Matrix4x4::identity(),
        }
    }

    pub fn inverse(&self) -> Transform {
        Transform {
            m: self.m_inv,
            m_inv: self.m,
        }
    }

    pub fn translate(delta: &Vector3f) -> Transform {
        let m = Matrix4x4::new(
            [1., 0., 0., delta.x],
            [0., 1., 0., delta.y],
            [0., 0., 1., delta.z],
            [0., 0., 0., 1.],
        );
        let m_inv = Matrix4x4::new(
            [1., 0., 0., -delta.x],
            [0., 1., 0., -delta.y],
            [0., 0., 1., -delta.z],
            [0., 0., 0., 1.],
        );
        Transform { m, m_inv }
    }

    /// rotate generates a Tranform for the rotation of theta (in degrees) about axis.
    pub fn rotate(theta: Float, axis: &Vector3f) -> Transform {
        let a = axis.normalize();
        let sin_theta = theta.to_radians().sin();
        let cos_theta = theta.to_radians().cos();
        let m = Matrix4x4 {
            // Compute rotation of first basis vector
            m: [
                [
                    a.x * a.x + (1. - a.x * a.x) * cos_theta,
                    a.x * a.y * (1. - cos_theta) - a.z * sin_theta,
                    a.x * a.z * (1. - cos_theta) + a.y * sin_theta,
                    0.,
                ],
                // Compute rotations of second and third basis vectors
                [
                    a.x * a.y * (1. - cos_theta) + a.z * sin_theta,
                    a.y * a.y + (1. - a.y * a.y) * cos_theta,
                    a.y * a.z * (1. - cos_theta) - a.x * sin_theta,
                    0.,
                ],
                [
                    a.x * a.z * (1. - cos_theta) - a.y * sin_theta,
                    a.y * a.z * (1. - cos_theta) + a.x * sin_theta,
                    a.z * a.z + (1. - a.z * a.z) * cos_theta,
                    0.,
                ],
                [0., 0., 0., 1.],
            ],
        };
        Transform {
            m,
            m_inv: m.transpose(),
        }
    }

    pub fn scale(sx: Float, sy: Float, sz: Float) -> Transform {
        Transform {
            m: Matrix4x4 {
                m: [
                    [sx, 0., 0., 0.],
                    [0., sy, 0., 0.],
                    [0., 0., sz, 0.],
                    [0., 0., 0., 1.],
                ],
            },
            m_inv: Matrix4x4 {
                m: [
                    [sx.recip(), 0., 0., 0.],
                    [0., sy.recip(), 0., 0.],
                    [0., 0., sz.recip(), 0.],
                    [0., 0., 0., 1.],
                ],
            },
        }
    }

    pub fn matrix(self) -> Matrix4x4 {
        self.m
    }

    pub fn matrix_inverse(self) -> Matrix4x4 {
        self.m_inv
    }
}

impl From<Matrix4x4> for Transform {
    fn from(m: Matrix4x4) -> Transform {
        Transform {
            m,
            m_inv: m.inverse(),
        }
    }
}

impl Mul<Transform> for Transform {
    type Output = Transform;
    fn mul(self, rhs: Transform) -> Transform {
        Transform {
            m: self.m * rhs.m,
            m_inv: self.m_inv * rhs.m_inv,
        }
    }
}
impl<'a, 'b> Mul<&'b mut Transform> for &'a mut Transform {
    type Output = Transform;
    fn mul(self, rhs: &'b mut Transform) -> Transform {
        Transform {
            m: self.m * rhs.m,
            m_inv: self.m_inv * rhs.m_inv,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_4x4_inverse() {
        // Identity
        let i = Matrix4x4::identity();
        assert_eq!(i.inverse() * i, i);

        let m = Matrix4x4::new(
            [2., 0., 0., 0.],
            [0., 3., 0., 0.],
            [0., 0., 4., 0.],
            [0., 0., 0., 1.],
        );
        assert_eq!(m.inverse() * m, i);
        assert_eq!(m * m.inverse(), i);
    }

    #[test]
    fn test_transform_mul() {
        // Test that std::ops::Mul compiles.
        let i: Matrix4x4 = Default::default();
        let m1: Matrix4x4 = Default::default();
        let m2: Matrix4x4 = Default::default();
        assert_eq!(m1 * m2, i);
    }

    #[test]
    fn test_scale() {
        // Test that std::ops::Mul compiles.
        let m1 = Matrix4x4::identity();
        let t = Transform::scale(2., 3., 4.);
        let m2 = Matrix4x4::new(
            [2., 0., 0., 0.],
            [0., 3., 0., 0.],
            [0., 0., 4., 0.],
            [0., 0., 0., 1.],
        );
        assert_eq!(t.m * m1, m2);
    }
}
