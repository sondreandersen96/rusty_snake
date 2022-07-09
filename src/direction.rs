#[derive(Debug)]
#[derive(PartialEq)]
pub enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}
impl Direction {
    pub fn as_str(&self) -> &'static str {
        match self {
            Direction::UP => "UP",
            Direction::DOWN => "DOWN",
            Direction::LEFT => "LEFT",
            Direction::RIGHT => "RIGHT",
        }
    }
}