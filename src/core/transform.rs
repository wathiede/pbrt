use core::pbrt::Float;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Matrix4x4 {
    m: [[Float; 4]; 4],
}
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Transform {
    m: Matrix4x4,
    m_inv: Matrix4x4,
}

impl Transform {
    pub fn inverse(&self) -> Transform {
        Transform {
            m: self.m_inv.clone(),
            m_inv: self.m.clone(),
        }
    }
}
