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

//! Linear equation helpers.  Contains a [Matrix4x4] type, [Transform] type (which stores a
//! [Matrix4x4] and its inverse), and helper for solving 2x2 linear systems.
//!
//! [Matrix4x4]: crate::core::transform::Matrix4x4
//! [Transform]: crate::core::transform::Transform
use std::{fmt, ops::Mul};

use log::error;

use crate::{
    core::geometry::{cross, Vector3f},
    float, Degree, Float,
};

/// Solve a 2x2 linear system in the form Ax = B.  For parameters `a` and `b`, the solution to `x`
/// will be returned if any exist.  None will be returned of the answer is numerically unstable or
/// doesn't exist.
///
/// # Examples
/// From http://www-users.math.umn.edu/~weim0024/pdf/37%20-%20Solve2x2.pdf
/// ```
/// # use pbrt::core::transform::solve_linear_system_2x2;
///
/// assert_eq!(
///     solve_linear_system_2x2([[3., -5.], [1., -4.]], [4., -1.]),
///     Some([3., 1.])
/// );
/// assert_eq!(
///     solve_linear_system_2x2([[2., -3.], [0., 4.]], [-8., 8.]),
///     Some([-1., 2.])
/// );
/// assert_eq!(
///     solve_linear_system_2x2([[5., -1.], [3., 2.]], [3., 20.]),
///     Some([2., 7.])
/// );
/// assert_eq!(
///     solve_linear_system_2x2([[2., -1.], [-4., 2.]], [7., 6.]),
///     None
/// );
/// assert_eq!(
///     solve_linear_system_2x2([[2., -1.], [-2., 1.]], [7., 3.]),
///     None
/// );
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
/// Matrix4x4 represents a 4x4 matrix in row-major form. So, element `m[i][j]` corresponds to m<sub>i,j</sub>
/// where `i` is the row number and `j` is the column number.
pub struct Matrix4x4 {
    m: [[Float; 4]; 4],
}

impl From<[Float; 16]> for Matrix4x4 {
    fn from(t: [Float; 16]) -> Self {
        Matrix4x4 {
            m: [
                [t[0], t[1], t[2], t[3]],
                [t[4], t[5], t[6], t[7]],
                [t[8], t[9], t[10], t[11]],
                [t[12], t[13], t[14], t[15]],
            ],
        }
    }
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
    /// # Examples
    ///
    /// ```
    /// use pbrt::core::transform::Matrix4x4;
    ///
    /// let m = Matrix4x4::new(
    ///     [2., 0., 0., 0.],
    ///     [3., 1., 0., 0.],
    ///     [4., 0., 1., 0.],
    ///     [5., 6., 7., 1.],
    /// );
    /// let m_t = Matrix4x4::new(
    ///     [2., 3., 4., 5.],
    ///     [0., 1., 0., 6.],
    ///     [0., 0., 1., 7.],
    ///     [0., 0., 0., 1.],
    /// );
    /// assert_eq!(m.transpose(), m_t);
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

    /// Returns a new matrix that is the inverse of self. If self is A, inverse returns A<sup>-1</sup>, where
    /// AA<sup>-1</sup> = I.
    /// This implementation uses a numerically stable Gauss–Jordan elimination routine to compute the inverse.
    ///
    /// # Examples
    ///
    /// ```
    /// use pbrt::core::transform::Matrix4x4;
    ///
    /// let i = Matrix4x4::identity();
    /// assert_eq!(i.inverse() * i, i);
    ///
    /// let m = Matrix4x4::new(
    ///     [2., 0., 0., 0.],
    ///     [0., 3., 0., 0.],
    ///     [0., 0., 4., 0.],
    ///     [0., 0., 0., 1.],
    /// );
    /// assert_eq!(m.inverse() * m, i);
    /// assert_eq!(m * m.inverse(), i);
    /// ```
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

    /// Implement matrix multiplication for `Matrix4x4`.
    ///
    /// # Examples
    /// ```
    /// use pbrt::core::transform::Matrix4x4;
    ///
    /// let i = Matrix4x4::identity();
    /// let m1 = Matrix4x4::identity();
    /// let m2 = Matrix4x4::identity();
    ///
    /// assert_eq!(m1 * m2, i);
    /// ```
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
                if d > float::EPSILON {
                    return false;
                }
            }
        }
        true
    }
}

/// `Transform` represents a `Matrix4x4` and its inverse.
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Transform {
    m: Matrix4x4,
    m_inv: Matrix4x4,
}

impl Transform {
    /// Returns a new `Transform` set to the identity matrix.
    ///
    /// # Examples
    /// ```
    /// use pbrt::core::transform::{Matrix4x4, Transform};
    ///
    /// assert_eq!(
    ///     Transform::identity(),
    ///     Matrix4x4::new(
    ///         [1., 0., 0., 0.],
    ///         [0., 1., 0., 0.],
    ///         [0., 0., 1., 0.],
    ///         [0., 0., 0., 1.]
    ///     )
    ///     .into()
    /// );
    /// ```
    pub fn identity() -> Transform {
        Transform {
            m: Matrix4x4::identity(),
            m_inv: Matrix4x4::identity(),
        }
    }

    /// Returns the inverse `Transform` of `self`.
    ///
    /// # Examples
    /// ```
    /// use pbrt::core::transform::{Matrix4x4, Transform};
    ///
    /// let t = Transform::identity();
    /// assert_eq!(
    ///     t.inverse(),
    ///     Matrix4x4::new(
    ///         [1., 0., 0., 0.],
    ///         [0., 1., 0., 0.],
    ///         [0., 0., 1., 0.],
    ///         [0., 0., 0., 1.]
    ///     )
    ///     .into()
    /// );
    /// ```
    pub fn inverse(&self) -> Transform {
        Transform {
            m: self.m_inv,
            m_inv: self.m,
        }
    }

    /// Creates a `Transform` representing the given translate factors.
    ///
    /// # Examples
    /// ```
    /// use pbrt::core::transform::{Matrix4x4, Transform};
    ///
    /// assert_eq!(
    ///     Transform::translate([2., 4., 6.]),
    ///     Matrix4x4::new(
    ///         [1., 0., 0., 2.],
    ///         [0., 1., 0., 4.],
    ///         [0., 0., 1., 6.],
    ///         [0., 0., 0., 1.]
    ///     )
    ///     .into()
    /// );
    /// ```
    pub fn translate<V>(delta: V) -> Transform
    where
        V: Into<Vector3f>,
    {
        let delta = delta.into();
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

    /// Creates a `Transform` representing a rotation of `theta` about `axis`.
    /// # Examples
    /// ```
    /// use pbrt::{
    ///     core::transform::{Matrix4x4, Transform},
    ///     Degree, Float,
    /// };
    ///
    /// let t_deg: Float = 180.;
    /// let t_rad = t_deg.to_radians();
    /// let s = t_rad.sin();
    /// let c = t_rad.cos();
    ///
    /// // Rotate about the x-axis.
    /// assert_eq!(
    ///     Transform::rotate(t_deg.into(), [1., 0., 0.]),
    ///     Matrix4x4::new(
    ///         [1., 0., 0., 0.],
    ///         [0., c, -s, 0.],
    ///         [0., s, c, 0.],
    ///         [0., 0., 0., 1.]
    ///     )
    ///     .into()
    /// );
    ///
    /// // Rotate about the y-axis.
    /// assert_eq!(
    ///     Transform::rotate(t_deg.into(), [0., 1., 0.]),
    ///     Matrix4x4::new(
    ///         [c, 0., s, 0.],
    ///         [0., 1., 0., 0.],
    ///         [-s, 0., c, 0.],
    ///         [0., 0., 0., 1.]
    ///     )
    ///     .into()
    /// );
    ///
    /// // Rotate about the z-axis.
    /// assert_eq!(
    ///     Transform::rotate(t_deg.into(), [0., 0., 1.]),
    ///     Matrix4x4::new(
    ///         [c, -s, 0., 0.],
    ///         [s, c, 0., 0.],
    ///         [0., 0., 1., 0.],
    ///         [0., 0., 0., 1.]
    ///     )
    ///     .into()
    /// );
    /// ```
    pub fn rotate<V>(theta: Degree, axis: V) -> Transform
    where
        V: Into<Vector3f>,
    {
        let axis = axis.into();
        let a = axis.normalize();
        let sin_theta = theta.0.to_radians().sin();
        let cos_theta = theta.0.to_radians().cos();
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

    /// Create a `Transform` based on the camera position (`pos`), the position of the object in
    /// focus (`look`) and an up vector (`up).
    pub fn look_at<V>(pos: V, look: V, up: V) -> Transform
    where
        V: Into<Vector3f>,
    {
        let pos = pos.into();
        let look = look.into();
        let up = up.into();

        let mut camera_to_world = Matrix4x4::identity();
        // Set 4th column of viewing matrix.
        camera_to_world.m[0][3] = pos.x;
        camera_to_world.m[1][3] = pos.y;
        camera_to_world.m[2][3] = pos.z;
        camera_to_world.m[3][3] = 1.;

        let dir = (look - pos).normalize();
        let right = cross(up.normalize(), dir).normalize();
        let new_up = cross(dir, right);
        camera_to_world.m[0][0] = right.x;
        camera_to_world.m[1][0] = right.y;
        camera_to_world.m[2][0] = right.z;
        camera_to_world.m[3][0] = 0.;
        camera_to_world.m[0][1] = new_up.x;
        camera_to_world.m[1][1] = new_up.y;
        camera_to_world.m[2][1] = new_up.z;
        camera_to_world.m[3][1] = 0.;
        camera_to_world.m[0][2] = dir.x;
        camera_to_world.m[1][2] = dir.y;
        camera_to_world.m[2][2] = dir.z;
        camera_to_world.m[3][2] = 0.;

        Transform {
            m: camera_to_world.inverse(),
            m_inv: camera_to_world,
        }
    }

    /// Creates a `Transform` representing the given scale factors.
    ///
    /// # Examples
    /// ```
    /// use pbrt::core::transform::{Matrix4x4, Transform};
    ///
    /// assert_eq!(
    ///     Transform::scale(2., 4., 6.),
    ///     Matrix4x4::new(
    ///         [2., 0., 0., 0.],
    ///         [0., 4., 0., 0.],
    ///         [0., 0., 6., 0.],
    ///         [0., 0., 0., 1.]
    ///     )
    ///     .into()
    /// );
    /// ```
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

    /// Returns the internal `Matrix4x4` of `self`.
    ///
    /// # Examples
    /// ```
    /// use pbrt::core::transform::{Matrix4x4, Transform};
    ///
    /// let t = Transform::identity();
    /// assert_eq!(
    ///     t.matrix(),
    ///     Matrix4x4::new(
    ///         [1., 0., 0., 0.],
    ///         [0., 1., 0., 0.],
    ///         [0., 0., 1., 0.],
    ///         [0., 0., 0., 1.]
    ///     )
    /// );
    /// ```
    pub fn matrix(self) -> Matrix4x4 {
        self.m
    }

    /// Returns the internal inverse `Matrix4x4` of `self`.
    ///
    /// # Examples
    /// ```
    /// use pbrt::core::transform::{Matrix4x4, Transform};
    ///
    /// let t = Transform::identity();
    /// assert_eq!(
    ///     t.matrix_inverse(),
    ///     Matrix4x4::new(
    ///         [1., 0., 0., 0.],
    ///         [0., 1., 0., 0.],
    ///         [0., 0., 1., 0.],
    ///         [0., 0., 0., 1.]
    ///     )
    /// );
    /// ```
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

impl From<[Float; 16]> for Transform {
    fn from(t: [Float; 16]) -> Self {
        Matrix4x4::from(t).into()
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
