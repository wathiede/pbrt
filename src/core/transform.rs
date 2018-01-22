use std::ops::Mul;

use core::pbrt::{degrees, radians, Float};
use core::geometry::Vector3f;

#[derive(Debug, Default, Clone, Copy, PartialEq)]
/// The matrix m is stored in row-major form, so element m[i][j] corresponds to mi , j , where i is
/// the row number and j is the column number.
pub struct Matrix4x4 {
    m: [[Float; 4]; 4],
}

impl Matrix4x4 {
    pub fn new() -> Matrix4x4 {
        Matrix4x4::new_with_values(
            [1., 0., 0., 0.],
            [0., 1., 0., 0.],
            [0., 0., 1., 0.],
            [0., 0., 0., 1.],
        )
    }

    pub fn new_with_values(
        r0: [Float; 4],
        r1: [Float; 4],
        r2: [Float; 4],
        r3: [Float; 4],
    ) -> Matrix4x4 {
        Matrix4x4 {
            m: [r0, r1, r2, r3],
        }
    }

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

    pub fn inverse(&self) -> Matrix4x4 {
        unimplemented!("matrix inverse is hard");
    }
}

impl Mul<Matrix4x4> for Matrix4x4 {
    type Output = Matrix4x4;
    fn mul(self, m2: Matrix4x4) -> Matrix4x4 {
        let m1 = self;
        let mut r: Matrix4x4 = Default::default();
        for i in 0..4 {
            for j in 0..4 {
                r.m[i][j] = m1.m[i][0] * m2.m[0][j] + m1.m[i][1] * m2.m[1][j]
                    + m1.m[i][2] * m2.m[2][j] + m1.m[i][3] * m2.m[3][j];
            }
        }
        r
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Transform {
    m: Matrix4x4,
    m_inv: Matrix4x4,
}

impl Transform {
    pub fn new_with_matrix(m: Matrix4x4) -> Transform {
        Transform {
            m,
            m_inv: m.inverse(),
        }
    }

    pub fn inverse(&self) -> Transform {
        Transform {
            m: self.m_inv.clone(),
            m_inv: self.m.clone(),
        }
    }

    pub fn translate(delta: Vector3f) -> Transform {
        let m = Matrix4x4::new_with_values(
            [1., 0., 0., delta.x],
            [0., 1., 0., delta.y],
            [0., 0., 1., delta.z],
            [0., 0., 0., 1.],
        );
        let m_inv = Matrix4x4::new_with_values(
            [1., 0., 0., -delta.x],
            [0., 1., 0., -delta.y],
            [0., 0., 1., -delta.z],
            [0., 0., 0., 1.],
        );
        Transform { m, m_inv }
    }

    /// rotate generates a Tranform for the rotation of theta (in degrees) about axis.
    pub fn rotate(theta: Float, axis: Vector3f) -> Transform {
        let a = axis.normalize();
        let sin_theta = radians(theta).sin();
        let cos_theta = radians(theta).cos();
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
                    [1. / sx, 0., 0., 0.],
                    [0., 1. / sy, 0., 0.],
                    [0., 0., 1. / sz, 0.],
                    [0., 0., 0., 1.],
                ],
            },
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
    fn test_transform_mul() {
        // Test that std::ops::Mul compiles.
        let i = Matrix4x4::new();
        let m1 = Matrix4x4::new();
        let m2 = Matrix4x4::new();
        assert_eq!(m1 * m2, i);
    }

}
