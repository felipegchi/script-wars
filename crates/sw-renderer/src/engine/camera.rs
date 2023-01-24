use cgmath::Point2;
use cgmath::Point3;
use cgmath::SquareMatrix;
use cgmath::Vector3;
use specs::Component;
use specs::DenseVecStorage;

#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.0,
    0.0, 0.0, 0.5, 1.0,
);

/// Stores the info about a camera.
#[derive(Component)]
pub struct Camera {
    pub(crate) aspect: f32,
    pub(crate) scale: f32,
    pub(crate) position: cgmath::Point2<f32>,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            aspect: 1.0,
            scale: 0.5,
            position: Point2::new(0.0, 0.0),
        }
    }
}

impl Camera {
    pub(crate) fn build_matrix(&self) -> cgmath::Matrix4<f32> {
        let translation = Vector3::new(self.position.x, self.position.y, 0.0);

        let proj = cgmath::ortho(-1.0 * self.aspect, 1.0 * self.aspect, -1.0, 1.0, -0.1, 1.0)
            * cgmath::Matrix4::from_translation(translation)
            * cgmath::Matrix4::from_scale(self.scale);
        
        OPENGL_TO_WGPU_MATRIX * proj
    }
}
