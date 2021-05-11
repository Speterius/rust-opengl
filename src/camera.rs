use crate::{Scalar, Vector3, Unit, normalize};
use crate::consts::*;


struct ClippingPlanes {
    near: Scalar,
    far: Scalar
}

impl ClippingPlanes {
    pub fn new(near: Scalar, far: Scalar) -> Self {
        Self {near, far}
    }
}

pub struct Camera {
    position: Vector3,
    direction: Unit<Vector3>,
    resolution: (u32, u32),
    fov: u32,
    aspect_ratio: Scalar,
    clipping_planes: ClippingPlanes
}

impl Camera {
    pub fn new(position: Vector3, direction: Vector3, resolution: (u32, u32), fov: u32) -> Self {
        Self {
            position,
            direction: normalize(direction),
            resolution,
            fov,
            aspect_ratio: resolution.0 as Scalar / resolution.1 as Scalar,
            clipping_planes: ClippingPlanes::new(0.1, 1024.0)
        }
    }

    pub fn update_resolution(&mut self, resolution: (u32, u32)) {
        self.resolution = resolution;
        self.aspect_ratio = resolution.0 as Scalar / resolution.1 as Scalar
    }

    /// Builds a frame specific perspective transform
    pub fn get_perspective_matrix(&self) -> [[f32; 4]; 4] {

        let fov = (self.fov as Scalar).to_radians();
        let f = 1.0 / (fov / 2.0).tan();
        let (znear, zfar) = (self.clipping_planes.near, self.clipping_planes.far);

        [
            [f / self.aspect_ratio, 0.0, 0.0, 0.0],
            [0.0, f, 0.0, 0.0],
            [0.0, 0.0, (zfar + znear) / (zfar - znear), 1.0],
            [0.0, 0.0, -(2.0 * zfar * znear) / (zfar - znear), 0.0],
        ]
    }

    /// Builds the frame specific view matrix from a given camera state
    pub fn get_view_matrix(&self) -> [[f32; 4]; 4] {
        let up = UP;
        let s = normalize(up.cross(&self.direction));
        let u = self.direction.cross(&s);

        let p = Vector3::new(
            -self.position.dot(&s),
            -self.position.dot(&u),
            -self.position.dot(&self.direction),
        );

        let f = self.direction;

        [
            [s[0], u[0], f[0], 0.0],
            [s[1], u[1], f[1], 0.0],
            [s[2], u[2], f[2], 0.0],
            [p[0], p[1], p[2], 1.0],
        ]
    }
}
