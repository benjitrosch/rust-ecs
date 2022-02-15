use crate::vector2::Vector2;
use crate::component::Component;

#[derive(Copy, Clone)]
pub struct Rigidbody {
    pub vel: Vector2
}

impl Component for Rigidbody {}
  