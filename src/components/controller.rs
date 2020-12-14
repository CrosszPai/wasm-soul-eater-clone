#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Controller {
    pub pressed: bool,
    pub key: &'static str,
}