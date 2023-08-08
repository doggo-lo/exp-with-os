use rust_decimal::prelude::*;
use rust_decimal_macros::dec;

pub mod Leader {
  use super::*;
  #[derive(Clone)]
  pub struct LeaderBuff(pub Decimal);
  pub const LEADER: LeaderBuff = LeaderBuff(dec!(1.2));
  pub const FORROWER: LeaderBuff = LeaderBuff(dec!(1));
}

pub mod Os {
  use super::*;
  #[derive(Clone)]
  pub struct OsBuff(pub Decimal);
  pub const IMPROVED: OsBuff = OsBuff(dec!(1.275));
  pub const NORMAL: OsBuff = OsBuff(dec!(1.25));
  pub const NOTHING: OsBuff = OsBuff(dec!(1.));
}

pub mod Skill {
  use super::*;
  pub struct SkillBuff(pub Decimal);
  pub const ALEXANDRA: SkillBuff = SkillBuff(dec!(0.4025));
  pub const MIGHTLY_R: SkillBuff = SkillBuff(dec!(0.38));
  pub const TOMY_WORKER_BIO: SkillBuff = SkillBuff(dec!(0.3));
  pub const TOMY_WORKER_AGS: SkillBuff = SkillBuff(dec!(0.6));
  pub const NONE: SkillBuff = SkillBuff(dec!(0.0));
}
