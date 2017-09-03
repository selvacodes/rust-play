use std::default::Default;
use die::Die;

#[derive(Debug, Default)]
pub struct Dice {
    list: Vec<Die>,
}

impl Dice {
    pub fn with_die_count(count: usize) -> Dice {
        Dice { list: vec![Default::default(); count] }
    }

    pub fn roll(val: Dice) -> Dice {
        Dice { list: val.list.into_iter().map(Die::roll).collect() }
    }

    pub fn value_of(dice_val: &Dice) -> Vec<u8> {
        dice_val
            .list
            .iter()
            .map(|val| Die::value_of(val))
            .collect::<Vec<u8>>()
    }

    pub fn total(dice_val: &Dice) -> u8 {
        Dice::value_of(dice_val).iter().sum()
    }
}