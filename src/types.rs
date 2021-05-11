use crate::{EmptyUniforms, UniformsStorage};

pub type Scalar = f32;
pub type Matrix4 = cgmath::Matrix4<Scalar>;
pub type Vector3 = cgmath::Vector3<Scalar>;
pub type FrameUniforms<'a> = UniformsStorage<'a, [[f32; 4]; 4], EmptyUniforms>;
