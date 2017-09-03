extern crate rand;
mod die;
mod dice;

use die::Die;
use dice::Dice;


fn main() {
    let dic: Die = Die::roll(Default::default());
    let dices: Dice = Dice::roll(Dice::with_die_count(5));
    println!(
        "{:?} {:?} {:?}",
        Die::value_of(&dic),
        Dice::value_of(&dices),
        Dice::total(&dices)
    );
}
