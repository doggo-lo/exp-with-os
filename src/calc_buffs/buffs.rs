pub trait Buff {
  fn get(&self) -> f64;
}

pub enum LeaderBuff {
  Leader,
  Forrower,
}

impl Buff for LeaderBuff {
  fn get(&self) -> f64 {
    match self {
      Self::Leader => 1.2,
      Self::Forrower => 1.0,
    }
  }
}

pub enum OsBuff {
  Improved,
  Normal,
  Nothing,
}

impl Buff for OsBuff {
  fn get(&self) -> f64 {
    match self {
      Self::Improved => 1.275,
      Self::Normal => 1.25,
      Self::Nothing => 1.0,
    }
  }
}
