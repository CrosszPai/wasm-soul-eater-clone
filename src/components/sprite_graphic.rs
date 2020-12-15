#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SpriteGraphic {
    pub name: &'static str,
    pub mirror: bool,
    pub cell_w: f64,
    pub cell_h: f64,
    pub row: i8,
}
