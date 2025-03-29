extern crate nalgebra;
use nalgebra::Matrix4;
use nalgebra::Vector2;
use nalgebra::Vector3;
use nalgebra::Vector4;
use std::cmp;

#[derive(Debug, Clone, Copy)]
pub struct Transform {
    transform: Matrix4<f32>,
}

impl Transform {
    pub fn new() -> Self {
        return Self {
            transform: Matrix4::identity(),
        };
    }

    pub fn from_mat(mat: &Matrix4<f32>) -> Self {
        return Self {
            transform: mat.clone(),
        };
    }

    pub fn GetMat(&self) -> &Matrix4<f32> {
        return &self.transform;
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

    pub fn ScaleBy(&mut self, scale: f32) {
        self.transform[(0, 0)] *= scale;
        self.transform[(1, 0)] *= scale;
        self.transform[(2, 0)] *= scale;

        self.transform[(0, 1)] *= scale;
        self.transform[(1, 1)] *= scale;
        self.transform[(2, 1)] *= scale;

        self.transform[(0, 2)] *= scale;
        self.transform[(1, 2)] *= scale;
        self.transform[(2, 2)] *= scale;
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

    pub fn Position(&self) -> Vector4<f32> {
        return Vector4::new(
            self.transform[(0, 3)],
            self.transform[(1, 3)],
            self.transform[(2, 3)],
            0.0,
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
        let position = self.Position();
        let (xs, ys, zs) = self.Scale();
        //self.SetScale(1.0, 1.0, 1.0);
        self.MoveTo(0.5, 0.5, 0.0);
        self.transform =
            Matrix4::from_axis_angle(&Vector3::z_axis(), angle.to_radians()) * self.transform;
        self.MoveTo(position.x, position.y, position.z);
        self.SetScale(xs, ys, zs);
    }
}

/*
 * The rect is a transform where 0,0 is bottom left and 1,1 is top right
 */
#[derive(Debug, Clone, Copy)]
pub struct Rect {
    transform: Transform,
    transform_inv: Matrix4<f32>,
}

impl Rect {
    pub fn FromTransform(transform: &Transform) -> Rect {
        return Self {
            transform: transform.clone(),
            transform_inv: transform.GetMat().try_inverse().expect(""),
        };
    }

    pub fn FromBounds(left: f32, bottom: f32, right: f32, top: f32) -> Rect {
        let mut t = Transform::new();
        t.MoveTo(left, bottom, 0.0);
        t.SetScale(right - left, top - bottom, 1.0);
        return Self::FromTransform(&t);
    }

    /*
     * Returns the bottom left in the parent coordinate
     */
    pub fn TopRight(&self) -> Vector4<f32> {
        let top_right = Vector4::new(1.0, 1.0, 0.0, 1.0);
        return self.transform.GetMat() * top_right;
    }

    pub fn BottomLeft(&self) -> Vector4<f32> {
        return self.transform.Position();
    }

    pub fn GlobalToLocal(&self, p: Vector4<f32>) -> Vector4<f32> {
        return self.transform.GetMat() * p;
    }

    pub fn LocalToGlobal(&self, p: Vector4<f32>) -> Vector4<f32> {
        return self.transform_inv * p;
    }

    pub fn Dims(&self) -> (f32, f32) {
        let width = (self.transform.GetMat() * Vector4::new(1.0, 0.0, 0.0, 0.0)).norm();
        let height = (self.transform.GetMat() * Vector4::new(0.0, 1.0, 0.0, 0.0)).norm();
        return (width, height);
    }

    /*
     * Returns a rect in parent space, IE sibling to the current rect
     * That is the bounding box of this rect. If there is no rotation applied
     * to the rect then this is effectivly just the identity function
     */
    pub fn BoundingRect(&self) -> Rect {
        let left_top = self.transform.GetMat() * Vector4::new(0.0, 1.0, 0.0, 1.0);
        let left_bottom = self.transform.GetMat() * Vector4::new(0.0, 0.0, 0.0, 1.0);
        let right_top = self.transform.GetMat() * Vector4::new(1.0, 1.0, 0.0, 1.0);
        let right_bottom = self.transform.GetMat() * Vector4::new(1.0, 0.0, 0.0, 1.0);

        let left = Self::Min(left_top.x, left_bottom.x, right_bottom.x, right_top.x);
        let right = Self::Max(left_top.x, left_bottom.x, right_bottom.x, right_top.x);
        let bottom = Self::Min(left_top.y, left_bottom.y, right_bottom.y, right_top.y);
        let top = Self::Max(left_top.y, left_bottom.y, right_bottom.y, right_top.y);

        return Self::FromBounds(left, bottom, right, top);
    }

    /*
     * Creates a transform matrix from the rect
     *
     * (0,0): would be the bottom left of the rect
     * (1,1): would be the top right
     */
    pub fn ToTransform(&self) -> Transform {
        return self.transform.clone();
    }

    fn Min(a: f32, b: f32, c: f32, d: f32) -> f32 {
        return a.min(b).min(c).min(d);
    }

    fn Max(a: f32, b: f32, c: f32, d: f32) -> f32 {
        return a.max(b).max(c).max(d);
    }
}
