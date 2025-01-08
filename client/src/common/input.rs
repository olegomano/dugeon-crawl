extern crate nalgebra;

pub enum Action {
    Move(nalgebra::Vector4<f32>),
    Zoom(f32),
    Quit(),
    None(),
}
