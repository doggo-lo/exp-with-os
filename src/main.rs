mod calc_buffs;

pub use calc_buffs::{Chicks, Unit};

fn main() {
  let exps_cover: Vec<_> = (2..5)
    .map(|x| {
      let chicks = Chicks::new(x).unwrap();
      let unit = Unit::new_with_cover(chicks);
      unit.sum_exp(10000.)
    })
    .collect();
  let exps_uncover: Vec<_> = (2..5)
    .map(|x| {
      let chicks = Chicks::new(x).unwrap();
      let unit = Unit::new_without_cover(chicks);
      unit.sum_exp(10000.)
    })
    .collect();
  dbg!(exps_cover);
  dbg!(exps_uncover);
}
