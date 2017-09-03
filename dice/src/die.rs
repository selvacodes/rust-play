extern crate rand;

use rand::Rng;
use std::default::Default;

#[derive(Debug, Clone)]
pub struct Die {
    value: u8,
}

impl Die {
    pub fn roll(_: Die) -> Die {
        Die { value: rand::thread_rng().gen_range(1, 6) }
    }

    pub fn value_of(die_val: &Die) -> u8 {
        die_val.value
    }
}

impl Default for Die {
    fn default() -> Self {
        Die { value: 0 }
    }
}
