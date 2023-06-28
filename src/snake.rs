use crate::direction::Direction;
use crate::point::Point;

#[derive(Debug)]
pub struct Snake {
  body: Vec<point>,
  direction: Direction,
  digesting: bool,
}

impl Snake {
  pub fn new(start: Point, length: u16, direction: Direction) -> Self{
    let opposite = direction.opposite();
    let body: Vec<Point> = (0..length)
      .into_iter() //std::ops::Range::into_iter -> Creates an iterator from a value.
      .map(|i| start.transform(opposite, i))
      .collect();

    Self {body, direction, digesting: false}
  }
}
