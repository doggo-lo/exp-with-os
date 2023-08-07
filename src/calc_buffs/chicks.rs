use thiserror::Error;

pub struct Chicks(pub i32);

#[derive(Error, Debug)]
pub enum ChicksError {
  #[error("A zero is too few. You must set from 2-4.")]
  Zero,
  #[error("It's too large. You must set from 2-4.")]
  TooLarge,
}

impl Chicks {
  pub fn new(count: i32) -> Result<Self, ChicksError> {
    match count {
      2..=4 => Ok(Self(count)),
      wrong => Err(match wrong {
        0 => ChicksError::Zero,
        _ => ChicksError::TooLarge,
      }),
    }
  }
}
