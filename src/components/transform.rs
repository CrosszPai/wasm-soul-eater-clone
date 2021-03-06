use vector2d::Vector2D;
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Tranform {
    pub velocity: Vector2D<f64>,
    pub position: Vector2D<f64>,
}
