#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub struct Point {
  pub x: u16,
  pub y: u16
}

impl Point {
  pub fn new(x: u16, y: u16) -> Self {
    Self { x, y }
  }

  pub fn transform(&self, direction: Direction, times: u16) -> Self {
    let times = times as i16;
    let transformation = match direction {
      Direction::Up => (0, -times),
      Direction::Right => (times, 0),
      Direction::Down => (0, times),
      Direction::Left => (-times, 0),
    };

    // "static" method -> Type::function(...)
    // 표현식이 문으로 바뀌는 경우(새미콜론을 사용하지 않는다면,) 반환 키워드를 완전히 생략.
    Self::new(
        Self::transform_value(self.x, transformation.0),
        Self::transform_value(self.y, transformation.1),
    )
  }


  fn transform_value(value: u16, by: i16) -> u16 {
    //dot notation to call an "instance" method -> instance.method(...)
    if by.is_negative() && by.abs() as u16 > value {
        panic!("Transforming value {} by {} would result in a negative number", value, by);
    } else {
      (value as i16 + by) as u16
    }
  }
}
