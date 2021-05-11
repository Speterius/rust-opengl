use crate::teapot::TEAPOT;
use crate::{Display, IndexBuffer, Normal, Vertex, VertexBuffer, Vector3, Scalar};


pub struct Transform {
    position: Vector3,
    scale: Scalar
}

pub struct SceneObject {
    pub vertex_bfr: VertexBuffer<Vertex>,
    pub normal_bfr: VertexBuffer<Normal>,
    pub ind_bfr: IndexBuffer<u16>,
    transform: Transform
}

impl SceneObject {
    pub fn new(display: &Display, vert: &[Vertex], norm: &[Normal], ind: &[u16]) -> Self {
        Self {
            vertex_bfr: VertexBuffer::new(display, vert).expect("Couldn't allocate Vertex Buffer."),
            normal_bfr: VertexBuffer::new(display, norm)
                .expect("Couldn't allocate Vertex Buffer for normals."),
            ind_bfr: IndexBuffer::new(display, glium::index::PrimitiveType::TrianglesList, ind)
                .expect("Couldn't allocate Index buffer"),
            transform: Transform {position: Vector3::zeros(), scale: 1.0}
        }
    }

    pub fn set_transform(&mut self, transform: Transform) {
        self.transform = transform;
    }

    pub fn set_position(&mut self, position: Vector3) {
        self.transform.position = position;
    }

    pub fn set_scale(&mut self, scale: Scalar) {
        self.transform.scale = scale;
    }

    pub fn teapot(display: &Display) -> Self {
        let mut teapot = SceneObject::new(display, &TEAPOT.0, &TEAPOT.1, &TEAPOT.2);
        teapot.set_scale(0.01);
        teapot
    }

    pub fn get_model_matrix(&self) -> [[f32; 4]; 4] {
        [
            [self.transform.scale, 0.0, 0.0, 0.0],
            [0.0, self.transform.scale, 0.0, 0.0],
            [0.0, 0.0, self.transform.scale, 0.0],
            [0.0, 0.0, 0.0, 1.0f32],
        ]
    }
}
