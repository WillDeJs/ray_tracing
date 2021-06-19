use crate::grfx::vector::Vec3D;

#[derive(Debug,Default)]
#[allow(non_snake_case)]
pub struct Ray {
    A : Vec3D,
    B : Vec3D,
}

impl Ray {
    pub fn new(a : Vec3D, b: Vec3D) -> Self {
        Self{A: a, B: b}
    }
    pub fn point_at(&self, t : f32) -> Vec3D {
        self.A + self.B * t
    }
    pub fn origin(&self) -> Vec3D {
        self.A
    }
    pub fn direction(&self) -> Vec3D {
        self.B
    }
}