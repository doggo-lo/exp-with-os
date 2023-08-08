mod unit;
use rust_decimal_macros::dec;
use unit::{Accompanyings, Chicks, HeadCount, UnitMaker};

fn main() {
  let exps_cover: Vec<_> = (2..6)
    .map(|x| {
      let count = HeadCount::new(x);
      let unit = UnitMaker::new(count)
        .add(Accompanyings::EMILY_2)
        .fill(Chicks::Cover);
      unit.sum_exp(dec!(10000))
    })
    .collect();
  let exps_uncover: Vec<_> = (2..6)
    .map(|x| {
      let count = HeadCount::new(x);
      let unit = UnitMaker::new(count)
        .add(Accompanyings::EMILY_2)
        .fill(Chicks::Uncover);
      unit.sum_exp(dec!(10000))
    })
    .collect();
  dbg!(exps_cover);
  dbg!(exps_uncover);
}
