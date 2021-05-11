use crate::implement_vertex;

pub type Scalar = f32;
pub type Matrix4 = nalgebra::Matrix4<Scalar>;
pub type Vector3 = nalgebra::Vector3<Scalar>;

#[derive(Copy, Clone)]
pub struct Color {
    pub r: Scalar,
    pub g: Scalar,
    pub b: Scalar,
}

impl Color {
    pub const fn new(r: Scalar, g: Scalar, b: Scalar) -> Self {
        Self {r, g, b}
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    pub position: (Scalar, Scalar, Scalar),
}

implement_vertex!(Vertex, position);

#[derive(Copy, Clone, Debug)]
pub struct Normal {
    pub normal: (Scalar, Scalar, Scalar),
}

implement_vertex!(Normal, normal);
