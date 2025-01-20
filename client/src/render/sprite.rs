extern crate file_manager;
extern crate nalgebra;
use file_manager::handle_t;
use nalgebra::Matrix4;
use nalgebra::Vector3;

pub struct Sprite {
    image: handle_t,
    transform: Matrix4<f32>,
}

impl Sprite {
    pub fn new(image: handle_t) -> Self {
        return Self {
            image: image,
            transform: Matrix4::identity(),
        };
    }

    pub fn Transform(&self) -> &Matrix4<f32> {
        return &self.transform;
    }

    pub fn Texture(&self) -> handle_t {
        return self.image;
    }

    pub fn SetScale(&mut self, x: f32, y: f32, z: f32) {
        let x_mag = self.transform.column(0).xyz().norm();
        let y_mag = self.transform.column(1).xyz().norm();
        let z_mag = self.transform.column(2).xyz().norm();

        self.transform[(0, 0)] *= (x / x_mag);
        self.transform[(1, 0)] *= (x / x_mag);
        self.transform[(2, 0)] *= (x / x_mag);

        self.transform[(0, 1)] *= (y / y_mag);
        self.transform[(1, 1)] *= (y / y_mag);
        self.transform[(2, 1)] *= (y / y_mag);

        self.transform[(0, 2)] *= (z / z_mag);
        self.transform[(1, 2)] *= (z / z_mag);
        self.transform[(2, 2)] *= (z / z_mag);
    }

    pub fn MoveTo(&mut self, x: f32, y: f32, z: f32) {
        self.transform[(0, 3)] = x;
        self.transform[(1, 3)] = y;
        self.transform[(2, 3)] = z;
    }

    pub fn Displace(&mut self, x: f32, y: f32, z: f32) {
        self.transform[(0, 3)] += x;
        self.transform[(1, 3)] += y;
        self.transform[(2, 3)] += z;
    }

    pub fn Position(&self) -> (f32, f32, f32) {
        return (
            self.transform[(0, 3)],
            self.transform[(1, 3)],
            self.transform[(2, 3)],
        );
    }

    pub fn Scale(&self) -> (f32, f32, f32) {
        return (
            self.transform.column(0).xyz().norm(),
            self.transform.column(1).xyz().norm(),
            self.transform.column(2).xyz().norm(),
        );
    }

    pub fn RotateOriginBy(&mut self, angle: f32) {
        let (x, y, z) = self.Position();
        let (xs, ys, zs) = self.Scale();
        //self.SetScale(1.0, 1.0, 1.0);
        self.MoveTo(0.5, 0.5, 0.0);
        self.transform =
            Matrix4::from_axis_angle(&Vector3::z_axis(), angle.to_radians()) * self.transform;
        self.MoveTo(x, y, z);
        self.SetScale(xs, ys, zs);
    }
}
