mod buffs;
mod chicks;
use buffs::{Buff, LeaderBuff, OsBuff};
pub use chicks::{Chicks, ChicksError};

pub struct BuffBank {
  leader: LeaderBuff,
  os: OsBuff,
}

fn mul_floored(target: f64, scale: f64) -> f64 {
  let result = target * scale;
  result.floor()
}

impl BuffBank {
  fn get_exp(&self, exp: f64) -> f64 {
    let buffs: [&(dyn Buff); 2] = [&self.leader, &self.os];
    buffs
      .iter()
      .fold(exp, |sum, buff| mul_floored(sum, buff.get()))
  }
}

fn sum_buff_all(banks: Vec<BuffBank>, exp: f64) -> f64 {
  banks.iter().map(|bank| bank.get_exp(exp)).sum()
}

pub struct Unit {
  chicks: Vec<BuffBank>,
}

fn make_with_default(mut base: Vec<BuffBank>, count: i32) -> Unit {
  let defaults: Vec<BuffBank> = (0..count)
    .map(|_| BuffBank {
      leader: LeaderBuff::Forrower,
      os: OsBuff::Normal,
    })
    .collect();
  base.extend(defaults);
  Unit { chicks: base }
}

impl Unit {
  // this supports os buff and leader buff covering, only.
  pub fn new_with_cover(chicks: Chicks) -> Self {
    let leader = BuffBank {
      leader: LeaderBuff::Leader,
      os: OsBuff::Improved,
    };
    let base = vec![leader];
    let count = chicks.0 - 1;
    make_with_default(base, count)
  }
  pub fn new_without_cover(chicks: Chicks) -> Self {
    let leader = BuffBank {
      leader: LeaderBuff::Leader,
      os: OsBuff::Normal,
    };
    let improved = BuffBank {
      leader: LeaderBuff::Forrower,
      os: OsBuff::Improved,
    };
    let base = vec![leader, improved];
    let count = chicks.0 - 2;
    make_with_default(base, count)
  }
  pub fn sum_exp(self, base: f64) -> f64 {
    sum_buff_all(self.chicks, base)
  }
}

#[cfg(test)]
mod unittest {
  use super::*;
  use rstest::*;

  #[fixture]
  fn chicks(#[default(4)] count: i32) -> Chicks {
    let chicks = Chicks::new(count);
    match chicks {
      Ok(chicks) => chicks,
      Err(e) => {
        panic!("error occurred in fixture. error :{}", e);
      }
    }
  }

  #[rstest]
  fn buffed_exp() {
    let bank = BuffBank {
      leader: LeaderBuff::Leader,
      os: OsBuff::Normal,
    };
    let exp = bank.get_exp(100.);
    // 100*1.25=125, 125*1.2=150
    let expected = 150.;
    assert_eq!(exp, expected);
  }

  #[rstest]
  fn new_with_cover(chicks: Chicks) {
    let unit = Unit::new_with_cover(chicks);
    let exp = unit.sum_exp(100.);
    // 100*1.2=120, 120*1.275=153, 125*3=375, 153+375=528
    let expected = 528.;
    assert_eq!(exp, expected);
  }

  #[rstest]
  fn new_without_cover(chicks: Chicks) {
    let unit = Unit::new_without_cover(chicks);
    let exp = unit.sum_exp(100.);
    // 100*1.25=125, 125*1.2=150, 100*1.275=127.5, 125*2=250
    // 150+127+250=527
    let expected = 527.;
    assert_eq!(exp, expected);
  }

  #[rstest]
  fn new_without_cover_short(#[with(2)] chicks: Chicks) {
    let unit = Unit::new_without_cover(chicks);
    let exp = unit.sum_exp(100.);
    // 100*1.25=125, 125*1.2=150, 100*1.275=127.5
    // 150+127=277
    let expected = 277.;
    assert_eq!(exp, expected);
  }
}
