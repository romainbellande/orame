use rand::Rng;

use super::utils::*;


#[derive(Clone)]
pub enum StarColor {
  Black,
  Yellow,
  Pink,
  Red,
  Blue,
  White,
  Green,
  Purple,
  Random,
}

impl StarColor {
  pub fn rand() -> StarColor {
    vec_pick(STAR_COLORS.to_vec())
  }
}

pub const STAR_COLORS: [StarColor; 9] = [
  StarColor::Black,
  StarColor::Yellow,
  StarColor::Pink,
  StarColor::Red,
  StarColor::Blue,
  StarColor::White,
  StarColor::Green,
  StarColor::Purple,
  StarColor::Random,
];

#[derive(Clone)]
pub struct StarBackground {
  r: u8,
  g: u8,
  b: u8,
  overflow: bool,
}

impl From<[(u8, u8); 3]> for StarBackground {
  fn from(color: [(u8, u8); 3]) -> Self {
    Self {
      r: get_rand_u8(color[0].0, color[0].1),
      g: get_rand_u8(color[1].0, color[1].1),
      b: get_rand_u8(color[2].0, color[2].1),
      overflow: false,
    }
  }
}

impl From<StarColor> for StarBackground {
  fn from(color: StarColor) -> Self {
    match color {
      StarColor::Black =>     Self::from([(0, 10), (0, 10), (0, 10)]),
      StarColor::Yellow =>     Self::from([(230, 255), (230, 255), (0, 20)]),
      StarColor::Pink => Self::from([(230, 255), (0, 20), (230, 255)]),
      StarColor::Red => Self::from([(230, 255), (0, 20), (0, 20)]),
      StarColor::Blue => Self::from([(0, 20), (0, 20), (230, 255)]),
      StarColor::White => Self::from([(230, 255), (230, 255), (230, 255)]),
      StarColor::Green => Self::from([(0, 20), (230, 255), (0, 20)]),
      StarColor::Purple => Self::from([(230, 255), (0, 20), (230, 255)]),
      StarColor::Random => Self::rand(),
    }
  }
}

impl StarBackground {
  pub fn new() -> Self {
    let color = vec_pick(STAR_COLORS.to_vec());
    Self::from(color)
  }

  pub fn rand() -> Self {
    Self {
      r: get_rand_u8(0, 255),
      g: get_rand_u8(0, 240),
      b: get_rand_u8(0, 255),
      overflow: false,
    }
  }

  pub fn set_red(&mut self, value: u8) {
    self.r = value;
  }

  pub fn set_green(&mut self, value: u8) {
    self.g = value;
  }

  pub fn set_blue(&mut self, value: u8) {
    self.b = value;
  }

  pub fn add_red(&mut self, value: u8) {
    let (new_value, overflow) = self.r.overflowing_add(value);
    self.r = new_value;
    self.overflow = overflow
  }

  pub fn add_green(&mut self, value: u8) {
    let (new_value, overflow) = self.g.overflowing_add(value);
    self.g = new_value;
    self.overflow = overflow
  }

  pub fn add_blue(&mut self, value: u8) {
    let (new_value, overflow) = self.b.overflowing_add(value);
    self.b = new_value;
    self.overflow = overflow
  }

  pub fn get_with_overflow(&self, value: u8) -> Self {
    if !self.overflow {
      self.clone()
    } else {
      Self {
        r: value,
        g: value,
        b: value,
        overflow: false,
      }
    }
  }

  pub fn boost(&mut self, value: u8) {
    self.add_red(value);
    self.add_green(value);
    self.add_blue(value);
  }

  pub fn rgba(&self, a: f32) -> String {
    format!("rgba({}, {}, {}, {})", self.r, self.g, self.b, a)
  }
}

impl ToString for StarBackground {
  fn to_string(&self) -> String {
    format!("rgb({}, {}, {})", self.r, self.g, self.b)
  }
}
